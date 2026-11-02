# Installation

## Requirements

- **Rust 1.85 or newer.** The crate uses the Rust 2024 edition, which first shipped in
  Rust 1.85. Check the installed version with `rustc --version` and update with `rustup update stable`.
- **Cargo**, which comes with Rust. If you don't have Rust yet, install it with [rustup](https://www.rust-lang.org/tools/install).

This crate is pure Rust with no system libraries to install, so it builds anywhere that Rust does,
including Linux, macOS, and Windows.

## Add the crate to your project

From your project's directory, run:

```shell
cargo add open-dis-rust bytes
```

Or add the dependencies to `Cargo.toml` yourself:

```toml
[dependencies]
open-dis-rust = "1"
bytes = "1"
```

The `bytes` because serialization writes into a `bytes::BytesMut` buffer.

The package name uses hyphens, but in Rust code, the crate is referred to with underscores:

```rust,ignore
use open_dis_rust::entity_information::EntityStatePdu;
```

### Async networking (optional)

The crate doesn't depend on any networking library; however, the `tokio` crate is among
the gold standards of Rust networking and async I/O.

```shell
cargo add tokio --features full
```

The [Quickstart](quickstart.md) guide uses `std::net::UdpSocket`, so `tokio` is not
needed to follow along.

## Validate

To quickly test the crate, place the following code in your application's `src/main.rs`:

```rust,ignore
use bytes::BytesMut;
use open_dis_rust::{common::Pdu, simulation_management::AcknowledgePdu};

fn main() {
    let mut pdu = AcknowledgePdu::new();
    let mut bytes = BytesMut::new();
    pdu.serialize(&mut bytes).expect("serialize failed");
    println!("open-dis-rust {} encoded the following {}-byte PDU: {:?}" open_dis_rust::get_version(), bytes.len(), pdu);
}
```

Then, execute `cargo run` to see a printout of the crate version and a view into the PDU.

## Build from source

If you plan on contributing to the development of the crate, or if you want to modify the code for your
own project, simply clone down the repo and build it yourself. You will need [git](https://git-scm.com)
and Rust.

```shell
git clone https://github.com/crhowell3/open-dis-rust.git
cd open-dis-rust

cargo build --release
cargo test -q
```

If you make modifications and wish to test them against personal projects, add this to your
Cargo.toml:

```toml
[dependencies]
open-dis-rust = { path = "path/to/open-dis-rust" }
```

## Read documentation offline

The API reference is published on [docs.rs](https://docs.rs/open-dis-rust). To build it
locally and open it in your browser:

```shell
cargo doc --open
```

To preview this book locally, install [mdBook](https://rust-lang.github.io/mdBook/) and
serve it from the `book` folder:

```shell
cargo install mdbook
mdbook serve book --open
```
