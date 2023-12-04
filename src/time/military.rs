use crate::{TimeUnit, Unsigned};
//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::time::Time;
use crate::macros::{
	impl_common,impl_const,
	impl_traits,
	impl_usize,impl_math,impl_impl_math,
	handle_over_u32,
};

//---------------------------------------------------------------------------------------------------- Military
/// Military time - `23:59:59`
///
/// This formats seconds into "military"-style 24-hour based `HH:MM:SS` formatting.
///
/// An overflowing input will wrap back around (like a real clock), e.g:
/// ```rust
/// # use readable::*;
/// // 23 hours, 59 minutes, 59 seconds.
/// assert_eq!(Military::from(86399), "23:59:59");
///
/// // 1 day (wraps).
/// assert_eq!(Military::from(86400), "00:00:00");
///
/// // 1 day and 1 second (wraps).
/// assert_eq!(Military::from(86401), "00:00:01");
/// ```
///
/// ## Size
/// [`Str<7>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<Military>(), 16);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::Military;
/// assert_eq!(Military::from(0),         "00:00:00");
/// assert_eq!(Military::from(1),         "00:00:01");
/// assert_eq!(Military::from(10),        "00:00:10");
/// assert_eq!(Military::from(60),        "00:01:00");
/// assert_eq!(Military::from(3599),      "00:59:59");
/// assert_eq!(Military::from(3600),      "01:00:00");
/// assert_eq!(Military::from(3600 * 2),  "02:00:00");
/// assert_eq!(Military::from(3600 * 3),  "03:00:00");
/// assert_eq!(Military::from(3600 * 4),  "04:00:00");
/// assert_eq!(Military::from(3600 * 5),  "05:00:00");
/// assert_eq!(Military::from(3600 * 6),  "06:00:00");
/// assert_eq!(Military::from(3600 * 7),  "07:00:00");
/// assert_eq!(Military::from(3600 * 8),  "08:00:00");
/// assert_eq!(Military::from(3600 * 9),  "09:00:00");
/// assert_eq!(Military::from(3600 * 10), "10:00:00");
/// assert_eq!(Military::from(3600 * 11), "11:00:00");
/// assert_eq!(Military::from(3600 * 12), "12:00:00");
/// assert_eq!(Military::from(3600 * 13), "13:00:00");
/// assert_eq!(Military::from(3600 * 14), "14:00:00");
/// assert_eq!(Military::from(3600 * 15), "15:00:00");
/// assert_eq!(Military::from(3600 * 16), "16:00:00");
/// assert_eq!(Military::from(3600 * 17), "17:00:00");
/// assert_eq!(Military::from(3600 * 18), "18:00:00");
/// assert_eq!(Military::from(3600 * 19), "19:00:00");
/// assert_eq!(Military::from(3600 * 20), "20:00:00");
/// assert_eq!(Military::from(3600 * 21), "21:00:00");
/// assert_eq!(Military::from(3600 * 22), "22:00:00");
/// assert_eq!(Military::from(3600 * 23), "23:00:00");
/// assert_eq!(Military::from(3600 * 24), "00:00:00");
/// assert_eq!(Military::from((3600 * 24) + 1),    "00:00:01");
/// assert_eq!(Military::from((3600 * 24) + 60),   "00:01:00");
/// assert_eq!(Military::from((3600 * 24) + 3599), "00:59:59");
/// assert_eq!(Military::from((3600 * 24) + 1830), "00:30:30");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Military(pub(super) u32, pub(super) Str<{ Military::MAX_LEN }>);

impl_traits!(Military, u32);
impl_math!(Military, u32);

