//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::num::{
	Int,
	constants::{
		MAX_LEN_NUM,ZERO_NUM,
		UNKNOWN_NUM,COMMA,MAX_UNSIGNED,
	},
};
use crate::macros::{
	impl_common,impl_const,
	impl_usize,impl_math,
	impl_traits,impl_impl_math,
};
use std::num::{
	NonZeroU8,NonZeroU16,NonZeroU32,
	NonZeroU64,NonZeroUsize,
	NonZeroI8,NonZeroI16,NonZeroI32,
	NonZeroI64,NonZeroIsize,
};

//---------------------------------------------------------------------------------------------------- Unsigned
/// Human readable unsigned integer.
///
/// ## Construction
/// For [`u8`], [`u16`], [`u32`], [`u64`], [`usize`] or any `NonZeroU*` variant:
/// - Use [`Unsigned::from`]
///
/// [`f32`] or [`f64`] inputs must use [`Unsigned::try_from`] and:
/// - (Negative) infinity floats ([`f32::INFINITY`], [`f32::NEG_INFINITY`])
/// - NaN floats ([`f32::NAN`])
/// - Signed floats (`-1.0`)
/// will all lead to `Err(Unsigned::unknown)` being returned.
/// ```rust
/// # use readable::*;
/// // Signed floats will fail.
/// assert_eq!(Unsigned::try_from(-1.0), Err(Unsigned::unknown()));
///
/// // Special floats like `f64::NAN` will fail.
/// assert_eq!(Unsigned::try_from(f64::NAN), Err(Unsigned::unknown()));
///
/// // Fractional parts will be ignored (not rounded)
/// assert_eq!(Unsigned::try_from(1.99).unwrap(), "1");
///
/// // A normal float.
/// assert_eq!(Unsigned::try_from(1.0).unwrap(), "1");
/// // Fractions are ignored.
/// assert_eq!(Unsigned::try_from(1.1).unwrap(), "1");
///
/// // These all fail.
/// assert!(Unsigned::try_from(-1.0).is_err());
/// assert!(Unsigned::try_from(f32::NAN).is_err());
/// assert!(Unsigned::try_from(f32::INFINITY).is_err());
/// assert!(Unsigned::try_from(f32::NEG_INFINITY).is_err());
/// ```
///
/// For [`i8`] and other signed integers, [`Unsigned::try_from`] must be used.
/// ```rust
/// # use readable::*;
/// // You can use `Unsigned::try_from` for a fallible `Result`
/// assert_eq!(Unsigned::try_from(i8::MAX).unwrap(), "127");
/// ```
/// [`Unsigned::unknown`] will be returned if `try_from` errors.
///
/// ## Size
/// [`Str<26>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<Unsigned>(), 40);
/// ```
///
/// ## Copy
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 26 byte array string, literally: [`Str<26>`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also either a [`String`].
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
/// ## Float Errors
/// - Inputting [`f64::NAN`] returns [`Unsigned::unknown`]
/// - Inputting [`f64::INFINITY`] returns [`Unsigned::unknown`]
/// - Inputting [`f64::NEG_INFINITY`] returns [`Unsigned::unknown`]
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
/// ```rust
/// # use readable::*;
/// assert!(Unsigned::from(10_u64) + 10 == Unsigned::from(20_u64));
/// assert!(Unsigned::from(10_u64) - 10 == Unsigned::from(0_u64));
/// assert!(Unsigned::from(10_u64) / 10 == Unsigned::from(1_u64));
/// assert!(Unsigned::from(10_u64) * 10 == Unsigned::from(100_u64));
/// assert!(Unsigned::from(10_u64) % 10 == Unsigned::from(0_u64));
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::Unsigned;
/// // From unsigned integers.
/// assert_eq!(Unsigned::from(100_u8),        "100");
/// assert_eq!(Unsigned::from(10_000_u16),    "10,000");
/// assert_eq!(Unsigned::from(100_000_u32),   "100,000");
/// assert_eq!(Unsigned::from(1_000_000_u64), "1,000,000");
///
/// // From floats.
/// assert_eq!(Unsigned::try_from(-1.0),                 Err(Unsigned::unknown()));
/// assert_eq!(Unsigned::try_from(1_000.123).unwrap(),   "1,000");
/// assert_eq!(Unsigned::try_from(100_000.123).unwrap(), "100,000");
/// assert_eq!(Unsigned::try_from(100_000.123).unwrap(), "100,000");
/// assert!(Unsigned::try_from(f32::NAN).is_err());
///
/// // From signed integers.
/// assert_eq!(Unsigned::try_from(100_i8),         Ok(Unsigned::from(100_u8)));
/// assert_eq!(Unsigned::try_from(-100_i8),        Err(Unsigned::unknown()));
/// assert_eq!(Unsigned::try_from(1_000_000_i64),  Ok(Unsigned::from(1_000_000_u32)));
/// assert_eq!(Unsigned::try_from(-1_000_000_i64), Err(Unsigned::unknown()));
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Unsigned(u64, Str<MAX_LEN_NUM>);

impl_math!(Unsigned, u64);
impl_traits!(Unsigned, u64);

//---------------------------------------------------------------------------------------------------- Unsigned Impl
impl Unsigned {
	// Impl Macros.
	impl_common!(u64);
	impl_const!();
	impl_usize!();

	#[inline]
	/// Create a new [`Unsigned`].
	///
	/// This internally just calls [`Unsigned::from`].
	pub fn new(u: u64) -> Self {
		Self::from(u)
	}

	#[inline]
	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Unsigned::zero(), 0);
	/// ```
	pub const fn zero() -> Self {
		Self(0, Str::from_static_str(ZERO_NUM))
	}

	#[inline]
	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Unsigned::max(), u64::MAX);
	/// ```
	pub const fn max() -> Self {
		Self(u64::MAX, Str::from_static_str(MAX_UNSIGNED))
	}

	#[inline]
	/// ```rust
	/// # use readable::num::*;
	/// assert_eq!(Unsigned::unknown(), UNKNOWN_NUM);
	/// ```
	pub const fn unknown() -> Self {
		Self(0, Str::from_static_str(UNKNOWN_NUM))
	}
}

