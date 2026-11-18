//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::records::{EntityType, ExerciseIdentifier, Timestamp};
use crate::siso::enums::{
    DISLiveEntitySubprotocolNumber, DeadReckoningAlgorithm, PduType, ProtocolFamily,
    ProtocolVersion,
};

macro_rules! fixed_binary {
    ($name:ident, $raw:ty, $bits:literal) => {
            #[doc = concat!(
                "A ", stringify!($bits), "-bit signed fixed binary number with `FRAC` fractional bits.\n\n",
                "On the wire it's a two's complement `", stringify!($raw), "`; its value is that integer ",
                "divided by 2^`FRAC`. Each Live Entity field defines its own `FRAC` and units, so use the ",
                "type alias for the field rather than choosing `FRAC` yourself."
            )]
            #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, deku::DekuRead, deku::DekuWrite)]
            #[deku(
                endian = "endian",
                ctx = "endian: deku::ctx::Endian",
                ctx_default = "deku::ctx::Endian::Big"
            )]
            pub struct $name<const FRAC: u32> {
                raw: $raw,
            }

            impl<const FRAC: u32> $name<FRAC> {
                /// Fails to compile when `FRAC` leaves no room for the sign bit.
                const VALID: () = assert!(FRAC < $bits, "FRAC must be less than the number of bits");

                #[allow(clippy::as_conversions, reason = "u32 to f64 is exact, and `From` isn't const")]
                const SCALE: f64 = (1u32 << FRAC) as f64;

                /// The difference between two adjacent values: 2^-`FRAC`.
                pub const RESOLUTION: f64 = 1.0 / Self::SCALE;

                /// The smallest value the type can hold.
                #[allow(clippy::as_conversions, reason = "integer to f64 is exact, and `From` isn't const")]
                pub const MIN: f64 = <$raw>::MIN as f64 / Self::SCALE;

                /// The largest value the type can hold.
                #[allow(clippy::as_conversions, reason = "integer to f64 is exact, and `From` isn't const")]
                pub const MAX: f64 = <$raw>::MAX as f64 / Self::SCALE;

                /// The value nearest to `value`, or `None` if `value` is out of range or isn't finite.
                #[must_use]
                pub fn new(value: f64) -> Option<Self> {
                    let () = Self::VALID;
                    let scaled = (value * Self::SCALE).round();
                    (f64::from(<$raw>::MIN)..=f64::from(<$raw>::MAX))
                        .contains(&scaled)
                        .then(|| Self::from_scaled(scaled))
                }

                /// The value nearest to `value`, clamped to [`MIN`](Self::MIN) and
                /// [`MAX`](Self::MAX). NaN becomes 0.
                #[must_use]
                pub fn saturating(value: f64) -> Self {
                    let () = Self::VALID;
                    Self::from_scaled((value * Self::SCALE).round())
                }

                /// The number's value.
                #[must_use]
                pub fn to_f64(self) -> f64 {
                    f64::from(self.raw) / Self::SCALE
                }

                /// A number from its wire value.
                #[must_use]
                pub const fn from_raw(raw: $raw) -> Self {
                    Self { raw }
                }

                /// The number's wire value.
                #[must_use]
                pub const fn to_raw(self) -> $raw {
                    self.raw
                }

                #[allow(
                    clippy::as_conversions,
                    clippy::cast_possible_truncation,
                    reason = "float-to-integer `as` saturates and maps NaN to 0, which is the behavior wanted"
                )]
                const fn from_scaled(scaled: f64) -> Self {
                    Self { raw: scaled as $raw }
                }
            }
        };
}

fixed_binary!(FixedBinary8, i8, 8);
fixed_binary!(FixedBinary16, i16, 16);

