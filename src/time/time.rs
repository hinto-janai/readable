//---------------------------------------------------------------------------------------------------- Use
use crate::time::TimeFull;
use crate::str::Str;
use crate::macros::{
	return_bad_float,impl_common,
	impl_const,impl_impl_math,impl_math,
	impl_usize,impl_traits,
};
use crate::itoa;

//---------------------------------------------------------------------------------------------------- Constants
/// ```rust
/// # use readable::time::*;
/// let date = "---y, --m, --d, --h, --m, --s";
/// assert_eq!(date.len(), MAX_LEN_TIME);
/// ```
pub const MAX_LEN_TIME: usize = 29;

/// [`u32`] & [`str`] returned when using [`Time::unknown`]
pub const UNKNOWN_TIME: (u32, &str) = (0, "???");

/// [`u32`] & [`str`] returned when using [`Time::zero`]
pub const ZERO_TIME: (u32, &str) = (0, "0s");

/// [`u32`] & [`str`] returned when using [`Time::second`]
pub const SECOND_TIME: (u32, &str) = (1, "1s");

/// [`u32`] & [`str`] returned when using [`Time::minute`]
pub const MINUTE_TIME: (u32, &str) = (60, "1m");

/// [`u32`] & [`str`] returned when using [`Time::hour`]
pub const HOUR_TIME: (u32, &str) = (3600, "1h");

/// [`u32`] & [`str`] returned when using [`Time::day`]
pub const DAY_TIME: (u32, &str) = (86400, "1d");

/// [`u32`] & [`str`] returned when using [`Time::month`]
pub const MONTH_TIME: (u32, &str) = (2630016, "1m");

/// [`u32`] & [`str`] returned when using [`Time::year`]
pub const YEAR_TIME: (u32, &str) = (31557600, "1y");

/// [`u32`] & [`str`] returned when using [`Time::max`]
pub const MAX_TIME: (u32, &str) = (u32::MAX, "136y, 1m, 5d, 19h, 54m, 39s");

//---------------------------------------------------------------------------------------------------- Time
/// Human-readable time
///
/// This formats numbers into an "uptime"-style time format,
/// suffixed with a single letter indicated the unit.
///
/// /// ## Size
/// [`Str<63>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<Time>(), 36);
/// ```
///
/// ## Warning
/// This stylizes both `minute` and `month` as `m`, thus:
/// ```rust
/// # use readable::*;
/// assert_eq!(Time::minute(), "1m");
/// assert_eq!(Time::month(),  "1m");
/// ```
///
/// Although, their inner number will be different and context may make it more clear:
/// ```
/// # use readable::*;
/// assert_eq!(Time::minute().inner(), 60);
/// assert_eq!(Time::month().inner(),  2630016);
///
/// assert_eq!(Time::minute() + 3601, "1h, 1m, 1s");
/// assert_eq!(Time::month() + 3661,  "1m, 1h, 1m, 1s");
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::Time;
/// assert_eq!(Time::from(0_u32),        "0s");
/// assert_eq!(Time::from(1_u32),        "1s");
/// assert_eq!(Time::from(2_u32),        "2s");
/// assert_eq!(Time::from(59_u32),       "59s");
/// assert_eq!(Time::from(60_u32),       "1m");
/// assert_eq!(Time::from(61_u32),       "1m, 1s");
/// assert_eq!(Time::from(62_u32),       "1m, 2s");
/// assert_eq!(Time::from(120_u32),      "2m");
/// assert_eq!(Time::from(121_u32),      "2m, 1s");
/// assert_eq!(Time::from(122_u32),      "2m, 2s");
/// assert_eq!(Time::from(179_u32),      "2m, 59s");
/// assert_eq!(Time::from(3599_u32),     "59m, 59s");
/// assert_eq!(Time::from(3600_u32),     "1h");
/// assert_eq!(Time::from(3601_u32),     "1h, 1s");
/// assert_eq!(Time::from(3602_u32),     "1h, 2s");
/// assert_eq!(Time::from(3660_u32),     "1h, 1m");
/// assert_eq!(Time::from(3720_u32),     "1h, 2m");
/// assert_eq!(Time::from(86399_u32),    "23h, 59m, 59s");
/// assert_eq!(Time::from(86400_u32),    "1d");
/// assert_eq!(Time::from(86401_u32),    "1d, 1s");
/// assert_eq!(Time::from(86402_u32),    "1d, 2s");
/// assert_eq!(Time::from(86460_u32),    "1d, 1m");
/// assert_eq!(Time::from(86520_u32),    "1d, 2m");
/// assert_eq!(Time::from(90000_u32),    "1d, 1h");
/// assert_eq!(Time::from(93600_u32),    "1d, 2h");
/// assert_eq!(Time::from(604799_u32),   "6d, 23h, 59m, 59s");
/// assert_eq!(Time::from(604800_u32),   "7d");
/// assert_eq!(Time::from(2630016_u32),  "1m");
/// assert_eq!(Time::from(3234815_u32),  "1m, 6d, 23h, 59m, 59s");
/// assert_eq!(Time::from(5260032_u32),  "2m");
/// assert_eq!(Time::from(31557600_u32), "1y");
/// assert_eq!(Time::from(63115200_u32), "2y");
/// assert_eq!(
///     Time::from(u32::MAX),
///     "136y, 1m, 5d, 19h, 54m, 39s",
/// );
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Time(pub(super) u32, pub(super) Str<MAX_LEN_TIME>);

