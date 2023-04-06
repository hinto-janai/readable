//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

use crate::inner::*;
use crate::macros::*;
use crate::constants::*;

//---------------------------------------------------------------------------------------------------- Int
/// Human readable signed integer.
///
/// [`Int::from`] takes a signed integer as input and returns a ready-to-[`print!()`] [`Int`].
///
/// [`f32`] or [`f64`] inputs will work, but:
/// - Fractional parts will be ignored
///
/// ## Cloning
/// [`Copy`] available, [`Clone`] is cheap.
///
/// The inner type is either a `&'static str` or a buffer
/// allocated on the stack, both are able to be cheaply `Copy`-ied:
/// ```rust
/// # use readable::Int;
/// let a = Int::from(100_000);
///
/// // Copy 'a'
/// let b = a;
///
/// // We can still use 'a'
/// assert!(a == 100_000);
/// ```
///
/// ## Exceptions
/// - [`f64::NAN`] outputs [`NAN`]
/// - [`f64::INFINITY`] outputs [`INFINITY`]
///
/// To disable checks for these, (you are _sure_ you don't have NaN's), enable the `ignore_nan_inf` feature flag.
///
/// ## Examples
/// ```rust
/// # use readable::Int;
/// // From u32.
/// assert!(Int::from(1_000_u32)     == "1,000");
/// assert!(Int::from(100_000_u32)   == "100,000");
/// assert!(Int::from(1_000_000_u32) == "1,000,000");
///
/// // From signed integers.
/// assert!(Int::from(-1_000)   == "-1,000");
/// assert!(Int::from(-100_000) == "-100,000");
/// assert!(Int::from(-100_000) == "-100,000");
///
/// // From floats.
/// assert!(Int::from(-1.0)        == "-1");
/// assert!(Int::from(1_000.123)   == "1,000");
/// assert!(Int::from(100_000.123) == "100,000");
/// assert!(Int::from(100_000.123) == "100,000");
/// ```

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Int(i64, Inner);

impl Int {
	impl_inner!(i64);
	impl_common!(i64);
	impl_isize!();
}

impl_traits!(Int, i64);
impl_from!(i8, i16, i32, i64, isize, Int);
impl_from_single!(u8, i64, Int);
impl_from_single!(u16, i64, Int);
impl_from_single!(u32, i64, Int);

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn unsigned() {
		assert!(Int::from(1_000_i64) == "1,000");
		assert!(Int::from(65_535_i64) == "65,535");
		assert!(Int::from(65_536_i64) == "65,536");
		assert!(Int::from(100_000_i64) == "100,000");
		assert!(Int::from(1_000_000_i64) == "1,000,000");
		assert!(Int::from(10_000_000_i64) == "10,000,000");
		assert!(Int::from(100_000_000_i64) == "100,000,000");
		assert!(Int::from(1_000_000_000_i64) == "1,000,000,000");
		assert!(Int::from(4_294_967_295_i64) == "4,294,967,295");
		assert!(Int::from(4_294_967_296_i64) == "4,294,967,296");
		assert!(Int::from(10_000_000_000_i64) == "10,000,000,000");
		assert!(Int::from(100_000_000_000_i64) == "100,000,000,000");
		assert!(Int::from(1_000_000_000_000_i64) == "1,000,000,000,000");
		assert!(Int::from(10_000_000_000_000_i64) == "10,000,000,000,000");
		assert!(Int::from(100_000_000_000_000_i64) == "100,000,000,000,000");
		assert!(Int::from(1_000_000_000_000_000_i64) == "1,000,000,000,000,000");
		assert!(Int::from(10_000_000_000_000_000_i64) == "10,000,000,000,000,000");
	}

	#[test]
	fn int() {
		assert!(Int::from(-1_000_i64) == "-1,000");
		assert!(Int::from(-65_535_i64) == "-65,535");
		assert!(Int::from(-65_536_i64) == "-65,536");
		assert!(Int::from(-100_000_i64) == "-100,000");
		assert!(Int::from(-1_000_000_i64) == "-1,000,000");
		assert!(Int::from(-10_000_000_i64) == "-10,000,000");
		assert!(Int::from(-100_000_000_i64) == "-100,000,000");
		assert!(Int::from(-1_000_000_000_i64) == "-1,000,000,000");
		assert!(Int::from(-4_294_967_295_i64) == "-4,294,967,295");
		assert!(Int::from(-4_294_967_296_i64) == "-4,294,967,296");
		assert!(Int::from(-10_000_000_000_i64) == "-10,000,000,000");
		assert!(Int::from(-100_000_000_000_i64) == "-100,000,000,000");
		assert!(Int::from(-1_000_000_000_000_i64) == "-1,000,000,000,000");
		assert!(Int::from(-10_000_000_000_000_i64) == "-10,000,000,000,000");
		assert!(Int::from(-100_000_000_000_000_i64) == "-100,000,000,000,000");
		assert!(Int::from(-1_000_000_000_000_000_i64) == "-1,000,000,000,000,000");
		assert!(Int::from(-10_000_000_000_000_000_i64) == "-10,000,000,000,000,000");

		assert!(Int::from(i64::MIN) == "-9,223,372,036,854,775,808");
		assert!(Int::from(i64::MAX) == "9,223,372,036,854,775,807");
	}

	#[test]
	fn float() {
		assert!(Int::from(-1_000.0) == "-1,000");
		assert!(Int::from(-65_535.0) == "-65,535");
		assert!(Int::from(-65_536.0) == "-65,536");
		assert!(Int::from(-100_000.0) == "-100,000");
		assert!(Int::from(-1_000_000.0) == "-1,000,000");
		assert!(Int::from(-10_000_000.0) == "-10,000,000");
		assert!(Int::from(-100_000_000.0) == "-100,000,000");
		assert!(Int::from(-1_000_000_000.0) == "-1,000,000,000");
		assert!(Int::from(-4_294_967_295.0) == "-4,294,967,295");
		assert!(Int::from(-4_294_967_296.0) == "-4,294,967,296");
		assert!(Int::from(-10_000_000_000.0) == "-10,000,000,000");
		assert!(Int::from(-100_000_000_000.0) == "-100,000,000,000");
		assert!(Int::from(-1_000_000_000_000.0) == "-1,000,000,000,000");
		assert!(Int::from(-10_000_000_000_000.0) == "-10,000,000,000,000");
		assert!(Int::from(-100_000_000_000_000.0) == "-100,000,000,000,000");
		assert!(Int::from(-1_000_000_000_000_000.0) == "-1,000,000,000,000,000");
		assert!(Int::from(i64::MIN as f64) == "-9,223,372,036,854,775,808");
		assert!(Int::from(i64::MAX as f64) == "9,223,372,036,854,775,807");
	}

	#[test]
	fn special() {
		assert!(Int::from(f64::NAN)          == crate::NAN);
		assert!(Int::from(f64::INFINITY)     == crate::INFINITY);
		assert!(Int::from(f64::NEG_INFINITY) == crate::INFINITY);
	}
}
