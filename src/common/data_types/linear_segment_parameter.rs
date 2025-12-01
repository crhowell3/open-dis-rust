//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::{WorldCoordinate, euler_angles::EulerAngles},
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Clone, Debug, Default)]
pub struct LinearSegmentParameter {
    pub segment_number: u8,
    pub segment_modification: u8,
    pub general_segment_appearance: u32,
    pub specific_segment_appearance: u32, // TODO(@anyone) Implement Specific Object Appearance
    pub segment_location: WorldCoordinate,
    pub segment_orientation: EulerAngles,
    pub segment_length: f32,
    pub segment_width: f32,
    pub segment_height: f32,
    pub segment_depth: f32,
    padding: u32,
}

impl LinearSegmentParameter {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        segment_number: u8,
        segment_modification: u8,
        general_segment_appearance: u32,
        specific_segment_appearance: u32,
        segment_location: WorldCoordinate,
        segment_orientation: EulerAngles,
        segment_length: f32,
        segment_width: f32,
        segment_height: f32,
        segment_depth: f32,
    ) -> Self {
        Self {
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
            padding: 0_u32,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.segment_number);
        buf.put_u8(self.segment_modification);
        buf.put_u32(self.general_segment_appearance);
        buf.put_u32(self.specific_segment_appearance);
        self.segment_location.serialize(buf);
        self.segment_orientation.serialize(buf);
        buf.put_f32(self.segment_length);
        buf.put_f32(self.segment_width);
        buf.put_f32(self.segment_height);
        buf.put_f32(self.segment_depth);
        buf.put_u32(self.padding);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            segment_number: buf.get_u8(),
            segment_modification: buf.get_u8(),
            general_segment_appearance: buf.get_u32(),
            specific_segment_appearance: buf.get_u32(),
            segment_location: WorldCoordinate::deserialize(buf),
            segment_orientation: EulerAngles::deserialize(buf),
            segment_length: buf.get_f32(),
            segment_width: buf.get_f32(),
            segment_height: buf.get_f32(),
            segment_depth: buf.get_f32(),
            padding: buf.get_u32(),
        }
    }
}

impl FieldSerialize for LinearSegmentParameter {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for LinearSegmentParameter {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for LinearSegmentParameter {
    fn field_len(&self) -> usize {
        self.segment_number.field_len()
            + self.segment_modification.field_len()
            + self.general_segment_appearance.field_len()
            + self.specific_segment_appearance.field_len()
            + self.segment_location.field_len()
            + self.segment_orientation.field_len()
            + self.segment_length.field_len()
            + self.segment_width.field_len()
            + self.segment_height.field_len()
            + self.segment_depth.field_len()
            + self.padding.field_len()
    }
}
