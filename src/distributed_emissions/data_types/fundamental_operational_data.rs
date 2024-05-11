//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct FundamentalOperationalData {
    pub system_status: u8,
    pub data_field1: u8,
    pub information_layers: u8,
    pub data_field2: u8,
    pub parameter1: u8,
    pub parameter2: u8,
    pub parameter3: u8,
    pub parameter4: u8,
    pub parameter5: u8,
    pub parameter6: u8,
}

impl FundamentalOperationalData {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        system_status: u8,
        data_field1: u8,
        information_layers: u8,
        data_field2: u8,
        parameter1: u8,
        parameter2: u8,
        parameter3: u8,
        parameter4: u8,
        parameter5: u8,
        parameter6: u8,
    ) -> Self {
        FundamentalOperationalData {
            system_status,
            data_field1,
            information_layers,
            data_field2,
            parameter1,
            parameter2,
            parameter3,
            parameter4,
            parameter5,
            parameter6,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.system_status);
        buf.put_u8(self.data_field1);
        buf.put_u8(self.information_layers);
        buf.put_u8(self.data_field2);
        buf.put_u8(self.parameter1);
        buf.put_u8(self.parameter2);
        buf.put_u8(self.parameter3);
        buf.put_u8(self.parameter4);
        buf.put_u8(self.parameter5);
        buf.put_u8(self.parameter6);
    }

    pub fn decode(buf: &mut BytesMut) -> FundamentalOperationalData {
        FundamentalOperationalData {
            system_status: buf.get_u8(),
            data_field1: buf.get_u8(),
            information_layers: buf.get_u8(),
            data_field2: buf.get_u8(),
            parameter1: buf.get_u8(),
            parameter2: buf.get_u8(),
            parameter3: buf.get_u8(),
            parameter4: buf.get_u8(),
            parameter5: buf.get_u8(),
            parameter6: buf.get_u8(),
        }
    }
}
