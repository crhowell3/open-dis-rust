#[derive(Clone, Debug, Default)]
pub struct IntercomCommunicationsParameters {
    pub record_type: u16,
    pub record_length: u16,
    pub record_specific_field: u32,
}
