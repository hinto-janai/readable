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
/// The fractional floating point may or may not be rounded up/down in the [`String`].
///
/// The default [`Float::from`] implementation will print `3` decimal numbers.
///
/// This can be changed by using different functions when initially
/// creating the [`Float`], or converting an existing [`Float`], for example:
/// ```
/// # use readable::Float;
/// let f2 = Float::from_2(3.0);
/// let f6 = Float::from_6(3.0);
/// let f9 = Float::from_9(f2.inner());
///
/// assert!(f2 == 3.00);
/// assert!(f6 == 3.000000);
/// assert!(f9 == 3.000000000);
///```
///
/// ## Cloning
/// [`Clone`] may be expensive:
/// ```rust
/// # use readable::Float;
/// // Probably cheap (stack allocated string).
/// let a = Float::from(100.0);
/// let b = a.clone();
///
/// // Probably expensive (heap allocated string).
/// let a = Float::from(f64::MAX);
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
/// - Combined with another [`Self`]: `Float::from(1.0) + Float::from(1.0)`
/// - Or with the inner number itself: `Float::from(1.0) + 1.0`
///
/// They also have the same `panic!()` behavior on overflow as the normal ones, because internally,
/// it is just calling `.inner() $OPERATOR $NUMBER`.
///
/// ```rust
/// # use readable::*;
/// // Regular operators.
/// assert!(Float::from(10.0) + 10.0 == Float::from(20.0));
/// assert!(Float::from(10.0) - 10.0 == Float::from(0.0));
/// assert!(Float::from(10.0) / 10.0 == Float::from(1.0));
/// assert!(Float::from(10.0) * 10.0 == Float::from(100.0));
/// assert!(Float::from(10.0) % 10.0 == Float::from(0.0));
/// ```
/// Overflow example (floats don't panic in this case):
/// ```rust
/// # use readable::*;
/// let n = Float::from(f64::MAX) + f64::MAX;
/// assert!(n.is_inf());
/// ```
///
/// # Examples
/// ```rust
/// # use readable::Float;
/// assert!(Float::from(0.0) == "0.000");
///
/// // This gets rounded up to '.568'
/// assert!(Float::from(1234.5678) == "1,234.568");
/// // To prevent that, use 4 point.
/// assert!(Float::from_4(1234.5678) == "1,234.5678");
/// ```

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Float(f64, #[cfg_attr(feature = "bincode", bincode(with_serde))] CompactString);

impl_math!(Float, f64);
impl_traits!(Float, f64);

// Implements `from_X` functions.
macro_rules! impl_new {
	( $num:tt ) => {
		paste::item! {
			#[doc = "Same as [`Float::from`] but with `" $num "` floating point."]
			pub fn [<from_ $num>](f: f64) -> Self {
				handle_nan_string!(f);

				let fract = &format_compact!(concat!("{:.", $num, "}"), f.fract())[2..];
				Self(f, format_compact!("{}.{}", str_u64!(f as u64), fract))
			}
		}
	}
}

impl Float {
	impl_common!(f64);
	impl_not_const!();
	impl_usize!();
	impl_isize!();

	#[inline]
	/// Returns a [`Float`] with the [`f64`] value of `0.0`.
	///
	/// The [`String`] is set to [`ZERO_FLOAT`].
	pub const fn zero() -> Self {
		Self(0.0, CompactString::new_inline(ZERO_FLOAT))
	}

	#[inline]
	/// Returns a [`Float`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to [`UNKNOWN_FLOAT`].
	pub const fn unknown() -> Self {
		Self(f64::NAN, CompactString::new_inline(UNKNOWN_FLOAT))
	}

	#[inline]
	/// Returns a [`Float`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to [`NAN`].
	pub const fn nan() -> Self {
		Self(f64::NAN, CompactString::new_inline(NAN))
	}

	#[inline]
	/// Returns a [`Float`] with the [`f64`] value of [`f64::INFINITY`].
	///
	/// The [`String`] is set to [`INFINITY`].
	pub const fn inf() -> Self {
		Self(f64::INFINITY, CompactString::new_inline(INFINITY))
	}

	#[inline]
	/// Calls [`f64::is_nan`].
	pub fn is_nan(&self) -> bool {
		self.0.is_nan()
	}

	#[inline]
	/// Calls [`f64::is_infinite`].
	pub fn is_inf(&self) -> bool {
		self.0.is_infinite()
	}

	#[inline]
	/// Same as [`Float::from`] but with no floating point on the inner [`String`].
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
	pub fn from_0(f: f64) -> Self {
		handle_nan_string!(f);
		Self(f, format_compact!("{}", str_u64!(f as u64)))
	}

	impl_new!(1);
	impl_new!(2);

	seq_macro::seq!(N in 4..=18 {
		impl_new!(N);
	});
}

// Implementation Macro.
macro_rules! impl_u {
	($( $number:ty ),*) => {
		$(
			impl From<$number> for Float {
				#[inline]
				fn from(number: $number) -> Self {
					Self(number as f64, format_compact!("{}.000", str_u64!(number as u64)))
				}
			}
		)*
	}
}
impl_u!(u8,u16,u32);

macro_rules! impl_i {
	($($number:ty),*) => {
		$(
			impl From<$number> for Float {
				#[inline]
				fn from(number: $number) -> Self {
					Self(number as f64, format_compact!("{}.000", str_i64!(number as i64)))
				}
			}
		)*
	}
}
impl_i!(i8,i16,i32);

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

		Self(number as f64, format_compact!("{}.{}", str_u64!(number as u64), fract))
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

		Self(number, format_compact!("{}.{}", str_u64!(number as u64), fract))
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
		assert!(Float::from_0( 0.1)              == "0");
		assert!(Float::from_1( 0.1)              == "0.1");
		assert!(Float::from_2( 0.01)             == "0.01");
		assert!(Float::from(        0.001)            == "0.001");
		assert!(Float::from_4( 0.0001)           == "0.0001");
		assert!(Float::from_5( 0.00001)          == "0.00001");
		assert!(Float::from_6( 0.000001)         == "0.000001");
		assert!(Float::from_7( 0.0000001)        == "0.0000001");
		assert!(Float::from_8( 0.00000001)       == "0.00000001");
		assert!(Float::from_9( 0.000000001)      == "0.000000001");
		assert!(Float::from_10(0.0000000001)     == "0.0000000001");
		assert!(Float::from_11(0.00000000001)    == "0.00000000001");
		assert!(Float::from_12(0.000000000001)   == "0.000000000001");
		assert!(Float::from_13(0.0000000000001)  == "0.0000000000001");
		assert!(Float::from_14(0.00000000000001) == "0.00000000000001");
	}
}
