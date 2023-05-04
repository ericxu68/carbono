# Carbono

[![crates.io](https://img.shields.io/crates/v/carbono.svg?style=flat-square)](https://crates.io/crates/carbono)
![cargo build](https://img.shields.io/github/actions/workflow/status/tjardoo/carbono/rust.yml?style=flat-square)
[![docs.rs](https://img.shields.io/docsrs/carbono?style=flat-square)](https://docs.rs/carbono)
[![crates.io](https://img.shields.io/crates/d/carbono.svg?style=flat-square)](https://crates.io/crates/carbono)

Carbono Dive is a simple Rust API extension for chrono DateTime.

```ini
[dependencies]
carbono = "0.1"
```

## Getting Started

```rust
use carbono::Carbono;

fn main() {
    let carbono = Carbono::now();

    println!("{}", carbono.datetime);
}
```
