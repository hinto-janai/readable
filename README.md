# Readable
[![Windows](https://github.com/hinto-janai/readable/actions/workflows/windows.yml/badge.svg)](https://github.com/hinto-janai/readable/actions/workflows/windows.yml) [![macOS](https://github.com/hinto-janai/readable/actions/workflows/macos.yml/badge.svg)](https://github.com/hinto-janai/readable/actions/workflows/macos.yml) [![Linux](https://github.com/hinto-janai/readable/actions/workflows/linux.yml/badge.svg)](https://github.com/hinto-janai/readable/actions/workflows/linux.yml) [![crates.io](https://img.shields.io/crates/v/readable.svg)](https://crates.io/crates/readable) [![docs.rs](https://docs.rs/readable/badge.svg)](https://docs.rs/readable)

Human **readable** data formatting.

This crate turns various data into human-readable strings.

Most of the internal strings are implemented as fixed length, stack allocated arrays that are [`Copy`](https://doc.rust-lang.org/stable/std/marker/trait.Copy.html)-able.

## Feature flags
| Flag             | Purpose |
|------------------|---------|
| `serde`          | Enables [`serde`](https://docs.rs/serde) on all types
| `ignore_nan_inf` | Disables checking `f64`'s for `f64::NAN`, `f64::INFINITY`, and `f64::NEG_INFINITY`
| `inline_time`    | Inlines any `Time` that is under `1 hour, 1 minute` (`0..=3660`)
| `inline_runtime` | Inlines ALL of `Runtime` (`0:00..99:59:59`/`0..359999`)
| `full`           | Enables everything above

**Warning:** The `inline_*` features are disabled by default. While they increase speed,
they also _heavily_ increase build time and binary size.

## Unsigned integers:
```rust
let a = readable::Unsigned::from(1000_u64);

assert!(a == 1000_u64);
assert!(a == "1,000");
```

## Signed integers:
```rust
let a = readable::Int::from(-1000);

assert!(a == -1000);
assert!(a == "-1,000");
```

## Floats:
```rust
let a = readable::Float::from(1000.123);

assert!(a == 1000.123);
assert!(a == "1,000.123");
```

## Percents:
```rust
let a = readable::Percent::from(1000.123);

assert!(a == 1000.123);
assert!(a == "1,000.12%");
```

## Runtime:
```rust
let a = readable::Runtime::from(11111_u16);

assert!(a == 11111);
assert!(a == "3:05:11");
```

## Time:
```rust
let a = readable::Time::from(86399_u64);

assert!(a == 86399_u64);
assert!(a == "23 hours, 59 minutes, 59 seconds");
```

## Date:
```rust
let a = readable::Date::from_str("2014-12-31", '.').unwrap();

assert!(a == (2014, 12, 31));
assert!(a == "2014.12.31");
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
This compares both the `u64` AND `String` inside `a` and `b`.
