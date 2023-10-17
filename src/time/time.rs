//---------------------------------------------------------------------------------------------------- Use
use crate::time::{Htop,TimeFull,TimeUnit};
use crate::str::Str;
use crate::macros::{
	return_bad_float,impl_common,
	impl_const,impl_impl_math,impl_math,
	impl_usize,impl_traits,
};
use crate::itoa;

//---------------------------------------------------------------------------------------------------- Time
/// Human-readable time
///
/// This formats numbers into an "uptime"-style time format,
/// suffixed with a single letter indicated the unit.
///
/// ## Size
/// [`Str<29>`] is used internally to represent the string.
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
/// assert_eq!(Time::month().inner(),  2678400);
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
/// assert_eq!(Time::from(2678400_u32),  "1m");
/// assert_eq!(Time::from(3283199_u32),  "1m, 6d, 23h, 59m, 59s");
/// assert_eq!(Time::from(5356800_u32),  "2m");
/// assert_eq!(Time::from(31536000_u32), "1y");
/// assert_eq!(Time::from(63072000_u32), "2y");
/// println!("{}", Time::from(u32::MAX));
/// assert_eq!(
///     Time::from(u32::MAX),
///     "136y, 2m, 8d, 6h, 28m, 15s",
/// );
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Time(pub(super) u32, pub(super) Str<{ Time::MAX_LEN }>);

impl_math!(Time, u32);
impl_traits!(Time, u32);

//---------------------------------------------------------------------------------------------------- Constants
impl Time {
	/// ```rust
	/// # use readable::*;
	/// let time = "---y, --m, --d, --h, --m, --s";
	/// assert_eq!(time.len(), Time::MAX_LEN);
	/// ```
	pub const MAX_LEN: usize = 29;

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::UNKNOWN, 0);
	/// assert_eq!(Time::UNKNOWN, "(unknown)");
	/// ```
	pub const UNKNOWN: Self = Self(0, Str::from_static_str("(unknown)"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::ZERO, 0);
	/// assert_eq!(Time::ZERO, "0s");
	/// ```
	pub const ZERO: Self = Self(0, Str::from_static_str("0s"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::SECOND, 1);
	/// assert_eq!(Time::SECOND, "1s");
	/// ```
	pub const SECOND: Self = Self(1, Str::from_static_str("1s"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::MINUTE, 60);
	/// assert_eq!(Time::MINUTE, "1m");
	/// ```
	pub const MINUTE: Self = Self(60, Str::from_static_str("1m"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::HOUR, 3600);
	/// assert_eq!(Time::HOUR, "1h");
	/// ```
	pub const HOUR: Self = Self(3600, Str::from_static_str("1h"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::DAY, 86400);
	/// assert_eq!(Time::DAY, "1d");
	/// ```
	pub const DAY: Self = Self(86400, Str::from_static_str("1d"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::MONTH, 2678400);
	/// assert_eq!(Time::MONTH, "1m");
	/// ```
	pub const MONTH: Self = Self(2678400, Str::from_static_str("1m"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::YEAR, 31536000);
	/// assert_eq!(Time::YEAR, "1y");
	/// ```
	pub const YEAR: Self = Self(31536000, Str::from_static_str("1y"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::MAX, u32::MAX);
	/// assert_eq!(Time::MAX, "136y, 2m, 8d, 6h, 28m, 15s");
	/// ```
	pub const MAX: Self = Self(u32::MAX, Str::from_static_str("136y, 2m, 8d, 6h, 28m, 15s"));
}

//---------------------------------------------------------------------------------------------------- Pub Impl
impl Time {
	impl_common!(u32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::zero(), Time::ZERO);
	/// ```
	pub const fn zero() -> Self {
		Self::ZERO
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::second(), Time::SECOND);
	/// ```
	pub const fn second() -> Self {
		Self::SECOND
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::minute(), Time::MINUTE);
	/// ```
	pub const fn minute() -> Self {
		Self::MINUTE
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::hour(), Time::HOUR);
	/// ```
	pub const fn hour() -> Self {
		Self::HOUR
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::day(), Time::DAY);
	/// ```
	pub const fn day() -> Self {
		Self::DAY
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::month(), Time::MONTH);
	/// ```
	pub const fn month() -> Self {
		Self::MONTH
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::year(), Time::YEAR);
	/// ```
	pub const fn year() -> Self {
		Self::YEAR
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::max(), Time::MAX);
	/// ```
	pub const fn max() -> Self {
		Self::MAX
	}


	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Time::unknown(), Time::UNKNOWN);
	/// ```
	pub const fn unknown() -> Self {
		Self::UNKNOWN
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(Time::UNKNOWN.is_unknown());
	/// assert!(!Time::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		match *self {
			Self::UNKNOWN => true,
			_ => false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl Time {
	#[inline]
	fn plural(
		s: &mut Str<{ Time::MAX_LEN }>,
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

		let years    = secs / 31_536_000;  // 365 days
		let ydays    = secs % 31_536_000;
		let months   = ydays / 2_678_400;  // 31 days
		let mdays    = ydays % 2_678_400;
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

//---------------------------------------------------------------------------------------------------- Other Time Impl.
macro_rules! impl_from_time {
	($this:ty => $($other:ty),* $(,)?) => { $(
		impl From<$other> for $this {
			#[inline]
			fn from(from: $other) -> Self {
				if from.is_unknown() {
					Self::unknown()
				} else {
					Self::from_priv(from.inner())
				}
			}
		}
		impl From<&$other> for $this {
			#[inline]
			fn from(from: &$other) -> Self {
				if from.is_unknown() {
					Self::unknown()
				} else {
					Self::from_priv(from.inner())
				}
			}
		}
	)*}
}
impl_from_time!(Time => TimeFull, Htop, TimeUnit);

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for Time {
	#[inline]
	fn from(duration: std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Duration> for Time {
	#[inline]
	fn from(duration: &std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<std::time::Instant> for Time {
	#[inline]
	fn from(instant: std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Instant> for Time {
	#[inline]
	fn from(instant: &std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<Time> for std::time::Duration {
	#[inline]
	fn from(value: Time) -> Self {
		std::time::Duration::from_secs(value.inner() as u64)
	}
}

impl From<&Time> for std::time::Duration {
	#[inline]
	fn from(value: &Time) -> Self {
		std::time::Duration::from_secs(value.inner() as u64)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn all_ints() {
		let mut f = 1_u64;
		while f < (Time::MAX.0 as u64) {
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
