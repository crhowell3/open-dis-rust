//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! Fixed binary numbers as used by the Live Entity PDUs

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
