//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::num::{
	Unsigned,
	constants::{
		MAX_LEN_NUM,ZERO_NUM,
		UNKNOWN_NUM,COMMA,
		MAX_INT,MIN_INT,
	},
};
use crate::macros::{
	impl_common,impl_const,
	impl_isize,impl_math,
	impl_traits,impl_impl_math,
};
use std::num::{
	NonZeroU8,NonZeroU16,NonZeroU32,
	NonZeroU64,NonZeroUsize,
	NonZeroI8,NonZeroI16,NonZeroI32,
	NonZeroI64,NonZeroIsize,
};

//---------------------------------------------------------------------------------------------------- Int
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Human readable signed integer.
///
/// ## Creation
/// For [`i8`], [`i16`], [`i32`], [`i64`], [`isize`] or any [`NonZeroI8`] variant:
/// - Use [`Int::from`]
///
/// [`f32`] or [`f64`] inputs must use [`Unsigned::try_from`] and:
/// - Fractional parts will be ignored
/// - Under/overflows will return [`Int::unknown`]
/// - Special floats like [`f64::NAN`] will return [`Int::unknown`]
///
/// For [`u8`] and other unsigned integers:
/// - You can use [`Int::from`] for anything under [`u32`]
/// - You need to use [`Int::try_from`] for anything above [`u32`]
/// - [`Int::unknown`] will be returned on error
///
/// ## Copy
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 26 byte array string, literally: [`Str<26>`].
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
/// ```rust
/// # use readable::*;
/// assert!(Int::from(10) + 10 == Int::from(20));
/// assert!(Int::from(10) - 10 == Int::from(0));
/// assert!(Int::from(10) / 10 == Int::from(1));
/// assert!(Int::from(10) * 10 == Int::from(100));
/// assert!(Int::from(10) % 10 == Int::from(0));
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
/// assert!(Int::try_from(-1.0).unwrap()        == "-1");
/// assert!(Int::try_from(1_000.123).unwrap()   == "1,000");
/// assert!(Int::try_from(100_000.123).unwrap() == "100,000");
/// assert!(Int::try_from(100_000.123).unwrap() == "100,000");
/// ```
pub struct Int(i64, Str<MAX_LEN_NUM>);

impl_math!(Int, i64);
impl_traits!(Int, i64);

//---------------------------------------------------------------------------------------------------- Int Impl
impl Int {
	impl_common!(i64);
	impl_const!();
	impl_isize!();

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Int::zero(), 0);
	/// assert_eq!(Int::zero(), "0");
	/// assert_eq!(Int::zero() + Int::zero(), 0);
	/// ```
	pub const fn zero() -> Self {
		Self(0, Str::from_static_str(ZERO_NUM))
	}

	#[inline]
	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Int::max(), i64::MAX);
	/// ```
	pub const fn max() -> Self {
		Self(i64::MAX, Str::from_static_str(MAX_INT))
	}

	#[inline]
	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Int::min(), i64::MIN);
	/// ```
	pub const fn min() -> Self {
		Self(i64::MIN, Str::from_static_str(MIN_INT))
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// # use readable::num::*;
	/// assert_eq!(Int::unknown(), UNKNOWN_NUM);
	/// ```
	pub const fn unknown() -> Self {
		Self(0, Str::from_static_str(UNKNOWN_NUM))
	}
}

