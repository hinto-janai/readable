//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use compact_str::format_compact;
use crate::macros::*;

//---------------------------------------------------------------------------------------------------- Constants (Private)
/// UTF-8 byte encoding of [`UNKNOWN_RUNTIME`], aka: `?:??`
const UNKNOWN_RUNTIME_BUFFER: [u8; 8] = [63, 58, 63, 63, 0, 0, 0, 0];

/// UTF-8 byte encoding of [`ZERO_RUNTIME`]
const ZERO_RUNTIME_BUFFER: [u8; 8] = [48, 58, 48, 48, 0, 0, 0, 0];

/// UTF-8 byte encoding of [`SECOND_RUNTIME`]
const SECOND_RUNTIME_BUFFER: [u8; 8] = [48, 58, 48, 49, 0, 0, 0, 0];

/// UTF-8 byte encoding of [`MINUTE_RUNTIME`]
const MINUTE_RUNTIME_BUFFER: [u8; 8] = [49, 58, 48, 48, 0, 0, 0, 0];

/// UTF-8 byte encoding of [`HOUR_RUNTIME`]
const HOUR_RUNTIME_BUFFER: [u8; 8] = [49, 58, 48, 48, 58, 48, 48, 0];

/// UTF-8 byte encoding of [`MAX_RUNTIME`]
const MAX_RUNTIME_BUFFER: [u8; 8] = [57, 57, 58, 53, 57, 58, 53, 57];

//---------------------------------------------------------------------------------------------------- Constants (Public)
/// [`str`] returned when using [`Runtime::unknown`]
pub const UNKNOWN_RUNTIME: &str = "?:??";

/// [`str`] returned when using [`Runtime::zero`]
pub const ZERO_RUNTIME: &str = "0:00";

/// [`str`] returned when using [`Runtime::second`]
pub const SECOND_RUNTIME: &str = "0:01";

/// [`str`] returned when using [`Runtime::minute`]
pub const MINUTE_RUNTIME: &str = "1:00";

/// [`str`] returned when using [`Runtime::hour`]
pub const HOUR_RUNTIME: &str = "1:00:00";

/// [`str`] for the max time [`Runtime`] can handle
pub const MAX_RUNTIME: &str = "99:59:59";

/// [`u32`] returned when calling [`Runtime::zero`]
pub const ZERO_RUNTIME_U32: u32 = 0;

/// [`u32`] returned when calling [`Runtime::second`]
pub const SECOND_RUNTIME_U32: u32 = 1;

/// [`u32`] returned when calling [`Runtime::minute`]
pub const MINUTE_RUNTIME_U32: u32 = 60;

/// [`u32`] returned when calling [`Runtime::hour`]
pub const HOUR_RUNTIME_U32: u32 = 3600;

/// The max input to [`Runtime`] before it returns [`MAX_RUNTIME`]
pub const MAX_RUNTIME_U32: u32 = 359999;

