//---------------------------------------------------------------------------------------------------- Use
use crate::macros::{impl_common, impl_const, impl_impl_math, impl_math, impl_traits, impl_usize};
use crate::num::{constants::COMMA, Int};
use crate::str::Str;
use std::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};

//---------------------------------------------------------------------------------------------------- Unsigned
/// Human readable unsigned integer.
///
/// ## Construction
/// For [`u8`], [`u16`], [`u32`], [`u64`], [`usize`] or any `NonZeroU*` variant:
/// - Use [`Unsigned::from`]
///
/// Floating point and signed integer ([`f32`], [`i32`]) inputs must use [`Unsigned::try_from`] and:
/// - (Negative) infinity floats ([`f32::INFINITY`], [`f32::NEG_INFINITY`])
/// - NaN floats ([`f32::NAN`])
/// - Signed floats (`-1.0`)
/// - Signed integers (`-1`)
///
/// will all lead to `Err(Unsigned::unknown)` being returned.
///
/// ```rust
/// # use readable::num::*;
/// // Signed floats will fail.
/// assert_eq!(Unsigned::try_from(-1.0), Err(Unsigned::UNKNOWN));
///
/// // Special floats like `f64::NAN` will fail.
/// assert_eq!(Unsigned::try_from(f64::NAN), Err(Unsigned::UNKNOWN));
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
/// ## Size
/// [`Str<26>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::num::*;
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
/// # use readable::num::Unsigned;
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
/// # use readable::num::*;
/// assert!(Unsigned::from(10_u64) + 10 == Unsigned::from(20_u64));
/// assert!(Unsigned::from(10_u64) - 10 == Unsigned::from(0_u64));
/// assert!(Unsigned::from(10_u64) / 10 == Unsigned::from(1_u64));
/// assert!(Unsigned::from(10_u64) * 10 == Unsigned::from(100_u64));
/// assert!(Unsigned::from(10_u64) % 10 == Unsigned::from(0_u64));
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::num::Unsigned;
/// // From unsigned integers.
/// assert_eq!(Unsigned::from(100_u8),        "100");
/// assert_eq!(Unsigned::from(10_000_u16),    "10,000");
/// assert_eq!(Unsigned::from(100_000_u32),   "100,000");
/// assert_eq!(Unsigned::from(1_000_000_u64), "1,000,000");
///
/// // From floats.
/// assert_eq!(Unsigned::try_from(-1.0),                 Err(Unsigned::UNKNOWN));
/// assert_eq!(Unsigned::try_from(1_000.123).unwrap(),   "1,000");
/// assert_eq!(Unsigned::try_from(100_000.123).unwrap(), "100,000");
/// assert_eq!(Unsigned::try_from(100_000.123).unwrap(), "100,000");
/// assert!(Unsigned::try_from(f32::NAN).is_err());
///
/// // From signed integers.
/// assert_eq!(Unsigned::try_from(100_i8),         Ok(Unsigned::from(100_u8)));
/// assert_eq!(Unsigned::try_from(-100_i8),        Err(Unsigned::UNKNOWN));
/// assert_eq!(Unsigned::try_from(1_000_000_i64),  Ok(Unsigned::from(1_000_000_u32)));
/// assert_eq!(Unsigned::try_from(-1_000_000_i64), Err(Unsigned::UNKNOWN));
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Unsigned(u64, Str<{ Unsigned::MAX_LEN }>);

const LEN: usize = 26;

impl_math!(Unsigned, u64);
impl_traits!(Unsigned, u64);

