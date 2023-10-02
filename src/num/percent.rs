//---------------------------------------------------------------------------------------------------- Use
use compact_str::{format_compact,CompactString};
use crate::num::constants::{
	UNKNOWN_PERCENT,NAN,INFINITY,
	ZERO_PERCENT,
};
use crate::macros::{
	return_bad_float,str_64,
	impl_common,impl_not_const,
	impl_usize,impl_isize,
	impl_math,impl_traits,
	impl_impl_math,
};

//---------------------------------------------------------------------------------------------------- Percent
/// Human readable percentage.
///
/// [`Percent::from`] input can be:
/// - [`u8`], [`u16`], [`u32`]
/// - [`i8`], [`i16`], [`i32`]
/// - [`f32`], [`f64`]
///
/// The default [`Percent::from`] implementation will print `2` decimal numbers.
///
/// Anything lower than `0.01` is rounded down to `0.00`.
///
/// This can be changed by using different functions when initially
/// creating the [`Percent`], or converting an existing [`Percent`], for example:
/// ```rust
/// # use readable::Percent;
/// let f0 = Percent::new_0(3.0);
/// let f2 = Percent::from(3.0);
/// let f3 = Percent::new_3(3.0);
/// let f4 = Percent::new_4(3.0);
///
/// assert!(f0 == "3%");
/// assert!(f2 == "3.00%");
/// assert!(f3 == "3.000%");
/// assert!(f4 == "3.0000%");
///```
///
/// ## Cloning
/// [`Clone`] may be expensive:
/// ```rust
/// # use readable::Percent;
/// // Probably cheap (stack allocated string).
/// let a = Percent::from(100.0);
/// let b = a.clone();
///
/// // Probably expensive (heap allocated string).
/// let a = Percent::from(f64::MAX);
/// let b = a.clone();
/// ```
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a [`CompactString`](https://docs.rs/compact_str) so that any string 24 bytes (12 bytes on 32-bit) or less are _stack_ allocated instead of _heap_ allocated.
///
/// The documentation will still refer to the inner string as a `String`. Anything returned will also be a `String`.
///
/// ## Float Errors
/// - Inputting [`f64::NAN`], [`f64::INFINITY`], [`f64::NEG_INFINITY`] or the [`f32`] variants returns errors
///
/// To disable checks for these, (you are _sure_ you don't have NaN's), enable the `ignore_nan_inf` feature flag.
///
/// ## Math
/// These operators are overloaded. They will always output a new [`Self`]:
/// - `Add +`
/// - `Sub -`
/// - `Div /`
/// - `Mul *`
/// - `Rem %`
///
/// They can either be:
/// - Combined with another [`Self`]: `Percent::from(1.0) + Percent::from(1.0)`
/// - Or with the inner number itself: `Percent::from(1.0) + 1.0`
///
/// They also have the same `panic!()` behavior on overflow as the normal ones, because internally,
/// it is just calling `.inner() $OPERATOR $NUMBER`.
///
/// ```rust
/// # use readable::*;
/// assert!(Percent::from(10.0) + 10.0 == Percent::from(20.0));
/// assert!(Percent::from(10.0) - 10.0 == Percent::from(0.0));
/// assert!(Percent::from(10.0) / 10.0 == Percent::from(1.0));
/// assert!(Percent::from(10.0) * 10.0 == Percent::from(100.0));
/// assert!(Percent::from(10.0) % 10.0 == Percent::from(0.0));
/// ```
/// Overflow example (floats don't panic in this case):
/// ```rust
/// # use readable::*;
/// let n = Percent::from(f64::MAX) + f64::MAX;
/// assert!(n.inner().is_infinite());
/// ```
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
/// assert!(Percent::from(1_u32)      == "1.00%");
/// assert!(Percent::from(1_000_u32)  == "1,000.00%");
/// assert!(Percent::from(10_000_u32) == "10,000.00%");
///
/// assert!(Percent::from(-1_i32)      == "-1.00%");
/// assert!(Percent::from(-1_000_i32)  == "-1,000.00%");
/// assert!(Percent::from(-10_000_i32) == "-10,000.00%");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Percent(f64, #[cfg_attr(feature = "bincode", bincode(with_serde))] CompactString);

// Implements `new_X` functions.
macro_rules! impl_new {
	( $num:tt ) => {
		paste::item! {
			#[doc = "Same as [`Percent::from`] but with `" $num "` floating point."]
			pub fn [<new_ $num>](f: f64) -> Self {
				return_bad_float!(f, Self::nan, Self::inf);

				let fract = &format_compact!(concat!("{:.", $num, "}"), f.fract())[2..];
				Self(f, format_compact!("{}.{}%", str_64!(f as u64), fract))
			}
		}
	}
}

// Implements `const_X` functions.
macro_rules! impl_const {
	( $num:tt ) => {
		paste::item! {
			#[doc = "Returns a [`Percent`] with the [`f64`] value of `" $num ".0`. \n\n\
			The [`String`] is set to `" $num ".00%`."]
			pub const fn [<const_ $num>]() -> Self {
				Self($num as f64, CompactString::new_inline(concat!($num, ".00%")))
			}
		}
	}
}

impl_math!(Percent, f64);
impl_traits!(Percent, f64);

impl Percent {
	impl_common!(f64);
	impl_not_const!();
	impl_usize!();
	impl_isize!();

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to `?.??%`.
	pub fn unknown() -> Self {
		Self(f64::NAN, CompactString::new_inline(UNKNOWN_PERCENT))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to `NaN`.
	pub fn nan() -> Self {
		Self(f64::NAN, CompactString::new_inline(NAN))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::INFINITY`].
	///
	/// The [`String`] is set to `∞`.
	pub fn inf() -> Self {
		Self(f64::INFINITY, CompactString::new_inline(INFINITY))
	}

	#[inline]
	/// Returns a [`Percent`] with the [`f64`] value of `0.0`.
	///
	/// The [`String`] is set to `0.00%`.
	pub const fn zero() -> Self {
		Self(0.0, CompactString::new_inline(ZERO_PERCENT))
	}

	seq_macro::seq!(N in 1..=100 {
		impl_const!(N);
	});

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
	pub fn new_0(f: f64) -> Self {
		return_bad_float!(f, Self::nan, Self::inf);
		Self(f, format_compact!("{}%", str_64!(f as u64)))
	}

	impl_new!(1);
	seq_macro::seq!(N in 3..=14 {
		impl_new!(N);
	});
}

// Implementation Macro.
macro_rules! impl_u {
	($( $number:ty ),*) => {
		$(
			impl From<$number> for Percent {
				#[inline]
				fn from(number: $number) -> Self {
					let f = number as f64;

					Self(f, format_compact!("{}.00%", str_64!(number as u64)))
				}
			}
		)*
	}
}
impl_u!(u8,u16,u32);

// Implementation Macro.
macro_rules! impl_i {
	($( $number:ty ),*) => {
		$(
			impl From<$number> for Percent {
				#[inline]
				fn from(number: $number) -> Self {
					let f = number as f64;

					Self(f, format_compact!("{}.00%", str_64!(number as i64)))
				}
			}
		)*
	}
}
impl_i!(i8,i16,i32);

impl From<f32> for Percent {
	#[inline]
	fn from(f: f32) -> Self {
		return_bad_float!(f, Self::nan, Self::inf);
		Self::from(f as f64)
	}
}

impl From<f64> for Percent {
	#[inline]
	fn from(f: f64) -> Self {
		return_bad_float!(f, Self::nan, Self::inf);

		let fract = &format_compact!("{:.2}", f.fract())[2..];

		Self(f, format_compact!("{}.{}%", str_64!(f as u64), fract))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn special() {
		assert_eq!(Percent::zero(),    "0.00%");
		assert_eq!(Percent::unknown(), "?.??%");
		assert_eq!(Percent::nan(),     NAN);
		assert_eq!(Percent::inf(),     INFINITY);

		assert_eq!(Percent::from(0.0), "0.00%");
		assert_eq!(Percent::from(f64::NAN), NAN);
		assert_eq!(Percent::from(f64::INFINITY), INFINITY);
		assert_eq!(Percent::from(f64::NEG_INFINITY), INFINITY);
	}

	#[test]
	fn percent() {
		assert_eq!(Percent::from(0.0),       "0.00%");
		assert_eq!(Percent::from(0.001),     "0.00%");
		assert_eq!(Percent::from(0.1),       "0.10%");
		assert_eq!(Percent::from(1.0),       "1.00%");
		assert_eq!(Percent::from(50.0),      "50.00%");
		assert_eq!(Percent::from(100.0),     "100.00%");
		assert_eq!(Percent::from(150.0),     "150.00%");
		assert_eq!(Percent::from(1_000.0),   "1,000.00%");
		assert_eq!(Percent::from(250_000.0), "250,000.00%");
	}

	#[test]
	fn percent_dot() {
		assert_eq!(Percent::new_1(0.0),         "0.0%");
		assert_eq!(Percent::new_1(1_000.123_4), "1,000.1%");
		assert_eq!(Percent::new_3(1_000.123_4), "1,000.123%");
		assert_eq!(Percent::new_4(1_000.123_4), "1,000.1234%");

		assert_eq!(Percent::new_1(0.1),             "0.1%");
		assert_eq!(Percent::new_1(10_000.123_4),    "10,000.1%");
		assert_eq!(Percent::new_3(100_000.123_4),   "100,000.123%");
		assert_eq!(Percent::new_4(1_000_000.123_4), "1,000,000.1234%");
	}

	#[test]
	fn from_unsigned() {
		assert_eq!(Percent::from(1_u32),         "1.00%");
		assert_eq!(Percent::from(1_000_u32),     "1,000.00%");
		assert_eq!(Percent::from(10_000_u32),    "10,000.00%");
		assert_eq!(Percent::from(100_000_u32),   "100,000.00%");
		assert_eq!(Percent::from(1_000_000_u32), "1,000,000.00%");
	}

	#[test]
	fn from_int() {
		assert_eq!(Percent::from(-1_i32),         "-1.00%");
		assert_eq!(Percent::from(-1_000_i32),     "-1,000.00%");
		assert_eq!(Percent::from(-10_000_i32),    "-10,000.00%");
		assert_eq!(Percent::from(-100_000_i32),   "-100,000.00%");
		assert_eq!(Percent::from(-1_000_000_i32), "-1,000,000.00%");
	}
}
