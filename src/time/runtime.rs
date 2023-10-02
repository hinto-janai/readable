//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::time::{RuntimePad,RuntimeMilli,RuntimeUnion};
use crate::macros::{
	impl_common,impl_const,
	impl_traits,return_bad_float,
	impl_usize,impl_math,impl_impl_math,
};

//---------------------------------------------------------------------------------------------------- Constants (Public)
/// The max length of [`Runtime`]'s and [`RuntimePad`]'s string.
pub const MAX_LEN_RUNTIME: usize = 8;

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

/// [`f32`] returned when calling [`Runtime::zero`]
pub const ZERO_RUNTIME_F32: f32 = 0.0;

/// [`f32`] returned when calling [`Runtime::second`]
pub const SECOND_RUNTIME_F32: f32 = 1.0;

/// [`f32`] returned when calling [`Runtime::minute`]
pub const MINUTE_RUNTIME_F32: f32 = 60.0;

/// [`f32`] returned when calling [`Runtime::hour`]
pub const HOUR_RUNTIME_F32: f32 = 3600.0;

/// Input greater to [`Runtime`] will make it return [`MAX_RUNTIME`]
pub const MAX_RUNTIME_F32: f32 = 359999.0;

//---------------------------------------------------------------------------------------------------- Runtime
/// Human readable "audio/video runtime" in `H:M:S` format.
///
/// [`Runtime::from`] input can be:
/// - [`u8`]
/// - [`u16`]
/// - [`u32`]
/// - [`u64`]
/// - [`u128`]
/// - [`usize`]
/// - [`f32`]
/// - [`f64`]
/// - [`std::time::Duration`]
/// - [`std::time::Instant`]
/// - Other [`Runtime`] types
///
/// Integer inputs are presumed to be in _seconds._
///
/// ## From other [`Runtime`] types
/// All [`Runtime`] types support lossless conversion with each other using [`From`].
///
/// For example, the millisecond data will not be lost even if you
/// go from [`RuntimeMilli`] -> [`Runtime`] -> [`RuntimeMilli`]
///
/// ```rust
/// # use readable::*;
/// // Millisecond data.
/// let milli = RuntimeMilli::from(1.555);
/// assert_eq!(milli, "00:00:01.555");
///
/// // Convert to `Runtime`.
/// let runtime = Runtime::from(milli);
/// assert_eq!(runtime, "0:01");
///
/// // Convert to `RuntimePad`.
/// let full = RuntimePad::from(runtime);
/// assert_eq!(full, "00:00:01");
///
/// // Convert back losslessly to [`RuntimeMilli`].
/// let milli2 = RuntimeMilli::from(full);
/// assert_eq!(milli2, "00:00:01.555");
/// assert_eq!(milli, milli2);
/// assert_eq!(milli2.inner(), 1.555);
/// ```
///
/// This is because the inner [`f32`] stored is simply copied,
/// only the formatted string is different.
///
/// Consider using the more efficient [`RuntimeUnion`] if you need to switch between formats often.
///
/// ## Errors
/// The max input is `359999` seconds, or: anything over `99:59:59`.
///
/// If the input is larger than [`MAX_RUNTIME`], [`Self::unknown()`] is returned.
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
/// but a 8 byte array buffer, literally: [`Str<8>`].
///
/// Since the max valid runtime is: `99:59:59` (8 characters, `360000` seconds), an 8 byte
/// buffer is used and enables this type to have [`Copy`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
/// ```rust
/// # use readable::Runtime;
/// let a = Runtime::from(100_000.0);
///
/// // Copy 'a', use 'b'.
/// let b = a;
/// assert_eq!(b, 100_000.0);
///
/// // We can still use 'a'
/// assert_eq!(a, 100_000.0);
/// ```
///
/// ## Size
/// ```rust
/// # use readable::time::*;
/// assert_eq!(std::mem::size_of::<Runtime>(), 16);
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
/// - Combined with another [`Self`]: `Runtime::from(1) + Runtime::from(1)`
/// - Or with the inner number itself: `Runtime::from(1) + 1`
///
/// ```rust
/// # use readable::*;
/// let n = Runtime::from(u32::MAX) + f32::MAX;
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
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Runtime(pub(super) f32, pub(super) Str<MAX_LEN_RUNTIME>);

impl_runtime! { // This macro is defined below.
	self  = Runtime,
	len   = MAX_LEN_RUNTIME,
	union = as_str,

	other = RuntimePad,
	other = RuntimeMilli,
}
impl_math!(Runtime, f32);
impl_traits!(Runtime, f32);