//---------------------------------------------------------------------------------------------------- Unsigned Constants
impl Unsigned {
    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Unsigned::ZERO, 0);
    /// assert_eq!(Unsigned::ZERO, "0");
    /// ```
    pub const ZERO: Self = Self(0, Str::from_static_str("0"));

    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Unsigned::MAX, u64::MAX);
    /// assert_eq!(Unsigned::MAX, "18,446,744,073,709,551,615");
    /// ```
    pub const MAX: Self = Self(u64::MAX, Str::from_static_str("18,446,744,073,709,551,615"));

    /// Returned when using [`Unsigned::UNKNOWN`] and error situations.
    ///
    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Unsigned::try_from(f64::NAN), Err(Unsigned::UNKNOWN));
    /// assert_eq!(Unsigned::UNKNOWN, 0);
    /// assert_eq!(Unsigned::UNKNOWN, "???");
    /// ```
    pub const UNKNOWN: Self = Self(0, Str::from_static_str("???"));

    /// The maximum string length of an [`Unsigned`].
    ///
    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Unsigned::MAX.len(), 26);
    /// ```
    pub const MAX_LEN: usize = LEN;
}

//---------------------------------------------------------------------------------------------------- Unsigned Impl
impl Unsigned {
    // Impl Macros.
    impl_common!(u64);
    impl_const!();
    impl_usize!();

    #[inline]
    #[must_use]
    /// ```rust
    /// # use readable::num::*;
    /// assert!(Unsigned::UNKNOWN.is_unknown());
    /// assert!(!Unsigned::ZERO.is_unknown());
    /// ```
    pub const fn is_unknown(&self) -> bool {
        matches!(*self, Self::UNKNOWN)
    }
}

//---------------------------------------------------------------------------------------------------- Private functions.
impl Unsigned {
    #[inline]
    fn from_priv(u: u64) -> Self {
        Self(u, Self::from_priv_inner(u))
    }

    // Main frontend function for construction.
    //
    // Branches out depending on the length of the number.
    #[inline]
    #[allow(clippy::match_overlapping_arm)]
    pub(super) fn from_priv_inner(u: u64) -> Str<LEN> {
        // Format the `u64` into a `str`.
        let mut itoa = crate::Itoa64::new();
        let itoa = itoa.format(u);

        // Create our destination string byte array.
        let mut s = [0; Self::MAX_LEN];

        // Match, write properly comma string
        // bytes and return the total length.
        let len = match itoa.len() {
            1 => {
                Self::from_1(&mut s, itoa);
                1
            }
            2 => {
                Self::from_2(&mut s, itoa);
                2
            }
            3 => {
                Self::from_3(&mut s, itoa);
                3
            }
            4 => {
                Self::from_4(&mut s, itoa);
                5
            }
            5 => {
                Self::from_5(&mut s, itoa);
                6
            }
            6 => {
                Self::from_6(&mut s, itoa);
                7
            }
            7 => {
                Self::from_7(&mut s, itoa);
                9
            }
            8 => {
                Self::from_8(&mut s, itoa);
                10
            }
            9 => {
                Self::from_9(&mut s, itoa);
                11
            }
            10 => {
                Self::from_10(&mut s, itoa);
                13
            }
            11 => {
                Self::from_11(&mut s, itoa);
                14
            }
            12 => {
                Self::from_12(&mut s, itoa);
                15
            }
            13 => {
                Self::from_13(&mut s, itoa);
                17
            }
            14 => {
                Self::from_14(&mut s, itoa);
                18
            }
            15 => {
                Self::from_15(&mut s, itoa);
                19
            }
            16 => {
                Self::from_16(&mut s, itoa);
                21
            }
            17 => {
                Self::from_17(&mut s, itoa);
                22
            }
            18 => {
                Self::from_18(&mut s, itoa);
                23
            }
            19 => {
                Self::from_19(&mut s, itoa);
                25
            }
            _ => {
                Self::from_20(&mut s, itoa);
                26
            }
        };

        // SAFETY: we're manually creating a `Str`.
        // This is okay because we filled the bytes
        // and know the length.
        unsafe { Str::from_raw(s, len) }
    }

