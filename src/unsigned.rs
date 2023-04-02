//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use compact_str::{format_compact,CompactString};
use super::{NAN,INFINITY};

//---------------------------------------------------------------------------------------------------- Constants
// The locale numbers are formatting in is English, which looks like: [1,000]
const LOCALE: num_format::Locale = num_format::Locale::en;

//---------------------------------------------------------------------------------------------------- Unsigned
/// Human readable unsigned integer.
///
/// [`From`] takes an unsigned integer as input and returns a ready-to-[`print!()`] [`Unsigned`].
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
/// | Input       | Output            |
/// |-------------|-------------------|
/// | `0`         | `0`
/// | `1`         | `1`
/// | `999`       | `999`
/// | `1000`      | `1,000`
/// | `1234567`   | `1,234,567`
/// | `100000000` | `100,000,000`
/// | `1.123`     | `1`
/// | `2000.123`  | `2,000`

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Unsigned(u64, CompactString);

impl std::fmt::Display for Unsigned {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", &self.1)
	}
}

impl Unsigned {
	#[inline]
	/// Returns a [`Self`] with the value `0`.
	pub fn new() -> Self {
		Self(0, CompactString::new("0"))
	}

	#[inline]
	/// Returns a [`Self`] with the `u64` set to `0`, but the [`String`] set to `???`.
	pub fn unknown() -> Self {
		Self(0, CompactString::new("???"))
	}

	#[inline]
	/// Return a borrowed [`str`] without consuming [`Self`].
	pub fn as_str(&self) -> &str {
		self.1.as_str()
	}

	#[inline]
	/// Returns the inner [`u64`].
	pub fn to_u64(&self) -> u64 {
		self.0
	}

	#[inline]
	/// Consumes [`Self]`, returning the inner [`String`].
	pub fn into_string(self) -> String {
		self.1.into_string()
	}

	#[inline]
	/// Consumes [`Self`], returning the inner [`u64`] and [`String`].
	pub fn into_raw(self) -> (u64, String) {
		(self.0, self.1.into_string())
	}
}

