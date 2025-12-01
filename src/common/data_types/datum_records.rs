//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug)]
pub struct FixedDatumRecord {
    pub datum_id: u32,
    pub datum_value: u32,
}

impl FixedDatumRecord {
    pub fn serialize(&self, buf: &mut BytesMut) {
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

impl FieldSerialize for FixedDatumRecord {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for FixedDatumRecord {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for FixedDatumRecord {
    fn field_len(&self) -> usize {
        8
    }
}

#[derive(Debug, Clone)]
pub struct VariableDatumRecord {
    pub datum_id: u32,
    pub length_bits: u32,
    pub value: Vec<u8>,
}

impl VariableDatumRecord {
    const fn bytes_count(length_bits: u32) -> usize {
        (length_bits as usize).div_ceil(8)
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
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
            for z in zeros {
                buf.put_u8(z);
            }
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let datum_id = buf.get_u32();
        let length_bits = buf.get_u32();
        let value_bytes = Self::bytes_count(length_bits);

        let mut value = vec![0u8; value_bytes];
        for _ in 0..value_bytes {
            value.push(buf.get_u8());
        }

        Self {
            datum_id,
            length_bits,
            value,
        }
    }
}

impl FieldSerialize for VariableDatumRecord {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for VariableDatumRecord {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for VariableDatumRecord {
    fn field_len(&self) -> usize {
        self.datum_id.field_len() + self.length_bits.field_len() + self.value.field_len()
    }
}