    #[inline]
    // 9
    pub(super) fn from_1(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0] = itoa[0];
    }

    #[inline]
    // 99
    pub(super) fn from_2(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..2].copy_from_slice(&itoa[0..2]);
    }

    #[inline]
    // 999
    pub(super) fn from_3(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..3].copy_from_slice(&itoa[0..3]);
    }

    #[inline]
    // 9,999
    pub(super) fn from_4(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0] = itoa[0];
        s[1] = COMMA;
        s[2..5].copy_from_slice(&itoa[1..4]);
    }

    #[inline]
    // 99,999
    pub(super) fn from_5(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..2].copy_from_slice(&itoa[0..2]);
        s[2] = COMMA;
        s[3..6].copy_from_slice(&itoa[2..5]);
    }

    #[inline]
    // 999,999
    pub(super) fn from_6(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..3].copy_from_slice(&itoa[0..3]);
        s[3] = COMMA;
        s[4..7].copy_from_slice(&itoa[3..6]);
    }

    #[inline]
    // 9,999,999
    pub(super) fn from_7(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0] = itoa[0];
        s[1] = COMMA;
        s[2..5].copy_from_slice(&itoa[1..4]);
        s[5] = COMMA;
        s[6..9].copy_from_slice(&itoa[4..7]);
    }

    #[inline]
    // 99,999,999
    pub(super) fn from_8(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..2].copy_from_slice(&itoa[0..2]);
        s[2] = COMMA;
        s[3..6].copy_from_slice(&itoa[2..5]);
        s[6] = COMMA;
        s[7..10].copy_from_slice(&itoa[5..8]);
    }

    #[inline]
    // 999,999,999
    pub(super) fn from_9(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
        s[0..3].copy_from_slice(&itoa[0..3]);
        s[3] = COMMA;
        s[4..7].copy_from_slice(&itoa[3..6]);
        s[7] = COMMA;
        s[8..11].copy_from_slice(&itoa[6..9]);
    }

    #[inline]
    // 9,999,999,999
    pub(super) fn from_10(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_11(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_12(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_13(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_14(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_15(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_16(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_17(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_18(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_19(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
    pub(super) fn from_20(s: &mut [u8; Self::MAX_LEN], itoa: &[u8]) {
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
impl_u!(u8, u16, u32, u64);
#[cfg(target_pointer_width = "64")]
impl_u!(usize);

//---------------------------------------------------------------------------------------------------- From `u128`
/// This will return [`Self::UNKNOWN`] wrapped
/// in [`Result::Err`] if the conversion fails.
impl TryFrom<u128> for Unsigned {
    type Error = Self;
    #[inline]
    fn try_from(num: u128) -> Result<Self, Self> {
        match u64::try_from(num) {
            Ok(u) => Ok(Self::from_priv(u)),
            _ => Err(Self::UNKNOWN),
        }
    }
}
/// This will return [`Self::UNKNOWN`] wrapped
/// in [`Result::Err`] if the conversion fails.
impl TryFrom<&u128> for Unsigned {
    type Error = Self;
    #[inline]
    fn try_from(num: &u128) -> Result<Self, Self> {
        match u64::try_from(*num) {
            Ok(u) => Ok(Self::from_priv(u)),
            _ => Err(Self::UNKNOWN),
        }
    }
}

//---------------------------------------------------------------------------------------------------- From `NonZeroU*`
macro_rules! impl_nonu {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Unsigned {
				#[inline]
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
impl_nonu!(NonZeroUsize, &NonZeroUsize);

//---------------------------------------------------------------------------------------------------- From `i*`
macro_rules! impl_i {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::UNKNOWN`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Unsigned {
				type Error = Self;
				#[inline]
				fn try_from(num: $from) -> Result<Self, Self> {
					match u64::try_from(num) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::UNKNOWN),
					}
				}
			}
			/// This will return [`Self::UNKNOWN`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<&$from> for Unsigned {
				type Error = Self;
				#[inline]
				fn try_from(num: &$from) -> Result<Self, Self> {
					match u64::try_from(*num) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::UNKNOWN),
					}
				}
			}
		)*
	}
}
impl_i!(i8, i16, i32, i64, isize);

//---------------------------------------------------------------------------------------------------- From `readable::Int`
macro_rules! impl_int {
	($( $from:ty ),*) => {
		$(
			/// This will return [`Self::UNKNOWN`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Unsigned {
				type Error = Self;
				#[inline]
				fn try_from(int: $from) -> Result<Self, Self> {
					match u64::try_from(int.inner()) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::UNKNOWN),
					}
				}
			}
		)*
	}
}
impl_int!(Int, &Int);

//---------------------------------------------------------------------------------------------------- From `NonZeroI*`
macro_rules! impl_noni {
	($( $from:ty ),* $(,)?) => {
		$(
			/// This will return [`Self::UNKNOWN`] wrapped
			/// in [`Result::Err`] if the conversion fails.
			impl TryFrom<$from> for Unsigned {
				type Error = Self;
				#[inline]
				fn try_from(num: $from) -> Result<Self, Self> {
					match u64::try_from(num.get()) {
						Ok(u) => Ok(Self::from_priv(u)),
						_ => Err(Self::UNKNOWN),
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
impl_noni!(NonZeroIsize, &NonZeroIsize);

//---------------------------------------------------------------------------------------------------- From `f32/f64`
macro_rules! impl_f {
    ($from:ty) => {
        /// This will return [`Self::UNKNOWN`] if the input float is
        /// `NAN`, `INFINITY`, negative, or higher than [`u64::MAX`].
        impl TryFrom<$from> for Unsigned {
            type Error = Self;
            #[inline]
            fn try_from(float: $from) -> Result<Self, Self> {
                match float.classify() {
                    std::num::FpCategory::Normal => (),
                    std::num::FpCategory::Nan => return Err(Self::UNKNOWN),
                    std::num::FpCategory::Infinite => return Err(Self::UNKNOWN),
                    _ => (),
                }

                if float.is_sign_negative() {
                    Err(Self::UNKNOWN)
                } else if float > u64::MAX as $from {
                    Err(Self::UNKNOWN)
                } else {
                    Ok(Self::from_priv(float as u64))
                }
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
        assert_eq!(Unsigned::from(1_000_u64), "1,000");
        assert_eq!(Unsigned::from(65_535_u64), "65,535");
        assert_eq!(Unsigned::from(65_536_u64), "65,536");
        assert_eq!(Unsigned::from(100_000_u64), "100,000");
        assert_eq!(Unsigned::from(1_000_000_u64), "1,000,000");
        assert_eq!(Unsigned::from(10_000_000_u64), "10,000,000");
        assert_eq!(Unsigned::from(100_000_000_u64), "100,000,000");
        assert_eq!(Unsigned::from(1_000_000_000_u64), "1,000,000,000");
        assert_eq!(Unsigned::from(4_294_967_295_u64), "4,294,967,295");
        assert_eq!(Unsigned::from(4_294_967_296_u64), "4,294,967,296");
        assert_eq!(Unsigned::from(10_000_000_000_u64), "10,000,000,000");
        assert_eq!(Unsigned::from(100_000_000_000_u64), "100,000,000,000");
        assert_eq!(Unsigned::from(1_000_000_000_000_u64), "1,000,000,000,000");
        assert_eq!(Unsigned::from(10_000_000_000_000_u64), "10,000,000,000,000");
        assert_eq!(
            Unsigned::from(100_000_000_000_000_u64),
            "100,000,000,000,000"
        );
        assert_eq!(
            Unsigned::from(1_000_000_000_000_000_u64),
            "1,000,000,000,000,000"
        );
        assert_eq!(
            Unsigned::from(10_000_000_000_000_000_u64),
            "10,000,000,000,000,000"
        );
        assert_eq!(
            Unsigned::from(18_446_744_073_709_551_615_u64),
            "18,446,744,073,709,551,615"
        );
    }

    #[test]
    fn float() {
        assert_eq!(Unsigned::try_from(1_000.0).unwrap(), "1,000");
        assert_eq!(Unsigned::try_from(65_535.0).unwrap(), "65,535");
        assert_eq!(Unsigned::try_from(65_536.0).unwrap(), "65,536");
        assert_eq!(Unsigned::try_from(100_000.0).unwrap(), "100,000");
        assert_eq!(Unsigned::try_from(1_000_000.0).unwrap(), "1,000,000");
        assert_eq!(Unsigned::try_from(10_000_000.0).unwrap(), "10,000,000");
        assert_eq!(Unsigned::try_from(100_000_000.0).unwrap(), "100,000,000");
        assert_eq!(
            Unsigned::try_from(1_000_000_000.0).unwrap(),
            "1,000,000,000"
        );
        assert_eq!(
            Unsigned::try_from(4_294_967_295.0).unwrap(),
            "4,294,967,295"
        );
        assert_eq!(
            Unsigned::try_from(4_294_967_296.0).unwrap(),
            "4,294,967,296"
        );
        assert_eq!(
            Unsigned::try_from(10_000_000_000.0).unwrap(),
            "10,000,000,000"
        );
        assert_eq!(
            Unsigned::try_from(100_000_000_000.0).unwrap(),
            "100,000,000,000"
        );
        assert_eq!(
            Unsigned::try_from(1_000_000_000_000.0).unwrap(),
            "1,000,000,000,000"
        );
        assert_eq!(
            Unsigned::try_from(10_000_000_000_000.0).unwrap(),
            "10,000,000,000,000"
        );
        assert_eq!(
            Unsigned::try_from(100_000_000_000_000.0).unwrap(),
            "100,000,000,000,000"
        );
        assert_eq!(
            Unsigned::try_from(1_000_000_000_000_000.0).unwrap(),
            "1,000,000,000,000,000"
        );
        assert_eq!(
            Unsigned::try_from(10_000_000_000_000_000.0).unwrap(),
            "10,000,000,000,000,000"
        );
        assert_eq!(
            Unsigned::try_from(18_446_744_073_709_551_615.0).unwrap(),
            "18,446,744,073,709,551,615"
        );
    }

    #[test]
    fn special() {
        assert_eq!(Unsigned::try_from(f64::NAN), Err(Unsigned::UNKNOWN));
        assert_eq!(Unsigned::try_from(f64::INFINITY), Err(Unsigned::UNKNOWN));
        assert_eq!(
            Unsigned::try_from(f64::NEG_INFINITY),
            Err(Unsigned::UNKNOWN)
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        let this: Unsigned = Unsigned::from(1000_u64);
        let json = serde_json::to_string(&this).unwrap();
        assert_eq!(json, r#"[1000,"1,000"]"#);

        let this: Unsigned = serde_json::from_str(&json).unwrap();
        assert_eq!(this, 1000);
        assert_eq!(this, "1,000");

        // Bad bytes.
        assert!(serde_json::from_str::<Unsigned>(&"---").is_err());

        // Unknown.
        let json = serde_json::to_string(&Unsigned::UNKNOWN).unwrap();
        assert_eq!(json, r#"[0,"???"]"#);
        assert!(serde_json::from_str::<Unsigned>(&json)
            .unwrap()
            .is_unknown());
    }

    #[test]
    #[cfg(feature = "bincode")]
    fn bincode() {
        let this: Unsigned = Unsigned::from(1000_u64);
        let config = bincode::config::standard();
        let bytes = bincode::encode_to_vec(&this, config).unwrap();

        let this: Unsigned = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert_eq!(this, 1000);
        assert_eq!(this, "1,000");

        // Unknown.
        let bytes = bincode::encode_to_vec(&Unsigned::UNKNOWN, config).unwrap();
        let this: Unsigned = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert!(this.is_unknown());
    }

    #[test]
    #[cfg(feature = "borsh")]
    fn borsh() {
        let this: Unsigned = Unsigned::from(1000_u64);
        let bytes = borsh::to_vec(&this).unwrap();

        let this: Unsigned = borsh::from_slice(&bytes).unwrap();
        assert_eq!(this, 1000);
        assert_eq!(this, "1,000");

        // Bad bytes.
        assert!(borsh::from_slice::<Unsigned>(b"bad .-;[]124/ bytes").is_err());

        // Unknown.
        let bytes = borsh::to_vec(&Unsigned::UNKNOWN).unwrap();
        let this: Unsigned = borsh::from_slice(&bytes).unwrap();
        assert!(this.is_unknown());
    }
}
