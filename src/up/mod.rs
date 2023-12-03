//! Uptime formatting
//!
//! "Uptime"-style strings, e.g:
//! ```rust
//! # use readable::*;
//! const SECONDS: usize = 158079;
//!
//! assert_eq!(Uptime::from(SECONDS),     "1d, 19h, 54m, 39s");
//! assert_eq!(UptimeFull::from(SECONDS), "1 day, 19 hours, 54 minutes, 39 seconds");
//! assert_eq!(Htop::from(SECONDS),       "1 day, 19:54:39");
//! ```
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
//! - Other [`Uptime`] types
//!
//! ## Errors
//! The max input is [`u32::MAX`] seconds.
//!
//! A [`Uptime::unknown()`] (or a variant's version of it) will be returned if the input is:
//! - A negative integer
//! - Larger than [`Uptime::MAX`]
//! - [`f32::NAN`], [`f32::INFINITY`], [`f32::NEG_INFINITY`] (or the [`f64`] versions)
//!
//! ## Uptime
//! This module contains [`Uptime`] which is a trait that allows direct conversion
//! from the _live_ system uptime to a type within this module, e.g:
//!
//! ```rust
//! # use readable::up::*;
//! // Introduce trait into scope.
//! use readable::Uptime;
//!
//! // Capture the _current_ system uptime,
//! // and format it into a `Uptime`.
//! let uptime: Uptime = Uptime::sys_uptime();
//! std::thread::sleep(std::time::Duration::from_secs(1));
//! # // Get around CI.
//! # let uptime = 1;
//! assert!(uptime >= 1);
//! ```
//!
//! Only the types within `readable::up` implement this trait.
//!
//! ## From other [`Uptime`] types
//! All types in this module support lossless conversion with each other using [`From`].
//!
//! If the type is an `unknown` variant, that will also be maintained.
//!
//! ```rust
//! # use readable::*;
//! // Uptime
//! let uptime = Uptime::from(86461);
//! assert_eq!(uptime, "1d, 1m, 1s");
//!
//! // UptimeFull
//! let uptime_full = UptimeFull::from(uptime);
//! assert_eq!(uptime_full, "1 day, 1 minute, 1 second");
//!
//! // Htop
//! let htop = Htop::from(uptime_full);
//! assert_eq!(htop, "1 day, 00:01:01");
//!
//! // ... wrapping full circle.
//! let uptime2 = Uptime::from(htop);
//! assert_eq!(uptime, uptime2);
//!
//! // Maintain the `unknown` variant.
//! let unknown = Uptime::unknown();
//! assert_eq!(unknown, Uptime::UNKNOWN);
//! assert_eq!(Htop::from(unknown), Htop::UNKNOWN);
//! ```
//!
//! ## Naive Uptime
//! These types naively assume that:
//! 1. Each day is `86400` seconds
//! 2. Each month is `31` days
//! 3. Each year is `365` days
//!
//! This is incorrect as not all months are 31 days long and leap years exist.
//!
//! ## Formatting
//! The formatting for [`Uptime`] & [`UptimeFull`] is:
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
//! let a = Uptime::from(100_000);
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
//! - Combined with another [`Self`]: `Uptime::from(1) + Uptime::from(1)`
//! - Or with the inner number itself: `Uptime::from(1) + 1`
//!
//! ```rust
//! # use readable::*;
//! assert!(Uptime::from(10_u32) + 10 == Uptime::from(20_u32));
//! assert!(Uptime::from(10_u32) - 10 == Uptime::from(0_u32));
//! assert!(Uptime::from(10_u32) / 10 == Uptime::from(1_u32));
//! assert!(Uptime::from(10_u32) * 10 == Uptime::from(100_u32));
//! assert!(Uptime::from(10_u32) % 10 == Uptime::from(0_u32));
//! ```

mod uptime;
pub use uptime::*;

mod uptime_full;
pub use uptime_full::*;

mod sys_uptime;
pub use sys_uptime::*;

mod htop;
pub use htop::*;
