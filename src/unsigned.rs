//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

use std::num::*;
use crate::inner::*;
use crate::macros::*;
use crate::constants::*;

//---------------------------------------------------------------------------------------------------- Unsigned
/// Human readable unsigned integer.
///
/// ## Creation
/// For [`u8`], [`u16`], [`u32`], [`u64`] or any [`NonZeroU8`] variant:
/// - Use [`Unsigned::from`]
///
/// [`f32`] or [`f64`] inputs will work, but:
/// - Signed floats will turn into `0`
/// - Fractional parts will be ignored
/// - Special floats like [`f64::NAN`] will return [`Unsigned::unknown`]
///
/// For [`i8`] and other signed integers:
/// - You need to use [`Unsigned::try_from`]
/// - [`Unsigned::unknown`] will be returned on error
///
/// ## Cloning
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 26 byte array buffer, literally: `[u8; 26]`.
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
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
/// - Inputting [`f64::NAN`] returns [`Unsigned::unknown`]
/// - Inputting [`f64::INFINITY`] returns [`Unsigned::unknown`]
/// - Inputting [`f64::NEG_INFINITY`] returns [`Unsigned::unknown`]
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
/// - Combined with another [`Self`]: `Unsigned::from(1) + Unsigned::from(1)`
/// - Or with the inner number itself: `Unsigned::from(1) + 1`
///
/// They also have the same `panic!()` behavior on overflow as the normal ones, because internally,
/// it is just calling `.inner() $OPERATOR $NUMBER`.
///
/// ```rust
/// # use readable::*;
/// assert!(Unsigned::from(10_u64) + 10 == Unsigned::from(20_u64));
/// assert!(Unsigned::from(10_u64) - 10 == Unsigned::from(0_u64));
/// assert!(Unsigned::from(10_u64) / 10 == Unsigned::from(1_u64));
/// assert!(Unsigned::from(10_u64) * 10 == Unsigned::from(100_u64));
/// assert!(Unsigned::from(10_u64) % 10 == Unsigned::from(0_u64));
/// ```
/// Overflow example:
/// ```rust,should_panic
/// # use readable::*;
/// let n = Unsigned::from(u64::MAX) + u64::MAX;
/// ```
///
/// ## Examples
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
/// assert!(Unsigned::from(f32::NAN)    == "???");
///
/// // From signed integers.
/// assert!(Unsigned::try_from(100_i8)         == Ok(Unsigned::from(100_u8)));
/// assert!(Unsigned::try_from(-100_i8)        == Err(Unsigned::unknown()));
/// assert!(Unsigned::try_from(1_000_000_i64)  == Ok(Unsigned::from(1_000_000_u32)));
/// assert!(Unsigned::try_from(-1_000_000_i64) == Err(Unsigned::unknown()));
/// ```

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Unsigned(u64, Buffer);

