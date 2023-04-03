//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use compact_str::{format_compact,CompactString};
use crate::constants::*;
use crate::macros::*;

//---------------------------------------------------------------------------------------------------- Percent
/// Human readable percentage.
///
/// Takes a floating point number as input and returns a ready-to-[`print!()`] [`Percent`].
///
/// The default [`From`] implementation will print `2` decimal numbers.
///
/// Anything lower than `0.01` is rounded down to `0.00`.
///
/// This can be changed by using different functions when initially
/// creating the [`Percent`], or converting an existing [`Percent`], for example:
/// ```rust
/// # use readable::Percent;
/// let f0 = Percent::new_0_point(3.0);
/// let f2 = Percent::from(3.0);
/// let f3 = Percent::new_3_point(3.0);
/// let f4 = Percent::new_4_point(3.0);
///
/// assert!(f0 == "3%");
/// assert!(f2 == "3.00%");
/// assert!(f3 == "3.000%");
/// assert!(f4 == "3.0000%");
///```
/// ## Performance
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
/// # use readable::Percent;
/// assert!(Percent::zero()    == "0.00%");
/// assert!(Percent::unknown() == "?.??%");
///
/// assert!(Percent::from(0.001)   == "0.00%");
/// assert!(Percent::from(0.1)     == "0.10%");
/// assert!(Percent::from(1.0)     == "1.00%");
/// assert!(Percent::from(100.0)   == "100.00%");
/// assert!(Percent::from(1_000.0) == "1,000.00%");
///
/// assert!(Percent::from(1_u64)      == "1.00%");
/// assert!(Percent::from(1_000_u64)  == "1,000.00%");
/// assert!(Percent::from(10_000_u64) == "10,000.00%");
///
/// assert!(Percent::from(-1_i64)      == "-1.00%");
/// assert!(Percent::from(-1_000_i64)  == "-1,000.00%");
/// assert!(Percent::from(-10_000_i64) == "-10,000.00%");
/// ```

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Percent(f64, CompactString);

impl Percent {
	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of `0.0`.
	///
	/// The [`String`] is set to `0.00%`.
	pub fn zero() -> Self {
		Self(0.0, CompactString::new(ZERO_PERCENT))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to `?.??%`.
	pub fn unknown() -> Self {
		Self(f64::NAN, CompactString::new(UNKNOWN_PERCENT))
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
	/// Same as [`Self::from`] but with no floating point on the inner [`String`].
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
	pub fn new_0_point(f: f64) -> Self {
		handle_nan_string!(f);
		Self(f, format_compact!("{}%", buf!(f as u64)))
	}

	#[inline]
	/// Same as [`Self::from`] but with `1` floating point.
	pub fn new_1_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.1}", f.fract())[2..];
		Self(f, format_compact!("{}.{}%", buf!(f as u64), fract))
	}

	#[inline]
	/// Same as [`Self::from`] but with `3` floating point.
	pub fn new_3_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.3}", f.fract())[2..];
		Self(f, format_compact!("{}.{}%", buf!(f as u64), fract))
	}

	#[inline]
	/// Same as [`Self::from`] but with `4` floating point.
	pub fn new_4_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.4}", f.fract())[2..];
		Self(f, format_compact!("{}.{}%", buf!(f as u64), fract))
	}

	#[inline]
	/// Same as [`Self::from`] but with `5` floating point.
	pub fn new_5_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.5}", f.fract())[2..];
		Self(f, format_compact!("{}.{}%", buf!(f as u64), fract))
	}

	#[inline]
	/// Same as [`Self::from`] but with `6` floating point.
	pub fn new_6_point(f: f64) -> Self {
		handle_nan_string!(f);

		let fract = &format_compact!("{:.6}", f.fract())[2..];
		Self(f, format_compact!("{}.{}%", buf!(f as u64), fract))
	}
}

// Implementation Macro.
macro_rules! impl_number {
	($number:ty) => {
		impl From<$number> for Percent {
			#[inline]
			fn from(number: $number) -> Self {
				Self(number as f64, format_compact!("{}.00%", buf!(number)))
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

impl From<f32> for Percent {
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

		let fract = &format_compact!("{:.2}", number.fract())[2..];
		Self(number as f64, format_compact!("{}.{}%", buf!(number as u64), fract))
	}
}

impl From<f64> for Percent {
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

		let fract = &format_compact!("{:.2}", number.fract())[2..];

		Self(number, format_compact!("{}.{}%", buf!(number as u64), fract))
	}
}

impl_traits!(Percent, f64);

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn special() {
		assert!(Percent::zero()    == "0.00%");
		assert!(Percent::unknown() == "?.??%");
		assert!(Percent::nan()     == NAN);
		assert!(Percent::inf()     == INFINITY);

		assert!(Percent::from(0.0) == "0.00%");
		assert!(Percent::from(f64::NAN) == NAN);
		assert!(Percent::from(f64::INFINITY) == INFINITY);
		assert!(Percent::from(f64::NEG_INFINITY) == INFINITY);
	}

	#[test]
	fn percent() {
		assert!(Percent::from(0.0)       == "0.00%");
		assert!(Percent::from(0.001)     == "0.00%");
		assert!(Percent::from(0.1)       == "0.10%");
		assert!(Percent::from(1.0)       == "1.00%");
		assert!(Percent::from(50.0)      == "50.00%");
		assert!(Percent::from(100.0)     == "100.00%");
		assert!(Percent::from(150.0)     == "150.00%");
		assert!(Percent::from(1_000.0)   == "1,000.00%");
		assert!(Percent::from(250_000.0) == "250,000.00%");
	}

	#[test]
	fn percent_dot() {
		assert!(Percent::new_1_point(0.0)        == "0.0%");
		assert!(Percent::new_1_point(1_000.1234) == "1,000.1%");
		assert!(Percent::new_3_point(1_000.1234) == "1,000.123%");
		assert!(Percent::new_4_point(1_000.1234) == "1,000.1234%");

		assert!(Percent::new_1_point(0.1)            == "0.1%");
		assert!(Percent::new_1_point(10_000.1234)    == "10,000.1%");
		assert!(Percent::new_3_point(100_000.1234)   == "100,000.123%");
		assert!(Percent::new_4_point(1_000_000.1234) == "1,000,000.1234%");
	}

	#[test]
	fn from_unsigned() {
		assert!(Percent::from(1_u64)         == "1.00%");
		assert!(Percent::from(1_000_u64)     == "1,000.00%");
		assert!(Percent::from(10_000_u64)    == "10,000.00%");
		assert!(Percent::from(100_000_u64)   == "100,000.00%");
		assert!(Percent::from(1_000_000_u64) == "1,000,000.00%");
	}

	#[test]
	fn from_int() {
		assert!(Percent::from(-1_i64)         == "-1.00%");
		assert!(Percent::from(-1_000_i64)     == "-1,000.00%");
		assert!(Percent::from(-10_000_i64)    == "-10,000.00%");
		assert!(Percent::from(-100_000_i64)   == "-100,000.00%");
		assert!(Percent::from(-1_000_000_i64) == "-1,000,000.00%");
	}
}
