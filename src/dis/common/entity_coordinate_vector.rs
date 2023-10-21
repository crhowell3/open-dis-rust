use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct EntityCoordinateVector {
    pub x_coordinate: f32,
    pub y_coordinate: f32,
    pub z_coordinate: f32,
}

impl EntityCoordinateVector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        EntityCoordinateVector {
            x_coordinate: x,
            y_coordinate: y,
            z_coordinate: z,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.x_coordinate);
        buf.put_f32(self.y_coordinate);
        buf.put_f32(self.z_coordinate);
    }

    pub fn decode(buf: &mut BytesMut) -> EntityCoordinateVector {
        EntityCoordinateVector {
            x_coordinate: buf.get_f32(),
            y_coordinate: buf.get_f32(),
            z_coordinate: buf.get_f32(),
        }
    }
}