//---------------------------------------------------------------------------------------------------- Runtime
/// Human readable "audio/video runtime" in `H:M:S` format.
///
/// [`Runtime::from`] input can be:
/// - [`u8`]
/// - [`u16`]
/// - [`u32`]
/// - [`u64`]
/// - [`usize`]
/// - [`f32`]
/// - [`f64`]
/// - [`std::time::Duration`].
/// - [`std::time::Instant`].
///
/// Integer inputs are presumed to be in _seconds._
///
/// ## Errors
/// The max input is `359999` seconds, or: `99:59:59`.
///
/// If the input is larger than [`MAX_RUNTIME`], [`|| Self::unknown()`] is returned.
///
/// ## Inlining
/// If the feature flag `inline_runtime` is enabled, ALL possible outputs of
/// [`Runtime`] are inlined as static bytes and any [`Runtime::from`] call will return
/// said static bytes.
///
/// **Warning:** This feature is disabled by default. While it increases speed,
/// it also _heavily_ increases build time and binary size.
///
/// ## Formatting rules
/// 1. `seconds` always has leading `0`.
/// 2. `minutes` only has a leading zero if `hours` isn't `0`.
/// 3. `hours` never has a leading `0`.
///
/// ## Cloning
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 8 byte array buffer, literally: `[u8; 8]`.
///
/// Since the max valid runtime is: `99:59:59` (8 characters, `359999` seconds), an 8 byte
/// buffer is used and enables this type to have [`Copy`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
/// ```rust
/// # use readable::Runtime;
/// let a = Runtime::from(100_000_u64);
///
/// // Copy 'a', use 'b'.
/// let b = a;
/// assert!(b == 100_000_u32);
///
/// // We can still use 'a'
/// assert!(a == 100_000_u32);
/// ```
///
/// ## Exceptions
/// - Inputting [`f64::NAN`], [`f64::INFINITY`], [`f64::NEG_INFINITY`] or the [`f32`] variants returns errors
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
/// - Combined with another [`Self`]: `Runtime::from(1) + Runtime::from(1)`
/// - Or with the inner number itself: `Runtime::from(1) + 1`
///
/// They also have the same `panic!()` behavior on overflow as the normal ones, because internally,
/// it is just calling `.inner() $OPERATOR $NUMBER`.
/// ```rust
/// # use readable::*;
/// assert_eq!(Runtime::from(10_u32) + 10, Runtime::from(20_u32));
/// assert_eq!(Runtime::from(10_u32) - 10, Runtime::from(0_u32));
/// assert_eq!(Runtime::from(10_u32) / 10, Runtime::from(1_u32));
/// assert_eq!(Runtime::from(10_u32) * 10, Runtime::from(100_u32));
/// assert_eq!(Runtime::from(10_u32) % 10, Runtime::from(0_u32));
/// ```
/// Overflow example (won't panic, will return unknown):
/// ```rust
/// # use readable::*;
/// let n = Runtime::from(u32::MAX) + u32::MAX;
/// assert!(n == Runtime::unknown());
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::Runtime;
/// // Always round down.
/// assert_eq!(Runtime::from(11.1111), "0:11");
/// assert_eq!(Runtime::from(11.9999), "0:11");
///
/// assert_eq!(Runtime::from(111.111), "1:51");
/// assert_eq!(Runtime::from(111.999), "1:51");
///
/// assert_eq!(Runtime::from(11111.1), "3:05:11");
/// assert_eq!(Runtime::from(11111.9), "3:05:11");
///
/// assert_eq!(Runtime::from(0.0), "0:00");
/// assert_eq!(Runtime::from(1.0), "0:01");
/// assert_eq!(Runtime::from(1.9), "0:01");
/// assert_eq!(Runtime::from(2.0), "0:02");
///
/// assert_eq!(Runtime::from(f32::NAN),      "?:??");
/// assert_eq!(Runtime::from(f64::INFINITY), "?:??");
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Runtime(u32, Buffer);

impl_math!(Runtime, u32);
impl_traits!(Runtime, u32);