impl_math!(Time, u32);
impl_traits!(Time, u32);

//---------------------------------------------------------------------------------------------------- Pub Impl
impl Time {
	impl_common!(u32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert_eq!(Time::zero(), 0_u32);
	/// assert_eq!(Time::zero(), "0s");
	/// ```
	pub const fn zero() -> Self {
		Self(ZERO_TIME.0, Str::from_static_str(ZERO_TIME.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert_eq!(Time::second(), 1_u32);
	/// assert_eq!(Time::second(), "1s");
	/// ```
	pub const fn second() -> Self {
		Self(SECOND_TIME.0, Str::from_static_str(SECOND_TIME.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert_eq!(Time::minute(), 60_u32);
	/// assert_eq!(Time::minute(), "1m");
	/// ```
	pub const fn minute() -> Self {
		Self(MINUTE_TIME.0, Str::from_static_str(MINUTE_TIME.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert_eq!(Time::hour(), 3600_u32);
	/// assert_eq!(Time::hour(), "1h");
	/// ```
	pub const fn hour() -> Self {
		Self(HOUR_TIME.0, Str::from_static_str(HOUR_TIME.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert_eq!(Time::day(), 86400_u32);
	/// assert_eq!(Time::day(), "1d");
	/// ```
	pub const fn day() -> Self {
		Self(DAY_TIME.0, Str::from_static_str(DAY_TIME.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert_eq!(Time::month(), 2630016_u32);
	/// assert_eq!(Time::month(), "1m");
	/// ```
	pub const fn month() -> Self {
		Self(MONTH_TIME.0, Str::from_static_str(MONTH_TIME.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert_eq!(Time::year(), 31557600_u32);
	/// assert_eq!(Time::year(), "1y");
	/// ```
	pub const fn year() -> Self {
		Self(YEAR_TIME.0, Str::from_static_str(YEAR_TIME.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert_eq!(Time::max(), u32::MAX);
	/// assert_eq!(Time::max(), "136y, 1m, 5d, 19h, 54m, 39s");
	/// ```
	pub const fn max() -> Self {
		Self(MAX_TIME.0, Str::from_static_str(MAX_TIME.1))
	}


	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert_eq!(Time::unknown(), 0_u32);
	/// assert_eq!(Time::unknown(), "???");
	/// ```
	pub const fn unknown() -> Self {
		Self(UNKNOWN_TIME.0, Str::from_static_str(UNKNOWN_TIME.1))
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl Time {
	#[inline]
	fn plural(
		s: &mut Str<MAX_LEN_TIME>,
		name: &'static str,
		value: u32,
		started: &mut bool,
	) {
		if value > 0 {
			if *started {
				s.push_str_unchecked(", ");
			}
			s.push_str_unchecked(itoa!(value));
			s.push_str_unchecked(name);
			*started = true;
		}
	}

	fn from_priv(secs: u32) -> Self {
		// #[cfg(feature = "inline_time")]
		// if secs <= 3660 {
		// 	// SAFETY:
		// 	// Cast `u64` to `u16` is safe because it's under 65_535.
		// 	return Self(secs, CompactString::new_inline(readable_inlined_time::inlined(secs as u16)))
		// }

		if secs == 0 {
			return Self::zero();
		}

		let years    = secs / 31_557_600;  // 365.25d
		let ydays    = secs % 31_557_600;
		let months   = ydays / 2_630_016;  // 30.44d
		let mdays    = ydays % 2_630_016;
		let days     = mdays / 86400;
		let day_secs = mdays % 86400;
		let hours    = day_secs / 3600;
		let minutes  = day_secs % 3600 / 60;
		let seconds  = day_secs % 60;

		let started = &mut false;
		let mut string = Str::new();
		let s = &mut string;
		Self::plural(s, "y", years,   started);
		Self::plural(s, "m", months,  started);
		Self::plural(s, "d", days,    started);
		Self::plural(s, "h", hours,   started);
		Self::plural(s, "m", minutes, started);
		Self::plural(s, "s", seconds, started);

		Self(secs, string)
	}
}

//---------------------------------------------------------------------------------------------------- "u*" impl
macro_rules! handle_over_u32 {
	($value:expr, $type:ty) => {
		if $value > (u32::MAX as $type) {
			return Self::unknown();
		}
	};
}

//---------------------------------------------------------------------------------------------------- "u*" impl
// Implementation Macro.
macro_rules! impl_u {
	($($u:ty),* $(,)?) => { $(
		impl From<$u> for Time {
			#[inline]
			fn from(u: $u) -> Self {
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for Time {
			#[inline]
			fn from(u: &$u) -> Self {
				Self::from_priv(*u as u32)
			}
		}
	)*}
}
impl_u!(u8,u16,u32);
#[cfg(not(target_pointer_width = "64"))]
impl_u!(usize);

macro_rules! impl_u_over {
	($($u:ty),* $(,)?) => { $(
		impl From<$u> for Time {
			#[inline]
			fn from(u: $u) -> Self {
				handle_over_u32!(u, $u);
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for Time {
			#[inline]
			fn from(u: &$u) -> Self {
				handle_over_u32!(*u, $u);
				Self::from_priv(*u as u32)
			}
		}
	)*}
}

impl_u_over!(u64,u128);
#[cfg(target_pointer_width = "64")]
impl_u_over!(usize);

//---------------------------------------------------------------------------------------------------- i* impl
macro_rules! impl_int {
	($($int:ty),* $(,)?) => { $(
		impl From<$int> for Time {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::unknown();
				}
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for Time {
			#[inline]
			fn from(int: &$int) -> Self {
				if int.is_negative() {
					return Self::unknown();
				}
				Self::from_priv(*int as u32)
			}
		}
	)*}
}
impl_int!(i8,i16,i32);
#[cfg(not(target_pointer_width = "64"))]
impl_u!(isize);

macro_rules! impl_int_over {
	($($int:ty),* $(,)?) => { $(
		impl From<$int> for Time {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::unknown();
				}
				handle_over_u32!(int, $int);
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for Time {
			#[inline]
			fn from(int: &$int) -> Self {
				if int.is_negative() {
					return Self::unknown();
				}
				handle_over_u32!(*int, $int);
				Self::from_priv(*int as u32)
			}
		}
	)*}
}
impl_int_over!(i64,i128);
#[cfg(target_pointer_width = "64")]
impl_u_over!(isize);

//---------------------------------------------------------------------------------------------------- "f" impl
macro_rules! impl_f {
	($float:ty) => {
		impl From<$float> for Time {
			#[inline]
			fn from(float: $float) -> Self {
				return_bad_float!(float, Self::unknown, Self::unknown);
				if float.is_sign_negative() {
					return Self::unknown();
				}
				handle_over_u32!(float, $float);
				Self::from_priv(float as u32)
			}
		}
		impl From<&$float> for Time {
			#[inline]
			fn from(float: &$float) -> Self {
				return_bad_float!(float, Self::unknown, Self::unknown);
				if float.is_sign_negative() {
					return Self::unknown();
				}
				handle_over_u32!(*float, $float);
				Self::from_priv(*float as u32)
			}
		}
	}
}
impl_f!(f32);
impl_f!(f64);

//---------------------------------------------------------------------------------------------------- From Time
impl From<TimeFull> for Time {
	#[inline]
	fn from(t: TimeFull) -> Self {
		if t.1 == UNKNOWN_TIME.1 {
			return Self::unknown();
		}
		Self::from_priv(t.0)
	}
}
impl From<&TimeFull> for Time {
	#[inline]
	fn from(t: &TimeFull) -> Self {
		if t.1 == UNKNOWN_TIME.1 {
			return Self::unknown();
		}
		Self::from_priv(t.0)
	}
}

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for Time {
	fn from(duration: std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Duration> for Time {
	fn from(duration: &std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<std::time::Instant> for Time {
	fn from(instant: std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Instant> for Time {
	fn from(instant: &std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn all_ints() {
		let mut f = 1_u64;
		while f < (MAX_TIME.0 as u64) {
			let t = Time::from(f);
			println!("t: {t}, f: {f}");
			assert_eq!(t, f as u32);
			f *= 10;
		}
	}

	#[test]
	fn over() {
		assert_ne!(Time::from(u32::MAX),            Time::unknown());
		assert_eq!(Time::from(u32::MAX as u64 + 1), Time::unknown());
		assert_eq!(Time::from(u64::MAX),            Time::unknown());
		assert_eq!(Time::from(f64::MAX),            Time::unknown());
		assert_eq!(Time::from(f32::MAX),            Time::unknown());
	}

	#[test]
	fn special() {
		assert_eq!(Time::from(f32::NAN),          Time::unknown());
		assert_eq!(Time::from(f32::INFINITY),     Time::unknown());
		assert_eq!(Time::from(f32::NEG_INFINITY), Time::unknown());
		assert_eq!(Time::from(f64::NAN),          Time::unknown());
		assert_eq!(Time::from(f64::INFINITY),     Time::unknown());
		assert_eq!(Time::from(f64::NEG_INFINITY), Time::unknown());
	}
}
