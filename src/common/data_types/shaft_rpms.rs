//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4(j-l)
pub struct ShaftRPMs {
    pub current_shaft_rpms: i16,
    pub ordered_shaft_rpms: i16,
    pub shaft_rpm_rate_of_change: i32,
}

impl ShaftRPMs {
    #[must_use]
    pub const fn new(
        current_shaft_rpms: i16,
        ordered_shaft_rpms: i16,
        shaft_rpm_rate_of_change: i32,
    ) -> Self {
        Self {
            current_shaft_rpms,
            ordered_shaft_rpms,
            shaft_rpm_rate_of_change,
        }
    }
}