impl Unsigned {
	impl_common!(u64);
	impl_const!();
	impl_usize!();
	impl_buffer!(MAX_BUF_LEN, UNKNOWN_BUFFER, UNKNOWN.len());

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(Unsigned::zero() == 0);
	/// ```
	pub const fn zero() -> Self {
		Self(0, Buffer::zero())
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(Unsigned::unknown() == UNKNOWN);
	/// ```
	pub const fn unknown() -> Self {
		Self(0, Buffer::unknown())
	}
}

macro_rules! impl_u {
	($( $from:ty ),*) => {
		$(
			impl From<$from> for Unsigned {
				fn from(uint: $from) -> Self {
					let u = uint as u64;
					Self(u, Buffer::from_u(u))
				}
			}
		)*
	}
}
impl_u!(u8,u16,u32,u64,usize);

macro_rules! impl_nonu {
	($( $from:ty ),*) => {
		$(
			impl From<$from> for Unsigned {
				fn from(uint: $from) -> Self {
					let u = uint.get() as u64;
					Self(u, Buffer::from_u(u))
				}
			}
		)*
	}
}
impl_nonu! {
	NonZeroU8,NonZeroU16,NonZeroU32,NonZeroU64,NonZeroUsize,
	&NonZeroU8,&NonZeroU16,&NonZeroU32,&NonZeroU64,&NonZeroUsize
}

macro_rules! impl_f {
	($from:ty) => {
		/// This will silently return [`Self::unknown`]
		/// if the input float is `NAN`, `INFINITY`, etc.
		///
		/// [`Self::zero`] will be returned on negative floats.
		impl From<$from> for Unsigned {
			fn from(float: $from) -> Self {
				handle_nan_runtime!(float);
				let u = float as u64;
				Self(u, Buffer::from_u(u))
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
			impl TryFrom<$from> for Unsigned {
				type Error = Self;
				fn try_from(num: $from) -> Result<Self, Self> {
					match u64::try_from(num) {
						Ok(u) => Ok(Self(u, Buffer::from_u(u))),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_try!(i8,i16,i32,i64,isize);

macro_rules! impl_noni {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::unknown`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Unsigned {
				type Error = Self;
				fn try_from(num: $from) -> Result<Self, Self> {
					match u64::try_from(num.get()) {
						Ok(u) => Ok(Self(u, Buffer::from_u(u))),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_noni! {
	NonZeroI8,NonZeroI16,NonZeroI32,NonZeroI64,NonZeroIsize,
	&NonZeroI8,&NonZeroI16,&NonZeroI32,&NonZeroI64,&NonZeroIsize
}

impl_math!(Unsigned, u64);
impl_traits!(Unsigned, u64);

//---------------------------------------------------------------------------------------------------- Buffer
// u64::MAX == "18_446_744_073_709_551_615".len() == 26
const MAX_BUF_LEN: usize = 26;
const COMMA: u8 = b',';

buffer!(MAX_BUF_LEN, UNKNOWN_BUFFER, UNKNOWN.len());

impl Buffer {
	#[inline(always)]
	const fn zero() -> Self {
		Self {
			buf: ZERO_NUM_BUFFER,
			len: 1,
		}
	}

	#[inline(always)]
	fn from_u(u: u64) -> Self {
		let mut buffer = itoa::Buffer::new();
		let string = &buffer.format(u).as_bytes();
		let mut buf = [0_u8; MAX_BUF_LEN];

		let len = match u {
			0..=9                         => { Self::from_1(&mut buf, &string); 1 },
			0..=99                        => { Self::from_2(&mut buf, &string); 2 },
			0..=999                       => { Self::from_3(&mut buf, &string); 3 },
			0..=9_999                     => { Self::from_4(&mut buf, &string); 5 },
			0..=99_999                    => { Self::from_5(&mut buf, &string); 6 },
			0..=999_999                   => { Self::from_6(&mut buf, &string); 7 },
			0..=9_999_999                 => { Self::from_7(&mut buf, &string); 9 },
			0..=99_999_999                => { Self::from_8(&mut buf, &string); 10 },
			0..=999_999_999               => { Self::from_9(&mut buf, &string); 11 },
			0..=9_999_999_999             => { Self::from_10(&mut buf, &string); 13 },
			0..=99_999_999_999            => { Self::from_11(&mut buf, &string); 14 },
			0..=999_999_999_999           => { Self::from_12(&mut buf, &string); 15 },
			0..=9_999_999_999_999         => { Self::from_13(&mut buf, &string); 17 },
			0..=99_999_999_999_999        => { Self::from_14(&mut buf, &string); 18 },
			0..=999_999_999_999_999       => { Self::from_15(&mut buf, &string); 19 },
			0..=9_999_999_999_999_999     => { Self::from_16(&mut buf, &string); 21 },
			0..=99_999_999_999_999_999    => { Self::from_17(&mut buf, &string); 22 },
			0..=999_999_999_999_999_999   => { Self::from_18(&mut buf, &string); 23 },
			0..=9_999_999_999_999_999_999 => { Self::from_19(&mut buf, &string); 25 },
			_ => { Self::from_20(&mut buf, &string); 26 },
		};

		Self { buf, len }
	}

	#[inline(always)]
	// 9
	fn from_1(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0] = string[0];
	}

	#[inline(always)]
	// 99
	fn from_2(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[..2].copy_from_slice(&string[..2]);
	}

	#[inline(always)]
	// 999
	fn from_3(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[..3].copy_from_slice(&string[..3]);
	}

	#[inline(always)]
	// 9,999
	fn from_4(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0] = string[0];
		buf[1] = COMMA;
		buf[2..5].copy_from_slice(&string[1..4]);
	}

	#[inline(always)]
	// 99,999
	fn from_5(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..2].copy_from_slice(&string[0..2]);
		buf[2] = COMMA;
		buf[3..6].copy_from_slice(&string[2..5]);
	}

	#[inline(always)]
	// 999,999
	fn from_6(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..3].copy_from_slice(&string[0..3]);
		buf[3] = COMMA;
		buf[4..7].copy_from_slice(&string[3..6]);
	}

	#[inline(always)]
	// 9,999,999
	fn from_7(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0] = string[0];
		buf[1] = COMMA;
		buf[2..5].copy_from_slice(&string[1..4]);
		buf[5] = COMMA;
		buf[6..9].copy_from_slice(&string[4..7]);
	}

	#[inline(always)]
	// 99,999,999
	fn from_8(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..2].copy_from_slice(&string[0..2]);
		buf[2] = COMMA;
		buf[3..6].copy_from_slice(&string[2..5]);
		buf[6] = COMMA;
		buf[7..10].copy_from_slice(&string[5..8]);
	}

	#[inline(always)]
	// 999,999,999
	fn from_9(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..3].copy_from_slice(&string[0..3]);
		buf[3] = COMMA;
		buf[4..7].copy_from_slice(&string[3..6]);
		buf[7] = COMMA;
		buf[8..11].copy_from_slice(&string[6..9]);
	}

	#[inline(always)]
	// 9,999,999,999
	fn from_10(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0] = string[0];
		buf[1] = COMMA;
		buf[2..5].copy_from_slice(&string[1..4]);
		buf[5] = COMMA;
		buf[6..9].copy_from_slice(&string[4..7]);
		buf[9] = COMMA;
		buf[10..13].copy_from_slice(&string[7..10]);
	}

	#[inline(always)]
	// 99,999,999,999
	fn from_11(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..2].copy_from_slice(&string[0..2]);
		buf[2] = COMMA;
		buf[3..6].copy_from_slice(&string[2..5]);
		buf[6] = COMMA;
		buf[7..10].copy_from_slice(&string[5..8]);
		buf[10] = COMMA;
		buf[11..14].copy_from_slice(&string[8..11]);
	}

	#[inline(always)]
	// 999,999,999,999
	fn from_12(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..3].copy_from_slice(&string[0..3]);
		buf[3] = COMMA;
		buf[4..7].copy_from_slice(&string[3..6]);
		buf[7] = COMMA;
		buf[8..11].copy_from_slice(&string[6..9]);
		buf[11] = COMMA;
		buf[12..15].copy_from_slice(&string[9..12]);
	}

	#[inline(always)]
	// 9,999,999,999,999
	fn from_13(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0] = string[0];
		buf[1] = COMMA;
		buf[2..5].copy_from_slice(&string[1..4]);
		buf[5] = COMMA;
		buf[6..9].copy_from_slice(&string[3..6]);
		buf[9] = COMMA;
		buf[10..13].copy_from_slice(&string[6..9]);
		buf[13] = COMMA;
		buf[14..17].copy_from_slice(&string[9..12]);
	}

	#[inline(always)]
	// 99,999,999,999,999
	fn from_14(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..2].copy_from_slice(&string[0..2]);
		buf[2] = COMMA;
		buf[3..6].copy_from_slice(&string[2..5]);
		buf[6] = COMMA;
		buf[7..10].copy_from_slice(&string[5..8]);
		buf[10] = COMMA;
		buf[11..14].copy_from_slice(&string[8..11]);
		buf[14] = COMMA;
		buf[15..18].copy_from_slice(&string[11..14]);
	}

	#[inline(always)]
	// 999,999,999,999,999
	fn from_15(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..3].copy_from_slice(&string[0..3]);
		buf[3] = COMMA;
		buf[4..7].copy_from_slice(&string[3..6]);
		buf[7] = COMMA;
		buf[8..11].copy_from_slice(&string[6..9]);
		buf[11] = COMMA;
		buf[12..15].copy_from_slice(&string[9..12]);
		buf[15] = COMMA;
		buf[16..19].copy_from_slice(&string[12..15]);
	}

	#[inline(always)]
	// 9,999,999,999,999,999
	fn from_16(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0] = string[0];
		buf[1] = COMMA;
		buf[2..5].copy_from_slice(&string[1..4]);
		buf[5] = COMMA;
		buf[6..9].copy_from_slice(&string[3..6]);
		buf[9] = COMMA;
		buf[10..13].copy_from_slice(&string[6..9]);
		buf[13] = COMMA;
		buf[14..17].copy_from_slice(&string[9..12]);
		buf[17] = COMMA;
		buf[18..21].copy_from_slice(&string[12..15]);
	}

	#[inline(always)]
	// 99,999,999,999,999,999
	fn from_17(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..2].copy_from_slice(&string[0..2]);
		buf[2] = COMMA;
		buf[3..6].copy_from_slice(&string[2..5]);
		buf[6] = COMMA;
		buf[7..10].copy_from_slice(&string[5..8]);
		buf[10] = COMMA;
		buf[11..14].copy_from_slice(&string[8..11]);
		buf[14] = COMMA;
		buf[15..18].copy_from_slice(&string[11..14]);
		buf[18] = COMMA;
		buf[19..22].copy_from_slice(&string[14..17]);
	}

	#[inline(always)]
	// 999,999,999,999,999,999
	fn from_18(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..3].copy_from_slice(&string[0..3]);
		buf[3] = COMMA;
		buf[4..7].copy_from_slice(&string[3..6]);
		buf[7] = COMMA;
		buf[8..11].copy_from_slice(&string[6..9]);
		buf[11] = COMMA;
		buf[12..15].copy_from_slice(&string[9..12]);
		buf[15] = COMMA;
		buf[16..19].copy_from_slice(&string[12..15]);
		buf[19] = COMMA;
		buf[20..23].copy_from_slice(&string[15..18]);
	}

	#[inline(always)]
	// 9,999,999,999,999,999,999
	fn from_19(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0] = string[0];
		buf[1] = COMMA;
		buf[2..5].copy_from_slice(&string[1..4]);
		buf[5] = COMMA;
		buf[6..9].copy_from_slice(&string[3..6]);
		buf[9] = COMMA;
		buf[10..13].copy_from_slice(&string[6..9]);
		buf[13] = COMMA;
		buf[14..17].copy_from_slice(&string[9..12]);
		buf[17] = COMMA;
		buf[18..21].copy_from_slice(&string[12..15]);
		buf[21] = COMMA;
		buf[22..25].copy_from_slice(&string[15..18]);
	}

	#[inline(always)]
	// 99,999,999,999,999,999
	fn from_20(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
		buf[0..2].copy_from_slice(&string[0..2]);
		buf[2] = COMMA;
		buf[3..6].copy_from_slice(&string[2..5]);
		buf[6] = COMMA;
		buf[7..10].copy_from_slice(&string[5..8]);
		buf[10] = COMMA;
		buf[11..14].copy_from_slice(&string[8..11]);
		buf[14] = COMMA;
		buf[15..18].copy_from_slice(&string[11..14]);
		buf[18] = COMMA;
		buf[19..22].copy_from_slice(&string[14..17]);
		buf[22] = COMMA;
		buf[23..26].copy_from_slice(&string[17..20]);
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

//	#[test]
//	fn aaaa() {
//		for i in 0..u32::MAX {
//			Buffer::from_u(i);
//		}
//	}

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
		assert!(Unsigned::from(f64::NAN)          == crate::UNKNOWN);
		assert!(Unsigned::from(f64::INFINITY)     == crate::UNKNOWN);
		assert!(Unsigned::from(f64::NEG_INFINITY) == crate::UNKNOWN);
	}
}