/// The 12-octet header of a Live Entity Information/Interaction PDU. It has a
/// subprotocol number in octet 10, where other PDUs have their PDU status record.
#[derive(Copy, Clone, Debug, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveEntityPduHeader {
    /// The version of the protocol
    pub protocol_version: ProtocolVersion,
    /// Exercise ID
    pub exercise_id: ExerciseIdentifier,
    /// Type of PDU set from the PDU when it is encoded. Read it with
    /// [`pdu_type`](Sef::pdu_type).
    pub(crate) pdu_type: PduType,
    /// Protocol family set from the PDU when it is encoded. Read it with
    /// [`protocol_family`](Sef::protocol_family).
    pub(crate) protocol_family: ProtocolFamily,
    /// When the data in the PDU was valid. New headers use [`Timestamp::now`].
    pub timestamp: Timestamp,
    /// Length of the PDU in octets, including the header. This value is set
    /// upon encoding. Read it with [`length`](Sef::length).
    pub(crate) length: u16,
    /// The subprotocol to be used to decode the PDU
    pub subprotocol_number: DISLiveEntitySubprotocolNumber,
    /// Padded so that this header maintains the same length as the PDU Header
    padding: u8,
}

impl Default for LiveEntityPduHeader {
    fn default() -> Self {
        Self {
            protocol_version: ProtocolVersion::IEEE1278_1_2012,
            exercise_id: ExerciseIdentifier::new(1),
            pdu_type: PduType::default(),
            protocol_family: ProtocolFamily::default(),
            timestamp: Timestamp::now(),
            length: 0,
            subprotocol_number: DISLiveEntitySubprotocolNumber::default(),
            padding: 0,
        }
    }
}

impl LiveEntityPduHeader {
    /// The PDU type, set from the PDU when it is encoded.
    #[must_use]
    pub const fn pdu_type(&self) -> PduType {
        self.pdu_type
    }

    /// The protocol family, set from the PDU when it is encoded.
    #[must_use]
    pub const fn protocol_family(&self) -> ProtocolFamily {
        self.protocol_family
    }

    /// The PDU's length in octets, header included, set when it is encoded.
    #[must_use]
    pub const fn length(&self) -> u16 {
        self.length
    }

    /// Encodes the header alone and appends it to `buf`
    pub fn serialize(&self, buf: &mut Vec<u8>) {
        let _ = crate::common::wire::write(self, buf);
    }

    /// Decodes a header from the front of `buf`, advancing `buf` past it. Pass it to
    /// [`Pdu::deserialize_without_header`](crate::common::Pdu::deserialize_without_header)
    /// to decode the rest of the PDU.
    ///
    /// # Errors
    ///
    /// Will return a `DekuError` if `buf` does not start with a valid header.
    pub fn deserialize(buf: &mut &[u8]) -> Result<Self, crate::common::DISError> {
        crate::common::wire::read(buf).map_err(crate::common::DISError::from_wire)
    }
}

impl crate::common::pdu::DerivedFields for LiveEntityPduHeader {
    #[cfg(test)]
    fn length(&self) -> u16 {
        self.length
    }

    fn set_pdu_type(&mut self, value: PduType) {
        self.pdu_type = value;
    }

    fn set_protocol_family(&mut self, value: ProtocolFamily) {
        self.protocol_family = value;
    }

    fn set_length(&mut self, value: u16) {
        self.length = value;
    }
}

/// Dead reckoning parameters in the compact Live Entity form.
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LEDeadReckoningParameters {
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub entity_linear_acceleration: LEEntityLinearAcceleration,
    pub entity_angular_velocity: LEEntityAngularVelocity,
}

impl LEDeadReckoningParameters {
    #[must_use]
    pub const fn new(
        dead_reckoning_algorithm: DeadReckoningAlgorithm,
        entity_linear_acceleration: LEEntityLinearAcceleration,
        entity_angular_velocity: LEEntityAngularVelocity,
    ) -> Self {
        Self {
            dead_reckoning_algorithm,
            entity_linear_acceleration,
            entity_angular_velocity,
        }
    }
}

/// One component of [`LEEntityAngularVelocity`].
pub type LEAngularVelocityComponent = FixedBinary8<0>;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.7
/// For all fields, assume right-hand rule for directionality
pub struct LEEntityAngularVelocity {
    /// The angular velocity in radians/second about the entity's X-axis
    pub rate_about_x_axis: LEAngularVelocityComponent,
    /// The angular velocity in radians/second about the entity's Y-axis
    pub rate_about_y_axis: LEAngularVelocityComponent,
    /// The angular velocity in radians/second about the entity's Z-axis
    pub rate_about_z_axis: LEAngularVelocityComponent,
}

