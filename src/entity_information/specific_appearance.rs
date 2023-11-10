use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct SpecificAppearance {
    pub land_platforms: LandPlatforms,
    pub air_platforms: AirPlatforms,
    pub surface_platforms: SurfacePlatforms,
    pub subsurface_platforms: SubSurfacePlatforms,
    pub space_platforms: SpacePlatforms,
    pub guided_munitions_platforms: GuidedMunitionsPlatforms,
    pub life_forms: LifeForms,
    pub environmentals: Environmentals,
}

impl Default for SpecificAppearance {
    fn default() -> Self {
        SpecificAppearance {
            land_platforms: LandPlatforms {
                launcher: Launcher::NotRaised,
                camouflage_type: CamouflageType::ForestCamoflage,
                concealed: Concealed::NotConcealed,
                unused: 0,
                frozen_status: FrozenStatus::NotFrozen,
                powerplant_status: PowerplantStatus::PowerplantOn,
                state: State::Active,
                tent: Tent::NotExtended,
                ramp: Ramp::Up,
                entity_specific: 0,
            },
            air_platforms: AirPlatforms {
                afterburner: Afterburner::AfterburnerNotOn,
                unused: 0,
                frozen_status: FrozenStatus::NotFrozen,
                powerplant_status: PowerplantStatus::PowerplantOn,
                state: State::Active,
                entity_specific: 0,
            },
            surface_platforms: SurfacePlatforms {
                unused: 0,
                frozen_status: FrozenStatus::NotFrozen,
                powerplant_status: PowerplantStatus::PowerplantOn,
                state: State::Active,
                entity_specific: 0,
            },
            subsurface_platforms: SubSurfacePlatforms {
                unused: 0,
                frozen_status: FrozenStatus::NotFrozen,
                powerplant_status: PowerplantStatus::PowerplantOn,
                state: State::Active,
                entity_specific: 0,
            },
            space_platforms: SpacePlatforms {
                unused: 0,
                frozen_status: FrozenStatus::NotFrozen,
                powerplant_status: PowerplantStatus::PowerplantOn,
                state: State::Active,
                entity_specific: 0,
            },
            guided_munitions_platforms: GuidedMunitionsPlatforms {
                launch_flash: LaunchFlash::LaunchFlashPresent,
                unused: 0,
                frozen_status: FrozenStatus::NotFrozen,
                powerplant_status: PowerplantStatus::PowerplantOn,
                state: State::Active,
                entity_specific: 0,
            },
            life_forms: LifeForms {
                lifeform_state: LifeFormState::UprightStandingStill,
                unused: 0,
                frozen_status: FrozenStatus::NotFrozen,
                activity_state: State::Active,
                weapon_1: LifeFormWeapon::NoWeaponPresent,
                weapon_2: LifeFormWeapon::NoWeaponPresent,
                entity_specific: 0,
            },
            environmentals: Environmentals {
                density: Density::Clear,
                unused: 0,
                entity_specific: 0,
            },
        }
    }
}

