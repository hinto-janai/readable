//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

use std::num::*;
use crate::macros::*;
use crate::num::constants::{
	MAX_BUF_LEN,UNKNOWN_NUM_BUFFER,
	UNKNOWN,ZERO_NUM_BUFFER,
};

//---------------------------------------------------------------------------------------------------- Unsigned
/// Human readable unsigned integer.
///
/// ## Construction
/// For [`u8`], [`u16`], [`u32`], [`u64`], [`usize`] or any `NonZeroU*` variant:
/// - Use [`Unsigned::from`]
///
/// [`f32`] or [`f64`] inputs must use [`Unsigned::try_from`].
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
/// ## Cloning
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`], but a 26 byte array buffer (`[u8; 26]`).
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Unsigned(u64, Buffer);

impl Unsigned {
	impl_common!(u64);
	impl_const!();
	impl_usize!();
	impl_buffer!(MAX_BUF_LEN, UNKNOWN_NUM_BUFFER, UNKNOWN.len());

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Unsigned::zero(), 0);
	/// ```
	pub const fn zero() -> Self {
		Self(0, Buffer::zero())
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// # use readable::num::*;
	/// assert_eq!(Unsigned::unknown(), UNKNOWN);
	/// ```
	pub const fn unknown() -> Self {
		Self(0, Buffer::unknown())
	}
}

//---------------------------------------------------------------------------------------------------- Unsigned
macro_rules! impl_u {
	($( $from:ty ),*) => {
		$(
			impl From<$from> for Unsigned {
				fn from(uint: $from) -> Self {
					let u = uint as u64;
					Self(u, Buffer::from_u(u))
				}
			}
			impl From<&$from> for Unsigned {
				fn from(uint: &$from) -> Self {
					let u = *uint as u64;
					Self(u, Buffer::from_u(u))
				}
			}
		)*
	}
}
impl_u!(u8,u16,u32,u64,usize);

//---------------------------------------------------------------------------------------------------- NonZeroU*
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

//---------------------------------------------------------------------------------------------------- Floats
macro_rules! impl_f {
	($from:ty) => {
		/// This will return [`Self::unknown`]
		/// if the input float is `NAN`, `INFINITY`, or negative.
		impl TryFrom<$from> for Unsigned {
			type Error = Self;
			fn try_from(float: $from) -> Result<Self, Self> {
				#[cfg(not(feature = "ignore_nan_inf"))]
				{
					match float.classify() {
						std::num::FpCategory::Normal   => (),
						std::num::FpCategory::Nan      => return Err(Self::unknown()),
						std::num::FpCategory::Infinite => return Err(Self::unknown()),
						_ => (),
					}
				}

				if float.is_sign_negative() {
					return Err(Self::unknown());
				}

				let u = float as u64;
				Ok(Self(u, Buffer::from_u(u)))
			}
		}
	}
}
impl_f!(f32);
impl_f!(f64);

//---------------------------------------------------------------------------------------------------- Int
macro_rules! impl_i {
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
impl_i!(i8,i16,i32,i64,isize);

//---------------------------------------------------------------------------------------------------- NonZeroI*
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

//---------------------------------------------------------------------------------------------------- Traits
impl_math!(Unsigned, u64);
impl_traits!(Unsigned, u64);

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
	fn from_u(u: u64) -> Self {
		let (buf, len) = crate::num::buf::from_u(u);
		Self { buf, len }
	}
}

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
