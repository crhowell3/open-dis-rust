use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3Float {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3Float {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3Float { x, y, z }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.x);
        buf.put_f32(self.y);
        buf.put_f32(self.z);
    }

    pub fn decode(buf: &mut BytesMut) -> Vector3Float {
        Vector3Float {
            x: buf.get_f32(),
            y: buf.get_f32(),
            z: buf.get_f32(),
        }
    }
}