impl LEEntityAngularVelocity {
    /// Create a new `AngularVelocity` struct with existing values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.28
pub struct LEEntityId {
    /// The unique identification number of the site
    pub site_number: u8,
    /// The unique identification number of the application
    pub application_number: u8,
    /// The unique identification number of the entity
    pub entity_number: u16,
}

impl LEEntityId {
    #[must_use]
    pub const fn new(site_number: u8, application_number: u8, entity_number: u16) -> Self {
        Self {
            site_number,
            application_number,
            entity_number,
        }
    }
}

/// One component of [`LEEntityLinearAcceleration`].
pub type LELinearAccelerationComponent = FixedBinary8<0>;

/// An entity's linear acceleration in the compact Live Entity form.
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LEEntityLinearAcceleration {
    pub first_vector_component: LELinearAccelerationComponent,
    pub second_vector_component: LELinearAccelerationComponent,
    pub third_vector_component: LELinearAccelerationComponent,
}

impl LEEntityLinearAcceleration {
    #[must_use]
    pub const fn new(
        x: LELinearAccelerationComponent,
        y: LELinearAccelerationComponent,
        z: LELinearAccelerationComponent,
    ) -> Self {
        Self {
            first_vector_component: x,
            second_vector_component: y,
            third_vector_component: z,
        }
    }
}

/// One angle of [`LEEulerAngles`].
pub type LEEulerAnglesComponent = FixedBinary8<0>;

/// Implemented according to IEEE 1278.1-2012 §6.2.32
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LEEulerAngles {
    /// Angle of rotation about the Z-axis
    pub psi: LEEulerAnglesComponent,
    /// Angle of rotation about the Y-axis
    pub theta: LEEulerAnglesComponent,
    /// Angle of rotation about the X-axis
    pub phi: LEEulerAnglesComponent,
}

impl LEEulerAngles {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub const fn new(
        psi: LEEulerAnglesComponent,
        theta: LEEulerAnglesComponent,
        phi: LEEulerAnglesComponent,
    ) -> Self {
        Self { psi, theta, phi }
    }
}

/// An event identifier in the compact Live Entity form: 8-bit site and application
/// numbers and a 16-bit event number.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveEventIdentifier {
    pub site_number: u8,
    pub application_number: u8,
    pub event_number: u16,
}

impl LiveEventIdentifier {
    #[must_use]
    pub const fn new(site_number: u8, application_number: u8, event_number: u16) -> Self {
        Self {
            site_number,
            application_number,
            event_number,
        }
    }
}

/// One component of [`LELinearVelocity`].
pub type LELinearVelocityComponent = FixedBinary16<0>;

/// An entity's linear velocity in the compact Live Entity form.
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LELinearVelocity {
    pub x_component: LELinearVelocityComponent,
    pub y_component: LELinearVelocityComponent,
    pub z_component: LELinearVelocityComponent,
}

impl LELinearVelocity {
    #[must_use]
    pub const fn new(
        x_component: LELinearVelocityComponent,
        y_component: LELinearVelocityComponent,
        z_component: LELinearVelocityComponent,
    ) -> Self {
        Self {
            x_component,
            y_component,
            z_component,
        }
    }
}

/// An entity's orientation as three 16-bit angles in the compact Live Entity form.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveEntityOrientation16 {
    pub psi: i16,
    pub theta: i16,
    pub phi: i16,
}

impl LiveEntityOrientation16 {
    #[must_use]
    pub const fn new(psi: i16, theta: i16, phi: i16) -> Self {
        Self { psi, theta, phi }
    }
}

/// One component of [`OrientationError`].
pub type OrientationErrorComponent = FixedBinary16<8>;

/// The estimated error in a live entity's orientation.
#[derive(Debug, Clone, Copy, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct OrientationError {
    pub azimuth_error: OrientationErrorComponent,
    pub elevation_error: OrientationErrorComponent,
    pub rotation_error: OrientationErrorComponent,
}

impl OrientationError {
    #[must_use]
    pub const fn new(
        azimuth_error: OrientationErrorComponent,
        elevation_error: OrientationErrorComponent,
        rotation_error: OrientationErrorComponent,
    ) -> Self {
        Self {
            azimuth_error,
            elevation_error,
            rotation_error,
        }
    }
}

