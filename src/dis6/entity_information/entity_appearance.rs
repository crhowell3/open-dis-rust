//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation 
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::BytesMut;

#[derive(Copy, Clone, Debug)]
pub struct EntityAppearance {
    pub general_appearance: GeneralAppearance;
    pub specific_appearance: SpecificAppearance;
}

impl EntityAppearance {
    pub fn new(general_appearance: GeneralAppearance, specific_appearance: SpecificAppearance) -> Self {
        EntityAppearance {
            general_appearance,
            specific_appearance
        }
    }
}
