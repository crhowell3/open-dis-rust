//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::SerializedLength,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct VariableParameter {
    pub record_type: u8,
    pub variable_parameter_field1: f64,
    pub variable_parameter_field2: u32,
    pub variable_parameter_field3: u16,
    pub variable_parameter_field4: u8,
}

impl VariableParameter {
    #[must_use]
    pub const fn new(
        record_type: u8,
        variable_parameter_field1: f64,
        variable_parameter_field2: u32,
        variable_parameter_field3: u16,
        variable_parameter_field4: u8,
    ) -> Self {
        Self {
            record_type,
            variable_parameter_field1,
            variable_parameter_field2,
            variable_parameter_field3,
            variable_parameter_field4,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.record_type);
        buf.put_f64(self.variable_parameter_field1);
        buf.put_u32(self.variable_parameter_field2);
        buf.put_u16(self.variable_parameter_field3);
        buf.put_u8(self.variable_parameter_field4);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            record_type: buf.get_u8(),
            variable_parameter_field1: buf.get_f64(),
            variable_parameter_field2: buf.get_u32(),
            variable_parameter_field3: buf.get_u16(),
            variable_parameter_field4: buf.get_u8(),
        }
    }
}

impl FieldSerialize for VariableParameter {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for VariableParameter {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for VariableParameter {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for VariableParameter {
    const LENGTH: usize = 16;
}