//---------------------------------------------------------------------------------------------------- Private functions.
impl Unsigned {
	#[inline(always)]
	fn from_priv(u: u64) -> Self {
		Self(u, Self::from_priv_inner(u))
	}

	// Main frontend function for construction.
	//
	// Branches out depending on the length of the number.
	#[inline]
	#[allow(clippy::match_overlapping_arm)]
	pub(super) fn from_priv_inner(u: u64) -> Str<MAX_LEN_NUM> {
		// Format the `u64` into a `str`.
		let mut itoa = crate::Itoa64::new();
		let itoa = itoa.format(u);

		// Create our destination string byte array.
		let mut s = [0; MAX_LEN_NUM];

		// Match, write properly comma string
		// bytes and return the total length.
		let len = match itoa.len() {
			1  => { Self::from_1(&mut s, itoa); 1 },
			2  => { Self::from_2(&mut s, itoa); 2 },
			3  => { Self::from_3(&mut s, itoa); 3 },
			4  => { Self::from_4(&mut s, itoa); 5 },
			5  => { Self::from_5(&mut s, itoa); 6 },
			6  => { Self::from_6(&mut s, itoa); 7 },
			7  => { Self::from_7(&mut s, itoa); 9 },
			8  => { Self::from_8(&mut s, itoa); 10 },
			9  => { Self::from_9(&mut s, itoa); 11 },
			10 => { Self::from_10(&mut s, itoa); 13 },
			11 => { Self::from_11(&mut s, itoa); 14 },
			12 => { Self::from_12(&mut s, itoa); 15 },
			13 => { Self::from_13(&mut s, itoa); 17 },
			14 => { Self::from_14(&mut s, itoa); 18 },
			15 => { Self::from_15(&mut s, itoa); 19 },
			16 => { Self::from_16(&mut s, itoa); 21 },
			17 => { Self::from_17(&mut s, itoa); 22 },
			18 => { Self::from_18(&mut s, itoa); 23 },
			19 => { Self::from_19(&mut s, itoa); 25 },
			_  => { Self::from_20(&mut s, itoa); 26 },
		};

		// SAFETY: we're manually creating a `Str`.
		// This is okay because we filled the bytes
		// and know the length.
		unsafe { Str::from_raw(s, len) }
	}

