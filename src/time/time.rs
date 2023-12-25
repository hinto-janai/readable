//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::macros::{
	impl_common,impl_const,
	impl_traits,impl_usize,impl_math,
	impl_impl_math,handle_over_u32,
};
use crate::time::{TimeUnit,Military};
#[cfg(feature = "num")]
use crate::num::Unsigned;

//---------------------------------------------------------------------------------------------------- Time
/// Clock time - `11:59:59 PM`
///
/// This formats seconds into `HH:MM:SS [AM|PM]` formatting.
///
/// An overflowing input will wrap back around (like a real clock), e.g:
/// ```rust
/// # use readable::time::*;
/// // 23 hours, 59 minutes, 59 seconds.
/// assert_eq!(Time::from(86399), "11:59:59 PM");
///
/// // 1 day (wraps).
/// assert_eq!(Time::from(86400), "12:00:00 AM");
///
/// // 1 day and 1 second (wraps).
/// assert_eq!(Time::from(86401), "12:00:01 AM");
/// ```
///
/// ## Size
/// [`Str<11>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::time::*;
/// assert_eq!(std::mem::size_of::<Time>(), 16);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::time::*;
/// assert_eq!(Time::from(0),         "12:00:00 AM");
/// assert_eq!(Time::from(1),         "12:00:01 AM");
/// assert_eq!(Time::from(10),        "12:00:10 AM");
/// assert_eq!(Time::from(60),        "12:01:00 AM");
/// assert_eq!(Time::from(3599),      "12:59:59 AM");
/// assert_eq!(Time::from(3600),      "1:00:00 AM");
/// assert_eq!(Time::from(3600 * 2),  "2:00:00 AM");
/// assert_eq!(Time::from(3600 * 3),  "3:00:00 AM");
/// assert_eq!(Time::from(3600 * 4),  "4:00:00 AM");
/// assert_eq!(Time::from(3600 * 5),  "5:00:00 AM");
/// assert_eq!(Time::from(3600 * 6),  "6:00:00 AM");
/// assert_eq!(Time::from(3600 * 7),  "7:00:00 AM");
/// assert_eq!(Time::from(3600 * 8),  "8:00:00 AM");
/// assert_eq!(Time::from(3600 * 9),  "9:00:00 AM");
/// assert_eq!(Time::from(3600 * 10), "10:00:00 AM");
/// assert_eq!(Time::from(3600 * 11), "11:00:00 AM");
/// assert_eq!(Time::from(3600 * 12), "12:00:00 PM");
/// assert_eq!(Time::from(3600 * 13), "1:00:00 PM");
/// assert_eq!(Time::from(3600 * 14), "2:00:00 PM");
/// assert_eq!(Time::from(3600 * 15), "3:00:00 PM");
/// assert_eq!(Time::from(3600 * 16), "4:00:00 PM");
/// assert_eq!(Time::from(3600 * 17), "5:00:00 PM");
/// assert_eq!(Time::from(3600 * 18), "6:00:00 PM");
/// assert_eq!(Time::from(3600 * 19), "7:00:00 PM");
/// assert_eq!(Time::from(3600 * 20), "8:00:00 PM");
/// assert_eq!(Time::from(3600 * 21), "9:00:00 PM");
/// assert_eq!(Time::from(3600 * 22), "10:00:00 PM");
/// assert_eq!(Time::from(3600 * 23), "11:00:00 PM");
/// assert_eq!(Time::from(3600 * 24), "12:00:00 AM");
/// assert_eq!(Time::from((3600 * 24) + 1),    "12:00:01 AM");
/// assert_eq!(Time::from((3600 * 24) + 60),   "12:01:00 AM");
/// assert_eq!(Time::from((3600 * 24) + 3599), "12:59:59 AM");
/// assert_eq!(Time::from((3600 * 24) + 1830), "12:30:30 AM");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Time(pub(super) u32, pub(super) Str<{ Time::MAX_LEN }>);

impl_traits!(Time, u32);
impl_math!(Time, u32);

//---------------------------------------------------------------------------------------------------- Time Constants
impl Time {
	/// The max length of [`Time`]'s string.
	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!("10:10:10 AM".len(), Time::MAX_LEN);
	/// ```
	pub const MAX_LEN: usize = 11;

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(Time::UNKNOWN, 0);
	/// assert_eq!(Time::UNKNOWN, "??:??:??");
	/// ```
	pub const UNKNOWN: Self = Self(0, Str::from_static_str("??:??:??"));

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(Time::ZERO, 0);
	/// assert_eq!(Time::ZERO, "12:00:00 AM");
	/// ```
	pub const ZERO: Self = Self(0, Str::from_static_str("12:00:00 AM"));

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(Time::MAX, 86399);
	/// assert_eq!(Time::MAX, "11:59:59 PM");
	/// ```
	pub const MAX: Self = Self(86399, Str::from_static_str("11:59:59 PM"));
}