impl Runtime {
	impl_common!(u32);
	impl_const!();
	impl_usize!();
	impl_buffer!(MAX_BUF_LEN, UNKNOWN_RUNTIME_BUFFER, Self::unknown().len());

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::unknown() == 0);
	/// assert!(Runtime::unknown() == "?:??");
	/// ```
	pub const fn unknown() -> Self {
		Self(ZERO_RUNTIME_U32, Buffer::unknown())
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::zero() == 0);
	/// assert!(Runtime::zero() == "0:00");
	/// ```
	pub const fn zero() -> Self {
		Self(ZERO_RUNTIME_U32, Buffer::zero())
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::second() == 1);
	/// assert!(Runtime::second() == "0:01");
	/// assert!(Runtime::second() == Runtime::from(1.0));
	/// ```
	pub const fn second() -> Self {
		Self(SECOND_RUNTIME_U32, Buffer::second())
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::minute() == 60);
	/// assert!(Runtime::minute() == "1:00");
	/// assert!(Runtime::minute() == Runtime::from(60.0));
	/// ```
	pub const fn minute() -> Self {
		Self(MINUTE_RUNTIME_U32, Buffer::minute())
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::hour() == 3600);
	/// assert!(Runtime::hour() == "1:00:00");
	/// assert!(Runtime::hour() == Runtime::from(3600.0));
	/// ```
	pub const fn hour() -> Self {
		Self(HOUR_RUNTIME_U32, Buffer::hour())
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::max() == 359999);
	/// assert!(Runtime::max() == "99:59:59");
	/// assert!(Runtime::max() == Runtime::from(359999.0));
	/// ```
	pub const fn max() -> Self {
		Self(MAX_RUNTIME_U32, Buffer::max())
	}

	#[allow(unreachable_code)]
	// Private function used in float `From`.
	//
	// INVARIANT:
	// `handle_float!()` should be
	// called before this function.
	fn priv_from(runtime: f64) -> Self {
		#[cfg(feature = "inline_runtime")]
		{
			let runtime = runtime as u32;
			return Self(runtime, Buffer::from_unchecked(readable_inlined_runtime::inlined(runtime)));
		}

		// Zero length.
		if runtime <= 0.0 {
			return Self::zero();
		}

		// Return unknown if over max.
		if runtime > MAX_RUNTIME_U32 as f64 {
			return Self::unknown();
		}

		// Cast to `u32` (rounds down implicitly).
	    let seconds = (runtime % 60.0) as u32;
	    let minutes = ((runtime / 60.0) % 60.0) as u32;
	    let hours   = ((runtime / 60.0) / 60.0) as u32;

		// Format.
		let string = if hours > 0 {
			format_compact!("{}:{:0>2}:{:0>2}", hours, minutes, seconds)
		} else {
			format_compact!("{}:{:0>2}", minutes, seconds)
		};

		Self(runtime as u32, Buffer::from_unchecked(string.as_bytes()))
	}
}

//---------------------------------------------------------------------------------------------------- Duration/Instant
impl From<std::time::Duration> for Runtime {
	#[inline]
	fn from(runtime: std::time::Duration) -> Self {
		let f = runtime.as_secs_f64();
		handle_float!(|| Self::unknown(), f);
		Self::priv_from(f)
	}
}

impl From<&std::time::Duration> for Runtime {
	#[inline]
	fn from(runtime: &std::time::Duration) -> Self {
		let f = runtime.as_secs_f64();
		handle_float!(|| Self::unknown(), f);
		Self::priv_from(f)
	}
}

impl From<std::time::Instant> for Runtime {
	#[inline]
	fn from(runtime: std::time::Instant) -> Self {
		let f = runtime.elapsed().as_secs_f64();
		handle_float!(|| Self::unknown(), f);
		Self::priv_from(f)
	}
}

impl From<&std::time::Instant> for Runtime {
	#[inline]
	fn from(runtime: &std::time::Instant) -> Self {
		let f = runtime.elapsed().as_secs_f64();
		handle_float!(|| Self::unknown(), f);
		Self::priv_from(f)
	}
}

//---------------------------------------------------------------------------------------------------- Floats
macro_rules! impl_f {
	($($from:ty),*) => {
		$(
			impl From<$from> for Runtime {
				fn from(float: $from) -> Self {
					#[cfg(not(feature = "ignore_nan_inf"))]
					{
						match float.classify() {
							std::num::FpCategory::Normal   => (),
							std::num::FpCategory::Nan      => return Self::unknown(),
							std::num::FpCategory::Infinite => return Self::unknown(),
							_ => (),
						}
					}

					// Handle NaN/Inf.
					Self::priv_from(float as f64)
				}
			}
			impl From<&$from> for Runtime {
				fn from(float: &$from) -> Self {
					#[cfg(not(feature = "ignore_nan_inf"))]
					{
						match float.classify() {
							std::num::FpCategory::Normal   => (),
							std::num::FpCategory::Nan      => return Self::unknown(),
							std::num::FpCategory::Infinite => return Self::unknown(),
							_ => (),
						}
					}

					// Handle NaN/Inf.
					Self::priv_from(*float as f64)
				}
			}
		)*
	}
}
impl_f!(f32, f64);

//---------------------------------------------------------------------------------------------------- Int
macro_rules! impl_int {
	($from:ty) => {
		impl From<$from> for Runtime {
			fn from(runtime: $from) -> Self {
				Self::priv_from(runtime as f64)
			}
		}
	}
}
impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(usize);

//---------------------------------------------------------------------------------------------------- Buffer
// "99:59:59".len() == 8
const MAX_BUF_LEN: usize = 8;

