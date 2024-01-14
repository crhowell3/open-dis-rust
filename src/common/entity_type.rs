//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

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

impl Default for EntityType {
    fn default() -> Self {
        EntityType {
            kind: Kind::Other,
            domain: 0,
            country: Country::Other,
            category: 0,
            subcategory: 0,
            specific: 0,
            extra: 0,
        }
    }
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
            2 => Country::Albania,
            3 => Country::Algeria,
            4 => Country::AmericanSamoa,
            5 => Country::Andorra,
            6 => Country::Angola,
            7 => Country::Anguilla,
            8 => Country::Antarctica,
            9 => Country::AntiguaAndBarbuda,
            10 => Country::Argentina,
            120 => Country::SouthKorea,
            224 => Country::UnitedKingdom,
            225 => Country::UnitedStates,
            260 => Country::Russia,
            265 => Country::Ukraine,
            _ => Country::Other,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Kind {
    Other = 0,
    Platform = 1,
    Munition = 2,
    LifeForm = 3,
    Environmental = 4,
    CulturalFeature = 5,
    Supply = 6,
    Radio = 7,
    Expendable = 8,
    SensorEmittor = 9,
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum Country {
    Other = 0,
    Afghanistan = 1,
    Albania = 2,
    Algeria = 3,
    AmericanSamoa = 4,
    Andorra = 5,
    Angola = 6,
    Anguilla = 7,
    Antarctica = 8,
    AntiguaAndBarbuda = 9,
    Argentina = 10,
    SouthKorea = 120,
    UnitedKingdom = 224,
    UnitedStates = 225,
    Russia = 260,
    Ukraine = 265,
}
