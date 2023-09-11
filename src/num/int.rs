//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

use std::num::*;
use crate::macros::{
	handle_float,
	handle_nan_string,
	impl_common,
	impl_const,
	impl_isize,
	impl_buffer,
	impl_math,
	impl_impl_math,
	impl_traits,
	buffer,
};
use crate::num::constants::{
	MAX_BUF_LEN,UNKNOWN_NUM_BUFFER,UNKNOWN,
	ZERO_NUM_BUFFER,
};

//---------------------------------------------------------------------------------------------------- Int
/// Human readable signed integer.
///
/// ## Creation
/// For [`i8`], [`i16`], [`i32`], [`i64`], [`isize`] or any [`NonZeroI8`] variant:
/// - Use [`Int::from`]
///
/// [`f32`] or [`f64`] inputs will work, but:
/// - Fractional parts will be ignored
/// - Under/overflows will return [`Int::unknown`]
/// - Special floats like [`f64::NAN`] will return [`Int::unknown`]
///
/// For [`u8`] and other unsigned integers:
/// - You can use [`Int::from`] for anything under [`u32`]
/// - You need to use [`Int::try_from`] for anything above [`u32`]
/// - [`Int::unknown`] will be returned on error
///
/// ## Cloning
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 26 byte array buffer, literally: `[u8; 26]`.
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
/// ```rust
/// # use readable::Int;
/// let a = Int::from(100_000);
///
/// // Copy 'a', use 'b'.
/// let b = a;
/// assert!(b == 100_000);
///
/// // We can still use 'a'
/// assert!(a == 100_000);
/// ```
///
/// ## Float Errors
/// - Inputting [`f64::NAN`] returns [`Int::unknown`]
/// - Inputting [`f64::INFINITY`] returns [`Int::unknown`]
/// - Inputting [`f64::NEG_INFINITY`] returns [`Int::unknown`]
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
/// - Combined with another [`Self`]: `Int::from(1) + Int::from(1)`
/// - Or with the inner number itself: `Int::from(1) + 1`
///
/// They also have the same `panic!()` behavior on overflow as the normal ones, because internally,
/// it is just calling `.inner() $OPERATOR $NUMBER`.
///
/// ```rust
/// # use readable::*;
/// assert!(Int::from(10) + 10 == Int::from(20));
/// assert!(Int::from(10) - 10 == Int::from(0));
/// assert!(Int::from(10) / 10 == Int::from(1));
/// assert!(Int::from(10) * 10 == Int::from(100));
/// assert!(Int::from(10) % 10 == Int::from(0));
/// ```
/// Overflow example:
/// ```rust,should_panic
/// # use readable::*;
/// let n = Int::from(i64::MAX) + i64::MAX;
/// ```
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
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Int(i64, Buffer);

impl Int {
	impl_common!(i64);
	impl_const!();
	impl_isize!();
	impl_buffer!(MAX_BUF_LEN, UNKNOWN_BUFFER, UNKNOWN.len());

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Int::zero(), 0);
	/// assert_eq!(Int::zero(), "0");
	/// assert_eq!(Int::zero() + Int::zero(), 0);
	/// ```
	pub const fn zero() -> Self {
		Self(0, Buffer::zero())
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// # use readable::num::*;
	/// assert_eq!(Int::unknown(), UNKNOWN);
	/// ```
	pub const fn unknown() -> Self {
		Self(0, Buffer::unknown())
	}
}

macro_rules! impl_i {
	($( $from:ty ),*) => {
		$(
			impl From<$from> for Int {
				fn from(int: $from) -> Self {
					let i = int as i64;
					Self(i, Buffer::from_i(i))
				}
			}
		)*
	}
}
impl_i!(i8,i16,i32,i64,isize,u8,u16,u32);

macro_rules! impl_noni {
	($( $from:ty ),*) => {
		$(
			impl From<$from> for Int {
				fn from(int: $from) -> Self {
					let i = int.get() as i64;
					Self(i, Buffer::from_i(i))
				}
			}
		)*
	}
}
impl_noni! {
	NonZeroI8,NonZeroI16,NonZeroI32,NonZeroI64,NonZeroIsize,
	&NonZeroI8,&NonZeroI16,&NonZeroI32,&NonZeroI64,&NonZeroIsize
}

macro_rules! impl_f {
	($from:ty) => {
		/// This will silently return [`Self::unknown`]
		/// if the input float is `NAN`, `INFINITY`, or under/overflows.
		impl From<$from> for Int {
			fn from(float: $from) -> Self {
				handle_float!(|| Self::unknown(), float);
				let i = float as i64;
				Self(i, Buffer::from_i(i))
			}
		}
	}
}
impl_f!(f32);
impl_f!(f64);

