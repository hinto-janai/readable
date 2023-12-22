//! Time formatting
//!
//! "Clock time" formatted strings:
//! ```rust
//! # use readable::*;
//! const SECONDS: usize = 86399;
//!
//! assert_eq!(Time::from(SECONDS),     "11:59:59 PM");
//! assert_eq!(Military::from(SECONDS), "23:59:59");
//! ```
//!
//! ## Wrapping
//! The max input is the seconds before a day is reached, `86399`.
//!
//! After which point, `time` types will wrap back around (like a real clock):
//! ```rust
//! # use readable::*;
//! const SECONDS: usize = 86399;
//!
//! assert_eq!(Time::from(SECONDS),     "11:59:59 PM");
//! assert_eq!(Military::from(SECONDS), "23:59:59");
//!
//! assert_eq!(Time::from(SECONDS) + 1,     "12:00:00 AM");
//! assert_eq!(Military::from(SECONDS) + 1, "00:00:00");
//! ```
//!
//! ## `SysTime`
//! This module contains [`SysTime`] which is a trait that allows direct conversion
//! from the _live_ system clock time to a type within this module, e.g:
//!
//! ```rust
//! # use readable::*;
//! // Introduce trait into scope.
//! use readable::SysTime;
//!
//! // Capture the _current_ system clock
//! // time and format it into a `Time`.
//! let time: Time = Time::sys_time();
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
//! let time = Time::from(86399);
//! assert_eq!(time, "11:59:59 PM");
//!
//! // Military
//! let military = Military::from(time);
//! assert_eq!(military, "23:59:59");
//!
//! // Maintain the `unknown` variant.
//! assert_eq!(Military::from(Time::UNKNOWN), Military::UNKNOWN);
//! ```
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
//! let a = Time::from(86399);
//!
//! // Copy 'a', use 'b'.
//! let b = a;
//! assert_eq!(b, 86399);
//!
//! // We can still use 'a'
//! assert_eq!(a, 86399);
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
mod time_unit;
pub use time_unit::*;

mod free;
pub use free::*;

mod time;
pub use time::*;

mod military;
pub use military::*;

mod systime;
pub use systime::*;