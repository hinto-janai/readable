//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use compact_str::{format_compact,CompactString};
use crate::macros::*;

//---------------------------------------------------------------------------------------------------- Runtime
/// Human readable "audio/video runtime" in `H:M:S` format.
///
/// [`From`] input can either be [`f32`], [`f64`], or [`std::time::Duration`].
/// [`f32`] and [`f64`] input are presumed to be in _seconds._
///
/// # Formatting rules:
/// 1. `seconds` always has leading `0`.
/// 2. `minutes` only has a leading zero if `hours` isn't `0`.
/// 3. `hours` never has a leading `0`.
///
/// # Exceptions
/// | Exceptions                                | [`String`] Output |
/// |-------------------------------------------|-------------------|
/// | [`f64::NAN`]                              | `NaN`
/// | [`f64::INFINITY`] & [`f64::NEG_INFINITY`] | `∞`
///
/// To disable checks for these, (you are _sure_ you don't have NaN's), enable the `ignore_nan_inf` feature flag.
///
/// # Examples
/// | Input      | Output             |
/// |------------|--------------------|
/// | `1.0`      | `0:01`
/// | `61.0`     | `1:01`
/// | `11.1111`  | `0:11`
/// | `111.111`  | `1:51`
/// | `11111.1`  | `3:05:11`

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Runtime(f64, CompactString);

impl_traits!(Runtime, f64);

impl Runtime {
	#[inline]
	/// Returns zero-second `(0.0, "0:00")` length.
	pub fn zero() -> Self {
		Self(0.0, CompactString::new("0:00"))
	}

	#[inline]
	/// Returns a [`Self`] with the `f64` set to `0.0`, but the [`String`] set to `?:??`.
	pub fn unknown() -> Self {
		Self(0.0, CompactString::new("?:??"))
	}

	#[inline]
	/// Returns one-second `(1.0, "0:01")` length.
	pub fn one() -> Self {
		Self(1.0, CompactString::new("0:01"))
	}

	#[inline]
	/// Return a borrowed [`str`] without consuming [`Self`].
	pub fn as_str(&self) -> &str {
		self.1.as_str()
	}

	#[inline]
	/// Returns a [`Copy`] of the inner [`f64`].
	pub fn f64(&self) -> f64 {
		self.0
	}

	#[inline]
	/// Consumes [`Self`] for the inner [`String`].
	pub fn into_string(self) -> String {
		self.1.into_string()
	}

	#[inline]
	/// Consumes [`Self`] for the inner `(f64, String)`.
	pub fn into_raw(self) -> (f64, String) {
		(self.0, self.1.into_string())
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
		// Zero length.
		if runtime == 0.0 {
			return Self::zero()
		}

		// Round up to one second length.
		if runtime < 1.0 {
			return Self::one()
		}

		// Handle NaN/Inf.
		handle_nan!(runtime);

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
		// Zero length.
		if runtime == 0.0 {
			return Self::zero()
		}

		// Round up to one second length.
		if runtime < 1.0 {
			return Self::one()
		}

		// `f32` -> `f64`.
		let runtime = runtime as f64;

		// Handle NaN/Inf.
		handle_nan!(runtime);

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

	#[test]
	fn runtime() {
		// Always round down.
		assert!(Runtime::from(11.1111) == "0:11");
		assert!(Runtime::from(11.9999) == "0:11");

		assert!(Runtime::from(111.111) == "1:51");
		assert!(Runtime::from(111.999) == "1:51");

		assert!(Runtime::from(11111.1) == "3:05:11");
		assert!(Runtime::from(11111.9) == "3:05:11");

		assert!(Runtime::from(0.0) == "0:00");
		assert!(Runtime::from(1.0) == "0:01");
		assert!(Runtime::from(1.9) == "0:01");
		assert!(Runtime::from(2.0) == "0:02");
	}
}
