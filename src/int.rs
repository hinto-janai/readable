//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Constants
// The locale numbers are formatting in is English, which looks like: [1,000]
const LOCALE: num_format::Locale = num_format::Locale::en;

//---------------------------------------------------------------------------------------------------- Int
/// Human readable integer.
///
/// [`From`] takes an unsigned number as input and returns a ready-to-[`print!()`] [`Int`].
///
/// [`f32`] or [`f64`] inputs will work, but the fractional parts will be ignored.
///
/// The inner fields are `(u64, String)` but they are not public.
///
/// # Examples
/// | Input       | [`String`] Output |
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
pub struct Int(u64, String);

impl std::fmt::Display for Int {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", &self.1.as_str())
	}
}

impl Int {
	#[inline]
	/// Returns a [`Self`] with the value `0`.
	pub fn new() -> Self {
		Self(0, String::from("0"))
	}

	#[inline]
	/// Returns a [`Self`] with the `u64` set to `0`, but the [`String`] set to `???`.
	pub fn unknown() -> Self {
		Self(0, String::from("???"))
	}

	#[inline]
	/// Return a borrowed [`str`] without consuming [`Self`].
	pub fn as_str(&self) -> &str {
		self.1.as_str()
	}

	#[inline]
	/// [`Clone`]'s and returns the inner [`String`].
	pub fn to_string(&self) -> String {
		self.1.clone()
	}

	#[inline]
	/// Returns the inner [`u64`].
	pub fn to_u64(&self) -> u64 {
		self.0
	}

	#[inline]
	/// Consumes [`Self]`, returning the inner [`String`].
	pub fn into_string(self) -> String {
		self.1
	}

	#[inline]
	/// Consumes [`Self`], returning the inner [`u64`] and [`String`].
	pub fn into_raw(self) -> (u64, String) {
		(self.0, self.1)
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
				Self(integer as u64, buf.as_str().to_string())
			}
		}
	};
}
impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(usize);
impl From<u64> for Int {
	#[inline]
	fn from(integer: u64) -> Self {
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&integer, &LOCALE);
		Self(integer, buf.as_str().to_string())
	}
}
impl From<f32> for Int {
	#[inline]
	fn from(integer: f32) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		if integer == f32::NAN {
			return Self(integer as u64, String::from(super::NAN))
		} else if integer == f32::INFINITY {
			return Self(integer as u64, String::from(super::INFINITY))
		} else if integer == f32::NEG_INFINITY {
			return Self(integer as u64, String::from(super::NEG_INFINITY))
		}

		let mut buf = num_format::Buffer::new();
		let integer = integer as u64;
		buf.write_formatted(&integer, &LOCALE);
		Self(integer, buf.as_str().to_string())
	}
}
impl From<f64> for Int {
	#[inline]
	fn from(integer: f64) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		if integer == f64::NAN {
			return Self(integer as u64, String::from(super::NAN))
		} else if integer == f64::INFINITY {
			return Self(integer as u64, String::from(super::INFINITY))
		} else if integer == f64::NEG_INFINITY {
			return Self(integer as u64, String::from(super::NEG_INFINITY))
		}

		let mut buf = num_format::Buffer::new();
		let integer = integer as u64;
		buf.write_formatted(&integer, &LOCALE);
		Self(integer, buf.as_str().to_string())
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	#[test]
	fn int() {
		assert!(Int::from(1_000).as_str() == "1,000");
		assert!(Int::from(65_535).as_str() == "65,535");
		assert!(Int::from(65_536).as_str() == "65,536");
		assert!(Int::from(100_000).as_str() == "100,000");
		assert!(Int::from(1_000_000).as_str() == "1,000,000");
		assert!(Int::from(10_000_000).as_str() == "10,000,000");
		assert!(Int::from(100_000_000).as_str() == "100,000,000");
		assert!(Int::from(1_000_000_000).as_str() == "1,000,000,000");
		assert!(Int::from(4_294_967_295).as_str() == "4,294,967,295");
		assert!(Int::from(4_294_967_296).as_str() == "4,294,967,296");
		assert!(Int::from(10_000_000_000).as_str() == "10,000,000,000");
		assert!(Int::from(100_000_000_000).as_str() == "100,000,000,000");
		assert!(Int::from(1_000_000_000_000).as_str() == "1,000,000,000,000");
		assert!(Int::from(10_000_000_000_000).as_str() == "10,000,000,000,000");
		assert!(Int::from(100_000_000_000_000).as_str() == "100,000,000,000,000");
		assert!(Int::from(1_000_000_000_000_000).as_str() == "1,000,000,000,000,000");
		assert!(Int::from(10_000_000_000_000_000).as_str() == "10,000,000,000,000,000");
		assert!(Int::from(18_446_744_073_709_551_615).as_str() == "18,446,744,073,709,551,615");
	}
}
