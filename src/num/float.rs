//---------------------------------------------------------------------------------------------------- Use
use compact_str::{format_compact,CompactString};
use crate::num::constants::{NAN,INFINITY};
use crate::macros::{
	return_bad_float,str_u64,str_i64,
	impl_common,impl_const,
	impl_usize,impl_isize,
	impl_math,impl_traits,
	impl_impl_math,
};
use crate::str::Str;
#[allow(unused_imports)]
use crate::num::{Int,Unsigned}; // docs

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
/// # use readable::num::Float;
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
/// inaccurate is around 14 decimal points (to the left and right combined).
///
/// Formatting [`Float`] is also quite slower than [`Unsigned`] and [`Int`].
///
/// ## Size
/// [`Str<20>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::num::*;
/// assert_eq!(std::mem::size_of::<Float>(), 32);
/// ```
///
/// ## Copy
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 22 byte array string, literally: [`Str<22>`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also either a [`String`].
/// ```rust
/// # use readable::num::Float;
/// let a = Float::from(100_000.0);
///
/// // Copy 'a', use 'b'.
/// let b = a;
/// assert!(b == 100_000.0);
///
/// // We can still use 'a'
/// assert!(a == 100_000.0);
/// ```
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
/// # use readable::num::*;
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
/// # use readable::num::Float;
/// assert_eq!(Float::from(0.0), "0.000");
///
/// // This gets rounded up to '.568'
/// assert_eq!(Float::from(1234.5678), "1,234.568");
/// // To prevent that, use 4 point.
/// assert_eq!(Float::from_4(1234.5678), "1,234.5678");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Float(f64, Str<{ Float::MAX_LEN }>);

const LEN: usize = 22; // 14 decimal point accuracy + 8 extra chars

impl_math!(Float, f64);
impl_traits!(Float, f64);

//---------------------------------------------------------------------------------------------------- Float Constants
impl Float {
	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Float::ZERO, 0.0);
	/// assert_eq!(Float::ZERO, "0.000");
	/// ```
	pub const ZERO: Self = Self(0.0, Str::from_static_str("0.000"));

	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Float::NAN, "NaN");
	/// assert!(Float::NAN.is_nan());
	/// ```
	pub const NAN: Self = Self(f64::NAN, Str::from_static_str(NAN));

	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Float::INFINITY, "inf");
	/// assert!(Float::INFINITY.is_infinite());
	/// ```
	pub const INFINITY: Self = Self(f64::INFINITY, Str::from_static_str(INFINITY));

	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Float::UNKNOWN, 0.0);
	/// assert_eq!(Float::UNKNOWN, "?.???");
	/// ```
	pub const UNKNOWN: Self = Self(0.0, Str::from_static_str("?.???"));

	/// The maximum string length of a [`Float`].
	///
	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Float::MAX_LEN, 22);
	/// ```
	pub const MAX_LEN: usize = LEN;
}

//---------------------------------------------------------------------------------------------------- Float Impl
// Implements `from_X` functions.
macro_rules! impl_new {
	( $num:tt ) => {
		paste::item! {
			#[doc = "Same as [`Float::from`] but with `" $num "` floating point."]
			#[must_use]
			pub fn [<from_ $num>](f: f64) -> Self {
				return_bad_float!(f, Self::NAN, Self::INFINITY);

				let fract = &format_compact!(concat!("{:.", $num, "}"), f.fract())[2..];
				let string = format_compact!("{}.{}", str_u64!(f as u64), fract);
				if string.len() > Self::MAX_LEN {
					Self::UNKNOWN
				} else {
					let mut s = Str::new();
					s.push_str_panic(string);
					Self(f, s)
				}
			}
		}
	}
}

impl Float {
	impl_common!(f64);
	impl_const!();
	impl_usize!();
	impl_isize!();

	#[inline]
	#[must_use]
	/// Calls [`f64::is_nan`].
	pub fn is_nan(&self) -> bool {
		self.0.is_nan()
	}

	#[inline]
	#[must_use]
	/// Calls [`f64::is_infinite`].
	pub fn is_infinite(&self) -> bool {
		self.0.is_infinite()
	}