//---------------------------------------------------------------------------------------------------- Private functions.
impl Int {
	// Main frontend function for construction.
	//
	// Branches out depending on the length of the number.
	#[inline]
	#[allow(clippy::match_overlapping_arm)]
	fn from_priv(i: i64) -> Self {
		// Format the `u64` into a `str`.
		let mut itoa = crate::Itoa64::new();
		let itoa = itoa.format(i);

		// Create our destination string byte array.
		let mut s = [0; MAX_LEN_NUM];

		let itoa_len = itoa.len();

		// Match, write properly comma itoa
		// bytes and return the total length.
		if i.is_negative() {
			let len = match itoa_len {
				// Must be at least two bytes: `-1`
				2 =>  { Self::from_neg_2(&mut s, itoa); 2 },
				3 =>  { Self::from_neg_3(&mut s, itoa); 3 },
				4 =>  { Self::from_neg_4(&mut s, itoa); 4 },
				5 =>  { Self::from_neg_5(&mut s, itoa); 6 },
				6 =>  { Self::from_neg_6(&mut s, itoa); 7 },
				7 =>  { Self::from_neg_7(&mut s, itoa); 8 },
				8 =>  { Self::from_neg_8(&mut s, itoa); 10 },
				9 =>  { Self::from_neg_9(&mut s, itoa); 11 },
				10 => { Self::from_neg_10(&mut s, itoa); 12 },
				11 => { Self::from_neg_11(&mut s, itoa); 14 },
				12 => { Self::from_neg_12(&mut s, itoa); 15 },
				13 => { Self::from_neg_13(&mut s, itoa); 16 },
				14 => { Self::from_neg_14(&mut s, itoa); 18 },
				15 => { Self::from_neg_15(&mut s, itoa); 19 },
				16 => { Self::from_neg_16(&mut s, itoa); 20 },
				17 => { Self::from_neg_17(&mut s, itoa); 22 },
				18 => { Self::from_neg_18(&mut s, itoa); 23 },
				19 => { Self::from_neg_19(&mut s, itoa); 24 },
				// We've covered all possible negative `i64` lengths.
				_ => { Self::from_neg_20(&mut s, itoa); 26 },
			};

			// SAFETY: we're manually creating a `Str`.
			// This is okay because we filled the bytes
			// and know the length.
			Self(i, unsafe { Str::from_raw(len, s) })

		} else {
			let len = match itoa_len {
				1  => { Unsigned::from_1(&mut s, itoa); 1 },
				2  => { Unsigned::from_2(&mut s, itoa); 2 },
				3  => { Unsigned::from_3(&mut s, itoa); 3 },
				4  => { Unsigned::from_4(&mut s, itoa); 5 },
				5  => { Unsigned::from_5(&mut s, itoa); 6 },
				6  => { Unsigned::from_6(&mut s, itoa); 7 },
				7  => { Unsigned::from_7(&mut s, itoa); 9 },
				8  => { Unsigned::from_8(&mut s, itoa); 10 },
				9  => { Unsigned::from_9(&mut s, itoa); 11 },
				10 => { Unsigned::from_10(&mut s, itoa); 13 },
				11 => { Unsigned::from_11(&mut s, itoa); 14 },
				12 => { Unsigned::from_12(&mut s, itoa); 15 },
				13 => { Unsigned::from_13(&mut s, itoa); 17 },
				14 => { Unsigned::from_14(&mut s, itoa); 18 },
				15 => { Unsigned::from_15(&mut s, itoa); 19 },
				16 => { Unsigned::from_16(&mut s, itoa); 21 },
				17 => { Unsigned::from_17(&mut s, itoa); 22 },
				18 => { Unsigned::from_18(&mut s, itoa); 23 },
				// We've covered all possible positive `i64` lengths.
				_ => { Unsigned::from_19(&mut s, itoa); 25 },
			};

			// SAFETY: we're manually creating a `Str`.
			// This is okay because we filled the bytes
			// and know the length.
			Self(i, unsafe { Str::from_raw(len, s) })
		}
	}

