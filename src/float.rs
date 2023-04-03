//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use compact_str::{format_compact,CompactString};
use crate::constants::*;
use crate::macros::*;

//---------------------------------------------------------------------------------------------------- Float
/// Human readable float.
///
/// Takes a floating point number as input and returns a ready-to-[`print!()`] [`Float`].
///
/// The default [`From`] implementation will print `3` decimal numbers.
///
/// This can be changed by using different functions when initially
/// creating the [`Float`], or converting an existing [`Float`], for example:
/// ```
/// # use readable::Float;
/// let f2 = Float::new_2_point(3.0);
/// let f6 = Float::new_6_point(3.0);
/// let f9 = Float::new_9_point(f2.f64());
///
/// println!("{}\n{}\n{}", f2, f6, f9);
///
/// // 3.00
/// // 3.000000
/// // 3.000000000
///```
/// # Exceptions
/// | Exceptions                                | [`String`] Output |
/// |-------------------------------------------|-------------------|
/// | [`f64::NAN`]                              | `NaN`
/// | [`f64::INFINITY`] & [`f64::NEG_INFINITY`] | `∞`
///
/// To disable checks for these, (you are _sure_ you don't have NaN's), enable the `ignore_nan_inf` feature flag.
///
/// # Examples
/// | Input              | Output            |
/// |--------------------|-------------------|
/// | `0.0`              | `0.000`
/// | `1234.571`         | `1,234.571`
/// | `1234.571`         | `1,234.571000`
/// | `99.123` (percent) | `99.12%`
/// | `0.001` (percent)  | `0%`

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Float(f64, CompactString);

