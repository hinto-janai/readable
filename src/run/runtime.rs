//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::run::{RuntimePad,RuntimeMilli,RuntimeUnion};
use crate::macros::{
	impl_common,impl_const,
	impl_traits,return_bad_float,
	impl_usize,impl_math,impl_impl_math,
};

//---------------------------------------------------------------------------------------------------- Runtime
/// Human readable "audio/video runtime" in `HH:MM:SS` format.
///
/// ## Formatting rules
/// 1. `seconds` always has a leading `0`
/// 2. `minutes` only has a leading zero if `hours` isn't `0`
/// 3. `hours` never has a leading `0`
///
/// ## Size
/// [`Str<8>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<Runtime>(), 16);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::*;
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
pub struct Runtime(pub(super) f32, pub(super) Str<{ Runtime::MAX_LEN }>);

impl_runtime! { // This macro is defined below.
	self  = Runtime,
	len   = Runtime::MAX_LEN,
	union = as_str,

	other = RuntimePad,
	other = RuntimeMilli,
}
impl_math!(Runtime, f32);
impl_traits!(Runtime, f32);

//---------------------------------------------------------------------------------------------------- Runtime Constants
impl Runtime {
	/// The max length of [`Runtime`]'s string.
	pub const MAX_LEN: usize = 8;

	/// [`f32`] returned when calling [`Runtime::zero`]
	pub const ZERO_F32: f32 = 0.0;

	/// [`f32`] returned when calling [`Runtime::second`]
	pub const SECOND_F32: f32 = 1.0;

	/// [`f32`] returned when calling [`Runtime::minute`]
	pub const MINUTE_F32: f32 = 60.0;

	/// [`f32`] returned when calling [`Runtime::hour`]
	pub const HOUR_F32: f32 = 3600.0;

	/// [`f32`] returned when calling [`Runtime::day`]
	pub const DAY_F32: f32 = 86400.0;

	/// Input greater to [`Runtime`] will make it return [`Self::MAX`]
	pub const MAX_F32: f32 = 359999.0;

	/// Returned when using [`Runtime::unknown`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::UNKNOWN, 0.0);
	/// assert_eq!(Runtime::UNKNOWN, "?:??");
	/// ```
	pub const UNKNOWN: Self = Self(Self::ZERO_F32, Str::from_static_str("?:??"));

	/// Returned when using [`Runtime::zero`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::ZERO, 0.0);
	/// assert_eq!(Runtime::ZERO, "0:00");
	/// ```
	pub const ZERO: Self = Self(Self::ZERO_F32, Str::from_static_str("0:00"));

	/// Returned when using [`Runtime::second`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::SECOND, 1.0);
	/// assert_eq!(Runtime::SECOND, "0:01");
	/// ```
	pub const SECOND: Self = Self(Self::SECOND_F32, Str::from_static_str("0:01"));

	/// Returned when using [`Runtime::minute`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::MINUTE, 60.0);
	/// assert_eq!(Runtime::MINUTE, "1:00");
	/// ```
	pub const MINUTE: Self = Self(Self::MINUTE_F32, Str::from_static_str("1:00"));

	/// Returned when using [`Runtime::hour`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::HOUR, 3600.0);
	/// assert_eq!(Runtime::HOUR, "1:00:00");
	/// ```
	pub const HOUR: Self = Self(Self::HOUR_F32, Str::from_static_str("1:00:00"));

	/// Returned when using [`Runtime::day`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::DAY, 86400.0);
	/// assert_eq!(Runtime::DAY, "24:00:00");
	/// ```
	pub const DAY: Self = Self(Self::DAY_F32, Str::from_static_str("24:00:00"));

	/// Returned when using [`Runtime::max`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::MAX, 359999.0);
	/// assert_eq!(Runtime::MAX, "99:59:59");
	/// ```
	pub const MAX: Self = Self(Self::MAX_F32, Str::from_static_str("99:59:59"));
}

//---------------------------------------------------------------------------------------------------- Runtime Impl
impl Runtime {
	impl_common!(f32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::unknown(), Runtime::UNKNOWN);
	/// ```
	pub const fn unknown() -> Self {
		Self::UNKNOWN
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::zero(), Runtime::ZERO);
	/// ```
	pub const fn zero() -> Self {
		Self::ZERO
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::second(), Runtime::SECOND);
	/// ```
	pub const fn second() -> Self {
		Self::SECOND
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::minute(), Runtime::MINUTE);
	/// ```
	pub const fn minute() -> Self {
		Self::MINUTE
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::hour(), Runtime::HOUR);
	/// ```
	pub const fn hour() -> Self {
		Self::HOUR
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::day(), Runtime::DAY);
	/// ```
	pub const fn day() -> Self {
		Self::DAY
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Runtime::max(), Runtime::MAX);
	/// ```
	pub const fn max() -> Self {
		Self::MAX
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(Runtime::UNKNOWN.is_unknown());
	/// assert!(!Runtime::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		match self.1.as_bytes() {
			b"?:??" => true,
			_ => false,
		}
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
		let mut buf = [0; Self::MAX_LEN];

