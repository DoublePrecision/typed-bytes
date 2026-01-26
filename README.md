# typed-bytes

> A `![no_std]`, const first Rust library for strongly-typed data size units (KiB, MiB, KB, MB, etc.).


## Features

- `no_std`
- `const`

## Why

During our work at [haxiom.io](https://haxiom.io) we found many cases of file size comparisons 
where we accidentally mixed IEC and SI units. Likewise, using plain `u64` for file sizes 
led to many bugs even if they are named correctly, for example `MAX_FILE_SIZE` could be 
interpreted as either bytes or kilobytes! This library provides a safe and type-safe way to 
handle file sizes with proper units, ensuring that the correct unit is used in each 
comparison. Note that most of the work has been lifted from the internal implementation at haxiom internal
repository.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typed-bytes = "1.0.0"
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

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.