// Implementation Macro.
macro_rules! impl_int {
	($int:ty) => {
		impl From<$int> for Unsigned {
			#[inline]
			fn from(integer: $int) -> Self {
				let mut buf = num_format::Buffer::new();
				buf.write_formatted(&integer, &LOCALE);

				// SAFETY: the buffer _should_ already be valid UTF-8.
				Self(integer as u64, unsafe { compact_str::CompactString::from_utf8_unchecked(buf.as_bytes()) })
			}
		}
	};
}
impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(usize);
impl From<u64> for Unsigned {
	#[inline]
	fn from(integer: u64) -> Self {
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&integer, &LOCALE);
		// SAFETY: the buffer _should_ already be valid UTF-8.
		Self(integer, unsafe { compact_str::CompactString::from_utf8_unchecked(buf.as_bytes()) })
	}
}
impl From<f32> for Unsigned {
	#[inline]
	fn from(integer: f32) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			let fpcat = integer.classify();
			use std::num::FpCategory;
			match fpcat {
				FpCategory::Normal   => (),
				FpCategory::Nan      => return Self(integer as u64, CompactString::new(NAN)),
				FpCategory::Infinite => return Self(integer as u64, CompactString::new(INFINITY)),
				_ => (),
			}
		}

		let mut buf = num_format::Buffer::new();
		let integer = integer as u64;
		buf.write_formatted(&integer, &LOCALE);
		// SAFETY: the buffer _should_ already be valid UTF-8.
		Self(integer, unsafe { compact_str::CompactString::from_utf8_unchecked(buf.as_bytes()) })
	}
}
impl From<f64> for Unsigned {
	#[inline]
	fn from(integer: f64) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			let fpcat = integer.classify();
			use std::num::FpCategory;
			match fpcat {
				FpCategory::Normal   => (),
				FpCategory::Nan      => return Self(integer as u64, CompactString::new(NAN)),
				FpCategory::Infinite => return Self(integer as u64, CompactString::new(INFINITY)),
				_ => (),
			}
		}

		let mut buf = num_format::Buffer::new();
		let integer = integer as u64;
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
		assert!(Unsigned::from(1_000_u64).as_str() == "1,000");
		assert!(Unsigned::from(65_535_u64).as_str() == "65,535");
		assert!(Unsigned::from(65_536_u64).as_str() == "65,536");
		assert!(Unsigned::from(100_000_u64).as_str() == "100,000");
		assert!(Unsigned::from(1_000_000_u64).as_str() == "1,000,000");
		assert!(Unsigned::from(10_000_000_u64).as_str() == "10,000,000");
		assert!(Unsigned::from(100_000_000_u64).as_str() == "100,000,000");
		assert!(Unsigned::from(1_000_000_000_u64).as_str() == "1,000,000,000");
		assert!(Unsigned::from(4_294_967_295_u64).as_str() == "4,294,967,295");
		assert!(Unsigned::from(4_294_967_296_u64).as_str() == "4,294,967,296");
		assert!(Unsigned::from(10_000_000_000_u64).as_str() == "10,000,000,000");
		assert!(Unsigned::from(100_000_000_000_u64).as_str() == "100,000,000,000");
		assert!(Unsigned::from(1_000_000_000_000_u64).as_str() == "1,000,000,000,000");
		assert!(Unsigned::from(10_000_000_000_000_u64).as_str() == "10,000,000,000,000");
		assert!(Unsigned::from(100_000_000_000_000_u64).as_str() == "100,000,000,000,000");
		assert!(Unsigned::from(1_000_000_000_000_000_u64).as_str() == "1,000,000,000,000,000");
		assert!(Unsigned::from(10_000_000_000_000_000_u64).as_str() == "10,000,000,000,000,000");
		assert!(Unsigned::from(18_446_744_073_709_551_615_u64).as_str() == "18,446,744,073,709,551,615");
	}

	#[test]
	fn float() {
		assert!(Unsigned::from(1_000.0).as_str() == "1,000");
		assert!(Unsigned::from(65_535.0).as_str() == "65,535");
		assert!(Unsigned::from(65_536.0).as_str() == "65,536");
		assert!(Unsigned::from(100_000.0).as_str() == "100,000");
		assert!(Unsigned::from(1_000_000.0).as_str() == "1,000,000");
		assert!(Unsigned::from(10_000_000.0).as_str() == "10,000,000");
		assert!(Unsigned::from(100_000_000.0).as_str() == "100,000,000");
		assert!(Unsigned::from(1_000_000_000.0).as_str() == "1,000,000,000");
		assert!(Unsigned::from(4_294_967_295.0).as_str() == "4,294,967,295");
		assert!(Unsigned::from(4_294_967_296.0).as_str() == "4,294,967,296");
		assert!(Unsigned::from(10_000_000_000.0).as_str() == "10,000,000,000");
		assert!(Unsigned::from(100_000_000_000.0).as_str() == "100,000,000,000");
		assert!(Unsigned::from(1_000_000_000_000.0).as_str() == "1,000,000,000,000");
		assert!(Unsigned::from(10_000_000_000_000.0).as_str() == "10,000,000,000,000");
		assert!(Unsigned::from(100_000_000_000_000.0).as_str() == "100,000,000,000,000");
		assert!(Unsigned::from(1_000_000_000_000_000.0).as_str() == "1,000,000,000,000,000");
		assert!(Unsigned::from(10_000_000_000_000_000.0).as_str() == "10,000,000,000,000,000");
		assert!(Unsigned::from(18_446_744_073_709_551_615.0).as_str() == "18,446,744,073,709,551,615");
	}

	#[test]
	fn special() {
		assert!(Unsigned::from(f64::NAN).as_str()          == crate::NAN);
		assert!(Unsigned::from(f64::INFINITY).as_str()     == crate::INFINITY);
		assert!(Unsigned::from(f64::NEG_INFINITY).as_str() == crate::INFINITY);
	}
}
