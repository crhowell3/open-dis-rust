//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct SecondaryOperationalData {
    pub operational_data1: u8,
    pub operational_data2: u8,
    pub number_of_iff_fundamental_parameter_records: u16,
}

impl SecondaryOperationalData {
    #[must_use]
    pub fn new(
        operational_data1: u8,
        operational_data2: u8,
        number_of_iff_fundamental_parameter_records: u16,
    ) -> Self {
        SecondaryOperationalData {
            operational_data1,
            operational_data2,
            number_of_iff_fundamental_parameter_records,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.operational_data1);
        buf.put_u8(self.operational_data2);
        buf.put_u16(self.number_of_iff_fundamental_parameter_records);
    }

    pub fn deserialize(buf: &mut BytesMut) -> SecondaryOperationalData {
        SecondaryOperationalData {
            operational_data1: buf.get_u8(),
            operational_data2: buf.get_u8(),
            number_of_iff_fundamental_parameter_records: buf.get_u16(),
        }
    }
}
