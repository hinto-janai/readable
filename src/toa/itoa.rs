// This `itoa` implementation is taken from `https://github.com/dtolnay/itoa`.

//---------------------------------------------------------------------------------------------------- Use
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::expl_impl_clone_on_copy,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::unreadable_literal
)]

use crate::toa::udiv128;

use core::mem::{self, MaybeUninit};
use core::{ptr, slice, str};
use std::borrow::Borrow;

//---------------------------------------------------------------------------------------------------- Itoa
/// Fast integer to string conversion
///
/// This struct represents a stack-based string converted from an [`Integer`]
/// ([`u8`] up until [`u128`] and [`i8`] up until [`i128`]).
///
/// It internally uses [`itoa`](https://docs.rs/itoa) by `dtolnay`,
/// however [`Itoa`] stores the string computation and can be turned into a [`&str`]
/// again and again after construction.
///
/// This does not do any `readable`-style formatting (adding commas), it simply
/// converts an integer into a string (but is much faster than [`format!()`]).
///
/// ## Example
/// ```rust
/// # use readable::toa::*;
/// let itoa = Itoa::new(1000);
/// assert_eq!(itoa, "1000");
///
/// let copy = itoa;
/// assert_eq!(itoa.as_str(), copy.as_str());
/// ```
///
/// ## Size
/// ```rust
/// # use readable::toa::*;
/// assert_eq!(std::mem::size_of::<Itoa>(), 42);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Itoa {
	len: u8,
	offset: u8,
	bytes: [MaybeUninit<u8>; I128_MAX_LEN],
}

impl Itoa {
    #[inline]
	#[allow(clippy::ptr_as_ptr, clippy::borrow_as_ptr)]
	/// Create a new [`Itoa`].
	///
	/// Takes any [`Integer`] from the standard library (but not floats).
	///
	/// ```rust
	/// # use readable::toa::Itoa;
	/// let itoa = Itoa::new(u128::MAX);
	/// assert_eq!(itoa, "340282366920938463463374607431768211455");
	///
	/// let itoa = Itoa::new(i128::MIN);
	/// assert_eq!(itoa, "-170141183460469231731687303715884105728");
	/// ```
    pub fn new<I: Integer>(i: I) -> Self {
        let mut bytes = [MaybeUninit::<u8>::uninit(); I128_MAX_LEN];

		// SAFETY: dtolnay
        let (len, offset) = i.write(unsafe {
            &mut *(&mut bytes as *mut [MaybeUninit<u8>; I128_MAX_LEN]
                as *mut <I as private::Sealed>::Itoa)
        });

		Self {
			len: len as u8,
			offset: offset as u8,
			bytes,
		}
    }

	#[inline]
	/// Turns [`Itoa`] into a `&str`.
	///
	/// ```rust
	/// # use readable::toa::Itoa;
	/// let itoa = Itoa::new(u128::MAX);
	/// assert_eq!(itoa, "340282366920938463463374607431768211455");
	///
	/// let itoa = Itoa::new(i128::MIN);
	/// assert_eq!(itoa, "-170141183460469231731687303715884105728");
	/// ```
	pub const fn as_str(&self) -> &str {
		// Safety: Constructors must set state correctly.
		unsafe {
			let slice = slice::from_raw_parts(
				self.bytes.as_ptr().offset(self.offset as isize).cast::<u8>(),
				self.len as usize
			);
			std::str::from_utf8_unchecked(slice)
		}
	}

	#[inline]
	#[allow(clippy::len_without_is_empty)]
	/// Returns the `str` byte length of this [`Itoa`]
	///
	/// ```rust
	/// # use readable::toa::Itoa;
	/// let itoa = Itoa::new(1000);
	/// assert_eq!(itoa.len(), 4);
	/// ```
	pub const fn len(&self) -> u8 {
		self.len
	}
}