//---------------------------------------------------------------------------------------------------- Impl
impl Time {
	impl_common!(u32);
	impl_const!();
	impl_usize!();

	#[inline]
	#[must_use]
	/// Create a [`Self`] from seconds
	///
	/// This behaves the exact same way as the [`From`]
	/// implementation, although this function is `const`.
	///
	/// ```rust
	/// # use readable::time::*;
	/// let from:    Time = Time::from(86399);
	/// const CONST: Time = Time::new(86399);
	///
	/// assert_eq!(from,  "11:59:59 PM");
	/// assert_eq!(CONST, "11:59:59 PM");
	/// assert_eq!(from, CONST);
	/// ```
	pub const fn new(total_seconds: u32) -> Self {
		Self::priv_from(total_seconds)
	}

	#[inline]
	#[must_use]
	/// Create a [`Self`] with specified `hours`, `minutes`, and `seconds`
	///
	/// This takes hours, minutes, and seconds and will convert the
	/// total time into a [`Time`] (maintaing the normal wrapping behavior).
	///
	/// A value being left as `None` is equal to `0`.
	///
	/// ```rust
	/// # use readable::time::*;
	/// let time = Time::new_specified(
	///     3,  // hours
	///     21, // minutes
	///     55, // seconds
	/// );
	/// assert_eq!(time, "3:21:55 AM");
	///
	/// // Overflowing to PM.
	/// let time = Time::new_specified(13, 21, 0);
	/// assert_eq!(time, "1:21:00 PM");
	///
	/// // Wrapping back around.
	/// let time = Time::new_specified(25, 1, 1);
	/// assert_eq!(time, "1:01:01 AM");
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
	#[must_use]
	/// ```rust
	/// # use readable::time::*;
	/// assert!(Time::UNKNOWN.is_unknown());
	/// assert!(!Time::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		matches!(self.1.as_bytes(), b"??:??:??")
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl Time {
	pub(super) const fn priv_from(total_seconds: u32) -> Self {
		// Format.
		const C: u8 = b':';
		const S: u8 = b' ';
		const M: u8 = b'M';

		let total_seconds = total_seconds % 86400;

		if total_seconds == 0 {
			return Self::ZERO;
		}

		let (hours, minutes, seconds) = crate::time::secs_to_clock(total_seconds);

		let h = Self::str_0_23(hours);
		let m = Self::str_0_59(minutes);
		let s = Self::str_0_59(seconds);
		let marker = if hours > 11 { b'P' } else { b'A' };

		let (buf, len): ([u8; Self::MAX_LEN], u8) = if h.len() == 1 {
			([
				h[0],
				C,
				m[0],
				m[1],
				C,
				s[0],
				s[1],
				S,
				marker,
				M,
				0,
			], Self::MAX_LEN as u8 - 1)
		} else {
			([
				h[0],
				h[1],
				C,
				m[0],
				m[1],
				C,
				s[0],
				s[1],
				S,
				marker,
				M,
			], Self::MAX_LEN as u8)
		};

		// SAFETY: we know the str len
		Self(total_seconds, unsafe { Str::from_raw(buf,len) })
	}

	#[inline]
	// INVARIANT: input must be 0..=23
	const fn str_0_23(u: u8) -> &'static [u8] {
		match u {
			0|12  =>  b"12",
			1|13  =>  b"1",
			2|14  =>  b"2",
			3|15  =>  b"3",
			4|16  =>  b"4",
			5|17  =>  b"5",
			6|18  =>  b"6",
			7|19  =>  b"7",
			8|20  =>  b"8",
			9|21  =>  b"9",
			10|22 => b"10",
			11|23 => b"11",
			_ => unreachable!(),
		}
	}

