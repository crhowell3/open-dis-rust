//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityType {
    pub kind: Kind,
    pub domain: u8,
    pub country: Country,
    pub category: u8,
    pub subcategory: u8,
    pub specific: u8,
    pub extra: u8,
}

impl EntityType {
    pub fn new(
        kind: Kind,
        domain: u8,
        country: Country,
        category: u8,
        subcategory: u8,
        specific: u8,
        extra: u8,
    ) -> Self {
        EntityType {
            kind,
            domain,
            country,
            category,
            subcategory,
            specific,
            extra,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.kind as u8);
        buf.put_u8(self.domain);
        buf.put_u16(self.country as u16);
        buf.put_u8(self.category);
        buf.put_u8(self.subcategory);
        buf.put_u8(self.specific);
        buf.put_u8(self.extra);
    }

    pub fn decode(buf: &mut BytesMut) -> EntityType {
        EntityType {
            kind: EntityType::decode_kind(buf.get_u8()),
            domain: buf.get_u8(),
            country: EntityType::decode_country(buf.get_u16()),
            category: buf.get_u8(),
            subcategory: buf.get_u8(),
            specific: buf.get_u8(),
            extra: buf.get_u8(),
        }
    }

    fn decode_kind(data: u8) -> Kind {
        match data {
            1 => Kind::Platform,
            2 => Kind::Munition,
            3 => Kind::LifeForm,
            4 => Kind::Environmental,
            5 => Kind::CulturalFeature,
            6 => Kind::Supply,
            7 => Kind::Radio,
            8 => Kind::Expendable,
            9 => Kind::SensorEmittor,
            _ => Kind::Other,
        }
    }

    fn decode_country(data: u16) -> Country {
        match data {
            1 => Country::Afghanistan,
            2 => Country::Argentina,
            3 => Country::Indonesia,
            4 => Country::Iran,
            5 => Country::Iraq,
            6 => Country::Ireland,
            7 => Country::Israel,
            8 => Country::Italy,
            9 => Country::NorthKorea,
            120 => Country::SouthKorea,
            224 => Country::UnitedKingdom,
            225 => Country::UnitedStates,
            260 => Country::Russia,
            265 => Country::Ukraine,
            _ => Country::Other,
        }
    }
}