		// Format.
		let len = if hours > 0 {
			Self::format_hms(&mut buf, hours, minutes, seconds)
		} else {
			Self::format_ms(&mut buf, minutes, seconds)
		};

		Self(runtime, unsafe { Str::from_raw(buf, len as u8) })
	}

	#[inline]
	pub(super) fn priv_from_inner(runtime: f32) -> Option<(f32, f32, f32)> {
		// Zero length.
		if runtime <= 0.0 {
			return Some((0.0, 0.0, 0.0));
		}

		// Return unknown if over max.
		if runtime > Self::MAX_F32 {
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
	fn format_hms(buf: &mut [u8; Self::MAX_LEN], hour: u8, min: u8, sec: u8) -> usize {
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
	fn format_ms(buf: &mut [u8; Self::MAX_LEN], min: u8, sec: u8) -> usize {
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

		//---------------------------------------------------------------------------------------------------- uint
		macro_rules! impl_u {
			($from:ty) => {
				impl From<$from> for $self {
					fn from(runtime: $from) -> Self {
						Self::priv_from(runtime as f32)
					}
				}
				impl From<&$from> for $self {
					fn from(runtime: &$from) -> Self {
						Self::priv_from(*runtime as f32)
					}
				}
			}
		}
		impl_u!(u8);
		impl_u!(u16);
		impl_u!(u32);
		impl_u!(u64);
		impl_u!(u128);
		impl_u!(usize);

		//---------------------------------------------------------------------------------------------------- Int
		macro_rules! impl_i {
			($from:ty) => {
				impl From<$from> for $self {
					fn from(runtime: $from) -> Self {
						if runtime.is_negative() {
							return Self::unknown();
						}
						Self::priv_from(runtime as f32)
					}
				}
				impl From<&$from> for $self {
					fn from(runtime: &$from) -> Self {
						if runtime.is_negative() {
							return Self::unknown();
						}
						Self::priv_from(*runtime as f32)
					}
				}
			}
		}
		impl_i!(i8);
		impl_i!(i16);
		impl_i!(i32);
		impl_i!(i64);
		impl_i!(i128);
		impl_i!(isize);

		//---------------------------------------------------------------------------------------------------- PartialEq
		$(
			impl PartialEq<$other> for $self {
				#[inline]
				fn eq(&self, other: &$other) -> bool {
					self.inner() == other.inner()
				}
			}
			impl PartialEq<&$other> for $self {
				#[inline]
				fn eq(&self, other: &&$other) -> bool {
					self.inner() == other.inner()
				}
			}
		)*

		//---------------------------------------------------------------------------------------------------- From `RuntimeUnion`
		$(
			impl From<RuntimeUnion> for $self {
				#[inline]
				fn from(runtime: RuntimeUnion) -> Self {
					Self(
						runtime.inner(),
						// SAFETY: Input string must be the same length.
						// We know `as_str_full()` always returns the correct str.
						Str::from_str(runtime.$str_function()),
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
						Str::from_str(runtime.$str_function()),
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

		let mut buf = [0; Runtime::MAX_LEN];
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

		let mut buf = [0; Runtime::MAX_LEN];
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
		for i in 0..Runtime::MAX_F32 as u32 {
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
		while f < Runtime::MAX_F32 {
			let rt = Runtime::from(f);
			println!("rt: {} - f: {}", rt, f);
			assert_eq!(rt, f);
			f += 0.1;
		}
	}

	#[test]
	fn overflow_float() {
		assert_eq!(Runtime::from(Runtime::MAX_F32 + 1.0), 0.0);
		assert_eq!(Runtime::from(Runtime::MAX_F32 + 1.0), Runtime::unknown());
	}

	#[test]
	fn overflow_uint() {
		assert_eq!(Runtime::from(Runtime::MAX_F32 + 1.0), 0.0);
		assert_eq!(Runtime::from(Runtime::MAX_F32 + 1.0), Runtime::unknown());
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
