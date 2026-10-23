//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Minefield protocol family

use bitflags::bitflags;
use deku::{
    DekuError, DekuReader, DekuWriter,
    ctx::Endian,
    no_std_io::{Read, Seek, Write},
    reader::Reader,
    writer::Writer,
};

use crate::common::{
    PduHeader,
    data_types::{
        ClockTime, EntityCoordinateVector, EntityId, EntityType, EulerAngles, MinefieldIdentifier,
        Point, WorldCoordinate,
    },
    enums::{ForceId, MinefieldStateProtocolMode, PduType, ProtocolFamily},
    wire,
};

/// Implemented according to IEEE 1278.1-2012 §7.9.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct MinefieldStatePdu {
    #[deku(skip)]
    header: PduHeader,
    pub minefield_id: MinefieldIdentifier,
    pub minefield_sequence: u16,
    pub force_id: ForceId,
    #[deku(update = "self.perimeter_points.len()")]
    pub number_of_perimeter_points: u8,
    pub minefield_type: EntityType,
    #[deku(
        update = "self.mine_type.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_mine_types)"
    )]
    pub number_of_mine_types: u16,
    pub minefield_location: WorldCoordinate,
    pub minefield_orientation: EulerAngles,
    pub appearance: u16,
    pub protocol_mode: MinefieldStateProtocolMode,
    #[deku(count = "number_of_perimeter_points")]
    pub perimeter_points: Vec<Point>,
    #[deku(count = "number_of_mine_types")]
    pub mine_type: Vec<EntityType>,
}
crate::impl_pdu!(
    MinefieldStatePdu,
    PduHeader,
    PduType::MinefieldState,
    ProtocolFamily::Minefield
);

/// Implemented according to IEEE 1278.1-2012 §7.9.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct MinefieldQueryPdu {
    #[deku(skip)]
    header: PduHeader,
    pub minefield_id: MinefieldIdentifier,
    pub requesting_entity_id: EntityId,
    pub request_id: u8,
    #[deku(update = "self.requested_perimeter_points.len()")]
    pub number_of_perimeter_points: u8,
    padding: u8,
    #[deku(update = "self.sensor_types.len()")]
    pub number_of_sensor_types: u8,
    pub data_filter: MinefieldDataFilter,
    pub requested_mine_type: EntityType,
    #[deku(count = "number_of_perimeter_points")]
    pub requested_perimeter_points: Vec<Point>,
    /// Minefield Sensor Type records (IEEE 1278.1-2012 §6.2.57), padded to a
    /// 32-bit boundary on the wire.
    #[deku(
        reader = "read_sensor_types(deku::reader, *number_of_sensor_types)",
        writer = "write_sensor_types(deku::writer, sensor_types)"
    )]
    pub sensor_types: Vec<u16>,
}
crate::impl_pdu!(
    MinefieldQueryPdu,
    PduHeader,
    PduType::MinefieldQuery,
    ProtocolFamily::Minefield
);

/// Implemented according to IEEE 1278.1-2012 §7.9.4
///
/// Mines are held one per [`Mine`]; on the wire each field is sent for every
/// mine before the next field (IEEE 1278.1-2012 §5.10.4.5 and Table 189).
/// `serialize` sets the mine and sensor counts and the data filter from the
/// mines, and fails if mines disagree about which optional fields they
/// carry.
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct MinefieldDataPdu {
    #[deku(skip)]
    header: PduHeader,
    pub minefield_id: MinefieldIdentifier,
    pub requesting_entity_id: EntityId,
    pub minefield_sequence_number: u16,
    pub request_id: u8,
    pub pdu_sequence_number: u8,
    pub number_of_pdus: u8,
    #[deku(update = "self.mines.len()")]
    pub number_of_mines_in_this_pdu: u8,
    #[deku(update = "self.sensor_types.len()")]
    pub number_of_sensor_types: u8,
    padding: u8,
    /// Which optional fields the mines carry.
    #[deku(
        update = "derived_data_filter(self.data_filter, &self.mines, self.sensor_types.len())?"
    )]
    pub data_filter: MinefieldDataFilter,
    pub mine_type: EntityType,
    /// Minefield Sensor Type records (IEEE 1278.1-2012 §6.2.57), padded to a
    /// 32-bit boundary on the wire.
    #[deku(
        reader = "read_sensor_types(deku::reader, *number_of_sensor_types)",
        writer = "write_sensor_types(deku::writer, sensor_types)"
    )]
    pub sensor_types: Vec<u16>,
    #[deku(
        reader = "read_mines(deku::reader, usize::from(*number_of_mines_in_this_pdu), usize::from(*number_of_sensor_types), *data_filter)",
        writer = "write_mines(deku::writer, mines, *data_filter, sensor_types.len())"
    )]
    pub mines: Vec<Mine>,
}
crate::impl_pdu!(
    MinefieldDataPdu,
    PduHeader,
    PduType::MinefieldData,
    ProtocolFamily::Minefield
);

