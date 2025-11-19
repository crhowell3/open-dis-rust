//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct FixedDatumRecord {
    pub datum_id: u32,
    pub datum_value: u32,
}

impl FixedDatumRecord {
    pub fn serialize(&mut self, buf: &mut BytesMut) {
        buf.put_u32(self.datum_id);
        buf.put_u32(self.datum_value);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let datum_id = buf.get_u32();
        let datum_value = buf.get_u32();
        Self {
            datum_id,
            datum_value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableDatumRecord {
    pub datum_id: u32,
    pub length_bits: u32,
    pub value: Vec<u8>,
}

impl VariableDatumRecord {
    fn bytes_count(length_bits: u32) -> usize {
        ((length_bits as usize) + 7) / 8
    }

    pub fn serialize(&mut self, buf: &mut BytesMut) {
        buf.put_u32(self.datum_id);
        buf.put_u32(self.length_bits);

        let expected_bytes = Self::bytes_count(self.length_bits);
        if self.value.len() != expected_bytes {
            // return Err(DISError::SerializationError { msg: "Length of datum does not equal expected bytes", source: None });
        }
        for i in 0..self.value.len() {
            buf.put_u8(self.value[i]);
        }

        let pad_bits = (64u32 - (self.length_bits % 64)) % 64;
        let pad_bytes = (pad_bits / 8) as usize;
        if pad_bytes > 0 {
            let zeros = vec![0u8; pad_bytes];
            for i in 0..pad_bytes {
                buf.put_u8(zeros[i]);
            }
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let datum_id = buf.get_u32();
        let length_bits = buf.get_u32();
        let value_bytes = Self::bytes_count(length_bits);

        let mut value = vec![0u8; value_bytes];
        for i in 0..value_bytes {
            value[i] = buf.get_u8();
        }

        Self {
            datum_id,
            length_bits,
            value,
        }
    }
}