	#[inline]
	// -9
	fn from_neg_2(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2])
	}

	#[inline]
	// -99
	fn from_neg_3(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
	}

	#[inline]
	// -999
	fn from_neg_4(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..4].copy_from_slice(&itoa[0..4]);
	}

	#[inline]
	// -9,999
	fn from_neg_5(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
		s[2] = COMMA;
		s[3..6].copy_from_slice(&itoa[2..5]);
	}

	#[inline]
	// -99,999
	fn from_neg_6(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
		s[3] = COMMA;
		s[4..7].copy_from_slice(&itoa[3..6]);
	}

	#[inline]
	// -999,999
	fn from_neg_7(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..4].copy_from_slice(&itoa[0..4]);
		s[4] = COMMA;
		s[5..8].copy_from_slice(&itoa[4..7]);
	}

	#[inline]
	// -9,999,999
	fn from_neg_8(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
		s[2] = COMMA;
		s[3..6].copy_from_slice(&itoa[2..5]);
		s[6] = COMMA;
		s[7..10].copy_from_slice(&itoa[5..8]);
	}

	#[inline]
	// -99,999,999
	fn from_neg_9(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
		s[3] = COMMA;
		s[4..7].copy_from_slice(&itoa[3..6]);
		s[7] = COMMA;
		s[8..11].copy_from_slice(&itoa[6..9]);
	}

	#[inline]
	// -999,999,999
	fn from_neg_10(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..4].copy_from_slice(&itoa[0..4]);
		s[4] = COMMA;
		s[5..8].copy_from_slice(&itoa[4..7]);
		s[8] = COMMA;
		s[9..12].copy_from_slice(&itoa[7..10]);
	}

	#[inline]
	// -9,999,999,999
	fn from_neg_11(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
		s[2] = COMMA;
		s[3..6].copy_from_slice(&itoa[2..5]);
		s[6] = COMMA;
		s[7..10].copy_from_slice(&itoa[5..8]);
		s[10] = COMMA;
		s[11..14].copy_from_slice(&itoa[8..11]);
	}

	#[inline]
	// -99,999,999,999
	fn from_neg_12(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
		s[3] = COMMA;
		s[4..7].copy_from_slice(&itoa[3..6]);
		s[7] = COMMA;
		s[8..11].copy_from_slice(&itoa[6..9]);
		s[11] = COMMA;
		s[12..15].copy_from_slice(&itoa[9..12]);
	}

	#[inline]
	// -999,999,999,999
	fn from_neg_13(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..4].copy_from_slice(&itoa[0..4]);
		s[4] = COMMA;
		s[5..8].copy_from_slice(&itoa[4..7]);
		s[8] = COMMA;
		s[9..12].copy_from_slice(&itoa[7..10]);
		s[12] = COMMA;
		s[13..16].copy_from_slice(&itoa[10..13]);
	}

	#[inline]
	// -9,999,999,999,999
	fn from_neg_14(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
		s[2] = COMMA;
		s[3..6].copy_from_slice(&itoa[2..5]);
		s[6] = COMMA;
		s[7..10].copy_from_slice(&itoa[5..8]);
		s[10] = COMMA;
		s[11..14].copy_from_slice(&itoa[8..11]);
		s[14] = COMMA;
		s[15..18].copy_from_slice(&itoa[11..14]);
	}

	#[inline]
	// -99,999,999,999,999
	fn from_neg_15(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
		s[3] = COMMA;
		s[4..7].copy_from_slice(&itoa[3..6]);
		s[7] = COMMA;
		s[8..11].copy_from_slice(&itoa[6..9]);
		s[11] = COMMA;
		s[12..15].copy_from_slice(&itoa[9..12]);
		s[15] = COMMA;
		s[16..19].copy_from_slice(&itoa[12..15]);
	}

	#[inline]
	// -999,999,999,999,999
	fn from_neg_16(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..4].copy_from_slice(&itoa[0..4]);
		s[4] = COMMA;
		s[5..8].copy_from_slice(&itoa[4..7]);
		s[8] = COMMA;
		s[9..12].copy_from_slice(&itoa[7..10]);
		s[12] = COMMA;
		s[13..16].copy_from_slice(&itoa[10..13]);
		s[16] = COMMA;
		s[17..20].copy_from_slice(&itoa[13..16]);
	}

	#[inline]
	// -9,999,999,999,999,999
	fn from_neg_17(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
		s[2] = COMMA;
		s[3..6].copy_from_slice(&itoa[2..5]);
		s[6] = COMMA;
		s[7..10].copy_from_slice(&itoa[5..8]);
		s[10] = COMMA;
		s[11..14].copy_from_slice(&itoa[8..11]);
		s[14] = COMMA;
		s[15..18].copy_from_slice(&itoa[11..14]);
		s[18] = COMMA;
		s[19..22].copy_from_slice(&itoa[14..17]);
	}

	#[inline]
	// -99,999,999,999,999,999
	fn from_neg_18(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
		s[3] = COMMA;
		s[4..7].copy_from_slice(&itoa[3..6]);
		s[7] = COMMA;
		s[8..11].copy_from_slice(&itoa[6..9]);
		s[11] = COMMA;
		s[12..15].copy_from_slice(&itoa[9..12]);
		s[15] = COMMA;
		s[16..19].copy_from_slice(&itoa[12..15]);
		s[19] = COMMA;
		s[20..23].copy_from_slice(&itoa[15..18]);
	}

	#[inline]
	// -999,999,999,999,999,999
	fn from_neg_19(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..4].copy_from_slice(&itoa[0..4]);
		s[4] = COMMA;
		s[5..8].copy_from_slice(&itoa[4..7]);
		s[8] = COMMA;
		s[9..12].copy_from_slice(&itoa[7..10]);
		s[12] = COMMA;
		s[13..16].copy_from_slice(&itoa[10..13]);
		s[16] = COMMA;
		s[17..20].copy_from_slice(&itoa[13..16]);
		s[20] = COMMA;
		s[21..24].copy_from_slice(&itoa[16..19]);
	}

	#[inline]
	// -9,999,999,999,999,999,999
	fn from_neg_20(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
		s[2] = COMMA;
		s[3..6].copy_from_slice(&itoa[2..5]);
		s[6] = COMMA;
		s[7..10].copy_from_slice(&itoa[5..8]);
		s[10] = COMMA;
		s[11..14].copy_from_slice(&itoa[8..11]);
		s[14] = COMMA;
		s[15..18].copy_from_slice(&itoa[11..14]);
		s[18] = COMMA;
		s[19..22].copy_from_slice(&itoa[14..17]);
		s[22] = COMMA;
		s[23..26].copy_from_slice(&itoa[17..20]);
	}
}

