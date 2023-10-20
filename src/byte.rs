//! Human-readable byte formatting

//---------------------------------------------------------------------------------------------------- Use
use std::num::{
	NonZeroU8,NonZeroU16,NonZeroU32,NonZeroU64,
	NonZeroI8,NonZeroI16,NonZeroI32,NonZeroI64,
	NonZeroUsize,NonZeroIsize,
};
use crate::itoa;
use crate::str::Str;
use crate::macros::{
	impl_traits,impl_impl_math,impl_usize,
	impl_math, impl_common, impl_const,
};

//---------------------------------------------------------------------------------------------------- Byte
/// Human-readable byte formatting
///
/// This takes bytes as input and will store a formatted
/// string with the proper unit with 3 decimal point.
///
/// The unit will increase as the inner number increases, for example:
/// ```rust
/// # use readable::*;
/// assert_eq!(Byte::from(1_u64),                         "1 B");
/// assert_eq!(Byte::from(999_u64),                       "999 B");
/// assert_eq!(Byte::from(1_000_u64),                     "1.000 KB");
/// assert_eq!(Byte::from(2_101_123_u64),                 "2.101 MB");
/// assert_eq!(Byte::from(75_525_513_844_u64),            "75.525 GB");
/// assert_eq!(Byte::from(912_264_341_125_323_u64),       "912.264 TB");
/// assert_eq!(Byte::from(8_116_364_000_125_821_u64),     "8.116 PB");
/// assert_eq!(Byte::from(1_567_112_131_103_513_123_u64), "1.567 EB");
/// assert_eq!(Byte::max(), "18.446 EB");
/// ```
///
/// The maximum input is [`u64::MAX`] or `18.446` exabytes.
///
/// ## Input
/// [`From`] input can be:
/// - Any unsigned integer [`u8`], [`usize`], etc
/// - Any signed integer [`i8`], [`isize`], etc
/// - [`f32`] or [`f64`]
/// - `NonZero` types like [`NonZeroU8`]
///
/// Inputs are presumed to be in bytes.
///
/// ## Errors
/// A [`Byte::unknown()`] will be returned if the input is:
/// - A negative integer
/// - Larger than [`u64::MAX`]
/// - [`f32::NAN`], [`f32::INFINITY`], [`f32::NEG_INFINITY`] (or the [`f64`] versions)
///
/// ## Math
/// These operators are overloaded. They will always output a new `Self`:
/// - `Add +`
/// - `Sub -`
/// - `Div /`
/// - `Mul *`
/// - `Rem %`
///
/// They can either be:
/// - Combined with another `Self`, e.g: `Byte::from(1.0) + Byte::from(1.0)`
/// - Or with the inner number itself: `BytePad::from(1.0) + 1.0`
///
/// ```rust
/// # use readable::*;
/// let byte = Byte::from(1.0);
/// assert_eq!(byte, "1 B");
///
/// let byte = byte + Byte::from(1.0);
/// assert_eq!(byte, "2 B");
/// ```
///
/// ## Size
/// [`Str<10>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<Byte>(), 24);
/// ```
///
/// ## Copy
/// [`Copy`] is available.
///
/// The actual strings used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a byte array buffer, literally: [`Str<10>`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
/// ```rust
/// # use readable::*;
/// let a = Byte::from(100_000);
///
/// // Copy 'a', use 'b'.
/// let b = a;
/// assert_eq!(b, 100_000);
///
/// // We can still use 'a'
/// assert_eq!(a, 100_000);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Byte(u64, Str<{ Byte::MAX_LEN }>);

impl_math!(Byte, u64);
impl_traits!(Byte, u64);