bitflags! {
    /// Data Filter record (IEEE 1278.1-2012 §6.2.16, Table 38): which optional
    /// fields a Minefield Data PDU carries, or a Minefield Query PDU
    /// requests. Bits 11-31 are padding and are kept as received.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct MinefieldDataFilter: u32 {
        const GROUND_BURIAL_DEPTH_OFFSET = 1 << 0;
        const WATER_BURIAL_DEPTH_OFFSET = 1 << 1;
        const SNOW_BURIAL_DEPTH_OFFSET = 1 << 2;
        const MINE_ORIENTATION = 1 << 3;
        const THERMAL_CONTRAST = 1 << 4;
        const REFLECTANCE = 1 << 5;
        const MINE_EMPLACEMENT_TIME = 1 << 6;
        const TRIP_DETONATION_WIRE = 1 << 7;
        const FUSING = 1 << 8;
        const SCALAR_DETECTION_COEFFICIENT = 1 << 9;
        const PAINT_SCHEME = 1 << 10;
    }
}
crate::common::enums::impl_deku_bitflags!(MinefieldDataFilter, u32);

/// One mine in a Minefield Data PDU. Optional fields must be present on
/// every mine in the PDU or on none.
#[derive(Debug, Default, Clone)]
pub struct Mine {
    pub location: EntityCoordinateVector,
    /// Required when the water or snow burial depth offset is sent.
    pub ground_burial_depth_offset: Option<f32>,
    pub water_burial_depth_offset: Option<f32>,
    pub snow_burial_depth_offset: Option<f32>,
    /// Required when any burial depth offset is sent.
    pub orientation: Option<EulerAngles>,
    pub thermal_contrast: Option<f32>,
    pub reflectance: Option<f32>,
    pub emplacement_time: Option<ClockTime>,
    pub entity_number: u16,
    /// SISO-REF-010 UID 192, a 16-bit record.
    pub fusing: Option<u16>,
    /// One per sensor type, in the order of the PDU's `sensor_types`.
    pub scalar_detection_coefficients: Option<Vec<u8>>,
    /// SISO-REF-010 UID 202, an 8-bit record.
    pub paint_scheme: Option<u8>,
    /// Each trip/detonation wire's vertices, in order.
    pub trip_wires: Option<Vec<Vec<EntityCoordinateVector>>>,
}

/// Implemented according to IEEE 1278.1-2012 §7.9.5
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct MinefieldResponseNackPdu {
    #[deku(skip)]
    header: PduHeader,
    pub minefield_id: EntityId,
    pub requesting_entity_id: EntityId,
    pub request_id: u8,
    #[deku(update = "self.missing_pdu_sequence_numbers.len()")]
    pub number_of_missing_pdus: u8,
    #[deku(count = "number_of_missing_pdus")]
    pub missing_pdu_sequence_numbers: Vec<u64>,
}
crate::impl_pdu!(
    MinefieldResponseNackPdu,
    PduHeader,
    PduType::MinefieldResponseNack,
    ProtocolFamily::Minefield
);

