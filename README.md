<h1 align="center">
  <img
    src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png"
    height="30"
    width="0px"
  />
  🦀 open-dis-rust
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
      src="https://img.shields.io/maintenance/yes/2026?style=for-the-badge&color=98971a&labelColor=282a36"
    />
  </a>
</p>

&nbsp;

## 💭 About

Rust implementation of the IEEE 1278.1-2012 Distributed Interactive Simulation (DIS) application protocol. This library was implemented according to the IEEE Std 1278.1-2012 publication as well as [SISO-REF-010-2023](https://www.sisostandards.org/resource/resmgr/reference_documents_/siso-ref-010-2023-v32.zip).

## 📕 Documentation

The documentation for the latest version of this library can be found [here](https://docs.rs/open-dis-rust/).
All previously published versions of this package can be found on [crates.io](https://crates.io/crates/open-dis-rust/versions),
and each version's respective documentation is accessible from there as well.

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

### Examples

This package contains some examples for transmitting PDUs via UDP. To run the example, both the
client and the server applications need to be executed. Start by running the server as follows:

```shell
cargo run --example udp_server
```

Then, in another terminal on the same machine, run the client as follows:

```shell
cargo run --example udp_client
```

An Acknowledge PDU will be transmitted from the client to the server, and then the data within the PDU
will be echoed back to the client and displayed in the terminal.

<p align="center">
  Copyright &copy; 2026
  <a href="https://github.com/crhowell3" target="_blank">Cameron Howell</a>
</p>
<p align="center">
  <a href="https://github.com/crhowell3/open-dis-rust/blob/main/LICENSE"
    ><img
      src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=BSD-2-Clause&logoColor=d9e0ee&colorA=282a36&colorB=b16286"
  /></a>
</p>
