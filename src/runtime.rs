//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use compact_str::{format_compact,CompactString};
use crate::macros::*;
use crate::constants::*;

//---------------------------------------------------------------------------------------------------- Runtime
/// Human readable "audio/video runtime" in `H:M:S` format.
///
/// [`From`] input can either be [`f32`], [`f64`], or [`std::time::Duration`].
/// [`f32`] and [`f64`] input are presumed to be in _seconds._
///
/// ## Formatting rules:
/// 1. `seconds` always has leading `0`.
/// 2. `minutes` only has a leading zero if `hours` isn't `0`.
/// 3. `hours` never has a leading `0`.
///
/// ## Performance
/// [`Clone`] is expensive.
/// ```rust,compile_fail
/// # use readable::Runtime;
/// let a = Runtime::from(100.0);
///
/// // Move 'a'
/// let b = a;
///
/// // We can't use 'a', it moved into 'b'.
/// // We must `.clone()`.
/// assert!(a == 100.0);
/// ```
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a [`CompactString`](https://docs.rs/compact_str) so that any string 24 bytes (12 bytes on 32-bit) or less are _stack_ allocated instead of _heap_ allocated.
///
/// The documentation will still refer to the inner string as a `String`. Anything returned will also be a `String`.
///
/// ## Exceptions
/// - [`f64::NAN`] outputs [`NAN`]
/// - [`f64::INFINITY`] outputs [`INFINITY`]
///
/// To disable checks for these, (you are _sure_ you don't have NaN's), enable the `ignore_nan_inf` feature flag.
///
/// ## Examples
/// ```rust
/// # use readable::Runtime;
/// // Always round down.
/// assert!(Runtime::from(11.1111) == "0:11");
/// assert!(Runtime::from(11.9999) == "0:11");
///
/// assert!(Runtime::from(111.111) == "1:51");
/// assert!(Runtime::from(111.999) == "1:51");
///
/// assert!(Runtime::from(11111.1) == "3:05:11");
/// assert!(Runtime::from(11111.9) == "3:05:11");
///
/// assert!(Runtime::from(0.0) == "0:00");
/// assert!(Runtime::from(1.0) == "0:01");
/// assert!(Runtime::from(1.9) == "0:01");
/// assert!(Runtime::from(2.0) == "0:02");
///
/// assert!(Runtime::from(f32::MIN) == "0:00");
/// assert!(Runtime::from(f64::MIN) == "0:00");
/// assert!(Runtime::from(f32::MAX) == "18446744073709551615:24:00");
/// assert!(Runtime::from(f64::MAX) == "18446744073709551615:56:08");
/// ```

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Runtime(f64, CompactString);

impl_traits!(Runtime, f64);

impl Runtime {
	impl_common!(f64);

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::zero() == 0.0);
	/// assert!(Runtime::zero() == "0:00");
	/// ```
	pub fn zero() -> Self {
		Self(0.0, CompactString::new(ZERO_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::unknown() == 0.0);
	/// assert!(Runtime::unknown() == "?:??");
	/// ```
	pub fn unknown() -> Self {
		Self(0.0, CompactString::new(UNKNOWN_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::second() == 1.0);
	/// assert!(Runtime::second() == "0:01");
	/// assert!(Runtime::second() == Runtime::from(1.0));
	/// ```
	pub fn second() -> Self {
		Self(1.0, CompactString::new("0:01"))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::minute() == 60.0);
	/// assert!(Runtime::minute() == "1:00");
	/// assert!(Runtime::minute() == Runtime::from(60.0));
	/// ```
	pub fn minute() -> Self {
		Self(60.0, CompactString::new("1:00"))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::hour() == 3600.0);
	/// assert!(Runtime::hour() == "1:00:00");
	/// assert!(Runtime::hour() == Runtime::from(3600.0));
	/// ```
	pub fn hour() -> Self {
		Self(3600.0, CompactString::new("1:00:00"))
	}
}

impl From<std::time::Duration> for Runtime {
	#[inline]
	fn from(duration: std::time::Duration) -> Self {
		Self::from(duration.as_secs_f64())
	}
}

impl From<&std::time::Duration> for Runtime {
	#[inline]
	fn from(duration: &std::time::Duration) -> Self {
		Self::from(duration.as_secs_f64())
	}
}

impl From<f64> for Runtime {
	fn from(runtime: f64) -> Self {
		// Handle NaN/Inf.
		handle_nan_string!(runtime);

		// Zero length.
		if runtime == 0.0 || runtime == f64::MIN {
			return Self::zero()
		}

		// Round up to one second length.
		if runtime < 1.0 {
			return Self::second()
		}

		// Cast to `u64` (implicitly rounds down).
	    let seconds = (runtime % 60.0) as u64;
	    let minutes = ((runtime / 60.0) % 60.0) as u64;
	    let hours   = ((runtime / 60.0) / 60.0) as u64;

		// Format.
		let string = if hours > 0 {
			format_compact!("{}:{:0>2}:{:0>2}", hours, minutes, seconds)
		} else {
			format_compact!("{}:{:0>2}", minutes, seconds)
		};

		Self(runtime, string)
	}
}

impl From<f32> for Runtime {
	fn from(runtime: f32) -> Self {
		// Handle NaN/Inf.
		handle_nan_string!(runtime);

		// Zero length.
		if runtime == 0.0 || runtime == f32::MIN {
			return Self::zero()
		}

		// Round up to one second length.
		if runtime < 1.0 {
			return Self::second()
		}

		// `f32` -> `f64`.
		let runtime = runtime as f64;

		// Cast to `u64` (implicitly rounds down).
	    let seconds = (runtime % 60.0) as u64;
	    let minutes = ((runtime / 60.0) % 60.0) as u64;
	    let hours   = ((runtime / 60.0) / 60.0) as u64;

		// Format.
		let string = if hours > 0 {
			format_compact!("{}:{:0>2}:{:0>2}", hours, minutes, seconds)
		} else {
			format_compact!("{}:{:0>2}", minutes, seconds)
		};

		Self(runtime, string)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use crate::constants::*;

	#[test]
	fn special() {
		assert!(Runtime::from(f32::NAN)          == NAN);
		assert!(Runtime::from(f32::INFINITY)     == INFINITY);
		assert!(Runtime::from(f32::NEG_INFINITY) == INFINITY);

		assert!(Runtime::from(f64::NAN)          == NAN);
		assert!(Runtime::from(f64::INFINITY)     == INFINITY);
		assert!(Runtime::from(f64::NEG_INFINITY) == INFINITY);
	}
}