// ---------------------------------------------------------------------------
// Wire helpers
// ---------------------------------------------------------------------------

fn read_sensor_types<R: Read + Seek>(
    reader: &mut Reader<R>,
    count: u8,
) -> Result<Vec<u16>, DekuError> {
    let sensor_types = wire::read_n(reader, usize::from(count), Endian::Big)?;
    wire::skip(
        reader,
        wire::padding_to(usize::from(count).saturating_mul(2), 4),
    )?;
    Ok(sensor_types)
}

fn write_sensor_types<W: Write + Seek>(
    writer: &mut Writer<W>,
    sensor_types: &[u16],
) -> Result<(), DekuError> {
    sensor_types.to_writer(writer, Endian::Big)?;
    wire::write_zeros(
        writer,
        wire::padding_to(sensor_types.len().saturating_mul(2), 4),
    )
}

fn mine_error(message: impl Into<String>) -> DekuError {
    DekuError::Assertion(message.into().into())
}

/// The data filter implied by the mines: a field's bit is set when every
/// mine carries it. Padding bits are kept.
fn derived_data_filter(
    current: MinefieldDataFilter,
    mines: &[Mine],
    sensor_types: usize,
) -> Result<MinefieldDataFilter, DekuError> {
    use MinefieldDataFilter as F;
    fn presence(
        mines: &[Mine],
        name: &str,
        has: impl Fn(&Mine) -> bool,
    ) -> Result<bool, DekuError> {
        let count = mines.iter().filter(|mine| has(mine)).count();
        if count == 0 {
            Ok(false)
        } else if count == mines.len() {
            Ok(true)
        } else {
            Err(mine_error(format!(
                "{name} is set on {count} of {} mines; it must be on all or none",
                mines.len()
            )))
        }
    }

    let fields = [
        (
            F::GROUND_BURIAL_DEPTH_OFFSET,
            presence(mines, "ground_burial_depth_offset", |m| {
                m.ground_burial_depth_offset.is_some()
            })?,
        ),
        (
            F::WATER_BURIAL_DEPTH_OFFSET,
            presence(mines, "water_burial_depth_offset", |m| {
                m.water_burial_depth_offset.is_some()
            })?,
        ),
        (
            F::SNOW_BURIAL_DEPTH_OFFSET,
            presence(mines, "snow_burial_depth_offset", |m| {
                m.snow_burial_depth_offset.is_some()
            })?,
        ),
        (
            F::MINE_ORIENTATION,
            presence(mines, "orientation", |m| m.orientation.is_some())?,
        ),
        (
            F::THERMAL_CONTRAST,
            presence(mines, "thermal_contrast", |m| m.thermal_contrast.is_some())?,
        ),
        (
            F::REFLECTANCE,
            presence(mines, "reflectance", |m| m.reflectance.is_some())?,
        ),
        (
            F::MINE_EMPLACEMENT_TIME,
            presence(mines, "emplacement_time", |m| m.emplacement_time.is_some())?,
        ),
        (
            F::TRIP_DETONATION_WIRE,
            presence(mines, "trip_wires", |m| m.trip_wires.is_some())?,
        ),
        (
            F::FUSING,
            presence(mines, "fusing", |m| m.fusing.is_some())?,
        ),
        (
            F::SCALAR_DETECTION_COEFFICIENT,
            presence(mines, "scalar_detection_coefficients", |m| {
                m.scalar_detection_coefficients.is_some()
            })?,
        ),
        (
            F::PAINT_SCHEME,
            presence(mines, "paint_scheme", |m| m.paint_scheme.is_some())?,
        ),
    ];
    let mut filter = MinefieldDataFilter::from_bits_retain(current.bits() & !F::all().bits());
    for (flag, present) in fields {
        filter.set(flag, present);
    }

    let any_burial = filter.intersects(
        F::GROUND_BURIAL_DEPTH_OFFSET | F::WATER_BURIAL_DEPTH_OFFSET | F::SNOW_BURIAL_DEPTH_OFFSET,
    );
    if any_burial && !filter.contains(F::GROUND_BURIAL_DEPTH_OFFSET | F::MINE_ORIENTATION) {
        return Err(mine_error(
            "a burial depth offset requires ground_burial_depth_offset and orientation on every mine",
        ));
    }
    if let Some(mine) = mines.iter().find(|m| {
        m.scalar_detection_coefficients
            .as_ref()
            .is_some_and(|c| c.len() != sensor_types)
    }) {
        let found = mine
            .scalar_detection_coefficients
            .as_ref()
            .map_or(0, Vec::len);
        return Err(mine_error(format!(
            "a mine has {found} scalar detection coefficients but the PDU has {sensor_types} sensor types"
        )));
    }
    Ok(filter)
}