impl Runtime {
	impl_common!(f32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert_eq!(Runtime::unknown(), 0.0);
	/// assert_eq!(Runtime::unknown(), "?:??");
	/// ```
	pub const fn unknown() -> Self {
		Self(ZERO_RUNTIME_F32, Str::from_static_str(UNKNOWN_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert_eq!(Runtime::zero(), 0.0);
	/// assert_eq!(Runtime::zero(), "0:00");
	/// ```
	pub const fn zero() -> Self {
		Self(ZERO_RUNTIME_F32, Str::from_static_str(ZERO_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert_eq!(Runtime::second(), 1.0);
	/// assert_eq!(Runtime::second(), "0:01");
	/// assert_eq!(Runtime::second(), Runtime::from(1.0));
	/// ```
	pub const fn second() -> Self {
		Self(SECOND_RUNTIME_F32, Str::from_static_str(SECOND_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert_eq!(Runtime::minute(), 60.0);
	/// assert_eq!(Runtime::minute(), "1:00");
	/// assert_eq!(Runtime::minute(), Runtime::from(60.0));
	/// ```
	pub const fn minute() -> Self {
		Self(MINUTE_RUNTIME_F32, Str::from_static_str(MINUTE_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert_eq!(Runtime::hour(), 3600.0);
	/// assert_eq!(Runtime::hour(), "1:00:00");
	/// assert_eq!(Runtime::hour(), Runtime::from(3600.0));
	/// ```
	pub const fn hour() -> Self {
		Self(HOUR_RUNTIME_F32, Str::from_static_str(HOUR_RUNTIME))
	}

	#[inline]
	/// ```rust
	/// # use readable::Runtime;
	/// assert_eq!(Runtime::max(), 359999.0);
	/// assert_eq!(Runtime::max(), "99:59:59");
	/// assert_eq!(Runtime::max(), Runtime::from(359999.0));
	/// ```
	pub const fn max() -> Self {
		Self(MAX_RUNTIME_F32, Str::from_static_str(MAX_RUNTIME))
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl Runtime {
	#[allow(unreachable_code)]
	#[inline]
	// Private function used in float `From`.
	//
	// INVARIANT:
	// `handle_float!()` should be
	// called before this function.
	pub(super) fn priv_from(runtime: f32) -> Self {
		let Some((h, m, s)) = Self::priv_from_inner(runtime) else {
			return Self::unknown();
		};

		if (h, m, s) == (0.0, 0.0, 0.0) {
			return Self::zero();
		}

		let (hours, minutes, seconds) = (h as u8, m as u8, s as u8);
		let mut buf = [0; MAX_LEN_RUNTIME];

		// Format.
		let len = if hours > 0 {
			Self::format_hms(&mut buf, hours, minutes, seconds)
		} else {
			Self::format_ms(&mut buf, minutes, seconds)
		};

		Self(runtime, unsafe { Str::from_raw(len as u8, buf) })
	}

	#[inline]
	pub(super) fn priv_from_inner(runtime: f32) -> Option<(f32, f32, f32)> {
		// Zero length.
		if runtime <= 0.0 {
			return Some((0.0, 0.0, 0.0));
		}

		// Return unknown if over max.
		if runtime > MAX_RUNTIME_F32 {
			return None;
		}

		let (hours, minutes, seconds) = if runtime < 60.0 {
			(0.0, 0.0, runtime)
		} else if runtime < 3600.0 {
			(0.0, runtime / 60.0, runtime % 60.0)
		} else {
			let hours   = runtime / 3600.0;
			let minutes = (runtime % 3600.0) / 60.0;
			let seconds = runtime % 60.0;
			(hours, minutes, seconds)
		};

		let hours = if hours >= 100.0 {
			99.0
		} else {
			hours
		};

		// println!("inner h: {}, m: {}, s: {}", hours, minutes, seconds);

		Some((hours, minutes, seconds))
	}

	#[inline]
	// 0 Padding for `hh:mm:ss` according to `Runtime` rules.
	//
	// INVARIANT: Assumes `hour` is 1 or greater.
	fn format_hms(buf: &mut [u8; MAX_LEN_RUNTIME], hour: u8, min: u8, sec: u8) -> usize {
		debug_assert!(hour >= 1);
		debug_assert!(hour < 100);
		debug_assert!(min < 60);
		debug_assert!(sec < 60);

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
	fn format_ms(buf: &mut [u8; MAX_LEN_RUNTIME], min: u8, sec: u8) -> usize {
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


//---------------------------------------------------------------------------------------------------- Runtime* Impl Macro
// This is a macro for implementing across all `Runtime`-like types.
macro_rules! impl_runtime {
	(
		self  = $self:ty,
		$(
			len   = $max_len:expr,
			union = $str_function:ident,
		)?
		$(
			other = $other:ty
		),* $(,)?
	) => {
		//---------------------------------------------------------------------------------------------------- Duration
		impl From<std::time::Duration> for $self {
			#[inline]
			fn from(runtime: std::time::Duration) -> Self {
				let f = runtime.as_secs_f32();
				Self::priv_from(f)
			}
		}

		impl From<&std::time::Duration> for $self {
			#[inline]
			fn from(runtime: &std::time::Duration) -> Self {
				let f = runtime.as_secs_f32();
				Self::priv_from(f)
			}
		}

		//---------------------------------------------------------------------------------------------------- Instant
		impl From<std::time::Instant> for $self {
			#[inline]
			fn from(runtime: std::time::Instant) -> Self {
				let f = runtime.elapsed().as_secs_f32();
				Self::priv_from(f)
			}
		}

		impl From<&std::time::Instant> for $self {
			#[inline]
			fn from(runtime: &std::time::Instant) -> Self {
				let f = runtime.elapsed().as_secs_f32();
				Self::priv_from(f)
			}
		}

		//---------------------------------------------------------------------------------------------------- From `Runtime`
		$(
			impl From<$other> for $self {
				#[inline]
				fn from(runtime: $other) -> Self {
					Self::priv_from(runtime.inner())
				}
			}
			impl From<&$other> for $self {
				#[inline]
				fn from(runtime: &$other) -> Self {
					Self::priv_from(runtime.inner())
				}
			}
		)*

		//---------------------------------------------------------------------------------------------------- Floats
		macro_rules! impl_f {
			($from:ty) => {
				impl From<$from> for $self {
					fn from(f: $from) -> Self {
						$crate::macros::return_bad_float!(f, Self::unknown, Self::unknown);

						Self::priv_from(f as f32)
					}
				}
				impl From<&$from> for $self {
					fn from(f: &$from) -> Self {
						$crate::macros::return_bad_float!(f, Self::unknown, Self::unknown);

						Self::priv_from(*f as f32)
					}
				}
			}
		}
		impl_f!(f32);
		impl_f!(f64);

		//---------------------------------------------------------------------------------------------------- Int
		macro_rules! impl_int {
			($from:ty) => {
				impl From<$from> for $self {
					fn from(runtime: $from) -> Self {
						Self::priv_from(runtime as f32)
					}
				}
			}
		}
		impl_int!(u8);
		impl_int!(u16);
		impl_int!(u32);
		impl_int!(u64);
		impl_int!(u128);
		impl_int!(usize);

		//---------------------------------------------------------------------------------------------------- From `RuntimeUnion`
		$(
			impl From<RuntimeUnion> for $self {
				#[inline]
				fn from(runtime: RuntimeUnion) -> Self {
					Self(
						runtime.inner(),
						// SAFETY: Input string must be the same length.
						// We know `as_str_full()` always returns the correct str.
						unsafe { Str::from_raw_str($max_len as u8, runtime.$str_function()) },
					)
				}
			}
			impl From<&RuntimeUnion> for $self {
				#[inline]
				fn from(runtime: &RuntimeUnion) -> Self {
					Self(
						runtime.inner(),
						// SAFETY: Input string must be the same length.
						// We know `as_str_full()` always returns the correct str.
						unsafe { Str::from_raw_str($max_len as u8, runtime.$str_function()) },
					)
				}
			}
		)?
	}
}
pub(super) use impl_runtime;

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _format_hms() {
		fn s(b: &[u8], l: usize) -> &str {
			std::str::from_utf8(&b[..l]).unwrap()
		}

		let mut buf = [0; MAX_LEN_RUNTIME];
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

		let mut buf = [0; MAX_LEN_RUNTIME];
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
		for i in 0..MAX_RUNTIME_F32 as u32 {
			let rt = Runtime::from(i);
			println!("rt:{} - i: {}", rt, i);
			assert_eq!(rt.inner() as u32, i);
			assert_eq!(rt.inner() as u32, i);
			println!("{}", rt);
		}
	}

	#[test]
	fn all_floats() {
		let mut f = 1.0;
		while f < MAX_RUNTIME_F32 {
			let rt = Runtime::from(f);
			println!("rt: {} - f: {}", rt, f);
			assert_eq!(rt, f);
			f += 0.1;
		}
	}

	#[test]
	fn overflow_float() {
		assert_eq!(Runtime::from(MAX_RUNTIME_F32 + 1.0), 0.0);
		assert_eq!(Runtime::from(MAX_RUNTIME_F32 + 1.0), Runtime::unknown());
	}

	#[test]
	fn overflow_uint() {
		assert_eq!(Runtime::from(MAX_RUNTIME_F32 + 1.0), 0.0);
		assert_eq!(Runtime::from(MAX_RUNTIME_F32 + 1.0), Runtime::unknown());
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
