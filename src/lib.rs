//! Human **readable** data formatting.
//!
//! This crate turns various data into human-readable strings.
//!
//! Most of the internal strings are implemented as fixed length, stack allocated arrays that are [`Copy`](https://doc.rust-lang.org/stable/std/marker/trait.Copy.html)-able.
//!
//! # Feature flags
//! | Flag             | Purpose |
//! |------------------|---------|
//! | `serde`          | Enables [`serde`](https://docs.rs/serde) on all types
//! | `ignore_nan_inf` | Disables checking `f64`'s for `f64::NAN`, `f64::INFINITY`, and `f64::NEG_INFINITY`
//! | `inline_date`    | Inlines any `Date` that is in `YYYY-MM-HH` format and is between year `1900-2100`
//! | `inline_time`    | Inlines any `Time` that is under `1 hour, 1 minute` (`0..=3660`)
//! | `inline_runtime` | Inlines ALL of `Runtime` (`0:00..99:59:59`/`0..=359999`)
//! | `full`           | Enables everything above
//!
//! **Warning:** The `inline_*` features are disabled by default. While they increase speed,
//! they also _heavily_ increase build time and binary size.
//!
//! ## Unsigned integers:
//! ```
//! let a = readable::Unsigned::from(1000_u64);
//!
//! assert!(a == 1000_u64);
//! assert!(a == "1,000");
//! ```
//!
//! ## Signed integers:
//! ```
//! let a = readable::Int::from(-1000);
//!
//! assert!(a == -1000);
//! assert!(a == "-1,000");
//! ```
//!
//! ## Floats:
//! ```
//! let a = readable::Float::from(1000.123);
//!
//! assert!(a == 1000.123);
//! assert!(a == "1,000.123");
//! ```
//!
//! ## Percents:
//! ```
//! let a = readable::Percent::from(1000.123);
//!
//! assert!(a == 1000.123);
//! assert!(a == "1,000.12%");
//! ```
//!
//! ## Runtime:
//! ```
//! let a = readable::Runtime::from(11111_u16);
//!
//! assert!(a == 11111);
//! assert!(a == "3:05:11");
//! ```
//!
//! ## Time:
//! ```
//! let a = readable::Time::from(86399_u64);
//!
//! assert!(a == 86399_u64);
//! assert!(a == "23 hours, 59 minutes, 59 seconds");
//! ```
//!
//! ## Date:
//! ```rust
//! let a = readable::Date::from_str("2014-12-31").unwrap();
//!
//! assert!(a == (2014, 12, 31));
//! assert!(a == "2014-12-31");
//! ```
//!
//! ## Comparison
//! All types implement `Display`, `PartialEq`, `PartialEq<&str>` and `PartialEq` for their inner number primitive.
//!
//! Example 1:
//! ```rust
//! let a = std::time::Duration::from_secs(86399);
//! let b = readable::Time::from(a);
//!
//! assert!(b == "23 hours, 59 minutes, 59 seconds");
//! ```
//! This is comparing `b`'s inner `String`.
//!
//! Example 2:
//! ```rust
//! let a = readable::Int::from(-1000);
//!
//! assert!(a == -1000);
//! ```
//! This is comparing `a`'s inner `i64`.
//!
//! Example 3:
//! ```rust
//! let a = readable::Unsigned::from(1000_u64);
//! let b = readable::Unsigned::from(1000_u64);
//!
//! assert!(a == b);
//! ```
//! This compares both the `u64` AND `String` inside `a` and `b`.

pub(crate) mod inner;
pub(crate) mod macros;
//pub(crate) mod utf8;

mod constants;
pub use constants::*;

mod date;
pub use date::*;

mod unsigned;
pub use unsigned::*;

mod int;
pub use int::*;

mod float;
pub use float::*;

mod percent;
pub use percent::*;

mod time;
pub use time::*;

mod runtime;
pub use runtime::*;
