//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct VelocityVector {
    pub first_vector_component: f32,
    pub second_vector_component: f32,
    pub third_vector_component: f32,
}

impl VelocityVector {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            first_vector_component: x,
            second_vector_component: y,
            third_vector_component: z,
        }
    }
}
