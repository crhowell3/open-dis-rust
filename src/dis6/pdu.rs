use std::io::{Read, Write};

pub struct Pdu {
    // The version of the protocol
    protocol_version: u8,
    // Exercise ID
    exercise_id: u8,
    // Type of PDU, unique for each PDU class
    pdu_type: u8,
    // Value that refers to the protocol family
    protocol_family: u8,
    // Timestamp value
    timestamp: u32,
    // Length, in bytes, of the PDU
    length: u16,
    // Zero-filled array of padding
    padding: i16,
}

impl Pdu {
    pub fn new() -> Self {
        Pdu {
            protocol_version: 6,
            exercise_id: 0,
            pdu_type: 0,
            protocol_family: 0,
            timestamp: 0,
            length: 0,
            padding: 0,
        }
    }

    pub fn get_protocol_version(&self) -> u8 {
        self.protocol_version
    }

    pub fn set_protocol_version(&mut self, x: u8) {
        self.protocol_version = x;
    }

    pub fn get_exercise_id(&self) -> u8 {
        self.exercise_id
    }

    pub fn set_exercise_id(&mut self, x: u8) {
        self.exercise_id = x;
    }

    pub fn get_pdu_type(&self) -> u8 {
        self.pdu_type
    }

    pub fn set_pdu_type(&mut self, x: u8) {
        self.pdu_type = x;
    }

    pub fn get_protocol_family(&self) -> u8 {
        self.protocol_family
    }

    pub fn set_protocol_family(&mut self, x: u8) {
        self.protocol_family = x;
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, x: u32) {
        self.timestamp = x;
    }

    pub fn get_length(&self) -> u16 {
        self.length
    }

    pub fn set_length(&mut self, x: u16) {
        self.length = x;
    }

    pub fn get_padding(&self) -> i16 {
        self.padding
    }

    pub fn set_padding(&mut self, x: i16) {
        self.padding = x;
    }

    pub fn marshal(&self, data_stream: &mut dyn Write) {
        data_stream.write_all(&[self.protocol_version]).unwrap();
        data_stream.write_all(&[self.exercise_id]).unwrap();
        data_stream.write_all(&[self.pdu_type]).unwrap();
        data_stream.write_all(&[self.protocol_family]).unwrap();
        data_stream
            .write_all(&self.timestamp.to_be_bytes())
            .unwrap();
        data_stream.write_all(&self.length.to_be_bytes()).unwrap();
        data_stream.write_all(&self.padding.to_be_bytes()).unwrap();
    }

    pub fn unmarshal(&mut self, data_stream: &mut dyn Read) {
        let mut buffer = [0; 1];
        data_stream.read_exact(&mut buffer).unwrap();
        self.protocol_version = buffer[0];

        data_stream.read_exact(&mut buffer).unwrap();
        self.exercise_id = buffer[0];

        data_stream.read_exact(&mut buffer).unwrap();
        self.pdu_type = buffer[0];

        data_stream.read_exact(&mut buffer).unwrap();
        self.protocol_family = buffer[0];

        let mut timestamp_bytes = [0; 4];
        data_stream.read_exact(&mut timestamp_bytes).unwrap();
        self.timestamp = u32::from_be_bytes(timestamp_bytes);

        let mut length_bytes = [0; 2];
        data_stream.read_exact(&mut length_bytes).unwrap();
        self.length = u16::from_be_bytes(length_bytes);

        let mut padding_bytes = [0; 2];
        data_stream.read_exact(&mut padding_bytes).unwrap();
        self.padding = i16::from_be_bytes(padding_bytes);
    }

    pub fn get_marshalled_size(&self) -> usize {
        12
    }

    pub fn equals(&self, rhs: &Pdu) -> bool {
        self.protocol_version == rhs.protocol_version
            && self.exercise_id == rhs.exercise_id
            && self.pdu_type == rhs.pdu_type
            && self.protocol_family == rhs.protocol_family
            && self.timestamp == rhs.timestamp
            && self.length == rhs.length
            && self.padding == rhs.padding
    }
}
