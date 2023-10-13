use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct SpecificAppearance {
    pub land_platforms: LandPlatforms,
    pub air_platforms: AirPlatforms,
    pub surface_platforms: SurfacePlatforms,
    pub subsurface_platforms: SubSurfacePlatforms,
    pub space_platforms: SpacePlatforms,
    pub guided_munitions_platforms: GuidedMunitionPlatforms,
    pub life_forms: LifeForms,
    pub environmentals: Environmentals,
}

impl SpecificAppearance {
    pub fn new(
        land_platforms: LandPlatforms,
        air_platforms: AirPlatforms,
        surface_platforms: SurfacePlatforms,
        subsurface_platforms: SubSurfacePlatforms,
        space_platforms: SpacePlatforms,
        guided_munitions_platforms: GuidedMunitionsPlatforms,
        life_forms: LifeForms,
        environmentals: Environmentals,
    ) -> Self {
        SpecificAppearance {
            land_platforms,
            air_platforms,
            surface_platforms,
            subsurface_platforms,
            space_platforms,
            guided_munitions_platforms,
            life_forms,
            environmentals,
        }
    }

    pub fn default() -> Self {
        SpecificAppearance { land_platforms: LandPlatforms {
            launcher: Launcher::NotRaised,
            camouflage_type: CamouflageType::ForestCamoflage,
        }, air_platforms: AirPlatforms {}, surface_platforms: , subsurface_platforms: , space_platforms: , guided_munitions_platforms: , life_forms: , environmentals:  }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LandPlatforms {
    pub launcher: Launcher,
    pub camouflage_type: CamouflageType,
    pub concealed: Concealed,
    pub unused: u8,
    pub frozen_status: FrozenStatus,
    pub powerplant_status: PowerplantStatus,
    pub state: State,
    pub tent: Tent,
    pub ramp: Ramp,
    pub entity_specific: u8,
}

impl LandPlatforms {
    pub fn serialize(&self, buf: &mut BytesMut) {
        let launcher: u16 = self.launcher as u16;
        let launcher = launcher << 15;
        let camouflage : u16 = self.camouflage_type as u16;
        let camouflage = camouflage << 13;
        let concealed : u16 = self.concealed as u16;
        let concealed = concealed << 12;
        let frozen_status : u16 = self.frozen_status as u16;
        let frozen_status = frozen_status << 10;
        let power_plant_status : u16 = self.powerplant_status as u16;
        let power_plant_status = power_plant_status << 9;
        let state : u16 = self.state as u16;
        let state = state << 8;
        let tent : u16 = self.tent as u16;
        let tent = tent << 7;
        let ramp : u16 = self.ramp as u16;
        let ramp = ramp << 6;

        let land_appearance = 0u16 | launcher | camouflage | concealed
            | frozen_status | power_plant_status | state | tent | ramp;
        buf.put_u16(land_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> LandPlatforms {
        let bytes = buf.get_u16();
        LandPlatforms {
            launcher: Launcher::from_u8((bytes >> 1) as u8),
            camouflage: CamouflageType::from_u8((bytes >> 2) as u8),
            concealed: Concealed::from_u8((bytes >> 1) as u8),
            unused: (bytes >> 1) as u8,
            frozen_status: FrozenStatus::from_u8((bytes >> 1) as u8),
            powerplant_status: PowerplantStatus::from_u8((bytes >> 1) as u8),
            state: State::from_u8((bytes >> 1) as u8),
            tent: Tent::from_u8((bytes >> 1) as u8),
            ramp: Ramp::from_u8((bytes >> 1) as u8),
            entity_specific: (bytes >> 6) as u8,
        }
    }
}
