//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.32
pub struct EulerAngles {
    /// Angle of rotation about the Z-axis
    pub psi: f32,
    /// Angle of rotation about the Y-axis
    pub theta: f32,
    /// Angle of rotation about the X-axis
    pub phi: f32,
}

impl EulerAngles {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn new(psi: f32, theta: f32, phi: f32) -> Self {
        EulerAngles { psi, theta, phi }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.psi);
        buf.put_f32(self.theta);
        buf.put_f32(self.phi);
    }

    pub fn decode(buf: &mut BytesMut) -> EulerAngles {
        EulerAngles {
            psi: buf.get_f32(),
            theta: buf.get_f32(),
            phi: buf.get_f32(),
        }
    }
}
