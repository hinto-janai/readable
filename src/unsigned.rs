//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

use crate::inner::*;
use crate::macros::*;
use crate::constants::*;

//---------------------------------------------------------------------------------------------------- Unsigned
/// Human readable unsigned integer.
///
/// [`Unsigned::from`] takes an unsigned integer as input and returns a ready-to-[`print!()`] [`Unsigned`].
///
/// [`f32`] or [`f64`] inputs will work, but:
/// - Signed floats will turn into `0`
/// - Fractional parts will be ignored
///
/// ## Cloning
/// [`Copy`] available, [`Clone`] is cheap.
///
/// The inner type is either a `&'static str` or a buffer
/// allocated on the stack, both are able to be cheaply `Copy`-ied:
/// ```rust
/// # use readable::Unsigned;
/// let a = Unsigned::from(100_000_u64);
///
/// // Copy 'a', use 'b'.
/// let b = a;
/// assert!(b == 100_000_u64);
///
/// // We can still use 'a'
/// assert!(a == 100_000_u64);
/// ```
///
/// ## Exceptions
/// - [`f64::NAN`] outputs [`NAN`]
/// - [`f64::INFINITY`] outputs [`INFINITY`]
///
/// To disable checks for these, (you are _sure_ you don't have NaN's), enable the `ignore_nan_inf` feature flag.
///
/// # Examples
/// ```rust
/// # use readable::Unsigned;
/// // From unsigned integers.
/// assert!(Unsigned::from(100_u8)        == "100");
/// assert!(Unsigned::from(10_000_u16)    == "10,000");
/// assert!(Unsigned::from(100_000_u32)   == "100,000");
/// assert!(Unsigned::from(1_000_000_u64) == "1,000,000");
///
/// // From floats.
/// assert!(Unsigned::from(-1.0)        == "0");
/// assert!(Unsigned::from(1_000.123)   == "1,000");
/// assert!(Unsigned::from(100_000.123) == "100,000");
/// assert!(Unsigned::from(100_000.123) == "100,000");
/// ```

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Unsigned(u64, Inner);

impl Unsigned {
	impl_inner!(u64);
	impl_common!(u64);
	impl_usize!();
}

impl_traits!(Unsigned, u64);
impl_from!(u8, u16, u32, u64, usize, Unsigned);

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn unsigned() {
		assert!(Unsigned::from(1_000_u64) == "1,000");
		assert!(Unsigned::from(65_535_u64) == "65,535");
		assert!(Unsigned::from(65_536_u64) == "65,536");
		assert!(Unsigned::from(100_000_u64) == "100,000");
		assert!(Unsigned::from(1_000_000_u64) == "1,000,000");
		assert!(Unsigned::from(10_000_000_u64) == "10,000,000");
		assert!(Unsigned::from(100_000_000_u64) == "100,000,000");
		assert!(Unsigned::from(1_000_000_000_u64) == "1,000,000,000");
		assert!(Unsigned::from(4_294_967_295_u64) == "4,294,967,295");
		assert!(Unsigned::from(4_294_967_296_u64) == "4,294,967,296");
		assert!(Unsigned::from(10_000_000_000_u64) == "10,000,000,000");
		assert!(Unsigned::from(100_000_000_000_u64) == "100,000,000,000");
		assert!(Unsigned::from(1_000_000_000_000_u64) == "1,000,000,000,000");
		assert!(Unsigned::from(10_000_000_000_000_u64) == "10,000,000,000,000");
		assert!(Unsigned::from(100_000_000_000_000_u64) == "100,000,000,000,000");
		assert!(Unsigned::from(1_000_000_000_000_000_u64) == "1,000,000,000,000,000");
		assert!(Unsigned::from(10_000_000_000_000_000_u64) == "10,000,000,000,000,000");
		assert!(Unsigned::from(18_446_744_073_709_551_615_u64) == "18,446,744,073,709,551,615");
	}

	#[test]
	fn float() {
		assert!(Unsigned::from(1_000.0) == "1,000");
		assert!(Unsigned::from(65_535.0) == "65,535");
		assert!(Unsigned::from(65_536.0) == "65,536");
		assert!(Unsigned::from(100_000.0) == "100,000");
		assert!(Unsigned::from(1_000_000.0) == "1,000,000");
		assert!(Unsigned::from(10_000_000.0) == "10,000,000");
		assert!(Unsigned::from(100_000_000.0) == "100,000,000");
		assert!(Unsigned::from(1_000_000_000.0) == "1,000,000,000");
		assert!(Unsigned::from(4_294_967_295.0) == "4,294,967,295");
		assert!(Unsigned::from(4_294_967_296.0) == "4,294,967,296");
		assert!(Unsigned::from(10_000_000_000.0) == "10,000,000,000");
		assert!(Unsigned::from(100_000_000_000.0) == "100,000,000,000");
		assert!(Unsigned::from(1_000_000_000_000.0) == "1,000,000,000,000");
		assert!(Unsigned::from(10_000_000_000_000.0) == "10,000,000,000,000");
		assert!(Unsigned::from(100_000_000_000_000.0) == "100,000,000,000,000");
		assert!(Unsigned::from(1_000_000_000_000_000.0) == "1,000,000,000,000,000");
		assert!(Unsigned::from(10_000_000_000_000_000.0) == "10,000,000,000,000,000");
		assert!(Unsigned::from(18_446_744_073_709_551_615.0) == "18,446,744,073,709,551,615");
	}

	#[test]
	fn special() {
		assert!(Unsigned::from(f64::NAN)          == crate::NAN);
		assert!(Unsigned::from(f64::INFINITY)     == crate::INFINITY);
		assert!(Unsigned::from(f64::NEG_INFINITY) == crate::INFINITY);
	}
}
