//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use compact_str::{format_compact,CompactString};
use super::{NAN,INFINITY};

//---------------------------------------------------------------------------------------------------- Constants
// The locale numbers are formatting in is English, which looks like: [1,000]
const LOCALE: num_format::Locale = num_format::Locale::en;

//---------------------------------------------------------------------------------------------------- Int
/// Human readable signed integer.
///
/// [`From`] takes an signed integer as input and returns a ready-to-[`print!()`] [`Int`].
///
/// [`f32`] or [`f64`] inputs will work, but the fractional parts will be ignored.
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
/// | Input        | Output            |
/// |--------------|-------------------|
/// | `0`          | `0`
/// | `1`          | `1`
/// | `-1`         | `-1`
/// | `999`        | `999`
/// | `-1000`      | `-1,000`
/// | `1234567`    | `1,234,567`
/// | `-100000000` | `-100,000,000`
/// | `1.123`      | `1`
/// | `-2000.123`  | `-2,000`

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Int(i64, CompactString);

impl std::fmt::Display for Int {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", &self.1)
	}
}

impl Int {
	#[inline]
	/// Returns a [`Self`] with the value `0`.
	pub fn new() -> Self {
		Self(0, CompactString::new("0"))
	}

	#[inline]
	/// Returns a [`Self`] with the `i64` set to `0`, but the [`String`] set to `???`.
	pub fn unknown() -> Self {
		Self(0, CompactString::new("???"))
	}

	#[inline]
	/// Return a borrowed [`str`] without consuming [`Self`].
	pub fn as_str(&self) -> &str {
		self.1.as_str()
	}

	#[inline]
	/// Returns the inner [`i64`].
	pub fn to_i64(&self) -> i64 {
		self.0
	}

	#[inline]
	/// Consumes [`Self]`, returning the inner [`String`].
	pub fn into_string(self) -> String {
		self.1.into_string()
	}

	#[inline]
	/// Consumes [`Self`], returning the inner [`i64`] and [`String`].
	pub fn into_raw(self) -> (i64, String) {
		(self.0, self.1.into_string())
	}
}

