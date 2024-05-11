//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct VariableParameter {
    pub record_type: u8,
    pub variable_parameter_field1: f64,
    pub variable_parameter_field2: u32,
    pub variable_parameter_field3: u16,
    pub variable_parameter_field4: u8,
}

impl Default for VariableParameter {
    fn default() -> Self {
        VariableParameter {
            record_type: 0,
            variable_parameter_field1: 0.0,
            variable_parameter_field2: 0,
            variable_parameter_field3: 0,
            variable_parameter_field4: 0,
        }
    }
}

impl VariableParameter {
    #[must_use]
    pub fn new(
        record_type: u8,
        variable_parameter_field1: f64,
        variable_parameter_field2: u32,
        variable_parameter_field3: u16,
        variable_parameter_field4: u8,
    ) -> Self {
        VariableParameter {
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

    pub fn decode(buf: &mut BytesMut) -> VariableParameter {
        VariableParameter {
            record_type: buf.get_u8(),
            variable_parameter_field1: buf.get_f64(),
            variable_parameter_field2: buf.get_u32(),
            variable_parameter_field3: buf.get_u16(),
            variable_parameter_field4: buf.get_u8(),
        }
    }
}
