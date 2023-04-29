//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

use std::num::*;
use crate::inner::*;
use crate::macros::*;
use crate::constants::*;

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
	/// assert!(Int::zero() == 0);
	/// ```
	pub const fn zero() -> Self {
		Self(0, Buffer::zero())
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(Int::unknown() == UNKNOWN);
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
				handle_nan_runtime!(float);
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
		let mut buffer = itoa::Buffer::new();
		let string = &buffer.format(i).as_bytes();
		let mut buf = [0_u8; MAX_BUF_LEN];

		if i.is_negative() {
			let len = match string.len() {
				// Must be at least two bytes: `-1`
				2 => { crate::buf::from_neg_2(&mut buf, &string); 2 },
				3 => { crate::buf::from_neg_3(&mut buf, &string); 3 },
				4 => { crate::buf::from_neg_4(&mut buf, &string); 4 },
				5 => { crate::buf::from_neg_5(&mut buf, &string); 6 },
				6 => { crate::buf::from_neg_6(&mut buf, &string); 7 },
				7 => { crate::buf::from_neg_7(&mut buf, &string); 8 },
				8 => { crate::buf::from_neg_8(&mut buf, &string); 10 },
				9 => { crate::buf::from_neg_9(&mut buf, &string); 11 },
				10 => { crate::buf::from_neg_10(&mut buf, &string); 12 },
				11 => { crate::buf::from_neg_11(&mut buf, &string); 14 },
				12 => { crate::buf::from_neg_12(&mut buf, &string); 15 },
				13 => { crate::buf::from_neg_13(&mut buf, &string); 16 },
				14 => { crate::buf::from_neg_14(&mut buf, &string); 18 },
				15 => { crate::buf::from_neg_15(&mut buf, &string); 19 },
				16 => { crate::buf::from_neg_16(&mut buf, &string); 20 },
				17 => { crate::buf::from_neg_17(&mut buf, &string); 22 },
				18 => { crate::buf::from_neg_18(&mut buf, &string); 23 },
				19 => { crate::buf::from_neg_19(&mut buf, &string); 24 },
				20 => { crate::buf::from_neg_20(&mut buf, &string); 26 },

				// We've covered all possible negative `i64` lengths.
				_ => unreachable!(),
			};
			Self { buf, len }
		} else {
			let len = match i {
				0..=9                         => { crate::buf::from_1(&mut buf, &string); 1 },
				0..=99                        => { crate::buf::from_2(&mut buf, &string); 2 },
				0..=999                       => { crate::buf::from_3(&mut buf, &string); 3 },
				0..=9_999                     => { crate::buf::from_4(&mut buf, &string); 5 },
				0..=99_999                    => { crate::buf::from_5(&mut buf, &string); 6 },
				0..=999_999                   => { crate::buf::from_6(&mut buf, &string); 7 },
				0..=9_999_999                 => { crate::buf::from_7(&mut buf, &string); 9 },
				0..=99_999_999                => { crate::buf::from_8(&mut buf, &string); 10 },
				0..=999_999_999               => { crate::buf::from_9(&mut buf, &string); 11 },
				0..=9_999_999_999             => { crate::buf::from_10(&mut buf, &string); 13 },
				0..=99_999_999_999            => { crate::buf::from_11(&mut buf, &string); 14 },
				0..=999_999_999_999           => { crate::buf::from_12(&mut buf, &string); 15 },
				0..=9_999_999_999_999         => { crate::buf::from_13(&mut buf, &string); 17 },
				0..=99_999_999_999_999        => { crate::buf::from_14(&mut buf, &string); 18 },
				0..=999_999_999_999_999       => { crate::buf::from_15(&mut buf, &string); 19 },
				0..=9_999_999_999_999_999     => { crate::buf::from_16(&mut buf, &string); 21 },
				0..=99_999_999_999_999_999    => { crate::buf::from_17(&mut buf, &string); 22 },
				0..=999_999_999_999_999_999   => { crate::buf::from_18(&mut buf, &string); 23 },
				0..=9_223_372_036_854_775_807 => { crate::buf::from_19(&mut buf, &string); 25 },

				// We've covered all possible positive `i64` lengths.
				_ => unreachable!(),
			};
			Self { buf, len }
		}
	}
}

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
