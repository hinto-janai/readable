//! Human-readable time
//!
//! "Uptime"-style string, e.g: `19h, 54m, 39s` and `19 hours, 54 minutes, 39 seconds`
//!
//! ## Input
//! **The input is always assumed to be in seconds.**
//!
//! [`From`] input can be:
//! - Any unsigned integer [`u8`], [`usize`], etc
//! - Any signed integer [`i8`], [`isize`], etc
//! - [`f32`] or [`f64`]
//! - [`std::time::Duration`]
//! - [`std::time::Instant`]
//! - Other [`Time`] types
//!
//! ## Errors
//! The max input is [`u32::MAX`] seconds.
//!
//! A [`Time::unknown()`] (or a variant's version of it) will be returned if the input is:
//! - A negative integer
//! - Larger than [`MAX_RUNTIME_F32`]
//! - [`f32::NAN`], [`f32::INFINITY`], [`f32::NEG_INFINITY`] (or the [`f64`] versions)
//!
//! ## Formatting
//! The lowest unit is `second`, the highest is `year`, and `week` is skipped in favor of `7 days`.
//!
//! ## Copy
//! [`Copy`] is available.
//!
//! The actual strings used internally are not [`String`](https://doc.rust-lang.org/std/string/struct.String.html)'s,
//! but byte array buffer(s). See the specific type for more details.
//!
//! The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
//!
//! ```
//! # use readable::*;
//! let a = Time::from(100_000);
//!
//! // Copy 'a', use 'b'.
//! let b = a;
//! assert_eq!(b, 100_000);
//!
//! // We can still use 'a'
//! assert_eq!(a, 100_000);
//! ```
//!
//! ## Math
//! These operators are overloaded. They will always output a new [`Self`]:
//! - `Add +`
//! - `Sub -`
//! - `Div /`
//! - `Mul *`
//! - `Rem %`
//!
//! They can either be:
//! - Combined with another [`Self`]: `Time::from(1) + Time::from(1)`
//! - Or with the inner number itself: `Time::from(1) + 1`
//!
//! They also have the same `panic!()` behavior on overflow as the normal ones, because internally,
//! it is just calling `.inner() $OPERATOR $NUMBER`.
//!
//! ```rust
//! # use readable::*;
//! assert!(Time::from(10_u32) + 10 == Time::from(20_u32));
//! assert!(Time::from(10_u32) - 10 == Time::from(0_u32));
//! assert!(Time::from(10_u32) / 10 == Time::from(1_u32));
//! assert!(Time::from(10_u32) * 10 == Time::from(100_u32));
//! assert!(Time::from(10_u32) % 10 == Time::from(0_u32));
//! ```
//!
//! ## Credit
//! The formatting code was taken from `https://docs.rs/humantime` (and modified).

mod time;
pub use time::*;

mod time_full;
pub use time_full::*;