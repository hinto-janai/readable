//---------------------------------------------------------------------------------------------------- Use
use crate::time::{
	Time,UNKNOWN_TIME,
};
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
/// let date = "--- years, -- months, -- days, -- hours, -- minutes, -- seconds";
/// assert_eq!(date.len(), MAX_LEN_TIME_FULL);
/// ```
pub const MAX_LEN_TIME_FULL: usize = 63;

/// [`u32`] & [`str`] returned when using [`TimeFull::unknown`]
pub const UNKNOWN_TIME_FULL: (u32, &str) = (0, "???");

/// [`u32`] & [`str`] returned when using [`TimeFull::zero`]
pub const ZERO_TIME_FULL: (u32, &str) = (0, "0 seconds");

/// [`u32`] & [`str`] returned when using [`TimeFull::second`]
pub const SECOND_TIME_FULL: (u32, &str) = (1, "1 second");

/// [`u32`] & [`str`] returned when using [`TimeFull::minute`]
pub const MINUTE_TIME_FULL: (u32, &str) = (60, "1 minute");

/// [`u32`] & [`str`] returned when using [`TimeFull::hour`]
pub const HOUR_TIME_FULL: (u32, &str) = (3600, "1 hour");

/// [`u32`] & [`str`] returned when using [`TimeFull::day`]
pub const DAY_TIME_FULL: (u32, &str) = (86400, "1 day");

/// [`u32`] & [`str`] returned when using [`TimeFull::month`]
pub const MONTH_TIME_FULL: (u32, &str) = (2630016, "1 month");

/// [`u32`] & [`str`] returned when using [`TimeFull::year`]
pub const YEAR_TIME_FULL: (u32, &str) = (31557600, "1 year");

/// [`u32`] & [`str`] returned when using [`TimeFull::max`]
pub const MAX_TIME_FULL: (u32, &str) = (u32::MAX, "136 years, 1 month, 5 days, 19 hours, 54 minutes, 39 seconds");

//---------------------------------------------------------------------------------------------------- TimeFull
/// [`TimeFull`] but with full specified words
///
/// This is the same type as [`TimeFull`], except, the
/// words specifying the time will not be abbreviated
/// and will be pluralized, e.g:
/// ```rust
/// # use readable::time::*;
/// assert_eq!(TimeFull::from(0), "0 seconds");
/// assert_eq!(TimeFull::from(1), "1 second");
/// assert_eq!(TimeFull::from(2), "2 seconds");
/// ```
///
/// /// ## Size
/// [`Str<63>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<TimeFull>(), 68);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::TimeFull;
/// assert_eq!(TimeFull::from(0_u32),        "0 seconds");
/// assert_eq!(TimeFull::from(1_u32),        "1 second");
/// assert_eq!(TimeFull::from(2_u32),        "2 seconds");
/// assert_eq!(TimeFull::from(59_u32),       "59 seconds");
/// assert_eq!(TimeFull::from(60_u32),       "1 minute");
/// assert_eq!(TimeFull::from(61_u32),       "1 minute, 1 second");
/// assert_eq!(TimeFull::from(62_u32),       "1 minute, 2 seconds");
/// assert_eq!(TimeFull::from(120_u32),      "2 minutes");
/// assert_eq!(TimeFull::from(121_u32),      "2 minutes, 1 second");
/// assert_eq!(TimeFull::from(122_u32),      "2 minutes, 2 seconds");
/// assert_eq!(TimeFull::from(179_u32),      "2 minutes, 59 seconds");
/// assert_eq!(TimeFull::from(3599_u32),     "59 minutes, 59 seconds");
/// assert_eq!(TimeFull::from(3600_u32),     "1 hour");
/// assert_eq!(TimeFull::from(3601_u32),     "1 hour, 1 second");
/// assert_eq!(TimeFull::from(3602_u32),     "1 hour, 2 seconds");
/// assert_eq!(TimeFull::from(3660_u32),     "1 hour, 1 minute");
/// assert_eq!(TimeFull::from(3720_u32),     "1 hour, 2 minutes");
/// assert_eq!(TimeFull::from(86399_u32),    "23 hours, 59 minutes, 59 seconds");
/// assert_eq!(TimeFull::from(86400_u32),    "1 day");
/// assert_eq!(TimeFull::from(86401_u32),    "1 day, 1 second");
/// assert_eq!(TimeFull::from(86402_u32),    "1 day, 2 seconds");
/// assert_eq!(TimeFull::from(86460_u32),    "1 day, 1 minute");
/// assert_eq!(TimeFull::from(86520_u32),    "1 day, 2 minutes");
/// assert_eq!(TimeFull::from(90000_u32),    "1 day, 1 hour");
/// assert_eq!(TimeFull::from(93600_u32),    "1 day, 2 hours");
/// assert_eq!(TimeFull::from(604799_u32),   "6 days, 23 hours, 59 minutes, 59 seconds");
/// assert_eq!(TimeFull::from(604800_u32),   "7 days");
/// assert_eq!(TimeFull::from(2630016_u32),  "1 month");
/// assert_eq!(TimeFull::from(3234815_u32),  "1 month, 6 days, 23 hours, 59 minutes, 59 seconds");
/// assert_eq!(TimeFull::from(5260032_u32),  "2 months");
/// assert_eq!(TimeFull::from(31557600_u32), "1 year");
/// assert_eq!(TimeFull::from(63115200_u32), "2 years");
/// assert_eq!(
///     TimeFull::from(u32::MAX),
///     "136 years, 1 month, 5 days, 19 hours, 54 minutes, 39 seconds",
/// );
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TimeFull(pub(super) u32, pub(super) Str<MAX_LEN_TIME_FULL>);