buffer!(MAX_BUF_LEN, UNKNOWN_RUNTIME_BUFFER, UNKNOWN_RUNTIME.len());

impl Buffer {
	#[inline(always)]
	const fn zero() -> Self {
		Self {
			buf: ZERO_RUNTIME_BUFFER,
			len: 4,
		}
	}

	#[inline(always)]
	const fn second() -> Self {
		Self {
			buf: SECOND_RUNTIME_BUFFER,
			len: 4,
		}
	}

	#[inline(always)]
	const fn minute() -> Self {
		Self {
			buf: MINUTE_RUNTIME_BUFFER,
			len: 4,
		}
	}

	#[inline(always)]
	const fn hour() -> Self {
		Self {
			buf: HOUR_RUNTIME_BUFFER,
			len: 7,
		}
	}

	#[inline(always)]
	const fn max() -> Self {
		Self {
			buf: MAX_RUNTIME_BUFFER,
			len: MAX_BUF_LEN,
		}
	}

	#[inline]
	// INVARIANT:
	// Assumes input is `1-8` bytes.
	fn from_unchecked(byte: &[u8]) -> Self {
		let len = byte.len();

		let mut buf = [0_u8; 8];
		buf[..len].copy_from_slice(&byte[..len]);

		Self {
			buf,
			len,
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn all_uint() {
		for i in 0..MAX_RUNTIME_U32 {
			let rt = Runtime::from(i);
			println!("rt:{} - i: {}", rt, i);
			assert_eq!(rt, i);
			assert_eq!(rt, i);
			println!("{}", rt);
		}
	}

	#[test]
	fn all_floats() {
		let mut f = 1.0;
		while (f as u32) < MAX_RUNTIME_U32 {
			let rt = Runtime::from(f);
			println!("rt: {} - f: {}", rt, f);
			assert_eq!(rt, f as u32);
			f += 0.1;
		}
	}

	#[test]
	fn overflow_float() {
		assert_eq!(Runtime::from(MAX_RUNTIME_U32 as f64 + 1.0), 0);
		assert_eq!(Runtime::from(MAX_RUNTIME_U32 as f64 + 1.0), Runtime::unknown());
	}

	#[test]
	fn overflow_uint() {
		assert_eq!(Runtime::from(MAX_RUNTIME_U32 + 1), 0);
		assert_eq!(Runtime::from(MAX_RUNTIME_U32 + 1), Runtime::unknown());
	}

	#[test]
	fn special() {
		assert_eq!(Runtime::from(f32::NAN),          Runtime::unknown());
		assert_eq!(Runtime::from(f32::INFINITY),     Runtime::unknown());
		assert_eq!(Runtime::from(f32::NEG_INFINITY), Runtime::unknown());
		assert_eq!(Runtime::from(f64::NAN),          Runtime::unknown());
		assert_eq!(Runtime::from(f64::INFINITY),     Runtime::unknown());
		assert_eq!(Runtime::from(f64::NEG_INFINITY), Runtime::unknown());
	}

	#[test]
	fn unknown_runtime_buffer() {
		assert!(UNKNOWN_RUNTIME.as_bytes()[..4] == UNKNOWN_RUNTIME_BUFFER[..4]);
	}

	#[test]
	fn zero_runtime_buffer() {
		assert!(ZERO_RUNTIME.as_bytes()[..3] == ZERO_RUNTIME_BUFFER[..3]);
	}


	#[test]
	fn second_runtime_buffer() {
		assert!(SECOND_RUNTIME.as_bytes()[..4] == SECOND_RUNTIME_BUFFER[..4]);
	}

	#[test]
	fn minute_runtime_buffer() {
		assert!(MINUTE_RUNTIME.as_bytes()[..4] == MINUTE_RUNTIME_BUFFER[..4]);
	}

	#[test]
	fn hour_runtime_buffer() {
		assert!(HOUR_RUNTIME.as_bytes()[..7] == HOUR_RUNTIME_BUFFER[..7]);
	}

	#[test]
	fn max_runtime_buffer() {
		assert!(MAX_RUNTIME.as_bytes() == MAX_RUNTIME_BUFFER);
	}
}
