//---------------------------------------------------------------------------------------------------- Use
use crate::macros::{impl_common, impl_const, impl_impl_math, impl_isize, impl_math, impl_traits};
use crate::num::{constants::COMMA, Unsigned};
use crate::str::Str;
use std::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};

//---------------------------------------------------------------------------------------------------- Int
/// Human readable signed integer.
///
/// ## Creation
/// For [`i8`], [`i16`], [`i32`], [`i64`], [`isize`] or any [`NonZeroI8`] variant:
/// - Use [`Int::from`]
///
/// [`f32`] or [`f64`] inputs must use [`Unsigned::try_from`] and:
/// - Fractional parts will be ignored
/// - Under/overflows will return [`Int::UNKNOWN`]
/// - Special floats like [`f64::NAN`] will return [`Int::UNKNOWN`]
///
/// For [`u8`] and other unsigned integers:
/// - You can use [`Int::from`] for anything under [`u32`]
/// - You need to use [`Int::try_from`] for anything above [`u32`]
/// - [`Int::UNKNOWN`] will be returned on error
///
/// ## Size
/// [`Str<26>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::num::*;
/// assert_eq!(std::mem::size_of::<Int>(), 40);
/// ```
///
/// ## Copy
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 26 byte array string, literally: [`Str<26>`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
/// ```rust
/// # use readable::num::Int;
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
/// # use readable::num::*;
/// assert!(Int::from(10) + 10 == Int::from(20));
/// assert!(Int::from(10) - 10 == Int::from(0));
/// assert!(Int::from(10) / 10 == Int::from(1));
/// assert!(Int::from(10) * 10 == Int::from(100));
/// assert!(Int::from(10) % 10 == Int::from(0));
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::num::Int;
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Int(i64, Str<LEN>);

const LEN: usize = 26;

impl_math!(Int, i64);
impl_traits!(Int, i64);

//---------------------------------------------------------------------------------------------------- Int Constants
impl Int {
    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Int::ZERO, 0);
    /// assert_eq!(Int::ZERO, "0");
    /// ```
    pub const ZERO: Self = Self(0, Str::from_static_str("0"));

    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Int::from(i64::MIN), i64::MIN);
    /// assert_eq!(Int::from(i64::MIN), "-9,223,372,036,854,775,808");
    /// ```
    pub const MIN: Self = Self(i64::MIN, Str::from_static_str("-9,223,372,036,854,775,808"));

    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Int::from(i64::MAX), Int::MAX);
    /// assert_eq!(Int::from(i64::MAX), "9,223,372,036,854,775,807");
    /// ```
    pub const MAX: Self = Self(i64::MAX, Str::from_static_str("9,223,372,036,854,775,807"));

    /// Returned on error situations.
    ///
    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Int::try_from(f64::NAN), Err(Int::UNKNOWN));
    /// assert_eq!(Int::UNKNOWN, 0);
    /// assert_eq!(Int::UNKNOWN, "???");
    /// ```
    pub const UNKNOWN: Self = Self(0, Str::from_static_str("???"));

    /// The maximum string length of an [`Int`].
    ///
    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Int::MIN.len(), 26);
    /// ```
    pub const MAX_LEN: usize = LEN;
}

//---------------------------------------------------------------------------------------------------- Int Impl
impl Int {
    impl_common!(i64);
    impl_const!();
    impl_isize!();

    #[inline]
    #[must_use]
    /// ```rust
    /// # use readable::num::*;
    /// assert!(Int::UNKNOWN.is_unknown());
    /// assert!(!Int::ZERO.is_unknown());
    /// ```
    pub const fn is_unknown(&self) -> bool {
        matches!(*self, Self::UNKNOWN)
    }
}

//---------------------------------------------------------------------------------------------------- Private functions.
impl Int {
    #[inline]
    fn from_priv(i: i64) -> Self {
        Self(i, Self::from_priv_inner(i))
    }