// Implementation Macro.
macro_rules! impl_int {
	($int:ty) => {
		impl From<$int> for Int {
			#[inline]
			fn from(integer: $int) -> Self {
				let mut buf = num_format::Buffer::new();
				buf.write_formatted(&integer, &LOCALE);

				// SAFETY: the buffer _should_ already be valid UTF-8.
				Self(integer as i64, unsafe { compact_str::CompactString::from_utf8_unchecked(buf.as_bytes()) })
			}
		}
	};
}
impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl From<i64> for Int {
	#[inline]
	fn from(integer: i64) -> Self {
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&integer, &LOCALE);
		// SAFETY: the buffer _should_ already be valid UTF-8.
		Self(integer, unsafe { compact_str::CompactString::from_utf8_unchecked(buf.as_bytes()) })
	}
}
impl From<f32> for Int {
	#[inline]
	fn from(integer: f32) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			let fpcat = integer.classify();
			use std::num::FpCategory;
			match fpcat {
				FpCategory::Normal   => (),
				FpCategory::Nan      => return Self(integer as i64, CompactString::new(NAN)),
				FpCategory::Infinite => return Self(integer as i64, CompactString::new(INFINITY)),
				_ => (),
			}
		}

		let mut buf = num_format::Buffer::new();
		let integer = integer as i64;
		buf.write_formatted(&integer, &LOCALE);
		// SAFETY: the buffer _should_ already be valid UTF-8.
		Self(integer, unsafe { compact_str::CompactString::from_utf8_unchecked(buf.as_bytes()) })
	}
}
impl From<f64> for Int {
	#[inline]
	fn from(integer: f64) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			let fpcat = integer.classify();
			use std::num::FpCategory;
			match fpcat {
				FpCategory::Normal   => (),
				FpCategory::Nan      => return Self(integer as i64, CompactString::new(NAN)),
				FpCategory::Infinite => return Self(integer as i64, CompactString::new(INFINITY)),
				_ => (),
			}
		}

		let mut buf = num_format::Buffer::new();
		let integer = integer as i64;
		buf.write_formatted(&integer, &LOCALE);
		// SAFETY: the buffer _should_ already be valid UTF-8.
		Self(integer, unsafe { compact_str::CompactString::from_utf8_unchecked(buf.as_bytes()) })
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn unsigned() {
		assert!(Int::from(1_000_i64).as_str() == "1,000");
		assert!(Int::from(65_535_i64).as_str() == "65,535");
		assert!(Int::from(65_536_i64).as_str() == "65,536");
		assert!(Int::from(100_000_i64).as_str() == "100,000");
		assert!(Int::from(1_000_000_i64).as_str() == "1,000,000");
		assert!(Int::from(10_000_000_i64).as_str() == "10,000,000");
		assert!(Int::from(100_000_000_i64).as_str() == "100,000,000");
		assert!(Int::from(1_000_000_000_i64).as_str() == "1,000,000,000");
		assert!(Int::from(4_294_967_295_i64).as_str() == "4,294,967,295");
		assert!(Int::from(4_294_967_296_i64).as_str() == "4,294,967,296");
		assert!(Int::from(10_000_000_000_i64).as_str() == "10,000,000,000");
		assert!(Int::from(100_000_000_000_i64).as_str() == "100,000,000,000");
		assert!(Int::from(1_000_000_000_000_i64).as_str() == "1,000,000,000,000");
		assert!(Int::from(10_000_000_000_000_i64).as_str() == "10,000,000,000,000");
		assert!(Int::from(100_000_000_000_000_i64).as_str() == "100,000,000,000,000");
		assert!(Int::from(1_000_000_000_000_000_i64).as_str() == "1,000,000,000,000,000");
		assert!(Int::from(10_000_000_000_000_000_i64).as_str() == "10,000,000,000,000,000");
	}

	#[test]
	fn int() {
		assert!(Int::from(-1_000_i64).as_str() == "-1,000");
		assert!(Int::from(-65_535_i64).as_str() == "-65,535");
		assert!(Int::from(-65_536_i64).as_str() == "-65,536");
		assert!(Int::from(-100_000_i64).as_str() == "-100,000");
		assert!(Int::from(-1_000_000_i64).as_str() == "-1,000,000");
		assert!(Int::from(-10_000_000_i64).as_str() == "-10,000,000");
		assert!(Int::from(-100_000_000_i64).as_str() == "-100,000,000");
		assert!(Int::from(-1_000_000_000_i64).as_str() == "-1,000,000,000");
		assert!(Int::from(-4_294_967_295_i64).as_str() == "-4,294,967,295");
		assert!(Int::from(-4_294_967_296_i64).as_str() == "-4,294,967,296");
		assert!(Int::from(-10_000_000_000_i64).as_str() == "-10,000,000,000");
		assert!(Int::from(-100_000_000_000_i64).as_str() == "-100,000,000,000");
		assert!(Int::from(-1_000_000_000_000_i64).as_str() == "-1,000,000,000,000");
		assert!(Int::from(-10_000_000_000_000_i64).as_str() == "-10,000,000,000,000");
		assert!(Int::from(-100_000_000_000_000_i64).as_str() == "-100,000,000,000,000");
		assert!(Int::from(-1_000_000_000_000_000_i64).as_str() == "-1,000,000,000,000,000");
		assert!(Int::from(-10_000_000_000_000_000_i64).as_str() == "-10,000,000,000,000,000");

		assert!(Int::from(i64::MIN).as_str() == "-9,223,372,036,854,775,808");
		assert!(Int::from(i64::MAX).as_str() == "9,223,372,036,854,775,807");
	}

	#[test]
	fn float() {
		assert!(Int::from(-1_000.0).as_str() == "-1,000");
		assert!(Int::from(-65_535.0).as_str() == "-65,535");
		assert!(Int::from(-65_536.0).as_str() == "-65,536");
		assert!(Int::from(-100_000.0).as_str() == "-100,000");
		assert!(Int::from(-1_000_000.0).as_str() == "-1,000,000");
		assert!(Int::from(-10_000_000.0).as_str() == "-10,000,000");
		assert!(Int::from(-100_000_000.0).as_str() == "-100,000,000");
		assert!(Int::from(-1_000_000_000.0).as_str() == "-1,000,000,000");
		assert!(Int::from(-4_294_967_295.0).as_str() == "-4,294,967,295");
		assert!(Int::from(-4_294_967_296.0).as_str() == "-4,294,967,296");
		assert!(Int::from(-10_000_000_000.0).as_str() == "-10,000,000,000");
		assert!(Int::from(-100_000_000_000.0).as_str() == "-100,000,000,000");
		assert!(Int::from(-1_000_000_000_000.0).as_str() == "-1,000,000,000,000");
		assert!(Int::from(-10_000_000_000_000.0).as_str() == "-10,000,000,000,000");
		assert!(Int::from(-100_000_000_000_000.0).as_str() == "-100,000,000,000,000");
		assert!(Int::from(-1_000_000_000_000_000.0).as_str() == "-1,000,000,000,000,000");
		assert!(Int::from(i64::MIN as f64).as_str() == "-9,223,372,036,854,775,808");
		assert!(Int::from(i64::MAX as f64).as_str() == "9,223,372,036,854,775,807");
	}

	#[test]
	fn special() {
		assert!(Int::from(f64::NAN).as_str()          == crate::NAN);
		assert!(Int::from(f64::INFINITY).as_str()     == crate::INFINITY);
		assert!(Int::from(f64::NEG_INFINITY).as_str() == crate::INFINITY);
	}
}