impl_math!(TimeFull, u32);
impl_traits!(TimeFull, u32);

//---------------------------------------------------------------------------------------------------- Pub Impl
impl TimeFull {
	impl_common!(u32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// ```rust
	/// # use readable::TimeFull;
	/// assert_eq!(TimeFull::zero(), 0_u32);
	/// assert_eq!(TimeFull::zero(), "0 seconds");
	/// ```
	pub const fn zero() -> Self {
		Self(ZERO_TIME_FULL.0, Str::from_static_str(ZERO_TIME_FULL.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::TimeFull;
	/// assert_eq!(TimeFull::second(), 1_u32);
	/// assert_eq!(TimeFull::second(), "1 second");
	/// ```
	pub const fn second() -> Self {
		Self(SECOND_TIME_FULL.0, Str::from_static_str(SECOND_TIME_FULL.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::TimeFull;
	/// assert_eq!(TimeFull::minute(), 60_u32);
	/// assert_eq!(TimeFull::minute(), "1 minute");
	/// ```
	pub const fn minute() -> Self {
		Self(MINUTE_TIME_FULL.0, Str::from_static_str(MINUTE_TIME_FULL.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::TimeFull;
	/// assert_eq!(TimeFull::hour(), 3600_u32);
	/// assert_eq!(TimeFull::hour(), "1 hour");
	/// ```
	pub const fn hour() -> Self {
		Self(HOUR_TIME_FULL.0, Str::from_static_str(HOUR_TIME_FULL.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::TimeFull;
	/// assert_eq!(TimeFull::day(), 86400_u32);
	/// assert_eq!(TimeFull::day(), "1 day");
	/// ```
	pub const fn day() -> Self {
		Self(DAY_TIME_FULL.0, Str::from_static_str(DAY_TIME_FULL.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::TimeFull;
	/// assert_eq!(TimeFull::month(), 2630016_u32);
	/// assert_eq!(TimeFull::month(), "1 month");
	/// ```
	pub const fn month() -> Self {
		Self(MONTH_TIME_FULL.0, Str::from_static_str(MONTH_TIME_FULL.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::TimeFull;
	/// assert_eq!(TimeFull::year(), 31557600_u32);
	/// assert_eq!(TimeFull::year(), "1 year");
	/// ```
	pub const fn year() -> Self {
		Self(YEAR_TIME_FULL.0, Str::from_static_str(YEAR_TIME_FULL.1))
	}

	#[inline]
	/// ```rust
	/// # use readable::TimeFull;
	/// assert_eq!(TimeFull::max(), u32::MAX);
	/// assert_eq!(TimeFull::max(), "136 years, 1 month, 5 days, 19 hours, 54 minutes, 39 seconds");
	/// ```
	pub const fn max() -> Self {
		Self(MAX_TIME_FULL.0, Str::from_static_str(MAX_TIME_FULL.1))
	}


	#[inline]
	/// ```rust
	/// # use readable::TimeFull;
	/// assert_eq!(TimeFull::unknown(), 0_u32);
	/// assert_eq!(TimeFull::unknown(), "???");
	/// ```
	pub const fn unknown() -> Self {
		Self(UNKNOWN_TIME_FULL.0, Str::from_static_str(UNKNOWN_TIME_FULL.1))
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl TimeFull {
	#[inline]
	fn plural(
		s: &mut Str<MAX_LEN_TIME_FULL>,
		name: &'static str,
		value: u32,
		started: &mut bool,
	) {
		if value > 0 {
			if *started {
				s.push_str_unchecked(", ");
			}
			s.push_str_unchecked(itoa!(value));
			s.push_str_unchecked(" ");
			s.push_str_unchecked(name);
			if value > 1 {
				s.push_str_unchecked("s");
			}
			*started = true;
		}
	}

	fn from_priv(secs: u32) -> Self {
		#[cfg(feature = "inline_time")]
		if secs <= 3660 {
			// SAFETY:
			// Cast `u64` to `u16` is safe because it's under 65_535.
			return Self(secs, CompactString::new_inline(readable_inlined_time::inlined(secs as u16)))
		}

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
		Self::plural(s, "year",   years,   started);
		Self::plural(s, "month",  months,  started);
		Self::plural(s, "day",    days,    started);
		Self::plural(s, "hour",   hours,   started);
		Self::plural(s, "minute", minutes, started);
		Self::plural(s, "second", seconds, started);

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
		impl From<$u> for TimeFull {
			#[inline]
			fn from(u: $u) -> Self {
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for TimeFull {
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
		impl From<$u> for TimeFull {
			#[inline]
			fn from(u: $u) -> Self {
				handle_over_u32!(u, $u);
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for TimeFull {
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
		impl From<$int> for TimeFull {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::unknown();
				}
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for TimeFull {
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
		impl From<$int> for TimeFull {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::unknown();
				}
				handle_over_u32!(int, $int);
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for TimeFull {
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
		impl From<$float> for TimeFull {
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
		impl From<&$float> for TimeFull {
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
impl From<Time> for TimeFull {
	#[inline]
	fn from(t: Time) -> Self {
		if t.1 == UNKNOWN_TIME.1 {
			return Self::unknown();
		}
		Self::from_priv(t.0)
	}
}
impl From<&Time> for TimeFull {
	#[inline]
	fn from(t: &Time) -> Self {
		if t.1 == UNKNOWN_TIME.1 {
			return Self::unknown();
		}
		Self::from_priv(t.0)
	}
}

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for TimeFull {
	fn from(duration: std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Duration> for TimeFull {
	fn from(duration: &std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<std::time::Instant> for TimeFull {
	fn from(instant: std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Instant> for TimeFull {
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
		while f < (MAX_TIME_FULL.0 as u64) {
			let t = TimeFull::from(f);
			println!("t: {t}, f: {f}");
			assert_eq!(t, f as u32);
			f *= 10;
		}
	}

	#[test]
	fn over() {
		assert_ne!(TimeFull::from(u32::MAX),            TimeFull::unknown());
		assert_eq!(TimeFull::from(u32::MAX as u64 + 1), TimeFull::unknown());
		assert_eq!(TimeFull::from(u64::MAX),            TimeFull::unknown());
		assert_eq!(TimeFull::from(f64::MAX),            TimeFull::unknown());
		assert_eq!(TimeFull::from(f32::MAX),            TimeFull::unknown());
	}

	#[test]
	fn special() {
		assert_eq!(TimeFull::from(f32::NAN),          TimeFull::unknown());
		assert_eq!(TimeFull::from(f32::INFINITY),     TimeFull::unknown());
		assert_eq!(TimeFull::from(f32::NEG_INFINITY), TimeFull::unknown());
		assert_eq!(TimeFull::from(f64::NAN),          TimeFull::unknown());
		assert_eq!(TimeFull::from(f64::INFINITY),     TimeFull::unknown());
		assert_eq!(TimeFull::from(f64::NEG_INFINITY), TimeFull::unknown());
	}
}