/// Whether the ground burial depth offset is sent (`G` in Table 189): with
/// any burial depth offset, since ground is their frame of reference.
fn sends_ground(filter: MinefieldDataFilter) -> bool {
    use MinefieldDataFilter as F;
    filter.intersects(
        F::GROUND_BURIAL_DEPTH_OFFSET | F::WATER_BURIAL_DEPTH_OFFSET | F::SNOW_BURIAL_DEPTH_OFFSET,
    )
}

/// Whether the mine orientation is sent (`R` in Table 189): when requested,
/// or with any burial depth offset.
fn sends_orientation(filter: MinefieldDataFilter) -> bool {
    filter.contains(MinefieldDataFilter::MINE_ORIENTATION) || sends_ground(filter)
}

/// Read `n` values of a per-mine field into each mine, if `present`.
fn read_each<'a, T, R>(
    reader: &mut Reader<R>,
    mines: &mut [Mine],
    present: bool,
    set: impl Fn(&mut Mine, T),
) -> Result<(), DekuError>
where
    T: DekuReader<'a, Endian>,
    R: Read + Seek,
{
    if present {
        for mine in mines {
            set(mine, T::from_reader_with_ctx(reader, Endian::Big)?);
        }
    }
    Ok(())
}

fn read_mines<R: Read + Seek>(
    reader: &mut Reader<R>,
    n: usize,
    m: usize,
    filter: MinefieldDataFilter,
) -> Result<Vec<Mine>, DekuError> {
    use MinefieldDataFilter as F;
    let mut mines: Vec<Mine> = wire::read_n::<EntityCoordinateVector, _>(reader, n, Endian::Big)?
        .into_iter()
        .map(|location| Mine {
            location,
            ..Mine::default()
        })
        .collect();

    read_each(reader, &mut mines, sends_ground(filter), |mine, v| {
        mine.ground_burial_depth_offset = Some(v);
    })?;
    read_each(
        reader,
        &mut mines,
        filter.contains(F::WATER_BURIAL_DEPTH_OFFSET),
        |mine, v| mine.water_burial_depth_offset = Some(v),
    )?;
    read_each(
        reader,
        &mut mines,
        filter.contains(F::SNOW_BURIAL_DEPTH_OFFSET),
        |mine, v| mine.snow_burial_depth_offset = Some(v),
    )?;
    read_each(reader, &mut mines, sends_orientation(filter), |mine, v| {
        mine.orientation = Some(v);
    })?;
    read_each(
        reader,
        &mut mines,
        filter.contains(F::THERMAL_CONTRAST),
        |mine, v| mine.thermal_contrast = Some(v),
    )?;
    read_each(
        reader,
        &mut mines,
        filter.contains(F::REFLECTANCE),
        |mine, v| mine.reflectance = Some(v),
    )?;
    read_each(
        reader,
        &mut mines,
        filter.contains(F::MINE_EMPLACEMENT_TIME),
        |mine, v| mine.emplacement_time = Some(v),
    )?;
    read_each(reader, &mut mines, true, |mine, v| mine.entity_number = v)?;
    read_each(reader, &mut mines, filter.contains(F::FUSING), |mine, v| {
        mine.fusing = Some(v);
    })?;

    // Entity numbers onward are 16- and 8-bit, so track the run for padding P2.
    let mut run = n.saturating_mul(2);
    if filter.contains(F::FUSING) {
        run = run.saturating_add(n.saturating_mul(2));
    }
    if filter.contains(F::SCALAR_DETECTION_COEFFICIENT) {
        for mine in &mut mines {
            mine.scalar_detection_coefficients = Some(wire::read_n(reader, m, Endian::Big)?);
        }
        run = run.saturating_add(n.saturating_mul(m));
    }
    if filter.contains(F::PAINT_SCHEME) {
        read_each(reader, &mut mines, true, |mine, v| {
            mine.paint_scheme = Some(v);
        })?;
        run = run.saturating_add(n);
    }
    wire::skip(reader, wire::padding_to(run, 4))?;

    if filter.contains(F::TRIP_DETONATION_WIRE) {
        let wires: Vec<u8> = wire::read_n(reader, n, Endian::Big)?;
        wire::skip(reader, wire::padding_to(n, 4))?;
        let total_wires: usize = wires.iter().map(|&w| usize::from(w)).sum();
        let vertex_counts: Vec<u8> = wire::read_n(reader, total_wires, Endian::Big)?;
        wire::skip(reader, wire::padding_to(total_wires, 4))?;
        let mut counts = vertex_counts.iter();
        for (mine, &wire_count) in mines.iter_mut().zip(&wires) {
            let mut trip_wires = Vec::with_capacity(usize::from(wire_count));
            for _ in 0..wire_count {
                let vertices = counts.next().copied().unwrap_or_default();
                trip_wires.push(wire::read_n(reader, usize::from(vertices), Endian::Big)?);
            }
            mine.trip_wires = Some(trip_wires);
        }
    }
    Ok(mines)
}

