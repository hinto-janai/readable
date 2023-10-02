//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::macros::{
	impl_common,impl_const,
	impl_traits,return_bad_float,
	impl_usize,impl_math,impl_impl_math,
};

//---------------------------------------------------------------------------------------------------- Constants (Private)
const LEN: usize = 8;

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
/// If the input is larger than [`MAX_RUNTIME`], [`Self::unknown()`] is returned.
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
/// 1. `seconds` always has a leading `0`
/// 2. `minutes` only has a leading zero if `hours` isn't `0`
/// 3. `hours` never has a leading `0`
///
/// ## Copy
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Runtime(u32, Str<8>);

impl_math!(Runtime, u32);
impl_traits!(Runtime, u32);

impl Runtime {
	impl_common!(u32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::unknown() == 0);
	/// assert!(Runtime::unknown() == "?:??");
	/// ```
	pub const fn unknown() -> Self {
		Self(ZERO_RUNTIME_U32, Str::from_static_str(UNKNOWN_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::zero() == 0);
	/// assert!(Runtime::zero() == "0:00");
	/// ```
	pub const fn zero() -> Self {
		Self(ZERO_RUNTIME_U32, Str::from_static_str(ZERO_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::second() == 1);
	/// assert!(Runtime::second() == "0:01");
	/// assert!(Runtime::second() == Runtime::from(1.0));
	/// ```
	pub const fn second() -> Self {
		Self(SECOND_RUNTIME_U32, Str::from_static_str(SECOND_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::minute() == 60);
	/// assert!(Runtime::minute() == "1:00");
	/// assert!(Runtime::minute() == Runtime::from(60.0));
	/// ```
	pub const fn minute() -> Self {
		Self(MINUTE_RUNTIME_U32, Str::from_static_str(MINUTE_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::hour() == 3600);
	/// assert!(Runtime::hour() == "1:00:00");
	/// assert!(Runtime::hour() == Runtime::from(3600.0));
	/// ```
	pub const fn hour() -> Self {
		Self(HOUR_RUNTIME_U32, Str::from_static_str(HOUR_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert!(Runtime::max() == 359999);
	/// assert!(Runtime::max() == "99:59:59");
	/// assert!(Runtime::max() == Runtime::from(359999.0));
	/// ```
	pub const fn max() -> Self {
		Self(MAX_RUNTIME_U32, Str::from_static_str(MAX_RUNTIME))
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

		let mut buf = [0; LEN];

		// Format.
		let len = if hours > 0 {
			Self::format_hms(&mut buf, hours, minutes, seconds)
		} else {
			Self::format_ms(&mut buf, minutes, seconds)
		};

		Self(runtime as u32, unsafe { Str::from_raw(len as u8, buf) })
	}
}

//---------------------------------------------------------------------------------------------------- Duration/Instant
impl From<std::time::Duration> for Runtime {
	#[inline]
	fn from(runtime: std::time::Duration) -> Self {
		let f = runtime.as_secs_f64();
		Self::priv_from(f)
	}
}

impl From<&std::time::Duration> for Runtime {
	#[inline]
	fn from(runtime: &std::time::Duration) -> Self {
		let f = runtime.as_secs_f64();
		Self::priv_from(f)
	}
}

impl From<std::time::Instant> for Runtime {
	#[inline]
	fn from(runtime: std::time::Instant) -> Self {
		let f = runtime.elapsed().as_secs_f64();
		Self::priv_from(f)
	}
}

impl From<&std::time::Instant> for Runtime {
	#[inline]
	fn from(runtime: &std::time::Instant) -> Self {
		let f = runtime.elapsed().as_secs_f64();
		Self::priv_from(f)
	}
}

//---------------------------------------------------------------------------------------------------- Floats
macro_rules! impl_f {
	($($from:ty),*) => {
		$(
			impl From<$from> for Runtime {
				fn from(f: $from) -> Self {
					return_bad_float!(f, Self::unknown, Self::unknown);

					// Handle NaN/Inf.
					Self::priv_from(f as f64)
				}
			}
			impl From<&$from> for Runtime {
				fn from(f: &$from) -> Self {
					return_bad_float!(f, Self::unknown, Self::unknown);

					// Handle NaN/Inf.
					Self::priv_from(*f as f64)
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

//---------------------------------------------------------------------------------------------------- Private impl
impl Runtime {
	#[inline]
	// 0 Padding for `hh:mm:ss` according to `Runtime` rules.
	//
	// INVARIANT: Assumes `hour` is 1 or greater.
	fn format_hms(buf: &mut [u8; LEN], hour: u32, min: u32, sec: u32) -> usize {
		const Z: u8 = b'0';
		const C: u8 = b':';

		let mut h = crate::ItoaTmp::new();
		let mut m = crate::ItoaTmp::new();
		let mut s = crate::ItoaTmp::new();
		let h = h.format(hour).as_bytes();
		let m = m.format(min).as_bytes();
		let s = s.format(sec).as_bytes();

		match (h.len(), m.len(), s.len()) {
			// 0:0:0
			(1, 1, 1) => {
				buf[0] = h[0];
				buf[1] = C;
				buf[2] = Z;
				buf[3] = m[0];
				buf[4] = C;
				buf[5] = Z;
				buf[6] = s[0];
				7
			},
			// 0:00:0
			(1, 2, 1) => {
				buf[0] = h[0];
				buf[1] = C;
				buf[2] = m[0];
				buf[3] = m[1];
				buf[4] = C;
				buf[5] = Z;
				buf[6] = s[0];
				7
			},
			// 0:0:00
			(1, 1, 2) => {
				buf[0] = h[0];
				buf[1] = C;
				buf[2] = Z;
				buf[3] = m[0];
				buf[4] = C;
				buf[5] = s[0];
				buf[6] = s[1];
				7
			},
			// 0:00:00
			(1, 2, 2) => {
				buf[0] = h[0];
				buf[1] = C;
				buf[2] = m[0];
				buf[3] = m[1];
				buf[4] = C;
				buf[5] = s[0];
				buf[6] = s[1];
				7
			},
			// 00:0:0
			(2, 1, 1) => {
				buf[0] = h[0];
				buf[1] = h[1];
				buf[2] = C;
				buf[3] = Z;
				buf[4] = m[0];
				buf[5] = C;
				buf[6] = Z;
				buf[7] = s[0];
				8
			},
			// 00:00:0
			(2, 2, 1) => {
				buf[0] = h[0];
				buf[1] = h[1];
				buf[2] = C;
				buf[3] = m[0];
				buf[4] = m[1];
				buf[5] = C;
				buf[6] = Z;
				buf[7] = s[0];
				8
			},
			// 00:0:00
			(2, 1, 2) => {
				buf[0] = h[0];
				buf[1] = h[1];
				buf[2] = C;
				buf[3] = Z;
				buf[4] = m[0];
				buf[5] = C;
				buf[6] = s[0];
				buf[7] = s[1];
				8
			},
			// 00:00:00
			// (2, 2, 2)
			_ => {
				debug_assert_eq!((h.len(), m.len(), s.len()), (2, 2, 2));

				buf[0] = h[0];
				buf[1] = h[1];
				buf[2] = C;
				buf[3] = m[0];
				buf[4] = m[1];
				buf[5] = C;
				buf[6] = s[0];
				buf[7] = s[1];
				8
			},
		}
	}

	#[inline]
	// 0 Padding for `mm:ss` according to `Runtime` rules.
	fn format_ms(buf: &mut [u8; LEN], min: u32, sec: u32) -> usize {
		const Z: u8 = b'0';
		const C: u8 = b':';

		let mut m = crate::ItoaTmp::new();
		let mut s = crate::ItoaTmp::new();
		let m = m.format(min).as_bytes();
		let s = s.format(sec).as_bytes();

		match (m.len(), s.len()) {
			// 0:0
			(1, 1) => {
				buf[0] = m[0];
				buf[1] = C;
				buf[2] = Z;
				buf[3] = s[0];
				4
			},

			// 0:00
			(1, 2) => {
				buf[0] = m[0];
				buf[1] = C;
				buf[2] = s[0];
				buf[3] = s[1];
				4
			},

			// 00:0
			(2, 1) => {
				buf[0] = m[0];
				buf[1] = m[1];
				buf[2] = C;
				buf[3] = Z;
				buf[4] = s[0];
				5
			},

			// 00:00
			// (2, 2)
			_ => {
				debug_assert_eq!((m.len(), s.len()), (2, 2));

				buf[0] = m[0];
				buf[1] = m[1];
				buf[2] = C;
				buf[3] = s[0];
				buf[4] = s[1];
				5
			},
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _format_hms() {
		fn s(b: &[u8], l: usize) -> &str {
			std::str::from_utf8(&b[..l]).unwrap()
		}

		let mut buf = [0; LEN];
		let buf = &mut buf;

		// 0:0:0
		let len = Runtime::format_hms(buf, 1, 1, 1);
		assert_eq!(s(buf, len), "1:01:01");

		// 0:00:0
		let len = Runtime::format_hms(buf, 1, 10, 1);
		assert_eq!(s(buf, len), "1:10:01");

		// 0:0:00
		let len = Runtime::format_hms(buf, 1, 1, 10);
		assert_eq!(s(buf, len), "1:01:10");

		// 0:00:00
		let len = Runtime::format_hms(buf, 1, 10, 10);
		assert_eq!(s(buf, len), "1:10:10");

		// 00:0:0
		let len = Runtime::format_hms(buf, 10, 1, 1);
		assert_eq!(s(buf, len), "10:01:01");

		// 00:00:0
		let len = Runtime::format_hms(buf, 10, 10, 1);
		assert_eq!(s(buf, len), "10:10:01");

		// 00:0:00
		let len = Runtime::format_hms(buf, 10, 1, 10);
		assert_eq!(s(buf, len), "10:01:10");

		// 00:00:00
		let len = Runtime::format_hms(buf, 10, 10, 10);
		assert_eq!(s(buf, len), "10:10:10");
	}

	#[test]
	fn _format_ms() {
		fn s(b: &[u8], l: usize) -> &str {
			std::str::from_utf8(&b[..l]).unwrap()
		}

		let mut buf = [0; LEN];
		let buf = &mut buf;

		// 0:0
		let len = Runtime::format_ms(buf, 1, 1);
		assert_eq!(s(buf, len), "1:01");

		// 00:0
		let len = Runtime::format_ms(buf, 10, 1);
		assert_eq!(s(buf, len), "10:01");

		// 0:00
		let len = Runtime::format_ms(buf, 1, 10);
		assert_eq!(s(buf, len), "1:10");

		// 00:00
		let len = Runtime::format_ms(buf, 10, 10);
		assert_eq!(s(buf, len), "10:10");
	}

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
}
