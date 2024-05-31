//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::{euler_angles::EulerAngles, vector3_double::Vector3Double},
    entity_information::specific_appearance::SpecificAppearance,
};

#[derive(Clone, Debug, Default)]
pub struct LinearSegmentParameter {
    pub segment_number: u8,
    pub segment_modification: u8,
    pub general_segment_appearance: u32,
    pub specific_segment_appearance: SpecificAppearance,
    pub segment_location: Vector3Double,
    pub segment_orientation: EulerAngles,
    pub segment_length: u16,
    pub segment_width: u16,
    pub segment_height: u16,
    pub segment_depth: u16,
}

impl LinearSegmentParameter {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        segment_number: u8,
        segment_modification: u8,
        general_segment_appearance: u32,
        specific_segment_appearance: SpecificAppearance,
        segment_location: Vector3Double,
        segment_orientation: EulerAngles,
        segment_length: u16,
        segment_width: u16,
        segment_height: u16,
        segment_depth: u16,
    ) -> Self {
        LinearSegmentParameter {
            segment_number,
            segment_modification,
            general_segment_appearance,
            specific_segment_appearance,
            segment_location,
            segment_orientation,
            segment_length,
            segment_width,
            segment_height,
            segment_depth,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.segment_number);
        buf.put_u8(self.segment_modification);
        buf.put_u32(self.general_segment_appearance);
        self.specific_segment_appearance.serialize(buf);
        self.segment_location.serialize(buf);
        self.segment_orientation.serialize(buf);
        buf.put_u16(self.segment_length);
        buf.put_u16(self.segment_width);
        buf.put_u16(self.segment_height);
        buf.put_u16(self.segment_depth);
    }

    pub fn decode(buf: &mut BytesMut) -> LinearSegmentParameter {
        LinearSegmentParameter {
            segment_number: buf.get_u8(),
            segment_modification: buf.get_u8(),
            general_segment_appearance: buf.get_u32(),
            specific_segment_appearance: SpecificAppearance::decode(buf),
            segment_location: Vector3Double::decode(buf),
            segment_orientation: EulerAngles::decode(buf),
            segment_length: buf.get_u16(),
            segment_width: buf.get_u16(),
            segment_height: buf.get_u16(),
            segment_depth: buf.get_u16(),
        }
    }
}