/// Write one per-mine field for every mine, or nothing when `present` is
/// false. Every mine must have the field when it is present.
fn write_each<T, W>(
    writer: &mut Writer<W>,
    mines: &[Mine],
    present: bool,
    name: &str,
    get: impl Fn(&Mine) -> Option<T>,
) -> Result<(), DekuError>
where
    T: DekuWriter<Endian>,
    W: Write + Seek,
{
    if present {
        for mine in mines {
            get(mine)
                .ok_or_else(|| {
                    mine_error(format!("the data filter sends {name}, but a mine has none"))
                })?
                .to_writer(writer, Endian::Big)?;
        }
    }
    Ok(())
}

fn write_mines<W: Write + Seek>(
    writer: &mut Writer<W>,
    mines: &[Mine],
    filter: MinefieldDataFilter,
    m: usize,
) -> Result<(), DekuError> {
    use MinefieldDataFilter as F;
    let n = mines.len();
    for mine in mines {
        mine.location.to_writer(writer, Endian::Big)?;
    }
    write_each(
        writer,
        mines,
        sends_ground(filter),
        "ground_burial_depth_offset",
        |m| m.ground_burial_depth_offset,
    )?;
    write_each(
        writer,
        mines,
        filter.contains(F::WATER_BURIAL_DEPTH_OFFSET),
        "water_burial_depth_offset",
        |m| m.water_burial_depth_offset,
    )?;
    write_each(
        writer,
        mines,
        filter.contains(F::SNOW_BURIAL_DEPTH_OFFSET),
        "snow_burial_depth_offset",
        |m| m.snow_burial_depth_offset,
    )?;
    write_each(
        writer,
        mines,
        sends_orientation(filter),
        "orientation",
        |m| m.orientation,
    )?;
    write_each(
        writer,
        mines,
        filter.contains(F::THERMAL_CONTRAST),
        "thermal_contrast",
        |m| m.thermal_contrast,
    )?;
    write_each(
        writer,
        mines,
        filter.contains(F::REFLECTANCE),
        "reflectance",
        |m| m.reflectance,
    )?;
    write_each(
        writer,
        mines,
        filter.contains(F::MINE_EMPLACEMENT_TIME),
        "emplacement_time",
        |m| m.emplacement_time,
    )?;
    write_each(writer, mines, true, "entity_number", |m| {
        Some(m.entity_number)
    })?;
    write_each(writer, mines, filter.contains(F::FUSING), "fusing", |m| {
        m.fusing
    })?;

    let mut run = n.saturating_mul(2);
    if filter.contains(F::FUSING) {
        run = run.saturating_add(n.saturating_mul(2));
    }
    if filter.contains(F::SCALAR_DETECTION_COEFFICIENT) {
        for mine in mines {
            let coefficients = mine
                .scalar_detection_coefficients
                .as_deref()
                .filter(|c| c.len() == m)
                .ok_or_else(|| {
                    mine_error(format!(
                        "every mine needs {m} scalar detection coefficients"
                    ))
                })?;
            coefficients.to_writer(writer, Endian::Big)?;
        }
        run = run.saturating_add(n.saturating_mul(m));
    }
    if filter.contains(F::PAINT_SCHEME) {
        write_each(writer, mines, true, "paint_scheme", |m| m.paint_scheme)?;
        run = run.saturating_add(n);
    }
    wire::write_zeros(writer, wire::padding_to(run, 4))?;

    if filter.contains(F::TRIP_DETONATION_WIRE) {
        write_trip_wires(writer, mines)?;
    }
    Ok(())
}

