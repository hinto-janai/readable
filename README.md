# Readable
[![CI](https://github.com/hinto-janai/readable/actions/workflows/ci.yml/badge.svg)](https://github.com/hinto-janai/readable/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/readable.svg)](https://crates.io/crates/readable) [![docs.rs](https://docs.rs/readable/badge.svg)](https://docs.rs/readable)

![readable](https://github.com/hinto-janai/readable/assets/101352116/2b4c0c1b-c80e-4b7a-a8b9-b86375382db9)

Human **readable** strings.

This library:
- Transforms various data types into human-readable strings
- Parses raw string data into human-readable versions
- Provides various string types and utilities

Most of the strings are implemented as fixed length, stack allocated arrays that are [`Copy`](https://doc.rust-lang.org/stable/std/marker/trait..html)-able.

In general, `readable` types are often used where you need to quickly format some data into a more human-friendly string, display it, then throw it away (although most `readable` types are perfectly fine to permanently store).

Creation of `readable` types is relatively performant.

## Examples
#### Unsigned integers:
```rust
let a = readable::Unsigned::from(1000_u64);
assert_eq!(a, 1000_u64);
assert_eq!(a, "1,000");
```
#### Signed integers:
```rust
let a = readable::Int::from(-1000);
assert_eq!(a, -1000);
assert_eq!(a, "-1,000");
```
#### Floats:
```rust
let a = readable::Float::from(1000.123);
assert_eq!(a, 1000.123);
assert_eq!(a, "1,000.123");
```
#### Percents:
```rust
let a = readable::Percent::from(1000.123);
assert_eq!(a, 1000.123);
assert_eq!(a, "1,000.12%");
```
#### Runtime:
```rust
let a = readable::Runtime::from(11111.0);
assert_eq!(a, 11111.0);
assert_eq!(a, "3:05:11");
```
#### Time:
```rust
let a = readable::TimeFull::from(86399_u32);
assert_eq!(a, 86399_u32);
assert_eq!(a, "23 hours, 59 minutes, 59 seconds");
```
#### Date:
```rust
let a = readable::Date::from_str("2014-12-31").unwrap();
assert_eq!(a, (2014, 12, 31));
assert_eq!(a, "2014-12-31");
```

## Comparison
All number types implement `PartialEq` against `str` and their internal numbers.

This is comparing `b`'s inner `String`:
```rust
let a = std::time::Duration::from_secs(86399);
let b = readable::TimeFull::from(a);
assert_eq!(b, "23 hours, 59 minutes, 59 seconds");
```
This is comparing `a`'s inner `i64`:
```rust
let a = readable::Int::from(-1000);
assert_eq!(a, -1000);
```
This compares both the `u64` AND `String` inside `a` and `b`:
```rust
let a = readable::Unsigned::from(1000_u64);
let b = readable::Unsigned::from(1000_u64);
assert_eq!(a, b);
```

## Arithmetic
All number types implement the common arithmetic operators `+`, `-`, `/`, `*`, `%`, outputting a new `Self`.
#### `+` Addition
```rust
let f1 = readable::Float::from(1.0);
let f2 = readable::Float::from(2.0);
assert_eq!(f1 + f2, 3.0);
```
#### `-` Subtraction
```rust
let p50 = readable::Percent::from(50.0);
let p25 = readable::Percent::from(25.0);
assert_eq!(p50 - p25, "25.00%");
```
#### `/` Division
```rust
let u100 = readable::Unsigned::from(100_u64);
let u10  = readable::Unsigned::from(10_u64);
assert_eq!(u100 / u10, 10);
```
#### `*` Muliplication
```rust
let u10 = readable::Unsigned::from(10_u64);
assert_eq!(u10 * u10, readable::Unsigned::from(100_u64));
```
#### `%` Modulo
```rust
let u10 = readable::Unsigned::from(10_u64);
assert_eq!(u10 % u10, 0);
```

## Feature Flags
| Flag             | Purpose |
|------------------|---------|
| `serde`          | Enables [`serde`](https://docs.rs/serde) on most types
| `bincode`        | Enables [`bincode 2.0.0-rc.3`](https://docs.rs/bincode/2.0.0-rc.3/bincode/index.html)'s `Encode/Decode` on most types
| `byte`           | Enables the `byte` module
| `date`           | Enables the `date` module
| `num`            | Enables the `num` module
| `run`            | Enables the `run` module
| `str`            | Enables the `str` module
| `time`           | Enables the `time` module
| `toa`            | Enables the `toa` module
| `full`           | Enables everything above. This is on by default.

## Re-Exports
Types are separated per module depending on what type of data they take as input, and what type of data they output.
```rust
use readable::num::{ // Number formatting
	Unsigned, // Formats u8, u16, u32, etc...
	Int,      // Formats i8, i16, i32, etc...
};
```

All major types are exported to the root, so they can be imported without specifying the full path:
```rust,ignore
use readable::HeadTail; // shorter, preferred.

use readable::str::HeadTail; // longer
```