	#[inline]
	// 9
	pub(super) fn from_1(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0] = itoa[0];
	}

	#[inline]
	// 99
	pub(super) fn from_2(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
	}

	#[inline]
	// 999
	pub(super) fn from_3(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
	}

	#[inline]
	// 9,999
	pub(super) fn from_4(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0] = itoa[0];
		s[1] = COMMA;
		s[2..5].copy_from_slice(&itoa[1..4]);
	}

	#[inline]
	// 99,999
	pub(super) fn from_5(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
		s[2] = COMMA;
		s[3..6].copy_from_slice(&itoa[2..5]);
	}

	#[inline]
	// 999,999
	pub(super) fn from_6(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
		s[3] = COMMA;
		s[4..7].copy_from_slice(&itoa[3..6]);
	}

	#[inline]
	// 9,999,999
	pub(super) fn from_7(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0] = itoa[0];
		s[1] = COMMA;
		s[2..5].copy_from_slice(&itoa[1..4]);
		s[5] = COMMA;
		s[6..9].copy_from_slice(&itoa[4..7]);
	}

	#[inline]
	// 99,999,999
	pub(super) fn from_8(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
		s[2] = COMMA;
		s[3..6].copy_from_slice(&itoa[2..5]);
		s[6] = COMMA;
		s[7..10].copy_from_slice(&itoa[5..8]);
	}

	#[inline]
	// 999,999,999
	pub(super) fn from_9(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
		s[3] = COMMA;
		s[4..7].copy_from_slice(&itoa[3..6]);
		s[7] = COMMA;
		s[8..11].copy_from_slice(&itoa[6..9]);
	}

	#[inline]
	// 9,999,999,999
	pub(super) fn from_10(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0] = itoa[0];
		s[1] = COMMA;
		s[2..5].copy_from_slice(&itoa[1..4]);
		s[5] = COMMA;
		s[6..9].copy_from_slice(&itoa[4..7]);
		s[9] = COMMA;
		s[10..13].copy_from_slice(&itoa[7..10]);
	}

	#[inline]
	// 99,999,999,999
	pub(super) fn from_11(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..2].copy_from_slice(&itoa[0..2]);
		s[2] = COMMA;
		s[3..6].copy_from_slice(&itoa[2..5]);
		s[6] = COMMA;
		s[7..10].copy_from_slice(&itoa[5..8]);
		s[10] = COMMA;
		s[11..14].copy_from_slice(&itoa[8..11]);
	}

	#[inline]
	// 999,999,999,999
	pub(super) fn from_12(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0..3].copy_from_slice(&itoa[0..3]);
		s[3] = COMMA;
		s[4..7].copy_from_slice(&itoa[3..6]);
		s[7] = COMMA;
		s[8..11].copy_from_slice(&itoa[6..9]);
		s[11] = COMMA;
		s[12..15].copy_from_slice(&itoa[9..12]);
	}

	#[inline]
	// 9,999,999,999,999
	pub(super) fn from_13(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0] = itoa[0];
		s[1] = COMMA;
		s[2..5].copy_from_slice(&itoa[1..4]);
		s[5] = COMMA;
		s[6..9].copy_from_slice(&itoa[4..7]);
		s[9] = COMMA;
		s[10..13].copy_from_slice(&itoa[7..10]);
		s[13] = COMMA;
		s[14..17].copy_from_slice(&itoa[10..13]);
	}

	#[inline]
	// 99,999,999,999,999
	pub(super) fn from_14(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
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
	// 999,999,999,999,999
	pub(super) fn from_15(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
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
	// 9,999,999,999,999,999
	pub(super) fn from_16(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0] = itoa[0];
		s[1] = COMMA;
		s[2..5].copy_from_slice(&itoa[1..4]);
		s[5] = COMMA;
		s[6..9].copy_from_slice(&itoa[4..7]);
		s[9] = COMMA;
		s[10..13].copy_from_slice(&itoa[7..10]);
		s[13] = COMMA;
		s[14..17].copy_from_slice(&itoa[10..13]);
		s[17] = COMMA;
		s[18..21].copy_from_slice(&itoa[13..16]);
	}

	#[inline]
	// 99,999,999,999,999,999
	pub(super) fn from_17(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
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
	// 999,999,999,999,999,999
	pub(super) fn from_18(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
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
	// 9,999,999,999,999,999,999
	pub(super) fn from_19(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
		s[0] = itoa[0];
		s[1] = COMMA;
		s[2..5].copy_from_slice(&itoa[1..4]);
		s[5] = COMMA;
		s[6..9].copy_from_slice(&itoa[4..7]);
		s[9] = COMMA;
		s[10..13].copy_from_slice(&itoa[7..10]);
		s[13] = COMMA;
		s[14..17].copy_from_slice(&itoa[10..13]);
		s[17] = COMMA;
		s[18..21].copy_from_slice(&itoa[13..16]);
		s[21] = COMMA;
		s[22..25].copy_from_slice(&itoa[16..19]);
	}

	#[inline]
	// 99,999,999,999,999,999,999
	pub(super) fn from_20(s: &mut [u8; MAX_LEN_NUM], itoa: &[u8]) {
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

//---------------------------------------------------------------------------------------------------- From `u*`
macro_rules! impl_u {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Unsigned {
				#[inline]
				fn from(uint: $from) -> Self {
					let u = uint as u64;
					Self::from_priv(u)
				}
			}
			impl From<&$from> for Unsigned {
				#[inline]
				fn from(uint: &$from) -> Self {
					let u = *uint as u64;
					Self::from_priv(u)
				}
			}
		)*
	}
}
impl_u!(u8,u16,u32,u64);
#[cfg(target_pointer_width = "64")]
impl_u!(usize);

//---------------------------------------------------------------------------------------------------- From `NonZeroU*`
macro_rules! impl_nonu {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Unsigned {
				fn from(uint: $from) -> Self {
					let u = uint.get() as u64;
					Self::from_priv(u)
				}
			}
		)*
	}
}
impl_nonu! {
	NonZeroU8,NonZeroU16,NonZeroU32,NonZeroU64,
	&NonZeroU8,&NonZeroU16,&NonZeroU32,&NonZeroU64,
}
#[cfg(target_pointer_width = "64")]
impl_nonu!(NonZeroUsize,&NonZeroUsize);

