//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::macros::{
	impl_common,impl_const,
	impl_traits,return_bad_float,
	impl_usize,impl_math,impl_impl_math,
};
use crate::time::{
	ZERO_RUNTIME_U32,
	SECOND_RUNTIME_U32,
	MINUTE_RUNTIME_U32,
	HOUR_RUNTIME_U32,
	MAX_RUNTIME_U32,
};

//---------------------------------------------------------------------------------------------------- Constants (Private)
const LEN: usize = 8;

//---------------------------------------------------------------------------------------------------- Constants (Public)
/// [`str`] returned when using [`RuntimeFull::unknown`]
pub const UNKNOWN_RUNTIME_FULL: &str = "??:??:??";

/// [`str`] returned when using [`RuntimeFull::zero`]
pub const ZERO_RUNTIME_FULL: &str = "00:00:00";

/// [`str`] returned when using [`RuntimeFull::second`]
pub const SECOND_RUNTIME_FULL: &str = "00:00:01";

/// [`str`] returned when using [`RuntimeFull::minute`]
pub const MINUTE_RUNTIME_FULL: &str = "00:01:00";

/// [`str`] returned when using [`RuntimeFull::hour`]
pub const HOUR_RUNTIME_FULL: &str = "01:00:00";

/// [`str`] for the max time [`RuntimeFull`] can handle
pub const MAX_RUNTIME_FULL: &str = "99:59:59";

//---------------------------------------------------------------------------------------------------- RuntimeFull
/// [`Runtime`], but always full length and pre-padded with zeros
///
/// This is the exact same type as [`Runtime`], except, the
/// numbers will _always_ be padding with `0`'s.
///
/// ```rust
/// # use readable::time::*;
/// let runtime = Runtime::minute();
/// assert_eq!(runtime, "1:00"); // hour left out, minute not padded
///
/// let runtime_zero = RuntimeFull::minute();
/// assert_eq!(runtime_zero, "00:01:00"); // always includes everything
///
/// let runtime = Runtime::hour();
/// assert_eq!(runtime, "1:00:00");
///
/// let runtime_zero = RuntimeFull::hour();
/// assert_eq!(runtime_zero, "01:00:00");
/// ```
///
/// [`RuntimeFull::from`] input can be:
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
/// If the input is larger than [`MAX_RUNTIME_FULL`], [`Self::unknown()`] is returned.
///
/// ## Inlining
/// If the feature flag `inline_runtime` is enabled, ALL possible outputs of
/// [`RuntimeFull`] are inlined as static bytes and any [`RuntimeFull::from`] call will return
/// said static bytes.
///
/// **Warning:** This feature is disabled by default. While it increases speed,
/// it also _heavily_ increases build time and binary size.
///
/// ## Copy
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 8 byte array buffer, literally: [`Str<8>`].
///
/// Since the max valid runtime is: `99:59:59` (8 characters, `359999` seconds), an 8 byte
/// buffer is used and enables this type to have [`Copy`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
/// ```rust
/// # use readable::RuntimeFull;
/// let a = RuntimeFull::from(100_000_u64);
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
/// Inputting [`f64::NAN`], [`f64::INFINITY`], [`f64::NEG_INFINITY`] or the [`f32`] variants returns errors
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
/// - Combined with another [`Self`]: `RuntimeFull::from(1) + RuntimeFull::from(1)`
/// - Or with the inner number itself: `RuntimeFull::from(1) + 1`
///
/// ```rust
/// # use readable::*;
/// let n = RuntimeFull::from(u32::MAX) + u32::MAX;
/// assert!(n == RuntimeFull::unknown());
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::RuntimeFull;
/// // Always round down.
/// assert_eq!(RuntimeFull::from(11.1111), "00:00:11");
/// assert_eq!(RuntimeFull::from(11.9999), "00:00:11");
///
/// assert_eq!(RuntimeFull::from(111.111), "00:01:51");
/// assert_eq!(RuntimeFull::from(111.999), "00:01:51");
///
/// assert_eq!(RuntimeFull::from(11111.1), "03:05:11");
/// assert_eq!(RuntimeFull::from(11111.9), "03:05:11");
///
/// assert_eq!(RuntimeFull::from(0.0), "00:00:00");
/// assert_eq!(RuntimeFull::from(1.0), "00:00:01");
/// assert_eq!(RuntimeFull::from(1.9), "00:00:01");
/// assert_eq!(RuntimeFull::from(2.0), "00:00:02");
///
/// assert_eq!(RuntimeFull::from(f32::NAN),      "??:??:??");
/// assert_eq!(RuntimeFull::from(f64::INFINITY), "??:??:??");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RuntimeFull(u32, Str<8>);

