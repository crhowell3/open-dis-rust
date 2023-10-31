#[derive(Copy, Clone, Debug)]
pub struct ElectromagneticEmissionSystemData {
    pub system_data_length: u8,
    pub number_of_beams: u8,
    pub emissions_padding2: u8,
    pub emitter_system: EmitterSystem,
    pub location: Vector3Float,
    pub beam_data_records: Vec<ElectromagneticEmissionBeamData>,
}