//---------------------------------------------------------------------------------------------------- ItoaTmp
/// A short-lived version of [`Itoa`]
///
/// This version doesn't save the formatting
/// computation, and is meant for cases where the
/// lifetime of the formatted output [`&str`] is very brief.
///
/// This version has less overhead, but the string
/// must be formatted everytime you need it.
///
/// See [`crate::itoa!()`] for a quick 1-line format macro.
///
/// ```rust
/// # use readable::toa::ItoaTmp;
/// assert_eq!(ItoaTmp::new().format(10), "10");
/// ```
///
/// You could keep a [`ItoaTmp`] around to use it
/// as a factory to keep formatting new strings,
/// as it will reuse the inner buffer:
/// ```rust
/// # use readable::toa::*;
/// let mut itoa = ItoaTmp::new();
///
/// assert_eq!(itoa.format(10), "10");
/// assert_eq!(itoa.format(20), "20");
/// assert_eq!(itoa.format(30), "30");
/// ```
///
/// ## Size
/// ```rust
/// # use readable::toa::*;
/// assert_eq!(std::mem::size_of::<ItoaTmp>(), 40);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct ItoaTmp {
	bytes: [MaybeUninit<u8>; I128_MAX_LEN],
}

impl ItoaTmp {
	#[inline]
	/// Create a new [`ItoaTmp`].
	pub const fn new() -> Self {
		Self { bytes: [MaybeUninit::<u8>::uninit(); I128_MAX_LEN] }
	}

	#[inline]
	#[allow(clippy::ptr_as_ptr, clippy::borrow_as_ptr)]
	/// Format an [`Integer`] into a [`&str`] with an existing [`ItoaTmp`]
	///
	/// ```rust
	/// # use readable::toa::ItoaTmp;
	/// // We can cheaply reuse this.
	/// let mut itoa = ItoaTmp::new();
	///
	/// assert_eq!(itoa.format(1),    "1");
	/// assert_eq!(itoa.format(10),   "10");
	/// assert_eq!(itoa.format(100),  "100");
	/// assert_eq!(itoa.format(1000), "1000");
	/// ```
	pub fn format<I: Integer>(&mut self, integer: I) -> &str {
		// SAFETY: dtolnay
		unsafe {
			let (len, offset) = integer.write(
				&mut *(&mut self.bytes as *mut [MaybeUninit<u8>; I128_MAX_LEN]
					as *mut <I as private::Sealed>::Itoa)
			);
			let slice = slice::from_raw_parts(
				self.bytes.as_ptr().offset(offset) as *const u8,
				len,
			);
			std::str::from_utf8_unchecked(slice)
		}
	}
}

//---------------------------------------------------------------------------------------------------- Private Itoa
// This is for usage in `Unsigned` and `Int`.
#[derive(Copy, Clone, Debug)]
pub(crate) struct Itoa64 {
	bytes: [MaybeUninit<u8>; U64_MAX_LEN],
}

impl Itoa64 {
	#[inline]
	pub(crate) const fn new() -> Self {
		Self { bytes: [MaybeUninit::<u8>::uninit(); U64_MAX_LEN] }
	}

	#[inline]
	#[allow(clippy::ptr_as_ptr, clippy::borrow_as_ptr)]
	pub(crate) fn format_str<I: Integer>(&mut self, integer: I) -> &str {
		// SAFETY: dtolnay
		unsafe {
			let (len, offset) = integer.write(
				&mut *(&mut self.bytes as *mut [MaybeUninit<u8>; U64_MAX_LEN]
					as *mut <I as private::Sealed>::Itoa)
			);
			let slice = slice::from_raw_parts(
				self.bytes.as_ptr().offset(offset) as *const u8,
				len,
			);
			std::str::from_utf8_unchecked(slice)
		}
	}