/// One component of [`PositionError`].
pub type PositionErrorComponent = FixedBinary16<3>;

/// The estimated error in a live entity's position.
#[derive(Debug, Clone, Copy, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct PositionError {
    pub horizontal_error: PositionErrorComponent,
    pub vertical_error: PositionErrorComponent,
}

impl PositionError {
    #[must_use]
    pub const fn new(
        horizontal_error: PositionErrorComponent,
        vertical_error: PositionErrorComponent,
    ) -> Self {
        Self {
            horizontal_error,
            vertical_error,
        }
    }
}

/// One component of [`RelativeWorldCoordinates`].
pub type RelativeWorldCoordinatesComponent = FixedBinary16<3>;

/// A live entity's location relative to a reference point.
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct RelativeWorldCoordinates {
    pub reference_point: u16,
    pub delta_x: RelativeWorldCoordinatesComponent,
    pub delta_y: RelativeWorldCoordinatesComponent,
    pub delta_z: RelativeWorldCoordinatesComponent,
}

impl RelativeWorldCoordinates {
    #[must_use]
    pub const fn new(
        reference_point: u16,
        delta_x: RelativeWorldCoordinatesComponent,
        delta_y: RelativeWorldCoordinatesComponent,
        delta_z: RelativeWorldCoordinatesComponent,
    ) -> Self {
        Self {
            reference_point,
            delta_x,
            delta_y,
            delta_z,
        }
    }
}

/// A location relative to a reference point in the compact Live Entity form.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveEntityRelativeLocation {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// The munition in a Live Entity Fire or Detonation PDU: its type, warhead, fuse, quantity,
/// and rate.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveMunitionDescriptor {
    pub munition_type: EntityType,
    pub warhead: u16,
    pub fuse: u16,
    pub quantity: u16,
    pub rate: u16,
}

#[cfg(test)]
#[allow(
    clippy::float_cmp,
    reason = "every value compared here is exactly representable"
)]
mod tests {
    use super::*;

    type Metres = FixedBinary16<3>;
    type Fine = FixedBinary8<5>;

    #[test]
    fn converts_with_the_binary_point_from_the_type() {
        assert_eq!(Metres::new(1.25).unwrap().to_raw(), 10);
        assert_eq!(Metres::from_raw(10).to_f64(), 1.25);
        assert_eq!(Fine::new(1.25).unwrap().to_raw(), 40);
        assert_eq!(Fine::RESOLUTION, 1.0 / 32.0);
    }

    #[test]
    fn rounds_to_the_nearest_step() {
        assert_eq!(Metres::new(1.3).unwrap().to_f64(), 1.25);
        assert_eq!(Metres::new(1.32).unwrap().to_f64(), 1.375);
        assert_eq!(Metres::new(-0.0625).unwrap().to_f64(), -0.125); // ties round away from zero
    }

    #[test]
    fn range_matches_the_integer_range() {
        assert_eq!(Metres::MAX, 4095.875);
        assert_eq!(Metres::MIN, -4096.0);
        assert_eq!(Fine::MAX, 127.0 / 32.0);
        assert!(Metres::new(Metres::MAX).is_some());
        assert!(Metres::new(4096.0).is_none());
        assert!(Metres::new(f64::NAN).is_none());
        assert!(Metres::new(f64::INFINITY).is_none());
    }

    #[test]
    fn saturating_clamps_and_maps_nan_to_zero() {
        assert_eq!(Metres::saturating(1e9).to_f64(), Metres::MAX);
        assert_eq!(Metres::saturating(-1e9).to_f64(), Metres::MIN);
        assert_eq!(Metres::saturating(f64::NAN).to_raw(), 0);
    }

    #[test]
    fn encodes_as_the_big_endian_integer() {
        let mut buf = Vec::new();
        crate::common::wire::write(&Metres::from_raw(-2), &mut buf).unwrap();
        assert_eq!(buf, [0xFF, 0xFE]);
        let back: Metres = crate::common::wire::read(&mut buf.as_slice()).unwrap();
        assert_eq!(back.to_f64(), -0.25);
    }
}