impl SpecificAppearance {
    #[allow(clippy::too_many_arguments)]
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

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.land_platforms.serialize(buf);
        self.air_platforms.serialize(buf);
        self.surface_platforms.serialize(buf);
        self.subsurface_platforms.serialize(buf);
        self.space_platforms.serialize(buf);
        self.guided_munitions_platforms.serialize(buf);
        self.life_forms.serialize(buf);
        self.environmentals.serialize(buf);
    }

    pub fn decode(buf: &mut BytesMut) -> SpecificAppearance {
        SpecificAppearance {
            land_platforms: LandPlatforms::decode(buf),
            air_platforms: AirPlatforms::decode(buf),
            surface_platforms: SurfacePlatforms::decode(buf),
            subsurface_platforms: SubSurfacePlatforms::decode(buf),
            space_platforms: SpacePlatforms::decode(buf),
            guided_munitions_platforms: GuidedMunitionsPlatforms::decode(buf),
            life_forms: LifeForms::decode(buf),
            environmentals: Environmentals::decode(buf),
        }
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
        let camouflage: u16 = self.camouflage_type as u16;
        let camouflage = camouflage << 13;
        let concealed: u16 = self.concealed as u16;
        let concealed = concealed << 12;
        let frozen_status: u16 = self.frozen_status as u16;
        let frozen_status = frozen_status << 10;
        let power_plant_status: u16 = self.powerplant_status as u16;
        let power_plant_status = power_plant_status << 9;
        let state: u16 = self.state as u16;
        let state = state << 8;
        let tent: u16 = self.tent as u16;
        let tent = tent << 7;
        let ramp: u16 = self.ramp as u16;
        let ramp = ramp << 6;

        let land_appearance = launcher
            | camouflage
            | concealed
            | frozen_status
            | power_plant_status
            | state
            | tent
            | ramp;
        buf.put_u16(land_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> LandPlatforms {
        let bytes = buf.get_u16();
        LandPlatforms {
            launcher: Launcher::from_u8((bytes >> 1) as u8),
            camouflage_type: CamouflageType::from_u8((bytes >> 2) as u8),
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

#[derive(Debug, Clone, Copy)]
pub struct AirPlatforms {
    pub afterburner: Afterburner,
    pub unused: u8,
    pub frozen_status: FrozenStatus,
    pub powerplant_status: PowerplantStatus,
    pub state: State,
    pub entity_specific: u8,
}

impl AirPlatforms {
    pub fn serialize(&self, buf: &mut BytesMut) {
        let afterburner: u16 = self.afterburner as u16;
        let afterburner = afterburner << 15;
        let frozen_status: u16 = self.frozen_status as u16;
        let frozen_status = frozen_status << 10;
        let power_plant_status: u16 = self.powerplant_status as u16;
        let power_plant_status = power_plant_status << 9;
        let state: u16 = self.state as u16;
        let state = state << 8;

        let air_appearance: u16 = afterburner | frozen_status | power_plant_status | state;
        buf.put_u16(air_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> AirPlatforms {
        let bytes = buf.get_u16();
        AirPlatforms {
            afterburner: Afterburner::from_u8((bytes >> 1) as u8),
            unused: (bytes >> 4) as u8,
            frozen_status: FrozenStatus::from_u8((bytes >> 1) as u8),
            powerplant_status: PowerplantStatus::from_u8((bytes >> 1) as u8),
            state: State::from_u8((bytes >> 1) as u8),
            entity_specific: (bytes >> 8) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SurfacePlatforms {
    pub unused: u8,
    pub frozen_status: FrozenStatus,
    pub powerplant_status: PowerplantStatus,
    pub state: State,
    pub entity_specific: u8,
}

impl SurfacePlatforms {
    pub fn serialize(&self, buf: &mut BytesMut) {
        let frozen_status: u16 = self.frozen_status as u16;
        let frozen_status = frozen_status << 10;
        let power_plant_status: u16 = self.powerplant_status as u16;
        let power_plant_status = power_plant_status << 9;
        let state: u16 = self.state as u16;
        let state = state << 8;

        let surface_appearance = frozen_status | power_plant_status | state;
        buf.put_u16(surface_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> SurfacePlatforms {
        let bytes = buf.get_u16();
        SurfacePlatforms {
            unused: (bytes >> 5) as u8,
            frozen_status: FrozenStatus::from_u8((bytes >> 1) as u8),
            powerplant_status: PowerplantStatus::from_u8((bytes >> 1) as u8),
            state: State::from_u8((bytes >> 1) as u8),
            entity_specific: (bytes >> 8) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubSurfacePlatforms {
    pub unused: u8,
    pub frozen_status: FrozenStatus,
    pub powerplant_status: PowerplantStatus,
    pub state: State,
    pub entity_specific: u8,
}

impl SubSurfacePlatforms {
    pub fn serialize(&self, buf: &mut BytesMut) {
        let frozen_status: u16 = self.frozen_status as u16;
        let frozen_status = frozen_status << 10;
        let power_plant_status: u16 = self.powerplant_status as u16;
        let power_plant_status = power_plant_status << 9;
        let state: u16 = self.state as u16;
        let state = state << 8;

        let subsurface_appearance = frozen_status | power_plant_status | state;
        buf.put_u16(subsurface_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> SubSurfacePlatforms {
        let bytes = buf.get_u16();
        SubSurfacePlatforms {
            unused: (bytes >> 5) as u8,
            frozen_status: FrozenStatus::from_u8((bytes >> 1) as u8),
            powerplant_status: PowerplantStatus::from_u8((bytes >> 1) as u8),
            state: State::from_u8((bytes >> 1) as u8),
            entity_specific: (bytes >> 8) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpacePlatforms {
    pub unused: u8,
    pub frozen_status: FrozenStatus,
    pub powerplant_status: PowerplantStatus,
    pub state: State,
    pub entity_specific: u8,
}

impl SpacePlatforms {
    pub fn serialize(&self, buf: &mut BytesMut) {
        let frozen_status: u16 = self.frozen_status as u16;
        let frozen_status = frozen_status << 10;
        let power_plant_status: u16 = self.powerplant_status as u16;
        let power_plant_status = power_plant_status << 9;
        let state: u16 = self.state as u16;
        let state = state << 8;

        let subsurface_appearance = frozen_status | power_plant_status | state;
        buf.put_u16(subsurface_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> SpacePlatforms {
        let bytes = buf.get_u16();
        SpacePlatforms {
            unused: (bytes >> 5) as u8,
            frozen_status: FrozenStatus::from_u8((bytes >> 1) as u8),
            powerplant_status: PowerplantStatus::from_u8((bytes >> 1) as u8),
            state: State::from_u8((bytes >> 1) as u8),
            entity_specific: (bytes >> 8) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GuidedMunitionsPlatforms {
    pub launch_flash: LaunchFlash,
    pub unused: u8,
    pub frozen_status: FrozenStatus,
    pub powerplant_status: PowerplantStatus,
    pub state: State,
    pub entity_specific: u8,
}

impl GuidedMunitionsPlatforms {
    pub fn serialize(&self, buf: &mut BytesMut) {
        let launch_flash: u16 = self.launch_flash as u16;
        let frozen_status: u16 = self.frozen_status as u16;
        let state: u16 = self.state as u16;

        let guided_appearance = (launch_flash << 15) | (frozen_status << 10) | (state << 8);
        buf.put_u16(guided_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> GuidedMunitionsPlatforms {
        let bytes = buf.get_u16();
        GuidedMunitionsPlatforms {
            launch_flash: LaunchFlash::from_u8((bytes >> 1) as u8),
            unused: (bytes >> 4) as u8,
            frozen_status: FrozenStatus::from_u8((bytes >> 1) as u8),
            powerplant_status: PowerplantStatus::from_u8((bytes >> 1) as u8),
            state: State::from_u8((bytes >> 1) as u8),
            entity_specific: (bytes >> 8) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LifeForms {
    pub lifeform_state: LifeFormState,
    pub unused: u8,
    pub frozen_status: FrozenStatus,
    pub activity_state: State,
    pub weapon_1: LifeFormWeapon,
    pub weapon_2: LifeFormWeapon,
    pub entity_specific: u8,
}

impl LifeForms {
    pub fn serialize(&self, buf: &mut BytesMut) {
        let life_form_state: u16 = self.lifeform_state as u16;
        let frozen_status: u16 = self.frozen_status as u16;
        let activity_state: u16 = self.activity_state as u16;
        let weapon_1: u16 = self.weapon_1 as u16;
        let weapon_2: u16 = self.weapon_2 as u16;

        let life_form_appearance = (life_form_state << 12)
            | (frozen_status << 10)
            | (activity_state << 8)
            | (weapon_1 << 6)
            | (weapon_2 << 4);
        buf.put_u16(life_form_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> LifeForms {
        let bytes = buf.get_u16();
        LifeForms {
            lifeform_state: LifeFormState::from_u8((bytes >> 4) as u8),
            unused: (bytes >> 1) as u8,
            frozen_status: FrozenStatus::from_u8((bytes >> 1) as u8),
            activity_state: State::from_u8((bytes >> 1) as u8),
            weapon_1: LifeFormWeapon::from_u8((bytes >> 2) as u8),
            weapon_2: LifeFormWeapon::from_u8((bytes >> 2) as u8),
            entity_specific: (bytes >> 4) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Environmentals {
    pub density: Density,
    pub unused: u8,
    pub entity_specific: u8,
}

impl Environmentals {
    pub fn serialize(&self, buf: &mut BytesMut) {
        let density: u16 = self.density as u16;
        let env_appearance = density << 12;
        buf.put_u16(env_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> Environmentals {
        let bytes = buf.get_u16();
        Environmentals {
            density: Density::from_u8((bytes >> 4) as u8),
            unused: (bytes >> 4) as u8,
            entity_specific: (bytes >> 8) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Launcher {
    NotRaised = 0,
    Raised = 1,
}

impl Launcher {
    pub fn from_u8(bit: u8) -> Launcher {
        match bit {
            0 => Launcher::NotRaised,
            1 => Launcher::Raised,
            _ => Launcher::NotRaised,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CamouflageType {
    DesertCamouflage = 0,
    WinterCamoflage = 1,
    ForestCamoflage = 2,
    Unused = 3,
}

impl CamouflageType {
    pub fn from_u8(bit: u8) -> CamouflageType {
        match bit {
            0 => CamouflageType::DesertCamouflage,
            1 => CamouflageType::WinterCamoflage,
            2 => CamouflageType::ForestCamoflage,
            3 => CamouflageType::Unused,
            _ => CamouflageType::Unused,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Concealed {
    NotConcealed = 0,
    EntityConcealed = 1,
}

impl Concealed {
    pub fn from_u8(bit: u8) -> Concealed {
        match bit {
            0 => Concealed::NotConcealed,
            1 => Concealed::EntityConcealed,
            _ => Concealed::NotConcealed,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FrozenStatus {
    NotFrozen = 0,
    Frozen = 1,
}

impl FrozenStatus {
    pub fn from_u8(bit: u8) -> FrozenStatus {
        match bit {
            0 => FrozenStatus::NotFrozen,
            1 => FrozenStatus::Frozen,
            _ => FrozenStatus::NotFrozen,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PowerplantStatus {
    PowerplantOff = 0,
    PowerplantOn = 1,
}

impl PowerplantStatus {
    pub fn from_u8(bit: u8) -> PowerplantStatus {
        match bit {
            0 => PowerplantStatus::PowerplantOff,
            1 => PowerplantStatus::PowerplantOn,
            _ => PowerplantStatus::PowerplantOff,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Active = 0,
    Deactivated = 1,
}

impl State {
    pub fn from_u8(bit: u8) -> State {
        match bit {
            0 => State::Active,
            1 => State::Deactivated,
            _ => State::Active,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tent {
    NotExtended = 0,
    Extended = 1,
}

impl Tent {
    pub fn from_u8(bit: u8) -> Tent {
        match bit {
            0 => Tent::NotExtended,
            1 => Tent::Extended,
            _ => Tent::NotExtended,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Ramp {
    Up = 0,
    Down = 1,
}

impl Ramp {
    pub fn from_u8(bit: u8) -> Ramp {
        match bit {
            0 => Ramp::Up,
            1 => Ramp::Down,
            _ => Ramp::Up,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Afterburner {
    AfterburnerNotOn = 0,
    AfterburnerOn = 1,
}

impl Afterburner {
    pub fn from_u8(bit: u8) -> Afterburner {
        match bit {
            0 => Afterburner::AfterburnerNotOn,
            1 => Afterburner::AfterburnerOn,
            _ => Afterburner::AfterburnerNotOn,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LaunchFlash {
    NoLaunchFlashPresent = 0,
    LaunchFlashPresent = 1,
}

impl LaunchFlash {
    pub fn from_u8(bit: u8) -> LaunchFlash {
        match bit {
            0 => LaunchFlash::NoLaunchFlashPresent,
            1 => LaunchFlash::LaunchFlashPresent,
            _ => LaunchFlash::NoLaunchFlashPresent,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LifeFormState {
    Null = 0,
    UprightStandingStill = 1,
    UprightWalking = 2,
    UprightRunning = 3,
    Kneeling = 4,
    Prone = 5,
    Crawling = 6,
    Swimming = 7,
    Parachuting = 8,
    Jumping = 9,
}

impl LifeFormState {
    pub fn from_u8(bit: u8) -> LifeFormState {
        match bit {
            0 => LifeFormState::Null,
            1 => LifeFormState::UprightStandingStill,
            2 => LifeFormState::UprightWalking,
            3 => LifeFormState::UprightRunning,
            4 => LifeFormState::Kneeling,
            5 => LifeFormState::Prone,
            6 => LifeFormState::Crawling,
            7 => LifeFormState::Swimming,
            8 => LifeFormState::Parachuting,
            9 => LifeFormState::Jumping,
            _ => LifeFormState::Null,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum LifeFormWeapon {
    #[default]
    NoWeaponPresent = 0,
    WeaponIsStowed = 1,
    WeaponIsDeployed = 2,
    WeaponInFiringPositon = 3,
}

impl LifeFormWeapon {
    pub fn from_u8(bit: u8) -> LifeFormWeapon {
        match bit {
            0 => LifeFormWeapon::NoWeaponPresent,
            1 => LifeFormWeapon::WeaponIsStowed,
            2 => LifeFormWeapon::WeaponIsDeployed,
            3 => LifeFormWeapon::WeaponInFiringPositon,
            _ => LifeFormWeapon::NoWeaponPresent,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Density {
    #[default]
    Clear = 0,
    Hazy = 1,
    Dense = 2,
    VeryDense = 3,
    Opaque = 4,
}

impl Density {
    pub fn from_u8(bit: u8) -> Density {
        match bit {
            0 => Density::Clear,
            1 => Density::Hazy,
            2 => Density::Dense,
            3 => Density::VeryDense,
            4 => Density::Opaque,
            _ => Density::Clear,
        }
    }
}