//---------------------------------------------------------------------------------------------------- Constants
/// 1 `byte`
const BYTE: u64 = 1;
/// 1 `kilobyte` in `bytes`
const KILOBYTE: u64 = 1_000;
/// 1 `megabyte` in `bytes`
const MEGABYTE: u64 = 1_000_000;
/// 1 `gigabyte` in `bytes`
const GIGABYTE: u64 = 1_000_000_000;
/// 1 `terabyte` in `bytes`
const TERABYTE: u64 = 1_000_000_000_000;
/// 1 `petabyte` in `bytes`
const PETABYTE: u64 = 1_000_000_000_000_000;
/// 1 `exabyte` in `bytes`
const EXABYTE: u64 = 1_000_000_000_000_000_000;
/// Number used when using [`Byte::zero()`] or when [`Byte::unknown()`] is encountered
const ZERO: u64 = 0;

//---------------------------------------------------------------------------------------------------- Constants
impl Byte {
	/// The maximum string length of a [`Byte`]
	/// ```rust
	/// # use readable::Byte;
	/// assert_eq!("xxx.xxx KB".len(), Byte::MAX_LEN);
	/// ```
	pub const MAX_LEN: usize = 10;

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::zero(), "0 B");
	/// assert_eq!(Byte::zero(), 0_u64);
	/// assert_eq!(Byte::zero(), Byte::from(0_u64));
	/// ```
	pub const ZERO: Byte = Byte(ZERO, Str::from_static_str("0 B"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::byte(), "1 B");
	/// assert_eq!(Byte::byte(), 1_u64);
	/// assert_eq!(Byte::byte(), Byte::from(1_u64));
	/// ```
	pub const BYTE: Byte = Byte(BYTE, Str::from_static_str("1 B"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::kilobyte(), "1.000 KB");
	/// assert_eq!(Byte::kilobyte(), 1_000_u64);
	/// assert_eq!(Byte::kilobyte(), Byte::from(1_000_u64));
	/// ```
	pub const KILOBYTE: Byte = Byte(KILOBYTE, Str::from_static_str("1.000 KB"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::megabyte(), "1.000 MB");
	/// assert_eq!(Byte::megabyte(), 1_000_000_u64);
	/// assert_eq!(Byte::megabyte(), Byte::from(1_000_000_u64));
	/// ```
	pub const MEGABYTE: Byte = Byte(MEGABYTE, Str::from_static_str("1.000 MB"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::gigabyte(), "1.000 GB");
	/// assert_eq!(Byte::gigabyte(), 1_000_000_000_u64);
	/// assert_eq!(Byte::gigabyte(), Byte::from(1_000_000_000_u64));
	/// ```
	pub const GIGABYTE: Byte = Byte(GIGABYTE, Str::from_static_str("1.000 GB"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::terabyte(), "1.000 TB");
	/// assert_eq!(Byte::terabyte(), 1_000_000_000_000_u64);
	/// assert_eq!(Byte::terabyte(), Byte::from(1_000_000_000_000_u64));
	/// ```
	pub const TERABYTE: Byte = Byte(TERABYTE, Str::from_static_str("1.000 TB"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::petabyte(), "1.000 PB");
	/// assert_eq!(Byte::petabyte(), 1_000_000_000_000_000_u64);
	/// assert_eq!(Byte::petabyte(), Byte::from(1_000_000_000_000_000_u64));
	/// ```
	pub const PETABYTE: Byte = Byte(PETABYTE, Str::from_static_str("1.000 PB"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::exabyte(), "1.000 EB");
	/// assert_eq!(Byte::exabyte(), 1_000_000_000_000_000_000_u64);
	/// assert_eq!(Byte::exabyte(), Byte::from(1_000_000_000_000_000_000_u64));
	/// ```
	pub const EXABYTE: Byte = Byte(EXABYTE, Str::from_static_str("1.000 EB"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::max(), Byte::from(u64::MAX));
	/// assert_eq!(Byte::max(), "18.446 EB");
	/// assert_eq!(Byte::max(), u64::MAX);
	/// ```
	pub const MAX: Byte = Byte(u64::MAX, Str::from_static_str("18.446 EB"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::unknown(), Byte::from(f32::NAN));
	/// assert_eq!(Byte::unknown(), Byte::from(-1));
	/// assert_eq!(Byte::unknown(), "???.??? B");
	/// ```
	pub const UNKNOWN: Byte = Byte(ZERO, Str::from_static_str("???.??? B"));
}

//---------------------------------------------------------------------------------------------------- Byte Impl
impl Byte {
	impl_common!(u64);
	impl_const!();
	impl_usize!();

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::zero(), Byte::ZERO);
	/// ```
	pub const fn zero() -> Self {
		Self::ZERO
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::byte(), Byte::BYTE);
	/// ```
    pub const fn byte() -> Self {
		Self::BYTE
    }

    #[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::kilobyte(), Byte::KILOBYTE);
	/// ```
    pub const fn kilobyte() -> Self {
		Self::KILOBYTE
    }

    #[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::megabyte(), Byte::MEGABYTE);
	/// ```
    pub const fn megabyte() -> Self {
		Self::MEGABYTE
    }

    #[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::gigabyte(), Byte::GIGABYTE);
	/// ```
    pub const fn gigabyte() -> Self {
		Self::GIGABYTE
    }

    #[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::terabyte(), Byte::TERABYTE);
	/// ```
    pub const fn terabyte() -> Self {
		Self::TERABYTE
    }

    #[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::petabyte(), Byte::PETABYTE);
	/// ```
    pub const fn petabyte() -> Self {
		Self::PETABYTE
    }

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::exabyte(), Byte::EXABYTE);
	/// ```
	pub const fn exabyte() -> Self {
		Self::EXABYTE
    }

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::max(), Byte::MAX);
	/// ```
	pub const fn max() -> Self {
		Self::MAX
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Byte::unknown(), Byte::UNKNOWN);
	/// ```
	pub const fn unknown() -> Self {
		Self::UNKNOWN
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(Byte::UNKNOWN.is_unknown());
	/// assert!(!Byte::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		match *self {
			Self::UNKNOWN => true,
			_ => false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Private Impl
impl Byte {
	fn from_priv(bytes: u64) -> Self {
		// If bytes is a perfect multiple, return literals.
		match bytes {
			ZERO     => return Self::zero(),
			BYTE     => return Self::byte(),
			KILOBYTE => return Self::kilobyte(),
			MEGABYTE => return Self::megabyte(),
			GIGABYTE => return Self::gigabyte(),
			TERABYTE => return Self::terabyte(),
			PETABYTE => return Self::petabyte(),
			EXABYTE  => return Self::exabyte(),
			_ => (),
		}

		const UNITS: [u8; 6] = [b'K', b'M', b'G', b'T', b'P', b'E'];
		const LN_KILOBYTE: f64 = 6.931471806; // ln 1024
		const Z:     u8 = b'0';
		const SPACE: u8 = b' ';
		const B:     u8 = b'B';
		const DOT:   u8 = b'.';

		// Our final string buffer.
		let mut b = [0; 10];

		// If bytes is `999 B` or less.
		if bytes < Self::KILOBYTE {
			let mut itoa = crate::ItoaTmp::new();
			let itoa = itoa.format(bytes).as_bytes();
			let len = itoa.len();
			b[..len].copy_from_slice(itoa);

			b[len] = SPACE;
			b[len + 1] = B;

			Self(bytes, unsafe { Str::from_raw(b, len as u8 + 2) })

		// Else calculate.
		} else {
			let size = bytes as f64;
			let exp = match (size.ln() / LN_KILOBYTE) as usize {
				e if e == 0 => 1,
				e => e,
			};

			// e.g, 111.222
			// 111
			let float = size / KILOBYTE.pow(exp as u32) as f64;
			// 222
			let fract = (float.fract() * 1_000.0) as u16;

			// 111 float as u16.
			let base = float as u16;

			// Format first 1-3 digits into buffer (111)
			let mut itoa = crate::ItoaTmp::new();
			let itoa = itoa.format(base).as_bytes();
			let idx = if base < 10 {
				b[0] = itoa[0];
				b[1] = DOT;
				2
			} else if base < 100 {
				b[0] = itoa[0];
				b[1] = itoa[1];
				b[2] = DOT;
				3
			} else {
				b[0] = itoa[0];
				b[1] = itoa[1];
				b[2] = itoa[2];
				b[3] = DOT;
				4
			};

			// Format 3 fractional digits into buffer (222)
			let mut itoa = crate::ItoaTmp::new();
			let itoa = itoa.format(fract).as_bytes();
			if fract < 10 {
				b[idx + 0] = Z;
				b[idx + 1] = Z;
				b[idx + 2] = itoa[0];
			} else if fract < 100 {
				b[idx + 0] = Z;
				b[idx + 1] = itoa[0];
				b[idx + 2] = itoa[1];
			} else {
				b[idx + 0] = itoa[0];
				b[idx + 1] = itoa[1];
				b[idx + 2] = itoa[2];
			}

			// Format ending ` uB` into ending
			// where `u` is the specific unit (K, G, T, etc).
			b[idx + 3] = SPACE;
			b[idx + 4] = UNITS[exp - 1];
			b[idx + 5] = B;

			Self(bytes, unsafe { Str::from_raw(b, idx as u8 + 6)})
		}
	}
}

//---------------------------------------------------------------------------------------------------- From `u*`
macro_rules! impl_u {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Byte {
				#[inline]
				fn from(uint: $from) -> Self {
					let u = uint as u64;
					Self::from_priv(u)
				}
			}
			impl From<&$from> for Byte {
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

//---------------------------------------------------------------------------------------------------- From `i*`
macro_rules! impl_i {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Byte {
				#[inline]
				fn from(uint: $from) -> Self {
					if uint.is_negative() {
						return Self::unknown();
					}
					let u = uint as u64;
					Self::from_priv(u)
				}
			}
			impl From<&$from> for Byte {
				#[inline]
				fn from(uint: &$from) -> Self {
					if uint.is_negative() {
						return Self::unknown();
					}
					let u = *uint as u64;
					Self::from_priv(u)
				}
			}
		)*
	}
}
impl_i!(i8,i16,i32,i64,isize);

//---------------------------------------------------------------------------------------------------- From `f32/f64`
macro_rules! impl_f {
	($from:ty) => {
		/// This will return [`Self::unknown`]
		/// if the input float is `NAN`, `INFINITY`, or negative.
		impl From<$from> for Byte {
			fn from(float: $from) -> Self {
				match float.classify() {
					std::num::FpCategory::Normal   => (),
					std::num::FpCategory::Nan      => return Self::unknown(),
					std::num::FpCategory::Infinite => return Self::unknown(),
					_ => (),
				}

				if float.is_sign_negative() {
					return Self::unknown();
				}

				Self::from_priv(float as u64)
			}
		}
	}
}
impl_f!(f32);
impl_f!(f64);

//---------------------------------------------------------------------------------------------------- From `NonZeroU*`
macro_rules! impl_nonu {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Byte {
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

//---------------------------------------------------------------------------------------------------- From `NonZeroU*`
macro_rules! impl_noni {
	($( $from:ty ),* $(,)?) => {
		$(
			impl From<$from> for Byte {
				fn from(int: $from) -> Self {
					let u = int.get();
					if u.is_negative() {
						return Self::unknown();
					}
					let u = u as u64;
					Self::from_priv(u)
				}
			}
		)*
	}
}
impl_noni! {
	NonZeroI8,NonZeroI16,NonZeroI32,NonZeroI64,
	&NonZeroI8,&NonZeroI16,&NonZeroI32,&NonZeroI64,
	NonZeroIsize,&NonZeroIsize,
}