	#[inline]
	#[must_use]
	/// ```rust
	/// # use readable::num::*;
	/// assert!(Float::UNKNOWN.is_unknown());
	/// assert!(!Float::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		matches!(self.as_str().as_bytes(), b"?.???")
	}

	#[inline]
	#[must_use]
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
		return_bad_float!(f, Self::NAN, Self::INFINITY);
		let string = crate::num::Unsigned::from_priv_inner(f as u64);
		if string.len() > Self::MAX_LEN {
			Self::UNKNOWN
		} else {
			let mut s = Str::new();
			s.push_str_panic(string);
			Self(f, s)
		}
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
					let string = format_compact!("{}.000", str_u64!(number as u64));
					if string.len() > Self::MAX_LEN {
						Self::UNKNOWN
					} else {
						let mut s = Str::new();
						s.push_str_panic(string);
						Self(number as f64, s)
					}
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
					let string = format_compact!("{}.000", str_i64!(number as i64));
					if string.len() > Self::MAX_LEN {
						Self::UNKNOWN
					} else {
						let mut s = Str::new();
						s.push_str_panic(string);
						Self(number as f64, s)
					}
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
		return_bad_float!(f, Self::NAN, Self::INFINITY);
		#[allow(clippy::cast_lossless)]
		Self::from(f as f64)
	}
}

impl From<f64> for Float {
	#[inline]
	fn from(f: f64) -> Self {
		return_bad_float!(f, Self::NAN, Self::INFINITY);

		let fract = &format_compact!("{:.3}", f.fract())[2..];
		let string = format_compact!("{}.{}", str_u64!(f as u64), fract);
		if string.len() > Self::MAX_LEN {
			Self::UNKNOWN
		} else {
			let mut s = Str::new();
			s.push_str_panic(string);
			Self(f, s)
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn special() {
		assert_eq!(Float::from(0.0), "0.000");
		assert_eq!(Float::ZERO,      "0.000");
		assert_eq!(Float::NAN,       NAN);
		assert_eq!(Float::INFINITY,  INFINITY);

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

	#[test]
	#[cfg(feature = "serde")]
	fn serde() {
		let this: Float = Float::from(1.0);
		let json = serde_json::to_string(&this).unwrap();
		assert_eq!(json, r#"[1.0,"1.000"]"#);

		let this: Float = serde_json::from_str(&json).unwrap();
		assert_eq!(this, 1.0);
		assert_eq!(this, "1.000");

		// Bad bytes.
		assert!(serde_json::from_str::<Float>(&"---").is_err());

		// Unknown.
		let json = serde_json::to_string(&Float::UNKNOWN).unwrap();
		assert_eq!(json, r#"[0.0,"?.???"]"#);
		assert!(serde_json::from_str::<Float>(&json).unwrap().is_unknown());
	}

	#[test]
	#[cfg(feature = "bincode")]
	fn bincode() {
		let this: Float = Float::from(1.0);
		let config = bincode::config::standard();
		let bytes = bincode::encode_to_vec(&this, config).unwrap();

		let this: Float = bincode::decode_from_slice(&bytes, config).unwrap().0;
		assert_eq!(this, 1.0);
		assert_eq!(this, "1.000");

		// Unknown.
		let bytes = bincode::encode_to_vec(&Float::UNKNOWN, config).unwrap();
		let this: Float = bincode::decode_from_slice(&bytes, config).unwrap().0;
		assert!(this.is_unknown());
	}

	#[test]
	#[cfg(feature = "borsh")]
	fn borsh() {
		let this: Float = Float::from(1.0);
		let bytes = borsh::to_vec(&this).unwrap();

		let this: Float = borsh::from_slice(&bytes).unwrap();
		assert_eq!(this, 1.0);
		assert_eq!(this, "1.000");

		// Bad bytes.
		assert!(borsh::from_slice::<Float>(b"bad .-;[]124/ bytes").is_err());

		// Unknown.
		let bytes = borsh::to_vec(&Float::UNKNOWN).unwrap();
		let this: Float = borsh::from_slice(&bytes).unwrap();
		assert!(this.is_unknown());
	}
}