impl_math!(RuntimeFull, u32);
impl_traits!(RuntimeFull, u32);

impl RuntimeFull {
	impl_common!(u32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// ```rust
	/// # use readable::RuntimeFull;
	/// assert!(RuntimeFull::unknown() == 0);
	/// assert!(RuntimeFull::unknown() == "??:??:??");
	/// ```
	pub const fn unknown() -> Self {
		Self(ZERO_RUNTIME_U32, Str::from_static_str(UNKNOWN_RUNTIME_FULL))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeFull;
	/// assert!(RuntimeFull::zero() == 0);
	/// assert!(RuntimeFull::zero() == "00:00:00");
	/// ```
	pub const fn zero() -> Self {
		Self(ZERO_RUNTIME_U32, Str::from_static_str(ZERO_RUNTIME_FULL))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeFull;
	/// assert!(RuntimeFull::second() == 1);
	/// assert!(RuntimeFull::second() == "00:00:01");
	/// assert!(RuntimeFull::second() == RuntimeFull::from(1.0));
	/// ```
	pub const fn second() -> Self {
		Self(SECOND_RUNTIME_U32, Str::from_static_str(SECOND_RUNTIME_FULL))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeFull;
	/// assert!(RuntimeFull::minute() == 60);
	/// assert!(RuntimeFull::minute() == "00:01:00");
	/// assert!(RuntimeFull::minute() == RuntimeFull::from(60.0));
	/// ```
	pub const fn minute() -> Self {
		Self(MINUTE_RUNTIME_U32, Str::from_static_str(MINUTE_RUNTIME_FULL))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeFull;
	/// assert!(RuntimeFull::hour() == 3600);
	/// assert!(RuntimeFull::hour() == "01:00:00");
	/// assert!(RuntimeFull::hour() == RuntimeFull::from(3600.0));
	/// ```
	pub const fn hour() -> Self {
		Self(HOUR_RUNTIME_U32, Str::from_static_str(HOUR_RUNTIME_FULL))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeFull;
	/// assert!(RuntimeFull::max() == 359999);
	/// assert!(RuntimeFull::max() == "99:59:59");
	/// assert!(RuntimeFull::max() == RuntimeFull::from(359999.0));
	/// ```
	pub const fn max() -> Self {
		Self(MAX_RUNTIME_U32, Str::from_static_str(MAX_RUNTIME_FULL))
	}

	#[allow(unreachable_code)]
	// Private function used in float `From`.
	//
	// INVARIANT:
	// `handle_float!()` should be
	// called before this function.
	fn priv_from(runtime: f64) -> Self {
		#[cfg(feature = "inline_runtime_full")]
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
		let mut buf = [0; LEN];
		Self::format(&mut buf, hours, minutes, seconds);

		Self(runtime as u32, unsafe { Str::from_raw(LEN as u8, buf) })
	}
}

//---------------------------------------------------------------------------------------------------- Duration/Instant
impl From<std::time::Duration> for RuntimeFull {
	#[inline]
	fn from(runtime: std::time::Duration) -> Self {
		let f = runtime.as_secs_f64();
		Self::priv_from(f)
	}
}

impl From<&std::time::Duration> for RuntimeFull {
	#[inline]
	fn from(runtime: &std::time::Duration) -> Self {
		let f = runtime.as_secs_f64();
		Self::priv_from(f)
	}
}

impl From<std::time::Instant> for RuntimeFull {
	#[inline]
	fn from(runtime: std::time::Instant) -> Self {
		let f = runtime.elapsed().as_secs_f64();
		Self::priv_from(f)
	}
}

impl From<&std::time::Instant> for RuntimeFull {
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
			impl From<$from> for RuntimeFull {
				fn from(f: $from) -> Self {
					return_bad_float!(f, Self::unknown, Self::unknown);

					// Handle NaN/Inf.
					Self::priv_from(f as f64)
				}
			}
			impl From<&$from> for RuntimeFull {
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
		impl From<$from> for RuntimeFull {
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
impl RuntimeFull {
	#[inline]
	// 0 Padding for `hh:mm:ss` according to `RuntimeFull` rules.
	fn format(buf: &mut [u8; LEN], hour: u32, min: u32, sec: u32) {
		debug_assert!(hour < 100);
		debug_assert!(min < 60);
		debug_assert!(sec < 60);

		const Z: u8 = b'0';
		const C: u8 = b':';
		// Colons are always in the same position.
		buf[2] = C;
		buf[5] = C;

		let mut h = crate::ItoaTmp::new();
		let mut m = crate::ItoaTmp::new();
		let mut s = crate::ItoaTmp::new();
		let h = h.format(hour).as_bytes();
		let m = m.format(min).as_bytes();
		let s = s.format(sec).as_bytes();

		if h.len() == 1 {
			buf[0] = Z;
			buf[1] = h[0];
		} else {
			buf[0] = h[0];
			buf[1] = h[1];
		}

		if m.len() == 1 {
			buf[3] = Z;
			buf[4] = m[0];
		} else {
			buf[3] = m[0];
			buf[4] = m[1];
		}

		if s.len() == 1 {
			buf[6] = Z;
			buf[7] = s[0];
		} else {
			buf[6] = s[0];
			buf[7] = s[1];
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _format_hms() {
		fn s(b: &[u8]) -> &str {
			std::str::from_utf8(&b).unwrap()
		}

		let mut buf = [0; LEN];
		let buf = &mut buf;

		// 0:0:0
		RuntimeFull::format(buf, 1, 1, 1);
		assert_eq!(s(buf), "01:01:01");

		// 0:00:0
		RuntimeFull::format(buf, 1, 10, 1);
		assert_eq!(s(buf), "01:10:01");

		// 0:0:00
		RuntimeFull::format(buf, 1, 1, 10);
		assert_eq!(s(buf), "01:01:10");

		// 0:00:00
		RuntimeFull::format(buf, 1, 10, 10);
		assert_eq!(s(buf), "01:10:10");

		// 00:0:0
		RuntimeFull::format(buf, 10, 1, 1);
		assert_eq!(s(buf), "10:01:01");

		// 00:00:0
		RuntimeFull::format(buf, 10, 10, 1);
		assert_eq!(s(buf), "10:10:01");

		// 00:0:00
		RuntimeFull::format(buf, 10, 1, 10);
		assert_eq!(s(buf), "10:01:10");

		// 00:00:00
		RuntimeFull::format(buf, 10, 10, 10);
		assert_eq!(s(buf), "10:10:10");

		// 0:0
		RuntimeFull::format(buf, 0, 1, 1);
		assert_eq!(s(buf), "00:01:01");

		// 00:0
		RuntimeFull::format(buf, 0, 10, 1);
		assert_eq!(s(buf), "00:10:01");

		// 0:00
		RuntimeFull::format(buf, 0, 1, 10);
		assert_eq!(s(buf), "00:01:10");

		// 00:00
		RuntimeFull::format(buf, 0, 10, 10);
		assert_eq!(s(buf), "00:10:10");
	}

	#[test]
	fn all_uint() {
		for i in 0..MAX_RUNTIME_U32 {
			let rt = RuntimeFull::from(i);
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
			let rt = RuntimeFull::from(f);
			println!("rt: {} - f: {}", rt, f);
			assert_eq!(rt, f as u32);
			f += 0.1;
		}
	}

	#[test]
	fn overflow_float() {
		assert_eq!(RuntimeFull::from(MAX_RUNTIME_U32 as f64 + 1.0), 0);
		assert_eq!(RuntimeFull::from(MAX_RUNTIME_U32 as f64 + 1.0), RuntimeFull::unknown());
	}

	#[test]
	fn overflow_uint() {
		assert_eq!(RuntimeFull::from(MAX_RUNTIME_U32 + 1), 0);
		assert_eq!(RuntimeFull::from(MAX_RUNTIME_U32 + 1), RuntimeFull::unknown());
	}

	#[test]
	fn special() {
		assert_eq!(RuntimeFull::from(f32::NAN),          RuntimeFull::unknown());
		assert_eq!(RuntimeFull::from(f32::INFINITY),     RuntimeFull::unknown());
		assert_eq!(RuntimeFull::from(f32::NEG_INFINITY), RuntimeFull::unknown());
		assert_eq!(RuntimeFull::from(f64::NAN),          RuntimeFull::unknown());
		assert_eq!(RuntimeFull::from(f64::INFINITY),     RuntimeFull::unknown());
		assert_eq!(RuntimeFull::from(f64::NEG_INFINITY), RuntimeFull::unknown());
	}
}