//---------------------------------------------------------------------------------------------------- Military Constants
impl Military {
	/// The max length of [`Military`]'s string.
	/// ```rust
	/// # use readable::*;
	/// assert_eq!("10:10:10".len(), Military::MAX_LEN);
	/// ```
	pub const MAX_LEN: usize = 8;

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Military::UNKNOWN, 0);
	/// assert_eq!(Military::UNKNOWN, "??:??:??");
	/// ```
	pub const UNKNOWN: Self = Self(0, Str::from_static_str("??:??:??"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Military::ZERO, 0);
	/// assert_eq!(Military::ZERO, "00:00:00");
	/// ```
	pub const ZERO: Self = Self(0, Str::from_static_str("00:00:00"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Military::MAX, 86399);
	/// assert_eq!(Military::MAX, "23:59:59");
	/// ```
	pub const MAX: Self = Self(86399, Str::from_static_str("23:59:59"));
}

//---------------------------------------------------------------------------------------------------- Impl
impl Military {
	impl_common!(u32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// Create a [`Self`] from seconds
	///
	/// This behaves the exact same way as the [`From`]
	/// implementation, although this function is `const`.
	///
	/// ```rust
	/// # use readable::*;
	/// let from:    Military = Military::from(86399);
	/// const CONST: Military = Military::new(86399);
	///
	/// assert_eq!(from,  "23:59:59");
	/// assert_eq!(CONST, "23:59:59");
	/// assert_eq!(from, CONST);
	/// ```
	pub const fn new(total_seconds: u32) -> Self {
		Self::priv_from(total_seconds)
	}

	#[inline]
	/// Create a [`Self`] with specified `hours`, `minutes`, and `seconds`
	///
	/// This takes hours, minutes, and seconds and will convert the
	/// total military into a [`Military`] (maintaing the normal wrapping behavior).
	///
	/// A value being left as `None` is equal to `0`.
	///
	/// ```rust
	/// # use readable::*;
	/// let military = Military::new_specified(
	/// 	3,  // hours
	/// 	21, // minutes
	/// 	55, // seconds
	/// );
	/// assert_eq!(military, "03:21:55");
	///
	/// // Overflowing to PM.
	/// let military = Military::new_specified(13, 21, 0);
	/// assert_eq!(military, "13:21:00");
	///
	/// // Wrapping back around.
	/// let military = Military::new_specified(25, 1, 1);
	/// assert_eq!(military, "01:01:01");
	/// ```
	pub const fn new_specified(
		hours: u8,
		minutes: u8,
		seconds: u8,
	) -> Self {
		Self::priv_from(
			(seconds as u32) +
			(minutes as u32 * 60) +
			(hours as u32 * 3600)
		)
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(Military::UNKNOWN.is_unknown());
	/// assert!(!Military::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		match self.1.as_bytes() {
			b"??:??:??" => true,
			_ => false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl Military {
	pub(super) const fn priv_from(total_seconds: u32) -> Self {
		let total_seconds = total_seconds % 86400;

		if total_seconds == 0 {
			return Self::ZERO;
		}

		let (hours, minutes, seconds) = crate::time::secs_to_clock(total_seconds);

		// Format.
		let h = Self::str_hour(hours);
		let m = Time::str_0_59(minutes);
		let s = Time::str_0_59(seconds);

		const C: u8 = b':';

		let buf: [u8; Self::MAX_LEN] = [
			h[0],
			h[1],
			C,
			m[0],
			m[1],
			C,
			s[0],
			s[1],
		];

		Self(total_seconds, unsafe { Str::from_raw(buf, Self::MAX_LEN as u8) })
	}

	#[inline]
	// INVARIANT: input must be 0..=23
	const fn str_hour(u: u8) -> &'static [u8] {
		match u {
			0 =>  b"00",
			1 =>  b"01",
			2 =>  b"02",
			3 =>  b"03",
			4 =>  b"04",
			5 =>  b"05",
			6 =>  b"06",
			7 =>  b"07",
			8 =>  b"08",
			9 =>  b"09",
			10 => b"10",
			11 => b"11",
			12 => b"12",
			13 => b"13",
			14 => b"14",
			15 => b"15",
			16 => b"16",
			17 => b"17",
			18 => b"18",
			19 => b"19",
			20 => b"20",
			21 => b"21",
			22 => b"22",
			23 => b"23",
			_ => unreachable!(),
		}
	}
}

//---------------------------------------------------------------------------------------------------- Floats
macro_rules! impl_f {
	($from:ty) => {
		impl From<$from> for Military {
			#[inline]
			fn from(f: $from) -> Self {
				$crate::macros::return_bad_float!(f, Self::UNKNOWN, Self::UNKNOWN);

				Self::priv_from(f as u32)
			}
		}
		impl From<&$from> for Military {
			#[inline]
			fn from(f: &$from) -> Self {
				$crate::macros::return_bad_float!(f, Self::UNKNOWN, Self::UNKNOWN);

				Self::priv_from(*f as u32)
			}
		}
	}
}
impl_f!(f32);
impl_f!(f64);

//---------------------------------------------------------------------------------------------------- uint
macro_rules! impl_u {
	($from:ty) => {
		impl From<$from> for Military {
			#[inline]
			fn from(seconds: $from) -> Self {
				Self::priv_from(seconds as u32)
			}
		}
		impl From<&$from> for Military {
			#[inline]
			fn from(seconds: &$from) -> Self {
				Self::from(*seconds)
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
		impl From<$from> for Military {
			#[inline]
			fn from(seconds: $from) -> Self {
				if seconds.is_negative() {
					return Self::UNKNOWN;
				}
				Self::priv_from(seconds as u32)
			}
		}
		impl From<&$from> for Military {
			#[inline]
			fn from(seconds: &$from) -> Self {
				if seconds.is_negative() {
					return Self::UNKNOWN;
				}
				Self::priv_from(*seconds as u32)
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

//---------------------------------------------------------------------------------------------------- Other
macro_rules! impl_other {
	($($from:ty),* $(,)?) => {
		$(
			impl From<$from> for Military {
				#[inline]
				fn from(other: $from) -> Self {
					if other.is_unknown() {
						return Self::UNKNOWN;
					}
					Self::priv_from(other.inner() as u32)
				}
			}
			impl From<&$from> for Military {
				#[inline]
				fn from(other: &$from) -> Self {
					if other.is_unknown() {
						return Self::UNKNOWN;
					}
					Self::priv_from(other.inner() as u32)
				}
			}
		)*
	}
}
impl_other!(Time, TimeUnit, Unsigned);

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for Military {
	#[inline]
	fn from(duration: std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::new(u as u32)
	}
}

impl From<&std::time::Duration> for Military {
	#[inline]
	fn from(duration: &std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::new(u as u32)
	}
}

impl From<Military> for std::time::Duration {
	#[inline]
	fn from(value: Military) -> Self {
		std::time::Duration::from_secs(value.inner() as u64)
	}
}

impl From<&Military> for std::time::Duration {
	#[inline]
	fn from(value: &Military) -> Self {
		std::time::Duration::from_secs(value.inner() as u64)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn _format_hms() {
// 		fn s(b: &[u8]) -> &str {
// 			std::str::from_utf8(&b).unwrap()
// 		}

// 		let mut buf = [0; Military::MAX_LEN];
// 		let buf = &mut buf;

// 		// 0:0:0
// 		Military::format(buf, 1, 1, 1);
// 		assert_eq!(s(buf), "01:01:01");

// 		// 0:00:0
// 		Military::format(buf, 1, 10, 1);
// 		assert_eq!(s(buf), "01:10:01");

// 		// 0:0:00
// 		Military::format(buf, 1, 1, 10);
// 		assert_eq!(s(buf), "01:01:10");

// 		// 0:00:00
// 		Military::format(buf, 1, 10, 10);
// 		assert_eq!(s(buf), "01:10:10");

// 		// 00:0:0
// 		Military::format(buf, 10, 1, 1);
// 		assert_eq!(s(buf), "10:01:01");

// 		// 00:00:0
// 		Military::format(buf, 10, 10, 1);
// 		assert_eq!(s(buf), "10:10:01");

// 		// 00:0:00
// 		Military::format(buf, 10, 1, 10);
// 		assert_eq!(s(buf), "10:01:10");

// 		// 00:00:00
// 		Military::format(buf, 10, 10, 10);
// 		assert_eq!(s(buf), "10:10:10");

// 		// 0:0
// 		Military::format(buf, 0, 1, 1);
// 		assert_eq!(s(buf), "00:01:01");

// 		// 00:0
// 		Military::format(buf, 0, 10, 1);
// 		assert_eq!(s(buf), "00:10:01");

// 		// 0:00
// 		Military::format(buf, 0, 1, 10);
// 		assert_eq!(s(buf), "00:01:10");

// 		// 00:00
// 		Military::format(buf, 0, 10, 10);
// 		assert_eq!(s(buf), "00:10:10");
// 	}

// 	#[test]
// 	fn all_uint() {
// 		for i in 0..Military::MAX_F32 as u32 {
// 			let rt = Military::from(i);
// 			println!("rt:{} - i: {}", rt, i);
// 			assert_eq!(rt.inner() as u32, i);
// 			assert_eq!(rt.inner() as u32, i);
// 			println!("{}", rt);
// 		}
// 	}

// 	#[test]
// 	fn all_floats() {
// 		let mut f = 0;
// 		while f <= Military::MAX_F32 {
// 			let rt = Military::from(f);
// 			println!("rt: {} - f: {}", rt.inner(), f);
// 			assert_eq!(rt, f);
// 			f += 0.1;
// 		}
// 	}

// 	#[test]
// 	fn overflow_float() {
// 		assert_eq!(Military::from(Military::MAX_F32 + 1.0), 0);
// 		assert_eq!(Military::from(Military::MAX_F32 + 1.0), Military::unknown());
// 	}

// 	#[test]
// 	fn overflow_uint() {
// 		assert_eq!(Military::from(Military::MAX_F32 + 1.0), 0);
// 		assert_eq!(Military::from(Military::MAX_F32 + 1.0), Military::unknown());
// 	}

// 	#[test]
// 	fn special() {
// 		assert_eq!(Military::from(f32::NAN),          Military::unknown());
// 		assert_eq!(Military::from(f32::INFINITY),     Military::unknown());
// 		assert_eq!(Military::from(f32::NEG_INFINITY), Military::unknown());
// 		assert_eq!(Military::from(f64::NAN),          Military::unknown());
// 		assert_eq!(Military::from(f64::INFINITY),     Military::unknown());
// 		assert_eq!(Military::from(f64::NEG_INFINITY), Military::unknown());
// 	}
// }
