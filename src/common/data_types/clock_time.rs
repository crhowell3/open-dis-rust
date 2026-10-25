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
/// Implemented according to IEEE 1278.1-2012 §6.2.14
pub struct ClockTime {
    /// The hours since 0000h 1 January 1970 UTC (The Epoch)
    pub hour: u32,
    /// Time past the hour indicated in the hour field
    pub time_past_hour: u32,
}

impl ClockTime {
    #[must_use]
    pub const fn new(h: u32, p: u32) -> Self {
        Self {
            hour: h,
            time_past_hour: p,
        }
    }
}
