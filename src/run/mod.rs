//! Runtime formatting
//!
//! This module includes various [`Runtime`] types meant for audio/video style formatting (`HH:MM:SS`).
//!
//! The basic type is [`Runtime`] which formats strings to what you would expect from most audio/video players, e.g:
//! ```rust
//! # use readable::*;
//! assert_eq!(Runtime::from(0),    "0:00");
//! assert_eq!(Runtime::from(60),   "1:00");
//! assert_eq!(Runtime::from(119),  "1:59");
//! assert_eq!(Runtime::from(3599), "59:59");
//! assert_eq!(Runtime::from(3600), "1:00:00");
//! assert_eq!(Runtime::max(),      "99:59:59");
//! ```
//!
//! Here's a diagram of:
//! - What the type's formatting look like
//! - What their sub/super-set relationship is
//!
//! <img src="https://github.com/hinto-janai/readable/assets/101352116/424b91fd-7df1-493c-bf85-fcb264470c75" width="50%"/>
//!
//! ## Input
//! [`From`] input can be:
//! - Any unsigned integer [`u8`], [`usize`], etc
//! - Any signed integer [`i8`], [`isize`], etc
//! - [`f32`] or [`f64`]
//! - [`std::time::Duration`]
//! - [`std::time::Instant`]
//! - Other [`Runtime`] types
//!
//! Integer inputs are presumed to be in _seconds._
//!
//! ## From other [`Runtime`] types
//! All [`Runtime`] types support lossless conversion with each other using [`From`].
//!
//! For example, the millisecond data will not be lost even if you
//! go from [`RuntimeMilli`] -> [`Runtime`] -> [`RuntimeMilli`]
//!
//! ```rust
//! # use readable::*;
//! // Millisecond data.
//! let milli = RuntimeMilli::from(1.555);
//! assert_eq!(milli, "00:00:01.555");
//!
//! // Convert to `Runtime`.
//! let runtime = Runtime::from(milli);
//! assert_eq!(runtime, "0:01");
//!
//! // Convert to `RuntimePad`.
//! let full = RuntimePad::from(runtime);
//! assert_eq!(full, "00:00:01");
//!
//! // Convert back losslessly to [`RuntimeMilli`].
//! let milli2 = RuntimeMilli::from(full);
//! assert_eq!(milli2, "00:00:01.555");
//! assert_eq!(milli, milli2);
//! assert_eq!(milli2.inner(), 1.555);
//! ```
//!
//! This is because the inner [`f32`] stored is simply copied,
//! only the formatted string is different.
//!
//! ## Errors
//! The max input is `359999` seconds, or: anything over `99:59:59`.
//!
//! A [`Runtime::unknown()`] (or the runtime variant's version of it) will be returned if the input is:
//! - A negative integer
//! - Larger than [`Runtime::MAX`]
//! - [`f32::NAN`], [`f32::INFINITY`], [`f32::NEG_INFINITY`] (or the [`f64`] versions)
//!
//! ## Math
//! These operators are overloaded. They will always output a new `Self`:
//! - `Add +`
//! - `Sub -`
//! - `Div /`
//! - `Mul *`
//! - `Rem %`
//!
//! They can either be:
//! - Combined with another `Self`, e.g: `Runtime::from(1.0) + Runtime::from(1.0)`
//! - Or with the inner number itself: `RuntimePad::from(1.0) + 1.0`
//!
//! ```rust
//! # use readable::*;
//! let runtime = Runtime::from(1.0) + Runtime::from(1.0);
//! assert_eq!(runtime, Runtime::from(2.0));
//! assert_eq!(runtime, "0:02");
//! assert_eq!(runtime, 2.0);
//!
//! let pad = RuntimePad::from(1.5) + 1.5;
//! assert_eq!(pad, RuntimePad::from(3.0));
//! assert_eq!(pad, "00:00:03");
//! assert_eq!(pad, 3.0);
//!
//! // Floating point error!
//! let milli = RuntimeMilli::from(2.0) + 1.555;
//! assert_eq!(milli.inner(), 3.5549998);
//!
//! // Use 1 more decimal to make sure
//! // weird rounding doesn't happen.
//! let milli = RuntimeMilli::from(2.0) + 1.5551;
//! assert_eq!(milli, RuntimeMilli::from(3.5551));
//! assert_eq!(milli, "00:00:03.555");
//! assert_eq!(milli, 3.5551);
//! ```
//!
//! ## Copy
//! [`Copy`] is available for all [`Runtime`] types.
//!
//! The actual strings used internally are not [`String`](https://doc.rust-lang.org/std/string/struct.String.html)'s,
//! but byte array buffer(s). See the specific type for more details.
//!
//! The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
//! ```rust
//! # use readable::*;
//! let a = Runtime::from(100_000.0);
//!
//! // Copy 'a', use 'b'.
//! let b = a;
//! assert_eq!(b, 100_000.0);
//!
//! // We can still use 'a'
//! assert_eq!(a, 100_000.0);
//! ```

mod runtime;
pub use runtime::*;

mod runtime_pad;
pub use runtime_pad::*;

mod runtime_milli;
pub use runtime_milli::*;

mod runtime_union;
pub use runtime_union::*;
