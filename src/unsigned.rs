//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

use std::num::*;
use crate::macros::*;
use crate::constants::*;

//---------------------------------------------------------------------------------------------------- Unsigned
/// Human readable unsigned integer.
///
/// ## Creation
/// For [`u8`], [`u16`], [`u32`], [`u64`], [`usize`] or any [`NonZeroU8`] variant:
/// - Use [`Unsigned::from`]
///
/// [`f32`] or [`f64`] inputs will work, but:
/// - Signed floats will turn into `0`
/// - Fractional parts will be ignored
/// - Under/overflows will return [`Unsigned::unknown`]
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
	impl_buffer!(MAX_BUF_LEN, UNKNOWN_NUM_BUFFER, UNKNOWN.len());

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
		let (buf, len) = crate::buf::from_u(u);
		Self { buf, len }
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

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
