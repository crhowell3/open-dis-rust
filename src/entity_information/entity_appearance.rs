//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use super::{general_appearance::GeneralAppearance, specific_appearance::SpecificAppearance};
use bytes::BytesMut;

#[derive(Copy, Clone, Debug, Default)]
pub struct EntityAppearance {
    pub general_appearance: GeneralAppearance,
    pub specific_appearance: SpecificAppearance,
}

impl EntityAppearance {
    pub fn new(
        general_appearance: GeneralAppearance,
        specific_appearance: SpecificAppearance,
    ) -> Self {
        EntityAppearance {
            general_appearance,
            specific_appearance,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.general_appearance.serialize(buf);
        self.specific_appearance.serialize(buf);
    }

    pub fn decode(buf: &mut BytesMut) -> EntityAppearance {
        EntityAppearance {
            general_appearance: GeneralAppearance::decode(buf),
            specific_appearance: SpecificAppearance::decode(buf),
        }
    }
}
