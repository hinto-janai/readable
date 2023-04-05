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
/// let f9 = Float::new_9_point(f2.inner());
///
/// assert!(f2 == 3.00);
/// assert!(f6 == 3.000000);
/// assert!(f9 == 3.000000000);
///```
///
/// ## Performance
/// [`Clone`] may be expensive:
/// ```rust,compile_fail
/// # use readable::Float;
/// let a = Float::from(100.0);
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
/// # Examples
/// | Input              | Output            |
/// |--------------------|-------------------|
/// | `0.0`              | `0.000`
/// | `1234.571`         | `1,234.571`
/// | `1234.571`         | `1,234.571000`

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Float(f64, CompactString);

impl_traits!(Float, f64);

impl Float {
	impl_common!(f64);

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of `0.0`.
	///
	/// The [`String`] is set to [`ZERO_FLOAT`].
	pub fn zero() -> Self {
		Self(0.0, CompactString::new(ZERO_FLOAT))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to [`UNKNOWN_FLOAT`].
	pub fn unknown() -> Self {
		Self(f64::NAN, CompactString::new(UNKNOWN_FLOAT))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to [`NAN`].
	pub fn nan() -> Self {
		Self(f64::NAN, CompactString::new(NAN))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::INFINITY`].
	///
	/// The [`String`] is set to [`INFINITY`].
	pub fn inf() -> Self {
		Self(f64::INFINITY, CompactString::new(INFINITY))
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
		handle_nan_string!(f);
		Self(f, format_compact!("{}", buf!(f as u64)))
	}

	#[inline]
	/// Create a new [`Self`]  but with `1` floating point.
	pub fn new_1_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.1}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `2` floating point.
	pub fn new_2_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.2}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `4` floating point.
	pub fn new_4_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.4}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `5` floating point.
	pub fn new_5_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.5}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `6` floating point.
	pub fn new_6_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.6}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `7` floating point.
	pub fn new_7_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.7}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `8` floating point.
	pub fn new_8_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.8}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `9` floating point.
	pub fn new_9_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.9}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `10` floating point.
	pub fn new_10_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.10}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `11` floating point.
	pub fn new_11_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.11}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `12` floating point.
	pub fn new_12_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.12}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `13` floating point.
	pub fn new_13_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.13}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `14` floating point.
	pub fn new_14_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.14}", f)[2..];
		Self(f, format_compact!("{}.{}", buf!(f as u64), fract))
	}
}

// Implementation Macro.
macro_rules! impl_number {
	($number:ty) => {
		impl From<$number> for Float {
			#[inline]
			fn from(number: $number) -> Self {
				Self(number as f64, format_compact!("{}.000", buf!(number)))
			}
		}
	}
}

impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(usize);
impl_number!(i8);
impl_number!(i16);
impl_number!(i32);
impl_number!(i64);
impl_number!(isize);

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

		Self(number as f64, format_compact!("{}.{}", buf!(number as u64).as_str(), fract))
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

		Self(number, format_compact!("{}.{}", buf!(number as u64).as_str(), fract))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn special() {
		assert!(Float::from(0.0) == "0.000");
		assert!(Float::zero()    == "0.000");
		assert!(Float::unknown() == UNKNOWN_FLOAT);
		assert!(Float::nan()     == NAN);
		assert!(Float::inf()     == INFINITY);

		assert!(Float::from(f64::NAN)          == NAN);
		assert!(Float::from(f64::INFINITY)     == INFINITY);
		assert!(Float::from(f64::NEG_INFINITY) == INFINITY);

		assert!(Float::from(f32::NAN)          == NAN);
		assert!(Float::from(f32::INFINITY)     == INFINITY);
		assert!(Float::from(f32::NEG_INFINITY) == INFINITY);
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
}
