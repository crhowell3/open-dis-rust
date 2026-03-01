//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::fixed_binary_8::FixedBinary8,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.32
pub struct LEEulerAngles {
    /// Angle of rotation about the Z-axis
    pub psi: FixedBinary8,
    /// Angle of rotation about the Y-axis
    pub theta: FixedBinary8,
    /// Angle of rotation about the X-axis
    pub phi: FixedBinary8,
}

impl LEEulerAngles {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub const fn new(psi: FixedBinary8, theta: FixedBinary8, phi: FixedBinary8) -> Self {
        Self { psi, theta, phi }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i8(self.psi.to_i8());
        buf.put_i8(self.theta.to_i8());
        buf.put_i8(self.phi.to_i8());
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            psi: FixedBinary8::from_i8(buf.get_i8()),
            theta: FixedBinary8::from_i8(buf.get_i8()),
            phi: FixedBinary8::from_i8(buf.get_i8()),
        }
    }
}

impl FieldSerialize for LEEulerAngles {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for LEEulerAngles {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for LEEulerAngles {
    fn field_len(&self) -> usize {
        3
    }
}
