//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct GridDataRecord {
    pub sample_type: u16,
    pub data_representation: u16,
    pub number_of_octets: u16,
    pub data_values: Vec<u8>,
    pub padding: u8,
}

impl GridDataRecord {
    #[must_use]
    pub fn new(
        sample_type: u16,
        data_representation: u16,
        number_of_octets: u16,
        data_values: Vec<u8>,
        padding: u8,
    ) -> Self {
        GridDataRecord {
            sample_type,
            data_representation,
            number_of_octets,
            data_values,
            padding,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.sample_type);
        buf.put_u16(self.data_representation);
        buf.put_u16(self.number_of_octets);
        for i in 0..self.data_values.len() {
            buf.put_u8(self.data_values[i]);
        }
        buf.put_u8(self.padding);
    }

    pub fn decode(buf: &mut BytesMut) -> GridDataRecord {
        let sample_type = buf.get_u16();
        let data_representation = buf.get_u16();
        let number_of_octets = buf.get_u16();
        let mut data_values: Vec<u8> = vec![];
        for _i in 0..number_of_octets {
            data_values.push(buf.get_u8());
        }
        let padding = buf.get_u8();
        GridDataRecord {
            sample_type,
            data_representation,
            number_of_octets,
            data_values,
            padding,
        }
    }
}
