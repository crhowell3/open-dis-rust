//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Clone, Debug, Default)]
pub struct ModulationParameters {
    pub record_specific_fields: Vec<u8>,
}

impl ModulationParameters {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            record_specific_fields: Vec::new(),
        }
    }
}