//---------------------------------------------------------------------------------------------------- From `i*`
macro_rules! impl_i {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Int {
				#[inline]
				fn from(int: $from) -> Self {
					let u = int as i64;
					Self::from_priv(u)
				}
			}
			impl From<&$from> for Int {
				#[inline]
				fn from(int: &$from) -> Self {
					let u = *int as i64;
					Self::from_priv(u)
				}
			}
		)*
	}
}
impl_i!(i8,i16,i32,i64,u8,u16,u32);
#[cfg(target_pointer_width = "64")]
impl_i!(isize);
#[cfg(not(target_pointer_width = "64"))]
impl_i!(usize);

//---------------------------------------------------------------------------------------------------- From `NonZeroI*`
macro_rules! impl_noni {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Int {
				fn from(int: $from) -> Self {
					let u = int.get() as i64;
					Self::from_priv(u)
				}
			}
		)*
	}
}
impl_noni! {
	NonZeroI8,NonZeroI16,NonZeroI32,NonZeroI64,
	&NonZeroI8,&NonZeroI16,&NonZeroI32,&NonZeroI64,
	NonZeroU8,NonZeroU16,NonZeroU32,
	&NonZeroU8,&NonZeroU16,&NonZeroU32,
}
#[cfg(target_pointer_width = "64")]
impl_noni!(NonZeroIsize,&NonZeroIsize);
#[cfg(not(target_pointer_width = "64"))]
impl_noni!(NonZeroUsize,&NonZeroUsize);