	#[inline]
	// INVARIANT: input must be 0..=59
	pub(super) const fn str_0_59(u: u8) -> &'static [u8] {
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
			24 => b"24",
			25 => b"25",
			26 => b"26",
			27 => b"27",
			28 => b"28",
			29 => b"29",
			30 => b"30",
			31 => b"31",
			32 => b"32",
			33 => b"33",
			34 => b"34",
			35 => b"35",
			36 => b"36",
			37 => b"37",
			38 => b"38",
			39 => b"39",
			40 => b"40",
			41 => b"41",
			42 => b"42",
			43 => b"43",
			44 => b"44",
			45 => b"45",
			46 => b"46",
			47 => b"47",
			48 => b"48",
			49 => b"49",
			50 => b"50",
			51 => b"51",
			52 => b"52",
			53 => b"53",
			54 => b"54",
			55 => b"55",
			56 => b"56",
			57 => b"57",
			58 => b"58",
			59 => b"59",
			_ => unreachable!(),
		}
	}
}

//---------------------------------------------------------------------------------------------------- Floats
macro_rules! impl_f {
	($from:ty) => {
		impl From<$from> for Time {
			#[inline]
			fn from(f: $from) -> Self {
				$crate::macros::return_bad_float!(f, Self::UNKNOWN, Self::UNKNOWN);

				Self::priv_from(f as u32)
			}
		}
		impl From<&$from> for Time {
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
		impl From<$from> for Time {
			#[inline]
			fn from(seconds: $from) -> Self {
				Self::priv_from(seconds as u32)
			}
		}
		impl From<&$from> for Time {
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
		impl From<$from> for Time {
			#[inline]
			fn from(seconds: $from) -> Self {
				if seconds.is_negative() {
					return Self::UNKNOWN;
				}
				Self::priv_from(seconds as u32)
			}
		}
		impl From<&$from> for Time {
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
			impl From<$from> for Time {
				#[inline]
				fn from(other: $from) -> Self {
					if other.is_unknown() {
						return Self::UNKNOWN;
					}
					Self::priv_from(other.inner() as u32)
				}
			}
			impl From<&$from> for Time {
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
impl_other!(Military, TimeUnit);
#[cfg(feature = "num")]
impl_other!(Unsigned);

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for Time {
	#[inline]
	fn from(duration: std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::new(u as u32)
	}
}

impl From<&std::time::Duration> for Time {
	#[inline]
	fn from(duration: &std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::new(u as u32)
	}
}

impl From<Time> for std::time::Duration {
	#[inline]
	fn from(value: Time) -> Self {
		Self::from_secs(value.inner().into())
	}
}

impl From<&Time> for std::time::Duration {
	#[inline]
	fn from(value: &Time) -> Self {
		Self::from_secs(value.inner().into())
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[cfg(feature = "serde")]
	fn serde() {
		let this: Time = Time::from(3599);
		let json = serde_json::to_string(&this).unwrap();
		assert_eq!(json, r#"[3599,"12:59:59 AM"]"#);

		let this: Time = serde_json::from_str(&json).unwrap();
		assert_eq!(this, 3599);
		assert_eq!(this, "12:59:59 AM");

		// Bad bytes.
		assert!(serde_json::from_str::<Time>(&"---").is_err());

		// Unknown.
		let json = serde_json::to_string(&Time::UNKNOWN).unwrap();
		assert_eq!(json, r#"[0,"??:??:??"]"#);
		assert!(serde_json::from_str::<Time>(&json).unwrap().is_unknown());
	}

	#[test]
	#[cfg(feature = "bincode")]
	fn bincode() {
		let this: Time = Time::from(3599);
		let config = bincode::config::standard();
		let bytes = bincode::encode_to_vec(&this, config).unwrap();

		let this: Time = bincode::decode_from_slice(&bytes, config).unwrap().0;
		assert_eq!(this, 3599);
		assert_eq!(this, "12:59:59 AM");

		// Unknown.
		let bytes = bincode::encode_to_vec(&Time::UNKNOWN, config).unwrap();
		let this: Time = bincode::decode_from_slice(&bytes, config).unwrap().0;
		assert!(this.is_unknown());
	}

	#[test]
	#[cfg(feature = "borsh")]
	fn borsh() {
		let this: Time = Time::from(3599);
		let bytes = borsh::to_vec(&this).unwrap();

		let this: Time = borsh::from_slice(&bytes).unwrap();
		assert_eq!(this, 3599);
		assert_eq!(this, "12:59:59 AM");

		// Bad bytes.
		assert!(borsh::from_slice::<Time>(b"bad .-;[]124/ bytes").is_err());

		// Unknown.
		let bytes = borsh::to_vec(&Time::UNKNOWN).unwrap();
		let this: Time = borsh::from_slice(&bytes).unwrap();
		assert!(this.is_unknown());
	}
}
