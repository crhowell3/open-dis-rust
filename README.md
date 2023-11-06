<h1 align="center">
  <img
    src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png"
    height="30"
    width="0px"
  />
  🦀 Open DIS Rust
  <img
    src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png"
    height="30"
    width="0px"
  />
</h1>

<p align="center">
  <a href="https://github.com/crhowell3/open-dis-rust/stargazers">
    <img
      alt="Stargazers"
      src="https://img.shields.io/github/stars/crhowell3/open-dis-rust?style=for-the-badge&logo=starship&color=b16286&logoColor=d9e0ee&labelColor=282a36"
    />
  </a>
  <a href="https://crates.io/crates/open-dis-rust">
    <img
      alt="Crates.io Version"
      src="https://img.shields.io/crates/v/open-dis-rust?style=for-the-badge&logo=rust&color=458588&logoColor=d9e0ee&labelColor=282a36"
    />
  </a>
  <a href="https://github.com/crhowell3/open-dis-rust/issues">
    <img
      alt="Issues"
      src="https://img.shields.io/github/issues/crhowell3/open-dis-rust?style=for-the-badge&logo=gitbook&color=d79921&logoColor=d9e0ee&labelColor=282a36"
    />
  </a>
  <a href="https://github.com/crhowell3/open-dis-rust/contributors">
    <img
      alt="Contributors"
      src="https://img.shields.io/github/contributors/crhowell3/open-dis-rust?style=for-the-badge&logo=opensourceinitiative&color=689d6a&logoColor=d9e0ee&labelColor=282a36"
    />
  </a>
  <br/>
  <a href="#">
    <img
      alt="Documentation"
      src="https://img.shields.io/docsrs/open-dis-rust?style=for-the-badge&logo=docsdotrs&color=98971a&logoColor=d9e0ee&labelColor=282a36"
    />
  </a>
  <a href="#">
    <img
      alt="Maintained"
      src="https://img.shields.io/maintenance/yes/2023?style=for-the-badge&color=98971a&labelColor=282a36"
    />
  </a>
</p>

&nbsp;

## 💭 About
Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation (DIS) application protocol. This library was implemented according to the IEEE Std 1278.1-2012 publication as well as [SISO-REF-010-2020](https://www.sisostandards.org/resource/resmgr/reference_documents_/siso-ref-010-2023-v31.zip).

## 📕 Documentation
The documentation for the latest version of this library can be found [here](https://docs.rs/open-dis-rust/). All previously published versions of this package can be found on [crates.io](https://crates.io/crates/open-dis-rust/versions), and each version's respective documentation is accessible from there as well.

## 🔰 Getting Started
### Installation
This library can be installed using cargo:
```shell
cargo add open-dis-rust
```
or by adding this to your project's Cargo.toml `[dependencies]` section:
```toml
open-dis-rust = "<insert version>"
```

### Example Usage
```rust
use bytes::BytesMut;

// Import the Open DIS crate
extern crate open_dis_rust;
use open_dis_rust::simulation_management::acknowledge_pdu::AcknowledgePdu;

// Create a new AcknowledgePdu with default data
let ack_pdu = AcknowledgePdu::default();
let mut buffer = BytesMut::new();
Pdu::serialize(&ack_pdu, &mut buffer);
```

<p align="center">
  Copyright &copy; 2023-present
  <a href="https://github.com/crhowell3" target="_blank">Cameron Howell</a>
</p>
<p align="center">
  <a href="https://github.com/crhowell3/open-dis-rust/blob/main/LICENSE"
    ><img
      src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=BSD-2-Clause&logoColor=d9e0ee&colorA=282a36&colorB=b16286"
  /></a>
</p>


