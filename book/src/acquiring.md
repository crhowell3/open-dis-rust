# Acquiring Open DIS Rust

- [From Crates.io](#crates.io)
- [Build from Source](#build-from-source)

### crates.io

In your project, run `cargo add open-dis-rust` or add `open-dis-rust = "<version>"` to your Cargo.toml.

### Build from Source

Clone the open-dis-rust GitHub repository and build with cargo.

Requirements:

* [Rust toolchain](https://www.rust-lang.org/tools/install)
* [Git version control](https://git-scm.com/)

```shell
# Clone the repo
git clone https://github.com/crhowell3/open-dis-rust.git

cd open-dis-rust

# Build and test
cargo build --release
cargo test
```