	#[inline]
	#[allow(clippy::ptr_as_ptr, clippy::borrow_as_ptr)]
	pub(crate) fn format<I: Integer>(&mut self, integer: I) -> &[u8] {
		// SAFETY: dtolnay
		unsafe {
			let (len, offset) = integer.write(
				&mut *(&mut self.bytes as *mut [MaybeUninit<u8>; U64_MAX_LEN]
					as *mut <I as private::Sealed>::Itoa)
			);
			slice::from_raw_parts(
				self.bytes.as_ptr().offset(offset) as *const u8,
				len,
			)
		}
	}
}

/// Quickly format an integer to a [`&str`]
///
/// This creates an [`ItoaTmp`] from an [`Integer`], returns the output [`&str`], and immediately goes out of scope.
///
/// The function signature would look something like:
/// ```rust,ignore
/// fn itoa<I: Integer>(integer: I) -> &'tmp str
/// where
///     'tmp: FreedAtEndOfStatement
/// ```
///
/// [`ItoaTmp`] is created and immediately dropped, thus it cannot be stored:
/// ```rust,ignore
/// # use readable::itoa::*;
/// let x = itoa!(10);
///         ^^^^^^^^^- temporary value is freed at the end of this statement
///
/// assert_eq!(x, "10");
/// ------------------- compile error: borrow later used here
/// ```
///
/// You must use the [`&str`] in 1 single statement:
/// ```rust
/// # use readable::itoa;
/// assert_eq!(itoa!(10), "10"); // ok
///
/// if itoa!(10) == "10" {
///     // ok
/// }
///
/// // ok
/// let string: String = itoa!(10).to_string();
/// assert_eq!(string, "10");
/// ```
///
/// The macro expands to `ItoaTmp::new().format(x)`:
/// ```rust
/// # use readable::itoa;
/// // These are the same.
///
/// itoa!(10);
///
/// readable::toa::ItoaTmp::new().format(10);
/// ```
#[macro_export]
macro_rules! itoa {
	($into_dtoa:expr) => {{
		$crate::toa::ItoaTmp::new().format($into_dtoa)
	}};
}