    // Main frontend function for construction.
    //
    // Branches out depending on the length of the number.
    #[inline]
    #[allow(clippy::match_overlapping_arm)]
    pub(super) fn from_priv_inner(i: i64) -> Str<LEN> {
        // Format the `u64` into a `str`.
        let mut itoa = crate::Itoa64::new();
        let itoa = itoa.format(i);

        // Create our destination string byte array.
        let mut s = [0; Self::MAX_LEN];

        let itoa_len = itoa.len();

        // Match, write properly comma itoa
        // bytes and return the total length.
        if i.is_negative() {
            let len = match itoa_len {
                // Must be at least two bytes: `-1`
                2 => {
                    Self::from_neg_2(&mut s, itoa);
                    2
                }
                3 => {
                    Self::from_neg_3(&mut s, itoa);
                    3
                }
                4 => {
                    Self::from_neg_4(&mut s, itoa);
                    4
                }
                5 => {
                    Self::from_neg_5(&mut s, itoa);
                    6
                }
                6 => {
                    Self::from_neg_6(&mut s, itoa);
                    7
                }
                7 => {
                    Self::from_neg_7(&mut s, itoa);
                    8
                }
                8 => {
                    Self::from_neg_8(&mut s, itoa);
                    10
                }
                9 => {
                    Self::from_neg_9(&mut s, itoa);
                    11
                }
                10 => {
                    Self::from_neg_10(&mut s, itoa);
                    12
                }
                11 => {
                    Self::from_neg_11(&mut s, itoa);
                    14
                }
                12 => {
                    Self::from_neg_12(&mut s, itoa);
                    15
                }
                13 => {
                    Self::from_neg_13(&mut s, itoa);
                    16
                }
                14 => {
                    Self::from_neg_14(&mut s, itoa);
                    18
                }
                15 => {
                    Self::from_neg_15(&mut s, itoa);
                    19
                }
                16 => {
                    Self::from_neg_16(&mut s, itoa);
                    20
                }
                17 => {
                    Self::from_neg_17(&mut s, itoa);
                    22
                }
                18 => {
                    Self::from_neg_18(&mut s, itoa);
                    23
                }
                19 => {
                    Self::from_neg_19(&mut s, itoa);
                    24
                }
                // We've covered all possible negative `i64` lengths.
                _ => {
                    Self::from_neg_20(&mut s, itoa);
                    26
                }
            };

            // SAFETY: we're manually creating a `Str`.
            // This is okay because we filled the bytes
            // and know the length.
            unsafe { Str::from_raw(s, len) }
        } else {
            let len = match itoa_len {
                1 => {
                    Unsigned::from_1(&mut s, itoa);
                    1
                }
                2 => {
                    Unsigned::from_2(&mut s, itoa);
                    2
                }
                3 => {
                    Unsigned::from_3(&mut s, itoa);
                    3
                }
                4 => {
                    Unsigned::from_4(&mut s, itoa);
                    5
                }
                5 => {
                    Unsigned::from_5(&mut s, itoa);
                    6
                }
                6 => {
                    Unsigned::from_6(&mut s, itoa);
                    7
                }
                7 => {
                    Unsigned::from_7(&mut s, itoa);
                    9
                }
                8 => {
                    Unsigned::from_8(&mut s, itoa);
                    10
                }
                9 => {
                    Unsigned::from_9(&mut s, itoa);
                    11
                }
                10 => {
                    Unsigned::from_10(&mut s, itoa);
                    13
                }
                11 => {
                    Unsigned::from_11(&mut s, itoa);
                    14
                }
                12 => {
                    Unsigned::from_12(&mut s, itoa);
                    15
                }
                13 => {
                    Unsigned::from_13(&mut s, itoa);
                    17
                }
                14 => {
                    Unsigned::from_14(&mut s, itoa);
                    18
                }
                15 => {
                    Unsigned::from_15(&mut s, itoa);
                    19
                }
                16 => {
                    Unsigned::from_16(&mut s, itoa);
                    21
                }
                17 => {
                    Unsigned::from_17(&mut s, itoa);
                    22
                }
                18 => {
                    Unsigned::from_18(&mut s, itoa);
                    23
                }
                // We've covered all possible positive `i64` lengths.
                _ => {
                    Unsigned::from_19(&mut s, itoa);
                    25
                }
            };

            // SAFETY: we're manually creating a `Str`.
            // This is okay because we filled the bytes
            // and know the length.
            unsafe { Str::from_raw(s, len) }
        }
    }

