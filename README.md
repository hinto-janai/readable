# Readable
[![Windows](https://github.com/hinto-janai/readable/actions/workflows/windows.yml/badge.svg)](https://github.com/hinto-janai/readable/actions/workflows/windows.yml) [![macOS](https://github.com/hinto-janai/readable/actions/workflows/macos.yml/badge.svg)](https://github.com/hinto-janai/readable/actions/workflows/macos.yml) [![Linux](https://github.com/hinto-janai/readable/actions/workflows/linux.yml/badge.svg)](https://github.com/hinto-janai/readable/actions/workflows/linux.yml) [![crates.io](https://img.shields.io/crates/v/readable.svg)](https://crates.io/crates/readable) [![docs.rs](https://docs.rs/readable/badge.svg)](https://docs.rs/readable)

Human **readable** data formatting.

This crate turns various data into human-readable strings.

For performance, the actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html), but a [`CompactString`](https://docs.rs/compact_str) so that any string 24 bytes (12 bytes on 32-bit) or less are _stack_ allocated instead of _heap_ allocated.

The documentation will still refer to the inner string as a `String`. Anything returned will also be a `String`.

## Feature flags
| Flag             | Purpose |
|------------------|---------|
| `serde`          | Enable [`serde`](https://docs.rs/serde) on all types
| `ignore_nan_inf` | Disable checking `f64`'s for `f64::NAN`, `f64::INFINITY`, and `f64::NEG_INFINITY`

## Examples
#### Unsigned Integers
```rust
let a = readable::Unsigned::from(1000);
println!("{}", a);

> 1,000
```

#### Signed Integers
```rust
let a = readable::Int::from(-1000);
println!("{}", a);

> -1,000
```

#### Floats
```rust
let a = readable::Float::from(1000.123);
let b = readable::Float::percent(1000.123);
println!("{}", a);
println!("{}", b);

> 1,000.123
> 1,000.12%
```

#### Runtime
```rust
let a = readable::Runtime::from(11111.1);
println!("{}", a);

> 3:05:11
```

#### Time
```rust
let a = std::time::Duration::from_secs(86399);
let b = readable::Time::from(a);
println!("{}", b);

> 23 hours, 59 minutes, 59 seconds
```

## Comparison
All types implement `Display`, `PartialEq`, `PartialEq<&str>` and `PartialEq` for their inner number primitive.

Example 1:
```rust
let a = std::time::Duration::from_secs(86399);
let b = readable::Time::from(a);

assert!(b == "23 hours, 59 minutes, 59 seconds");
```
This is comparing `b`'s inner `String`.

Example 2:
```rust
let a = readable::Int::from(-1000);

assert!(a == -1000);
```
This is comparing `a`'s inner `i64`.

Example 3:
```rust
let a = readable::Unsigned::from(1000);
let b = readable::Unsigned::from(1000);

assert!(a == b);
```
This compare both the `u64` AND `String` inside `a` and `b`.
