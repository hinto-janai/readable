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
//! - Larger than [`Time::MAX`]
//! - [`f32::NAN`], [`f32::INFINITY`], [`f32::NEG_INFINITY`] (or the [`f64`] versions)
//!
//! ## Uptime
//! This module contains [`Uptime`] which is a trait that allows direct conversion
//! from the _live_ system uptime to a type within this module, e.g:
//!
//! ```rust
//! # use readable::time::*;
//! // Introduce trait into scope.
//! use readable::Uptime;
//!
//! // Capture the _current_ system uptime,
//! // and format it into a `Time`.
//! let time: Time = Time::uptime();
//! std::thread::sleep(std::time::Duration::from_secs(1));
//! # // Get around CI.
//! # let time = 1;
//! assert!(time >= 1);
//! ```
//!
//! Only the types within `readable::time` implement this trait.
//!
//! ## From other [`Time`] types
//! All types in this module support lossless conversion with each other using [`From`].
//!
//! If the type is an `unknown` variant, that will also be maintained.
//!
//! ```rust
//! # use readable::*;
//! // Time
//! let time = Time::from(86461);
//! assert_eq!(time, "1d, 1m, 1s");
//!
//! // TimeFull
//! let time_full = TimeFull::from(time);
//! assert_eq!(time_full, "1 day, 1 minute, 1 second");
//!
//! // Htop
//! let htop = Htop::from(time_full);
//! assert_eq!(htop, "1 day, 00:01:01");
//!
//! // ... wrapping full circle.
//! let time2 = Time::from(htop);
//! assert_eq!(time, time2);
//!
//! // Maintain the `unknown` variant.
//! let unknown = Time::unknown();
//! assert_eq!(unknown, Time::UNKNOWN);
//! assert_eq!(Htop::from(unknown), Htop::UNKNOWN);
//! ```
//!
//! ## Naive Time
//! These types naively assume that:
//! 1. Each day is `86400` seconds
//! 2. Each month is `31` days
//! 3. Each year is `365` days
//!
//! This is incorrect as not all months are 31 days long and leap years exist.
//!
//! ## Formatting
//! The formatting for [`Time`] & [`TimeFull`] is:
//! - The lowest unit is `second`
//! - The highest is `year`
//! - `week` is skipped in favor of `7 days`
//!
//! See [`Htop`] for its formatting rules.
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
//! ```rust
//! # use readable::*;
//! assert!(Time::from(10_u32) + 10 == Time::from(20_u32));
//! assert!(Time::from(10_u32) - 10 == Time::from(0_u32));
//! assert!(Time::from(10_u32) / 10 == Time::from(1_u32));
//! assert!(Time::from(10_u32) * 10 == Time::from(100_u32));
//! assert!(Time::from(10_u32) % 10 == Time::from(0_u32));
//! ```

mod time;
pub use time::*;

mod time_full;
pub use time_full::*;

mod uptime;
pub use uptime::*;

mod htop;
pub use htop::*;