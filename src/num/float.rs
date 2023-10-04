//---------------------------------------------------------------------------------------------------- Use
use compact_str::{format_compact,CompactString};
use crate::num::{
	Unsigned,Int,
	constants::{
		NAN,UNKNOWN_FLOAT,
		INFINITY,ZERO_FLOAT,
	},
};
use crate::macros::{
	return_bad_float,str_u64,str_i64,
	impl_common,impl_not_const,
	impl_usize,impl_isize,
	impl_math,impl_traits,
	impl_impl_math,
};

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
/// ## Warning
/// This type (and this library in general) is meant for fast and
/// simple data formatting, and not necessarily correctness.
///
/// [`Float`] internally converts to a `u64` to add commas and as such
/// the maximum input values for [`Float`] before it starts becoming
/// inaccurate is somewhere right before [`u64::MAX`].
///
/// Formatting [`Float`] is also quite slower than [`Unsigned`] and [`Int`].
///
/// ## Size
/// This type may or may not be heap allocated.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<Float>(), 32);
/// ```
///
/// ## Cloning
/// [`Clone`] may be a heap allocation clone:
/// ```rust
/// # use readable::Float;
/// // Stack allocated string.
/// let a = Float::from(100.0);
/// let b = a.clone();
///
/// // Heap allocated string.
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
/// Inputting [`f64::NAN`], [`f64::INFINITY`], [`f64::NEG_INFINITY`] or the [`f32`] variants returns errors
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
/// ```rust
/// # use readable::*;
/// // Regular operators.
/// assert!(Float::from(10.0) + 10.0 == Float::from(20.0));
/// assert!(Float::from(10.0) - 10.0 == Float::from(0.0));
/// assert!(Float::from(10.0) / 10.0 == Float::from(1.0));
/// assert!(Float::from(10.0) * 10.0 == Float::from(100.0));
/// assert!(Float::from(10.0) % 10.0 == Float::from(0.0));
/// ```
///
/// # Examples
/// ```rust
/// # use readable::Float;
/// assert_eq!(Float::from(0.0), "0.000");
///
/// // This gets rounded up to '.568'
/// assert_eq!(Float::from(1234.5678), "1,234.568");
/// // To prevent that, use 4 point.
/// assert_eq!(Float::from_4(1234.5678), "1,234.5678");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Float(f64, #[cfg_attr(feature = "bincode", bincode(with_serde))] CompactString);

impl_math!(Float, f64);
impl_traits!(Float, f64);

//---------------------------------------------------------------------------------------------------- Float Impl
// Implements `from_X` functions.
macro_rules! impl_new {
	( $num:tt ) => {
		paste::item! {
			#[doc = "Same as [`Float::from`] but with `" $num "` floating point."]
			pub fn [<from_ $num>](f: f64) -> Self {
				return_bad_float!(f, Self::nan, Self::inf);

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
		return_bad_float!(f, Self::nan, Self::inf);
		Self(f, CompactString::from(str_u64!(f as u64)))
	}

	seq_macro::seq!(N in 1..=14 {
		impl_new!(N);
	});
}

//---------------------------------------------------------------------------------------------------- From `u*`
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
impl_u!(u8,u16,u32,u64,usize);

//---------------------------------------------------------------------------------------------------- From `i*`
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
impl_i!(i8,i16,i32,i64,isize);

//---------------------------------------------------------------------------------------------------- From `f32/f64`
impl From<f32> for Float {
	#[inline]
	fn from(f: f32) -> Self {
		return_bad_float!(f, Self::nan, Self::inf);
		Self::from(f as f64)
	}
}

impl From<f64> for Float {
	#[inline]
	fn from(f: f64) -> Self {
		return_bad_float!(f, Self::nan, Self::inf);

		let fract = &format_compact!("{:.3}", f.fract())[2..];

		Self(f, format_compact!("{}.{}", str_u64!(f as u64), fract))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn special() {
		assert_eq!(Float::from(0.0), "0.000");
		assert_eq!(Float::zero(),    "0.000");
		assert_eq!(Float::unknown(), UNKNOWN_FLOAT);
		assert_eq!(Float::nan(),     NAN);
		assert_eq!(Float::inf(),     INFINITY);

		assert_eq!(Float::from(f64::NAN),          NAN);
		assert_eq!(Float::from(f64::INFINITY),     INFINITY);
		assert_eq!(Float::from(f64::NEG_INFINITY), INFINITY);

		assert_eq!(Float::from(f32::NAN),           NAN);
		assert_eq!(Float::from(f32::INFINITY),      INFINITY);
		assert_eq!(Float::from(f32::NEG_INFINITY), INFINITY);
	}

	#[test]
	fn float() {
		assert_eq!(Float::from_0(0.1),               "0");
		assert_eq!(Float::from_1(0.1),               "0.1");
		assert_eq!(Float::from_2(0.01),              "0.01");
		assert_eq!(Float::from(0.001),               "0.001");
		assert_eq!(Float::from_4(0.0001),            "0.0001");
		assert_eq!(Float::from_5(0.00001),           "0.00001");
		assert_eq!(Float::from_6(0.000001),          "0.000001");
		assert_eq!(Float::from_7(0.0000001),         "0.0000001");
		assert_eq!(Float::from_8(0.00000001),        "0.00000001");
		assert_eq!(Float::from_9(0.000000001),       "0.000000001");
		assert_eq!(Float::from_10(0.0000000001),     "0.0000000001");
		assert_eq!(Float::from_11(0.00000000001),    "0.00000000001");
		assert_eq!(Float::from_12(0.000000000001),   "0.000000000001");
		assert_eq!(Float::from_13(0.0000000000001),  "0.0000000000001");
		assert_eq!(Float::from_14(0.00000000000001), "0.00000000000001");
	}
}
