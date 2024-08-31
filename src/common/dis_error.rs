//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#[derive(Debug)]
/// Enumeration for describing types of errors that may occur related to PDUs
pub enum DISError {
    InvalidDISHeader,
}