//---------------------------------------------------------------------------------------------------- From `i*`
macro_rules! impl_i {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::unknown`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Unsigned {
				type Error = Self;
				fn try_from(num: $from) -> Result<Self, Self> {
					match u64::try_from(num) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_i!(i8,i16,i32,i64,isize);

//---------------------------------------------------------------------------------------------------- From `readable::Int`
macro_rules! impl_int {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::unknown`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Unsigned {
				type Error = Self;
				fn try_from(int: $from) -> Result<Self, Self> {
					match u64::try_from(int.inner()) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_int!(Int,&Int);

//---------------------------------------------------------------------------------------------------- From `NonZeroI*`
macro_rules! impl_noni {
	($( $from:ty ),* $(,)?) => {
		$(
			/// This will return [`Self::unknown`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Unsigned {
				type Error = Self;
				fn try_from(num: $from) -> Result<Self, Self> {
					match u64::try_from(num.get()) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::unknown()),
					}
				}
			}
		)*
	}
}
impl_noni! {
	NonZeroI8,NonZeroI16,NonZeroI32,NonZeroI64,
	&NonZeroI8,&NonZeroI16,&NonZeroI32,&NonZeroI64,
}
#[cfg(target_pointer_width = "64")]
impl_noni!(NonZeroIsize,&NonZeroIsize);