//---------------------------------------------------------------------------------------------------- From `u64/usize`
macro_rules! impl_try {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::unknown`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Int {
				type Error = Self;
				fn try_from(num: $from) -> Result<Self, Self> {
					match i64::try_from(num) {
						Ok(i) => Ok(Self::from_priv(i)),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_try!(u64);
#[cfg(target_pointer_width = "64")]
impl_try!(usize);

//---------------------------------------------------------------------------------------------------- From `readable::Unsigned`
macro_rules! impl_unsigned {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::unknown`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Int {
				type Error = Self;
				fn try_from(num: $from) -> Result<Self, Self> {
					match i64::try_from(num.inner()) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_unsigned!(Unsigned,&Unsigned);

//---------------------------------------------------------------------------------------------------- From `NonZeroU*`
macro_rules! impl_nonu {
	($( $from:ty ),* $(,)?) => {
		$(
			/// This will return [`Self::unknown`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Int {
				type Error = Self;
				fn try_from(num: $from) -> Result<Self, Self> {
					match i64::try_from(num.get()) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_noni! {
	NonZeroU64,&NonZeroU64,
}
#[cfg(target_pointer_width = "64")]
impl_noni!(NonZeroUsize,&NonZeroUsize);

//---------------------------------------------------------------------------------------------------- From `f32/f64`
macro_rules! impl_f {
	($from:ty) => {
		/// This will return [`Self::unknown`]
		/// if the input float is `NAN`, `INFINITY`, or negative.
		impl TryFrom<$from> for Int {
			type Error = Self;
			fn try_from(float: $from) -> Result<Self, Self> {
				match float.classify() {
					std::num::FpCategory::Normal   => (),
					std::num::FpCategory::Nan      => return Err(Self::unknown()),
					std::num::FpCategory::Infinite => return Err(Self::unknown()),
					_ => (),
				}

				Ok(Self::from_priv(float as i64))
			}
		}
	}
}
impl_f!(f32);
impl_f!(f64);

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
		assert_eq!(Int::try_from(-1_000.0).unwrap(),                 "-1,000");
		assert_eq!(Int::try_from(-65_535.0).unwrap(),                "-65,535");
		assert_eq!(Int::try_from(-65_536.0).unwrap(),                "-65,536");
		assert_eq!(Int::try_from(-100_000.0).unwrap(),               "-100,000");
		assert_eq!(Int::try_from(-1_000_000.0).unwrap(),             "-1,000,000");
		assert_eq!(Int::try_from(-10_000_000.0).unwrap(),            "-10,000,000");
		assert_eq!(Int::try_from(-100_000_000.0).unwrap(),           "-100,000,000");
		assert_eq!(Int::try_from(-1_000_000_000.0).unwrap(),         "-1,000,000,000");
		assert_eq!(Int::try_from(-4_294_967_295.0).unwrap(),         "-4,294,967,295");
		assert_eq!(Int::try_from(-4_294_967_296.0).unwrap(),         "-4,294,967,296");
		assert_eq!(Int::try_from(-10_000_000_000.0).unwrap(),        "-10,000,000,000");
		assert_eq!(Int::try_from(-100_000_000_000.0).unwrap(),       "-100,000,000,000");
		assert_eq!(Int::try_from(-1_000_000_000_000.0).unwrap(),     "-1,000,000,000,000");
		assert_eq!(Int::try_from(-10_000_000_000_000.0).unwrap(),    "-10,000,000,000,000");
		assert_eq!(Int::try_from(-100_000_000_000_000.0).unwrap(),   "-100,000,000,000,000");
		assert_eq!(Int::try_from(-1_000_000_000_000_000.0).unwrap(), "-1,000,000,000,000,000");
		assert_eq!(Int::try_from(i64::MIN as f64).unwrap(),          "-9,223,372,036,854,775,808");
		assert_eq!(Int::try_from(i64::MAX as f64).unwrap(),          "9,223,372,036,854,775,807");
	}

	#[test]
	fn special() {
		assert_eq!(Int::try_from(f64::NAN),          Err(Int::unknown()));
		assert_eq!(Int::try_from(f64::INFINITY),     Err(Int::unknown()));
		assert_eq!(Int::try_from(f64::NEG_INFINITY), Err(Int::unknown()));
	}
}