    #[inline]
    // -9
    fn from_neg_2(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..2].copy_from_slice(&itoa[0..2]);
    }

    #[inline]
    // -99
    fn from_neg_3(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..3].copy_from_slice(&itoa[0..3]);
    }

    #[inline]
    // -999
    fn from_neg_4(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..4].copy_from_slice(&itoa[0..4]);
    }

    #[inline]
    // -9,999
    fn from_neg_5(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..2].copy_from_slice(&itoa[0..2]);
        s[2] = COMMA;
        s[3..6].copy_from_slice(&itoa[2..5]);
    }

    #[inline]
    // -99,999
    fn from_neg_6(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..3].copy_from_slice(&itoa[0..3]);
        s[3] = COMMA;
        s[4..7].copy_from_slice(&itoa[3..6]);
    }

    #[inline]
    // -999,999
    fn from_neg_7(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..4].copy_from_slice(&itoa[0..4]);
        s[4] = COMMA;
        s[5..8].copy_from_slice(&itoa[4..7]);
    }

    #[inline]
    // -9,999,999
    fn from_neg_8(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..2].copy_from_slice(&itoa[0..2]);
        s[2] = COMMA;
        s[3..6].copy_from_slice(&itoa[2..5]);
        s[6] = COMMA;
        s[7..10].copy_from_slice(&itoa[5..8]);
    }

    #[inline]
    // -99,999,999
    fn from_neg_9(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..3].copy_from_slice(&itoa[0..3]);
        s[3] = COMMA;
        s[4..7].copy_from_slice(&itoa[3..6]);
        s[7] = COMMA;
        s[8..11].copy_from_slice(&itoa[6..9]);
    }

    #[inline]
    // -999,999,999
    fn from_neg_10(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..4].copy_from_slice(&itoa[0..4]);
        s[4] = COMMA;
        s[5..8].copy_from_slice(&itoa[4..7]);
        s[8] = COMMA;
        s[9..12].copy_from_slice(&itoa[7..10]);
    }

    #[inline]
    // -9,999,999,999
    fn from_neg_11(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    fn from_neg_12(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    fn from_neg_13(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    fn from_neg_14(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    fn from_neg_15(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    fn from_neg_16(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    fn from_neg_17(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    fn from_neg_18(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    fn from_neg_19(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    fn from_neg_20(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
impl_i!(i8, i16, i32, i64, u8, u16, u32);
#[cfg(target_pointer_width = "64")]
impl_i!(isize);
#[cfg(not(target_pointer_width = "64"))]
impl_i!(usize);

//---------------------------------------------------------------------------------------------------- From `NonZeroI*`
macro_rules! impl_noni {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Int {
				#[inline]
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
impl_noni!(NonZeroIsize, &NonZeroIsize);
#[cfg(not(target_pointer_width = "64"))]
impl_noni!(NonZeroUsize, &NonZeroUsize);

//---------------------------------------------------------------------------------------------------- From `u64/usize`
macro_rules! impl_try {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::UNKNOWN`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Int {
				type Error = Self;
				#[inline]
				fn try_from(num: $from) -> Result<Self, Self> {
					match i64::try_from(num) {
						Ok(i) => Ok(Self::from_priv(i)),
						_ => Err(Self::UNKNOWN),
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
			/// This will return [`Self::UNKNOWN`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Int {
				type Error = Self;
				#[inline]
				fn try_from(num: $from) -> Result<Self, Self> {
					match i64::try_from(num.inner()) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::UNKNOWN),
					}
				}
			}
		)*
	}
}
impl_unsigned!(Unsigned, &Unsigned);

//---------------------------------------------------------------------------------------------------- From `NonZeroU*`
macro_rules! impl_nonu {
	($( $from:ty ),* $(,)?) => {
		$(
			/// This will return [`Self::UNKNOWN`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Int {
				type Error = Self;
				#[inline]
				fn try_from(num: $from) -> Result<Self, Self> {
					match i64::try_from(num.get()) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::UNKNOWN),
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
impl_noni!(NonZeroUsize, &NonZeroUsize);

//---------------------------------------------------------------------------------------------------- From `f32/f64`
macro_rules! impl_f {
    ($from:ty) => {
        /// This will return [`Self::UNKNOWN`]
        /// if the input float is `NAN`, `INFINITY`, or negative.
        impl TryFrom<$from> for Int {
            type Error = Self;
            #[inline]
            fn try_from(float: $from) -> Result<Self, Self> {
                match float.classify() {
                    std::num::FpCategory::Normal => (),
                    std::num::FpCategory::Nan => return Err(Self::UNKNOWN),
                    std::num::FpCategory::Infinite => return Err(Self::UNKNOWN),
                    _ => (),
                }

                Ok(Self::from_priv(float as i64))
            }
        }
    };
}
impl_f!(f32);
impl_f!(f64);

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned() {
        assert_eq!(Int::from(1_000_i64), "1,000");
        assert_eq!(Int::from(65_535_i64), "65,535");
        assert_eq!(Int::from(65_536_i64), "65,536");
        assert_eq!(Int::from(100_000_i64), "100,000");
        assert_eq!(Int::from(1_000_000_i64), "1,000,000");
        assert_eq!(Int::from(10_000_000_i64), "10,000,000");
        assert_eq!(Int::from(100_000_000_i64), "100,000,000");
        assert_eq!(Int::from(1_000_000_000_i64), "1,000,000,000");
        assert_eq!(Int::from(4_294_967_295_i64), "4,294,967,295");
        assert_eq!(Int::from(4_294_967_296_i64), "4,294,967,296");
        assert_eq!(Int::from(10_000_000_000_i64), "10,000,000,000");
        assert_eq!(Int::from(100_000_000_000_i64), "100,000,000,000");
        assert_eq!(Int::from(1_000_000_000_000_i64), "1,000,000,000,000");
        assert_eq!(Int::from(10_000_000_000_000_i64), "10,000,000,000,000");
        assert_eq!(Int::from(100_000_000_000_000_i64), "100,000,000,000,000");
        assert_eq!(
            Int::from(1_000_000_000_000_000_i64),
            "1,000,000,000,000,000"
        );
        assert_eq!(
            Int::from(10_000_000_000_000_000_i64),
            "10,000,000,000,000,000"
        );
    }

    #[test]
    fn int() {
        assert_eq!(Int::from(-1_000_i64), "-1,000");
        assert_eq!(Int::from(-65_535_i64), "-65,535");
        assert_eq!(Int::from(-65_536_i64), "-65,536");
        assert_eq!(Int::from(-100_000_i64), "-100,000");
        assert_eq!(Int::from(-1_000_000_i64), "-1,000,000");
        assert_eq!(Int::from(-10_000_000_i64), "-10,000,000");
        assert_eq!(Int::from(-100_000_000_i64), "-100,000,000");
        assert_eq!(Int::from(-1_000_000_000_i64), "-1,000,000,000");
        assert_eq!(Int::from(-4_294_967_295_i64), "-4,294,967,295");
        assert_eq!(Int::from(-4_294_967_296_i64), "-4,294,967,296");
        assert_eq!(Int::from(-10_000_000_000_i64), "-10,000,000,000");
        assert_eq!(Int::from(-100_000_000_000_i64), "-100,000,000,000");
        assert_eq!(Int::from(-1_000_000_000_000_i64), "-1,000,000,000,000");
        assert_eq!(Int::from(-10_000_000_000_000_i64), "-10,000,000,000,000");
        assert_eq!(Int::from(-100_000_000_000_000_i64), "-100,000,000,000,000");
        assert_eq!(
            Int::from(-1_000_000_000_000_000_i64),
            "-1,000,000,000,000,000"
        );
        assert_eq!(
            Int::from(-10_000_000_000_000_000_i64),
            "-10,000,000,000,000,000"
        );

        assert_eq!(Int::from(i64::MIN), "-9,223,372,036,854,775,808");
        assert_eq!(Int::from(i64::MAX), "9,223,372,036,854,775,807");
    }

    #[test]
    fn float() {
        assert_eq!(Int::try_from(-1_000.0).unwrap(), "-1,000");
        assert_eq!(Int::try_from(-65_535.0).unwrap(), "-65,535");
        assert_eq!(Int::try_from(-65_536.0).unwrap(), "-65,536");
        assert_eq!(Int::try_from(-100_000.0).unwrap(), "-100,000");
        assert_eq!(Int::try_from(-1_000_000.0).unwrap(), "-1,000,000");
        assert_eq!(Int::try_from(-10_000_000.0).unwrap(), "-10,000,000");
        assert_eq!(Int::try_from(-100_000_000.0).unwrap(), "-100,000,000");
        assert_eq!(Int::try_from(-1_000_000_000.0).unwrap(), "-1,000,000,000");
        assert_eq!(Int::try_from(-4_294_967_295.0).unwrap(), "-4,294,967,295");
        assert_eq!(Int::try_from(-4_294_967_296.0).unwrap(), "-4,294,967,296");
        assert_eq!(Int::try_from(-10_000_000_000.0).unwrap(), "-10,000,000,000");
        assert_eq!(
            Int::try_from(-100_000_000_000.0).unwrap(),
            "-100,000,000,000"
        );
        assert_eq!(
            Int::try_from(-1_000_000_000_000.0).unwrap(),
            "-1,000,000,000,000"
        );
        assert_eq!(
            Int::try_from(-10_000_000_000_000.0).unwrap(),
            "-10,000,000,000,000"
        );
        assert_eq!(
            Int::try_from(-100_000_000_000_000.0).unwrap(),
            "-100,000,000,000,000"
        );
        assert_eq!(
            Int::try_from(-1_000_000_000_000_000.0).unwrap(),
            "-1,000,000,000,000,000"
        );
        assert_eq!(
            Int::try_from(i64::MIN as f64).unwrap(),
            "-9,223,372,036,854,775,808"
        );
        assert_eq!(
            Int::try_from(i64::MAX as f64).unwrap(),
            "9,223,372,036,854,775,807"
        );
    }

    #[test]
    fn special() {
        assert_eq!(Int::try_from(f64::NAN), Err(Int::UNKNOWN));
        assert_eq!(Int::try_from(f64::INFINITY), Err(Int::UNKNOWN));
        assert_eq!(Int::try_from(f64::NEG_INFINITY), Err(Int::UNKNOWN));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        let this: Int = Int::from(-1000);
        let json = serde_json::to_string(&this).unwrap();
        assert_eq!(json, r#"[-1000,"-1,000"]"#);

        let this: Int = serde_json::from_str(&json).unwrap();
        assert_eq!(this, -1000);
        assert_eq!(this, "-1,000");

        // Bad bytes.
        assert!(serde_json::from_str::<Int>(&"---").is_err());

        // Unknown.
        let json = serde_json::to_string(&Int::UNKNOWN).unwrap();
        assert_eq!(json, r#"[0,"???"]"#);
        assert!(serde_json::from_str::<Int>(&json).unwrap().is_unknown());
    }

    #[test]
    #[cfg(feature = "bincode")]
    fn bincode() {
        let this: Int = Int::from(-1000);
        let config = bincode::config::standard();
        let bytes = bincode::encode_to_vec(&this, config).unwrap();

        let this: Int = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert_eq!(this, -1000);
        assert_eq!(this, "-1,000");

        // Unknown.
        let bytes = bincode::encode_to_vec(&Int::UNKNOWN, config).unwrap();
        let this: Int = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert!(this.is_unknown());
    }

    #[test]
    #[cfg(feature = "borsh")]
    fn borsh() {
        let this: Int = Int::from(-1000);
        let bytes = borsh::to_vec(&this).unwrap();

        let this: Int = borsh::from_slice(&bytes).unwrap();
        assert_eq!(this, -1000);
        assert_eq!(this, "-1,000");

        // Bad bytes.
        assert!(borsh::from_slice::<Int>(b"bad .-;[]124/ bytes").is_err());

        // Unknown.
        let bytes = borsh::to_vec(&Int::UNKNOWN).unwrap();
        let this: Int = borsh::from_slice(&bytes).unwrap();
        assert!(this.is_unknown());
    }
}