macro_rules! impl_try {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::unknown`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Int {
				type Error = Self;
				fn try_from(num: $from) -> Result<Self, Self> {
					match i64::try_from(num) {
						Ok(i) => Ok(Self(i, Buffer::from_i(i))),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_try!(u64,usize);

macro_rules! impl_noni {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::unknown`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Int {
				type Error = Self;
				fn try_from(num: $from) -> Result<Self, Self> {
					match i64::try_from(num.get()) {
						Ok(i) => Ok(Self(i, Buffer::from_i(i))),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_noni! {
	NonZeroU8,NonZeroU16,NonZeroU32,NonZeroU64,NonZeroUsize,
	&NonZeroU8,&NonZeroU16,&NonZeroU32,&NonZeroU64,&NonZeroUsize
}

impl_math!(Int, i64);
impl_traits!(Int, i64);

//---------------------------------------------------------------------------------------------------- Buffer
buffer!(MAX_BUF_LEN, UNKNOWN_NUM_BUFFER, UNKNOWN.len());

impl Buffer {
	#[inline(always)]
	const fn zero() -> Self {
		Self {
			buf: ZERO_NUM_BUFFER,
			len: 1,
		}
	}

	#[inline(always)]
	fn from_i(i: i64) -> Self {
		let (buf, len) = crate::num::buf::from_i(i);
		Self { buf, len }
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn unsigned() {
		assert_eq!(Int::from(1_000_i64),                  "1,000");
		assert_eq!(Int::from(65_535_i64),                 "65,535");
		assert_eq!(Int::from(65_536_i64),                 "65,536");
		assert_eq!(Int::from(100_000_i64),                "100,000");
		assert_eq!(Int::from(1_000_000_i64),              "1,000,000");
		assert_eq!(Int::from(10_000_000_i64),             "10,000,000");
		assert_eq!(Int::from(100_000_000_i64),            "100,000,000");
		assert_eq!(Int::from(1_000_000_000_i64),          "1,000,000,000");
		assert_eq!(Int::from(4_294_967_295_i64),          "4,294,967,295");
		assert_eq!(Int::from(4_294_967_296_i64),          "4,294,967,296");
		assert_eq!(Int::from(10_000_000_000_i64),         "10,000,000,000");
		assert_eq!(Int::from(100_000_000_000_i64),        "100,000,000,000");
		assert_eq!(Int::from(1_000_000_000_000_i64),      "1,000,000,000,000");
		assert_eq!(Int::from(10_000_000_000_000_i64),     "10,000,000,000,000");
		assert_eq!(Int::from(100_000_000_000_000_i64),    "100,000,000,000,000");
		assert_eq!(Int::from(1_000_000_000_000_000_i64),  "1,000,000,000,000,000");
		assert_eq!(Int::from(10_000_000_000_000_000_i64), "10,000,000,000,000,000");
	}

	#[test]
	fn int() {
		assert_eq!(Int::from(-1_000_i64),                  "-1,000");
		assert_eq!(Int::from(-65_535_i64),                 "-65,535");
		assert_eq!(Int::from(-65_536_i64),                 "-65,536");
		assert_eq!(Int::from(-100_000_i64),                "-100,000");
		assert_eq!(Int::from(-1_000_000_i64),              "-1,000,000");
		assert_eq!(Int::from(-10_000_000_i64),             "-10,000,000");
		assert_eq!(Int::from(-100_000_000_i64),            "-100,000,000");
		assert_eq!(Int::from(-1_000_000_000_i64),          "-1,000,000,000");
		assert_eq!(Int::from(-4_294_967_295_i64),          "-4,294,967,295");
		assert_eq!(Int::from(-4_294_967_296_i64),          "-4,294,967,296");
		assert_eq!(Int::from(-10_000_000_000_i64),         "-10,000,000,000");
		assert_eq!(Int::from(-100_000_000_000_i64),        "-100,000,000,000");
		assert_eq!(Int::from(-1_000_000_000_000_i64),      "-1,000,000,000,000");
		assert_eq!(Int::from(-10_000_000_000_000_i64),     "-10,000,000,000,000");
		assert_eq!(Int::from(-100_000_000_000_000_i64),    "-100,000,000,000,000");
		assert_eq!(Int::from(-1_000_000_000_000_000_i64),  "-1,000,000,000,000,000");
		assert_eq!(Int::from(-10_000_000_000_000_000_i64), "-10,000,000,000,000,000");

		assert_eq!(Int::from(i64::MIN), "-9,223,372,036,854,775,808");
		assert_eq!(Int::from(i64::MAX), "9,223,372,036,854,775,807");
	}

	#[test]
	fn float() {
		assert_eq!(Int::from(-1_000.0),                 "-1,000");
		assert_eq!(Int::from(-65_535.0),                "-65,535");
		assert_eq!(Int::from(-65_536.0),                "-65,536");
		assert_eq!(Int::from(-100_000.0),               "-100,000");
		assert_eq!(Int::from(-1_000_000.0),             "-1,000,000");
		assert_eq!(Int::from(-10_000_000.0),            "-10,000,000");
		assert_eq!(Int::from(-100_000_000.0),           "-100,000,000");
		assert_eq!(Int::from(-1_000_000_000.0),         "-1,000,000,000");
		assert_eq!(Int::from(-4_294_967_295.0),         "-4,294,967,295");
		assert_eq!(Int::from(-4_294_967_296.0),         "-4,294,967,296");
		assert_eq!(Int::from(-10_000_000_000.0),        "-10,000,000,000");
		assert_eq!(Int::from(-100_000_000_000.0),       "-100,000,000,000");
		assert_eq!(Int::from(-1_000_000_000_000.0),     "-1,000,000,000,000");
		assert_eq!(Int::from(-10_000_000_000_000.0),    "-10,000,000,000,000");
		assert_eq!(Int::from(-100_000_000_000_000.0),   "-100,000,000,000,000");
		assert_eq!(Int::from(-1_000_000_000_000_000.0), "-1,000,000,000,000,000");
		assert_eq!(Int::from(i64::MIN as f64),          "-9,223,372,036,854,775,808");
		assert_eq!(Int::from(i64::MAX as f64),          "9,223,372,036,854,775,807");
	}

	#[test]
	fn special() {
		assert_eq!(Int::from(f64::NAN),          UNKNOWN);
		assert_eq!(Int::from(f64::INFINITY),     UNKNOWN);
		assert_eq!(Int::from(f64::NEG_INFINITY), UNKNOWN);
	}
}