impl Float {
	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of `0.0`.
	///
	/// The [`String`] is set to `0.000`.
	pub fn zero() -> Self {
		Self(0.0, CompactString::new("0.000"))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to `???`.
	pub fn unknown() -> Self {
		Self(f64::NAN, CompactString::new(UNKNOWN))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to `NaN`.
	pub fn nan() -> Self {
		Self(f64::NAN, CompactString::new(NAN))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::INFINITY`].
	///
	/// The [`String`] is set to `∞`.
	pub fn inf() -> Self {
		Self(f64::INFINITY, CompactString::new(INFINITY))
	}

	#[inline]
	/// Truncates to `2` floating point and appends a `%`.
	///
	/// Anything lower than `0.01` is rounded down to `0.00`.
	///
	/// ## Examples
	/// | Input | String Output |
	/// |-------|---------------|
	/// | 0.0   | `0.00%`
	/// | 0.001 | `0.00%`
	/// | 0.01  | `0.01%`
	/// | 0.1   | `0.10%`
	/// | 1.0   | `1.00%`
	/// | 50.0  | `50.00%`
	/// | 100.0 | `100.00%`
	/// | 150.0 | `150.00%`
	/// | 1000.0 | `1,000.00%`
	/// | 250000.0 | `250,000.00%`
	pub fn percent(f: f64) -> Self {
		handle_nan!(f);

		if f < 0.01 {
			Self(0.0, CompactString::new("0.00%"))
		} else if f >= 1000.0 {
			let mut buf = num_format::Buffer::new();
			buf.write_formatted(&(f as u64), &LOCALE);
			let fract = &format_compact!("{:.2}", f.fract())[2..];
			Self(f, format_compact!("{}.{:.2}%", buf, fract))
		} else {
			Self(f, format_compact!("{:.2}%", f))
		}
	}

	#[inline]
	/// Return a borrowed [`str`] without consuming [`Self`].
	pub fn as_str(&self) -> &str {
		self.1.as_str()
	}

	#[inline]
	/// Returns the inner [`f64`].
	pub fn f64(&self) -> f64 {
		self.0
	}

	#[inline]
	/// Consumes [`Self]`, returning the inner [`String`].
	pub fn into_string(self) -> String {
		self.1.into_string()
	}

	#[inline]
	/// Consumes [`Self`], returning the inner [`f64`] and [`String`].
	pub fn into_raw(self) -> (f64, String) {
		(self.0, self.1.into_string())
	}

	#[inline]
	/// Same as [`Self::from`] but with no floating point on the inner [`String`].
	///
	/// The inner [`f64`] stays the same as the input.
	///
	/// This does not round _up_ or _down_, it completely ignores the floating point.
	///
	/// ## Examples
	/// | Input  | String Output |
	/// |--------|---------------|
	/// | 0.0    | `0`
	/// | 50.123 | `50`
	/// | 100.1  | `100`
	pub fn new_0_point(f: f64) -> Self {
		handle_nan!(f);

		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}", buf.as_str()))
	}

	#[inline]
	/// Create a new [`Self`]  but with `1` floating point.
	pub fn new_1_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.1}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `2` floating point.
	pub fn new_2_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.2}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `4` floating point.
	pub fn new_4_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.4}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `5` floating point.
	pub fn new_5_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.5}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `6` floating point.
	pub fn new_6_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.6}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `7` floating point.
	pub fn new_7_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.7}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `8` floating point.
	pub fn new_8_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.8}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `9` floating point.
	pub fn new_9_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.9}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `10` floating point.
	pub fn new_10_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.10}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `11` floating point.
	pub fn new_11_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.11}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `12` floating point.
	pub fn new_12_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.12}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `13` floating point.
	pub fn new_13_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.13}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `14` floating point.
	pub fn new_14_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.14}", f)[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}", buf.as_str(), fract))
	}

	#[inline]
	/// Same as [`Self::percent`] but with no floating point on the inner [`String`].
	///
	/// The inner [`f64`] stays the same as the input.
	///
	/// This does not round _up_ or _down_, it completely ignores the floating point.
	///
	/// ## Examples
	/// | Input  | String Output |
	/// |--------|---------------|
	/// | 0.0    | `0%`
	/// | 50.123 | `50%`
	/// | 100.1  | `100%`
	pub fn percent_0_point(f: f64) -> Self {
		handle_nan!(f);

		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}%", buf.as_str()))
	}

	#[inline]
	/// Same as [`Self::percent`] but with `1` floating point.
	pub fn percent_1_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.1}", f.fract())[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}%", buf.as_str(), fract))
	}

	#[inline]
	/// Same as [`Self::percent`] but with `2` floating point.
	pub fn percent_2_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.2}", f.fract())[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}%", buf.as_str(), fract))
	}

	#[inline]
	/// Same as [`Self::percent`] but with `4` floating point.
	pub fn percent_4_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format_compact!("{:.4}", f.fract())[2..];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format_compact!("{}.{}%", buf.as_str(), fract))
	}
}

// Implementation Macro.
macro_rules! impl_number {
	($number:ty) => {
		impl From<$number> for Float {
			#[inline]
			fn from(number: $number) -> Self {
				let mut buf = num_format::Buffer::new();
				buf.write_formatted(&(number as u64), &LOCALE);
				Self(number as f64, format_compact!("{}.000", buf.as_str()))
			}
		}
	}
}
impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(usize);

impl From<f32> for Float {
	#[inline]
	fn from(number: f32) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			let fpcat = number.classify();
			use std::num::FpCategory;
			match fpcat {
				FpCategory::Normal   => (),
				FpCategory::Nan      => return Self(number as f64, CompactString::new(NAN)),
				FpCategory::Infinite => return Self(number as f64, CompactString::new(INFINITY)),
				_ => (),
			}
		}

		let fract = &format_compact!("{:.3}", number.fract())[2..];

		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(number as u32), &LOCALE);
		Self(number as f64, format_compact!("{}.{}", buf.as_str(), fract))
	}
}

impl From<f64> for Float {
	#[inline]
	fn from(number: f64) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			let fpcat = number.classify();
			use std::num::FpCategory;
			match fpcat {
				FpCategory::Normal   => (),
				FpCategory::Nan      => return Self(number, CompactString::new(NAN)),
				FpCategory::Infinite => return Self(number, CompactString::new(INFINITY)),
				_ => (),
			}
		}

		let fract = &format_compact!("{:.3}", number.fract())[2..];

		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(number as u64), &LOCALE);
		Self(number, format_compact!("{}.{}", buf.as_str(), fract))
	}
}

impl_traits!(Float, f64);

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn special() {
		assert!(Float::zero()    == "0.000");
		assert!(Float::unknown() == UNKNOWN);
		assert!(Float::nan()     == NAN);
		assert!(Float::inf()     == INFINITY);

		assert!(Float::from(0.0) == "0.000");
		assert!(Float::from(f64::NAN) == NAN);
		assert!(Float::from(f64::INFINITY) == INFINITY);
		assert!(Float::from(f64::NEG_INFINITY) == INFINITY);
	}

	#[test]
	fn float() {
		assert!(Float::new_0_point( 0.1)              == "0");
		assert!(Float::new_1_point( 0.1)              == "0.1");
		assert!(Float::new_2_point( 0.01)             == "0.01");
		assert!(Float::from(        0.001)            == "0.001");
		assert!(Float::new_4_point( 0.0001)           == "0.0001");
		assert!(Float::new_5_point( 0.00001)          == "0.00001");
		assert!(Float::new_6_point( 0.000001)         == "0.000001");
		assert!(Float::new_7_point( 0.0000001)        == "0.0000001");
		assert!(Float::new_8_point( 0.00000001)       == "0.00000001");
		assert!(Float::new_9_point( 0.000000001)      == "0.000000001");
		assert!(Float::new_10_point(0.0000000001)     == "0.0000000001");
		assert!(Float::new_11_point(0.00000000001)    == "0.00000000001");
		assert!(Float::new_12_point(0.000000000001)   == "0.000000000001");
		assert!(Float::new_13_point(0.0000000000001)  == "0.0000000000001");
		assert!(Float::new_14_point(0.00000000000001) == "0.00000000000001");
	}

	#[test]
	fn percent() {
		assert!(Float::percent(0.0)       == "0.00%");
		assert!(Float::percent(0.001)     == "0.00%");
		assert!(Float::percent(0.1)       == "0.10%");
		assert!(Float::percent(1.0)       == "1.00%");
		assert!(Float::percent(50.0)      == "50.00%");
		assert!(Float::percent(100.0)     == "100.00%");
		assert!(Float::percent(150.0)     == "150.00%");
		assert!(Float::percent(1_000.0)   == "1,000.00%");
		assert!(Float::percent(250_000.0) == "250,000.00%");
	}

	#[test]
	fn percent_dot() {
		assert!(Float::percent_1_point(0.0)        == "0.0%");
		assert!(Float::percent_1_point(1_000.1234) == "1,000.1%");
		assert!(Float::percent_2_point(1_000.1234) == "1,000.12%");
		assert!(Float::percent_4_point(1_000.1234) == "1,000.1234%");

		assert!(Float::percent_1_point(0.1)            == "0.1%");
		assert!(Float::percent_1_point(10_000.1234)    == "10,000.1%");
		assert!(Float::percent_2_point(100_000.1234)   == "100,000.12%");
		assert!(Float::percent_4_point(1_000_000.1234) == "1,000,000.1234%");
	}
}
