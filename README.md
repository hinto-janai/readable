# Readable
[![CI](https://github.com/hinto-janai/readable/actions/workflows/ci.yml/badge.svg)](https://github.com/hinto-janai/readable/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/readable.svg)](https://crates.io/crates/readable) [![docs.rs](https://docs.rs/readable/badge.svg)](https://docs.rs/readable)

![readable](https://github.com/hinto-janai/readable/assets/101352116/2b4c0c1b-c80e-4b7a-a8b9-b86375382db9)

Human **readable** strings.

This library:
- Transforms various data types into human-readable strings
- Parses raw string data into human-readable versions
- Provides various string types and utilities

Most of the strings are implemented as fixed sized stack allocated arrays that are [`Copy`](https://doc.rust-lang.org/stable/std/marker/trait..html)-able.

In general, `readable` types are often used where you need to quickly format some data into a more human-friendly string, display it, then throw it away (although most `readable` types are perfectly fine to permanently store).

Creation of `readable` types is relatively performant.

## Examples
#### Unsigned
```rust
use readable::*;
assert_eq!(Unsigned::from(1000_u64), "1,000");
```
#### Int
```rust
use readable::*;
assert_eq!(Int::from(-1000), "-1,000");
```
#### Float
```rust
use readable::*;
assert_eq!(Float::from(1000.123), "1,000.123");
```
#### Percent
```rust
use readable::*;
assert_eq!(Percent::from(1000.123), "1,000.12%");
```
#### Runtime
```rust
use readable::*;
assert_eq!(Runtime::from(311.123),      "5:11");
assert_eq!(RuntimePad::from(311.123),   "00:05:11");
assert_eq!(RuntimeMilli::from(311.123), "00:05:11.123");
```
#### Uptime
```rust
use readable::*;
assert_eq!(Uptime::from(172799_u32),     "1d, 23h, 59m, 59s");
assert_eq!(UptimeFull::from(172799_u32), "1 day, 23 hours, 59 minutes, 59 seconds");
assert_eq!(Htop::from(172799_u32),       "1 day, 23:59:59");
```
#### Date
```rust
use readable::*;
assert_eq!(Date::from_ymd(2014, 12, 31).unwrap(), "2014-12-31");
assert_eq!(Nichi::new(2014, 12, 31).unwrap(),     "Wed, Dec 31, 2014");
assert_eq!(NichiFull::new(2014, 12, 31).unwrap(), "Wednesday, December 31st, 2014");
```
#### Time
```rust
use readable::*;
assert_eq!(Time::new(86399),     "11:59:59 PM");
assert_eq!(Military::new(86399), "23:59:59");
```
#### Byte
```rust
use readable::*;
assert_eq!(Byte::from(1234), "1.234 KB");
```

## Comparison
All number types implement `PartialEq` against `str` and their internal numbers.

This is comparing `b`'s inner `String`:
```rust
use readable::*;
let a = std::time::Duration::from_secs(86399);
let b = UptimeFull::from(a);
assert_eq!(b, "23 hours, 59 minutes, 59 seconds");
```
This is comparing `a`'s inner `i64`:
```rust
use readable::*;
let a = Int::from(-1000);
assert_eq!(a, -1000);
```
This compares both the `u64` AND `String` inside `a` and `b`:
```rust
use readable::*;
let a = Unsigned::from(1000_u64);
let b = Unsigned::from(1000_u64);
assert_eq!(a, b);
```

## Arithmetic
All number types implement the common arithmetic operators `+`, `-`, `/`, `*`, `%`, outputting a new `Self`.
#### `+` Addition
```rust
use readable::*;
let f1 = Float::from(1.0);
let f2 = Float::from(2.0);
assert_eq!(f1 + f2, 3.0);
```
#### `-` Subtraction
```rust
use readable::*;
let p50 = Percent::from(50.0);
let p25 = Percent::from(25.0);
assert_eq!(p50 - p25, "25.00%");
```
#### `/` Division
```rust
use readable::*;
let u100 = Unsigned::from(100_u64);
let u10  = Unsigned::from(10_u64);
assert_eq!(u100 / u10, 10);
```
#### `*` Multiplication
```rust
use readable::*;
let u10 = Unsigned::from(10_u64);
assert_eq!(u10 * u10, Unsigned::from(100_u64));
```
#### `%` Modulo
```rust
use readable::*;
let u10 = Unsigned::from(10_u64);
assert_eq!(u10 % u10, 0);
```

## Feature Flags
| Flag             | Purpose |
|------------------|---------|
| `serde`          | Enables [`serde`](https://docs.rs/serde) on most types
| `bincode`        | Enables [`bincode 2.0.0-rc.3`](https://docs.rs/bincode/2.0.0-rc.3/bincode/index.html)'s `Encode/Decode` on most types

## Re-exports
Types are separated per module depending on what type of data they take as input, and what type of data they output.
```rust
use readable::num::{ // Number formatting
	Unsigned, // Formats u8, u16, u32, etc...
	Int,      // Formats i8, i16, i32, etc...
};
```

All major types are exported to the root, so they can be imported without specifying the full path:
```rust
// shorter
use readable::HeadTail;

// longer
// use readable::str::HeadTail;
```
