# typed-bytes

> A `![no_std]`, const first Rust library for strongly-typed data size units (KiB, MiB, KB, MB, etc.).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typed-bytes = "0.1.0"
```

## Usage

This library provides both IEC and SI units.

### IEC Units (Base 2)

```rust
use typed_bytes::iec::{KiB, MiB, GiB};
use typed_bytes::Bytes;

let size = KiB(5);
let bytes: Bytes = size.into(); // 5 * 1024 = 5120 bytes

assert_eq!(bytes, Bytes(5120));
assert_eq!(MiB(1).as_kib(), KiB(1024));
```

### SI Units (Base 10)

```rust
use typed_bytes::si::{KB, MB, GB};
use typed_bytes::Bytes;

let size = KB(5);
let bytes: Bytes = size.into(); // 5 * 1000 = 5000 bytes

assert_eq!(bytes, Bytes(5000));
assert_eq!(MB(1).as_kb(), KB(1000));
```

## Comparisons & Safety

To prevent "footguns" (accidental mixing of binary and decimal units), there are **no implicit comparisons** between SI and IEC units. You must strictly convert them to a common type (like `Bytes`) or explicitly convert one to the other before comparing.

This compilation failure is a feature, not a bug:

```rust,compile_fail
use typed_bytes::iec::KiB;
use typed_bytes::si::KB;

let kib = KiB(1);
let kb = KB(1);

// This fails to compile!
if kib > kb {
    println!("Mixed comparison");
}
```

This will compile: 

```rust
use typed_bytes::iec::KiB;
use typed_bytes::si::KB;
use typed_bytes::Bytes;

let kib = KiB(1);
let kb = KB(1);

if Bytes::from(kib) > Bytes::from(kb) {
    println!("1 KiB is greater than 1 KB");
}
```