//---------------------------------------------------------------------------------------------------- Itoa Traits
impl std::ops::Deref for Itoa {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl AsRef<str> for Itoa {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl Borrow<str> for Itoa {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

impl PartialEq<str> for Itoa {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}

impl PartialEq<&str> for Itoa {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}

impl PartialEq<String> for Itoa {
	fn eq(&self, other: &String) -> bool {
		self.as_str() == other
	}
}

impl<T: Integer> std::convert::From<T> for Itoa {
	fn from(integer: T) -> Self {
		Self::new(integer)
	}
}

impl std::fmt::Display for Itoa {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

//---------------------------------------------------------------------------------------------------- Integer
/// An integer that can be written into an [`Itoa`].
///
/// ```rust
/// # use readable::toa::Itoa;
/// let itoa = Itoa::new(-2147483648_i64);
/// assert_eq!(itoa, "-2147483648");
///
/// // NonZero types work too.
/// let itoa = Itoa::new(std::num::NonZeroU32::new(1000).unwrap());
/// assert_eq!(itoa, "1000");
///
/// // ⚠️ Manual lossy conversion.
/// let itoa = Itoa::new(134.425 as u8);
/// assert_eq!(itoa, "134"); // decimal truncated
/// ```
pub trait Integer: private::Sealed {}

// Seal to prevent downstream implementations of the Integer trait.
mod private {
    pub trait Sealed: Copy {
        type Itoa: 'static;
        fn write(self, buf: &mut Self::Itoa) -> (usize, isize);
    }
}

//---------------------------------------------------------------------------------------------------- Itoa internal stuff
const DEC_DIGITS_LUT: &[u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

// Adaptation of the original implementation at
// https://github.com/rust-lang/rust/blob/b8214dc6c6fc20d0a660fb5700dca9ebf51ebe89/src/libcore/fmt/num.rs#L188-L266
macro_rules! impl_Integer {
    ($($max_len:expr => $t:ident),* as $conv_fn:ident) => {$(
        impl Integer for $t {}

        impl private::Sealed for $t {
            type Itoa = [MaybeUninit<u8>; $max_len];

            #[allow(unused_comparisons)]
            #[inline]
            fn write(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> (usize, isize) {
                let is_nonnegative = self >= 0;
                let mut n = if is_nonnegative {
                    self as $conv_fn
                } else {
                    // convert the negative num to positive by summing 1 to it's 2 complement
                    (!(self as $conv_fn)).wrapping_add(1)
                };
                let mut curr = buf.len() as isize;
                let buf_ptr = buf.as_mut_ptr().cast::<u8>();
                let lut_ptr = DEC_DIGITS_LUT.as_ptr();

				// SAFETY: dtolnay
                unsafe {
                    // need at least 16 bits for the 4-characters-at-a-time to work.
                    if mem::size_of::<$t>() >= 2 {
                        // eagerly decode 4 characters at a time
                        while n >= 10000 {
                            let rem = (n % 10000) as isize;
                            n /= 10000;

                            let d1 = (rem / 100) << 1;
                            let d2 = (rem % 100) << 1;
                            curr -= 4;
                            ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                            ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
                        }
                    }

                    // if we reach here numbers are <= 9999, so at most 4 chars long
                    let mut n = n as isize; // possibly reduce 64bit math

                    // decode 2 more chars, if > 2 chars
                    if n >= 100 {
                        let d1 = (n % 100) << 1;
                        n /= 100;
                        curr -= 2;
                        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                    }

                    // decode last 1 or 2 chars
                    if n < 10 {
                        curr -= 1;
                        *buf_ptr.offset(curr) = (n as u8) + b'0';
                    } else {
                        let d1 = n << 1;
                        curr -= 2;
                        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                    }

                    if !is_nonnegative {
                        curr -= 1;
                        *buf_ptr.offset(curr) = b'-';
                    }
                }

                let len = buf.len() - curr as usize;
				(len, curr)
            }
        }
    )*};
}

macro_rules! impl_non_zero {
	($($max_len:expr => $t:ident => $inner:ty),*) => {$(
		impl Integer for $t {}

        impl private::Sealed for $t {
            type Itoa = [MaybeUninit<u8>; $max_len];

            #[allow(unused_comparisons)]
            #[inline]
            fn write(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> (usize, isize) {
				private::Sealed::write(self.get(), buf)
			}
		}
	)*};
}

const I8_MAX_LEN: usize = 4;
const U8_MAX_LEN: usize = 3;
const I16_MAX_LEN: usize = 6;
const U16_MAX_LEN: usize = 5;
const I32_MAX_LEN: usize = 11;
const U32_MAX_LEN: usize = 10;
const I64_MAX_LEN: usize = 20;
const U64_MAX_LEN: usize = 20;

impl_Integer!(
    I8_MAX_LEN => i8,
    U8_MAX_LEN => u8,
    I16_MAX_LEN => i16,
    U16_MAX_LEN => u16,
    I32_MAX_LEN => i32,
    U32_MAX_LEN => u32
    as u32);

impl_Integer!(I64_MAX_LEN => i64, U64_MAX_LEN => u64 as u64);

use std::num::{
	NonZeroI8,
	NonZeroU8,
	NonZeroI16,
	NonZeroU16,
	NonZeroI32,
	NonZeroU32,
	NonZeroI128,
	NonZeroU128,
	NonZeroIsize,
	NonZeroUsize,
};

impl_non_zero! {
    I8_MAX_LEN  => NonZeroI8  => i8,
    U8_MAX_LEN  => NonZeroU8  => u8,
    I16_MAX_LEN => NonZeroI16 => i16,
    U16_MAX_LEN => NonZeroU16 => u16,
    I32_MAX_LEN => NonZeroI32 => i32,
    U32_MAX_LEN => NonZeroU32 => u32
}

#[cfg(target_pointer_width = "16")]
impl_Integer!(I16_MAX_LEN => isize, U16_MAX_LEN => usize as u16);
#[cfg(target_pointer_width = "16")]
impl_non_zero!(I16_MAX_LEN => NonZeroIsize => i16, U16_MAX_LEN => NonZeroUsize => u16);

#[cfg(target_pointer_width = "32")]
impl_Integer!(I32_MAX_LEN => isize, U32_MAX_LEN => usize as u32);
#[cfg(target_pointer_width = "32")]
impl_non_zero!(I32_MAX_LEN => NonZeroIsize => i32, U32_MAX_LEN => NonZeroUsize => u32);

#[cfg(target_pointer_width = "64")]
impl_Integer!(I64_MAX_LEN => isize, U64_MAX_LEN => usize as u64);
#[cfg(target_pointer_width = "64")]
impl_non_zero!(I64_MAX_LEN => NonZeroIsize => i64, U64_MAX_LEN => NonZeroUsize => u64);

macro_rules! impl_Integer128 {
    ($($max_len:expr => $t:ident),*) => {$(
        impl Integer for $t {}

        impl private::Sealed for $t {
            type Itoa = [MaybeUninit<u8>; $max_len];

            #[allow(unused_comparisons)]
            #[inline]
            fn write(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> (usize, isize) {
                let is_nonnegative = self >= 0;
                let n = if is_nonnegative {
                    self as u128
                } else {
                    // convert the negative num to positive by summing 1 to it's 2 complement
                    (!(self as u128)).wrapping_add(1)
                };
                let mut curr = buf.len() as isize;
                let buf_ptr = buf.as_mut_ptr().cast::<u8>();

				// SAFETY: dtolnay
                unsafe {
                    // Divide by 10^19 which is the highest power less than 2^64.
                    let (n, rem) = udiv128::udivmod_1e19(n);
                    let buf1 = buf_ptr.offset(curr - U64_MAX_LEN as isize).cast::<[MaybeUninit<u8>; U64_MAX_LEN]>();
                    curr -= rem.write(&mut *buf1).0 as isize;

                    if n != 0 {
                        // Memset the base10 leading zeros of rem.
                        let target = buf.len() as isize - 19;
                        ptr::write_bytes(buf_ptr.offset(target), b'0', (curr - target) as usize);
                        curr = target;

                        // Divide by 10^19 again.
                        let (n, rem) = udiv128::udivmod_1e19(n);
                        let buf2 = buf_ptr.offset(curr - U64_MAX_LEN as isize).cast::<[MaybeUninit<u8>; U64_MAX_LEN]>();
                        curr -= rem.write(&mut *buf2).0 as isize;

                        if n != 0 {
                            // Memset the leading zeros.
                            let target = buf.len() as isize - 38;
                            ptr::write_bytes(buf_ptr.offset(target), b'0', (curr - target) as usize);
                            curr = target;

                            // There is at most one digit left
                            // because u128::max / 10^19 / 10^19 is 3.
                            curr -= 1;
                            *buf_ptr.offset(curr) = (n as u8) + b'0';
                        }
                    }

                    if !is_nonnegative {
                        curr -= 1;
                        *buf_ptr.offset(curr) = b'-';
                    }

                    let len = buf.len() - curr as usize;
					(len, curr)
                }
            }
        }
    )*};
}

const U128_MAX_LEN: usize = 39;
const I128_MAX_LEN: usize = 40;

impl_Integer128!(I128_MAX_LEN => i128, U128_MAX_LEN => u128);
impl_non_zero!(I128_MAX_LEN => NonZeroI128 => i128, U128_MAX_LEN => NonZeroU128 => u128);

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn all_u128_lengths() {
		for i in [
			0_u128,
			1_u128,
			10_u128,
			100_u128,
			1000_u128,
			10000_u128,
			100000_u128,
			1000000_u128,
			10000000_u128,
			100000000_u128,
			1000000000_u128,
			10000000000_u128,
			100000000000_u128,
			1000000000000_u128,
			10000000000000_u128,
			100000000000000_u128,
			1000000000000000_u128,
			10000000000000000_u128,
			100000000000000000_u128,
			1000000000000000000_u128,
			10000000000000000000_u128,
			100000000000000000000_u128,
			1000000000000000000000_u128,
			10000000000000000000000_u128,
			100000000000000000000000_u128,
			1000000000000000000000000_u128,
			10000000000000000000000000_u128,
			100000000000000000000000000_u128,
			1000000000000000000000000000_u128,
			10000000000000000000000000000_u128,
			100000000000000000000000000000_u128,
			1000000000000000000000000000000_u128,
			10000000000000000000000000000000_u128,
			100000000000000000000000000000000_u128,
			1000000000000000000000000000000000_u128,
			10000000000000000000000000000000000_u128,
			100000000000000000000000000000000000_u128,
			1000000000000000000000000000000000000_u128,
			10000000000000000000000000000000000000_u128,
			100000000000000000000000000000000000000_u128,
		] {
			let fmt = format!("{i}");
			assert_eq!(Itoa::new(i), fmt);
		}
	}

	#[test]
	fn all_i128_lengths() {
		for i in [
			0_i128,
			1_i128,
			10_i128,
			100_i128,
			1000_i128,
			10000_i128,
			100000_i128,
			1000000_i128,
			10000000_i128,
			100000000_i128,
			1000000000_i128,
			10000000000_i128,
			100000000000_i128,
			1000000000000_i128,
			10000000000000_i128,
			100000000000000_i128,
			1000000000000000_i128,
			10000000000000000_i128,
			100000000000000000_i128,
			1000000000000000000_i128,
			10000000000000000000_i128,
			100000000000000000000_i128,
			1000000000000000000000_i128,
			10000000000000000000000_i128,
			100000000000000000000000_i128,
			1000000000000000000000000_i128,
			10000000000000000000000000_i128,
			100000000000000000000000000_i128,
			1000000000000000000000000000_i128,
			10000000000000000000000000000_i128,
			100000000000000000000000000000_i128,
			1000000000000000000000000000000_i128,
			10000000000000000000000000000000_i128,
			100000000000000000000000000000000_i128,
			1000000000000000000000000000000000_i128,
			10000000000000000000000000000000000_i128,
			100000000000000000000000000000000000_i128,
			1000000000000000000000000000000000000_i128,
			10000000000000000000000000000000000000_i128,
			100000000000000000000000000000000000000_i128,
			-1_i128,
			-10_i128,
			-100_i128,
			-1000_i128,
			-10000_i128,
			-100000_i128,
			-1000000_i128,
			-10000000_i128,
			-100000000_i128,
			-1000000000_i128,
			-10000000000_i128,
			-100000000000_i128,
			-1000000000000_i128,
			-10000000000000_i128,
			-100000000000000_i128,
			-1000000000000000_i128,
			-10000000000000000_i128,
			-100000000000000000_i128,
			-1000000000000000000_i128,
			-10000000000000000000_i128,
			-100000000000000000000_i128,
			-1000000000000000000000_i128,
			-10000000000000000000000_i128,
			-100000000000000000000000_i128,
			-1000000000000000000000000_i128,
			-10000000000000000000000000_i128,
			-100000000000000000000000000_i128,
			-1000000000000000000000000000_i128,
			-10000000000000000000000000000_i128,
			-100000000000000000000000000000_i128,
			-1000000000000000000000000000000_i128,
			-10000000000000000000000000000000_i128,
			-100000000000000000000000000000000_i128,
			-1000000000000000000000000000000000_i128,
			-10000000000000000000000000000000000_i128,
			-100000000000000000000000000000000000_i128,
			-1000000000000000000000000000000000000_i128,
			-10000000000000000000000000000000000000_i128,
			-100000000000000000000000000000000000000_i128,
		] {
			let fmt = format!("{i}");
			assert_eq!(Itoa::new(i), fmt);
		}
	}
}
