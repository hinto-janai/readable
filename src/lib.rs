//! Human **readable** data formatting.
//!
//! This crate turns various data into human-readable strings.
//!
//! For performance, the actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html), but a [`CompactString`](https://docs.rs/compact_str) so that any string 24 bytes (12 bytes on 32-bit) or less are _stack_ allocated instead of _heap_ allocated.
//!
//! The documentation will still refer to the inner string as a [`String`]. Anything returned will also be a [`String`].
//!
//! # Feature flags
//! | Flag             | Purpose |
//! |------------------|---------|
//! | `serde`          | Enable [`serde`](https://docs.rs/serde) on all types
//! | `ignore_nan_inf` | Disable checking [`f64`]'s for [`f64::NAN`], [`f64::INFINITY`], and [`f64::NEG_INFINITY`]
//!
//! ## Integers:
//! ```
//! let a = readable::Int::from(1000_u16);
//! println!("{}", a);
//!
//! // 1,000
//! ```
//!
//! ## Floats:
//! ```
//! let a = readable::Float::from(1000.123);
//! let b = readable::Float::percent(1000.123);
//! println!("{}", a);
//! println!("{}", b);
//!
//! // 1,000.123
//! // 1,000.12%
//! ```
//!
//! ## Runtime:
//! ```
//! let a = readable::Runtime::from(11111.1);
//! println!("{}", a);
//!
//! // 3:05:11
//! ```
//!
//! ## Time:
//! ```
//! let a = std::time::Duration::from_secs(86399);
//! let b = readable::Time::from(a);
//! println!("{}", b);
//!
//! // 23 hours, 59 minutes, 59 seconds
//! ```

mod int;
pub use int::*;

mod float;
pub use float::*;

mod time;
pub use time::*;

mod runtime;
pub use runtime::*;