//---------------------------------------------------------------------------------------------------- From `f32/f64`
macro_rules! impl_f {
	($from:ty) => {
		/// This will return [`Self::unknown`]
		/// if the input float is `NAN`, `INFINITY`, or negative.
		impl TryFrom<$from> for Unsigned {
			type Error = Self;
			fn try_from(float: $from) -> Result<Self, Self> {
				match float.classify() {
					std::num::FpCategory::Normal   => (),
					std::num::FpCategory::Nan      => return Err(Self::unknown()),
					std::num::FpCategory::Infinite => return Err(Self::unknown()),
					_ => (),
				}

				if float.is_sign_negative() {
					return Err(Self::unknown());
				}

				Ok(Self::from_priv(float as u64))
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
		assert_eq!(Unsigned::from(1_000_u64),                      "1,000");
		assert_eq!(Unsigned::from(65_535_u64),                     "65,535");
		assert_eq!(Unsigned::from(65_536_u64),                     "65,536");
		assert_eq!(Unsigned::from(100_000_u64),                    "100,000");
		assert_eq!(Unsigned::from(1_000_000_u64),                  "1,000,000");
		assert_eq!(Unsigned::from(10_000_000_u64),                 "10,000,000");
		assert_eq!(Unsigned::from(100_000_000_u64),                "100,000,000");
		assert_eq!(Unsigned::from(1_000_000_000_u64),              "1,000,000,000");
		assert_eq!(Unsigned::from(4_294_967_295_u64),              "4,294,967,295");
		assert_eq!(Unsigned::from(4_294_967_296_u64),              "4,294,967,296");
		assert_eq!(Unsigned::from(10_000_000_000_u64),             "10,000,000,000");
		assert_eq!(Unsigned::from(100_000_000_000_u64),            "100,000,000,000");
		assert_eq!(Unsigned::from(1_000_000_000_000_u64),          "1,000,000,000,000");
		assert_eq!(Unsigned::from(10_000_000_000_000_u64),         "10,000,000,000,000");
		assert_eq!(Unsigned::from(100_000_000_000_000_u64),        "100,000,000,000,000");
		assert_eq!(Unsigned::from(1_000_000_000_000_000_u64),      "1,000,000,000,000,000");
		assert_eq!(Unsigned::from(10_000_000_000_000_000_u64),     "10,000,000,000,000,000");
		assert_eq!(Unsigned::from(18_446_744_073_709_551_615_u64), "18,446,744,073,709,551,615");
	}

	#[test]
	fn float() {
		assert_eq!(Unsigned::try_from(1_000.0).unwrap(),                      "1,000");
		assert_eq!(Unsigned::try_from(65_535.0).unwrap(),                     "65,535");
		assert_eq!(Unsigned::try_from(65_536.0).unwrap(),                     "65,536");
		assert_eq!(Unsigned::try_from(100_000.0).unwrap(),                    "100,000");
		assert_eq!(Unsigned::try_from(1_000_000.0).unwrap(),                  "1,000,000");
		assert_eq!(Unsigned::try_from(10_000_000.0).unwrap(),                 "10,000,000");
		assert_eq!(Unsigned::try_from(100_000_000.0).unwrap(),                "100,000,000");
		assert_eq!(Unsigned::try_from(1_000_000_000.0).unwrap(),              "1,000,000,000");
		assert_eq!(Unsigned::try_from(4_294_967_295.0).unwrap(),              "4,294,967,295");
		assert_eq!(Unsigned::try_from(4_294_967_296.0).unwrap(),              "4,294,967,296");
		assert_eq!(Unsigned::try_from(10_000_000_000.0).unwrap(),             "10,000,000,000");
		assert_eq!(Unsigned::try_from(100_000_000_000.0).unwrap(),            "100,000,000,000");
		assert_eq!(Unsigned::try_from(1_000_000_000_000.0).unwrap(),          "1,000,000,000,000");
		assert_eq!(Unsigned::try_from(10_000_000_000_000.0).unwrap(),         "10,000,000,000,000");
		assert_eq!(Unsigned::try_from(100_000_000_000_000.0).unwrap(),        "100,000,000,000,000");
		assert_eq!(Unsigned::try_from(1_000_000_000_000_000.0).unwrap(),      "1,000,000,000,000,000");
		assert_eq!(Unsigned::try_from(10_000_000_000_000_000.0).unwrap(),     "10,000,000,000,000,000");
		assert_eq!(Unsigned::try_from(18_446_744_073_709_551_615.0).unwrap(), "18,446,744,073,709,551,615");
	}

	#[test]
	fn special() {
		assert_eq!(Unsigned::try_from(f64::NAN),          Err(Unsigned::unknown()));
		assert_eq!(Unsigned::try_from(f64::INFINITY),     Err(Unsigned::unknown()));
		assert_eq!(Unsigned::try_from(f64::NEG_INFINITY), Err(Unsigned::unknown()));
	}
}