/// Write the trip/detonation wire counts, vertex counts and vertices, each
/// run padded to a 32-bit boundary (Table 189, `P3` and `P4`).
fn write_trip_wires<W: Write + Seek>(
    writer: &mut Writer<W>,
    mines: &[Mine],
) -> Result<(), DekuError> {
    let n = mines.len();
    let mut all_wires = Vec::with_capacity(n);
    for mine in mines {
        let wires = mine
            .trip_wires
            .as_deref()
            .ok_or_else(|| mine_error("the data filter sends trip wires, but a mine has none"))?;
        all_wires.push(wires);
    }
    let count = |len: usize, what: &str| {
        u8::try_from(len).map_err(|_| mine_error(format!("a mine has more than 255 {what}")))
    };
    for wires in &all_wires {
        count(wires.len(), "trip wires")?.to_writer(writer, Endian::Big)?;
    }
    wire::write_zeros(writer, wire::padding_to(n, 4))?;
    let mut total_wires = 0usize;
    for wire_vertices in all_wires.iter().flat_map(|w| w.iter()) {
        count(wire_vertices.len(), "vertices on one wire")?.to_writer(writer, Endian::Big)?;
        total_wires = total_wires.saturating_add(1);
    }
    wire::write_zeros(writer, wire::padding_to(total_wires, 4))?;
    for vertex in all_wires.iter().flat_map(|w| w.iter()).flatten() {
        vertex.to_writer(writer, Endian::Big)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{
            constants::BITS_PER_BYTE,
            pdu::{Pdu, to_bytes},
        },
        pdu_tests,
    };
    use bytes::Buf;

    pdu_tests!(
        minefield_state_pdu_tests, MinefieldStatePdu,
        pdu_type: PduType::MinefieldState,
        default_bits: 576,
        wrong_type: MinefieldQueryPdu,
    );
    pdu_tests!(
        minefield_query_pdu_tests, MinefieldQueryPdu,
        pdu_type: PduType::MinefieldQuery,
        default_bits: 320,
        wrong_type: MinefieldDataPdu,
    );
    pdu_tests!(
        minefield_data_pdu_tests, MinefieldDataPdu,
        pdu_type: PduType::MinefieldData,
        default_bits: 352,
        wrong_type: MinefieldResponseNackPdu,
    );
    pdu_tests!(
        minefield_response_nack_pdu_tests, MinefieldResponseNackPdu,
        pdu_type: PduType::MinefieldResponseNack,
        default_bits: 208,
        wrong_type: MinefieldDataPdu,
    );
}
