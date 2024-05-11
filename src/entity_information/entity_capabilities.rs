use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct EntityCapabilities {
    pub ammunition_supply: bool,
    pub fuel_supply: bool,
    pub recovery: bool,
    pub repair: bool,
}

impl EntityCapabilities {
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)]
    pub fn new(ammunition_supply: bool, fuel_supply: bool, recovery: bool, repair: bool) -> Self {
        EntityCapabilities {
            ammunition_supply,
            fuel_supply,
            recovery,
            repair,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        let ammunition_supply = u32::from(self.ammunition_supply) << 31;
        let fuel_supply = u32::from(self.fuel_supply) << 30;
        let recovery = u32::from(self.recovery) << 29;
        let repair = u32::from(self.repair) << 28;
        let capabilities = ammunition_supply | fuel_supply | recovery | repair;
        buf.put_u32(capabilities);
    }

    pub fn decode(buf: &mut BytesMut) -> EntityCapabilities {
        let bytes = buf.get_u32();
        EntityCapabilities {
            ammunition_supply: (bytes >> 1) != 0,
            fuel_supply: (bytes >> 1) != 0,
            recovery: (bytes >> 1) != 0,
            repair: (bytes >> 1) != 0,
        }
    }
}
