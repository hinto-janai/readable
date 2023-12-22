//---------------------------------------------------------------------------------------------------- Use
use crate::up::{Uptime,Htop};
use crate::time::TimeUnit;
use crate::str::Str;
use crate::macros::{
	return_bad_float,impl_common,
	impl_const,impl_impl_math,impl_math,
	impl_usize,impl_traits,handle_over_u32,
};
use crate::itoa;

//---------------------------------------------------------------------------------------------------- UptimeFull
/// [`Uptime`] but with full specified words
///
/// This is the same type as [`Uptime`], except, the
/// words specifying the time will not be abbreviated
/// and will be pluralized, e.g:
/// ```rust
/// # use readable::*;
/// assert_eq!(UptimeFull::from(0), "0 seconds");
/// assert_eq!(UptimeFull::from(1), "1 second");
/// assert_eq!(UptimeFull::from(2), "2 seconds");
/// ```
///
/// ## Size
/// [`Str<63>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<UptimeFull>(), 68);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::*;
/// assert_eq!(UptimeFull::from(0_u32),        "0 seconds");
/// assert_eq!(UptimeFull::from(1_u32),        "1 second");
/// assert_eq!(UptimeFull::from(2_u32),        "2 seconds");
/// assert_eq!(UptimeFull::from(59_u32),       "59 seconds");
/// assert_eq!(UptimeFull::from(60_u32),       "1 minute");
/// assert_eq!(UptimeFull::from(61_u32),       "1 minute, 1 second");
/// assert_eq!(UptimeFull::from(62_u32),       "1 minute, 2 seconds");
/// assert_eq!(UptimeFull::from(120_u32),      "2 minutes");
/// assert_eq!(UptimeFull::from(121_u32),      "2 minutes, 1 second");
/// assert_eq!(UptimeFull::from(122_u32),      "2 minutes, 2 seconds");
/// assert_eq!(UptimeFull::from(179_u32),      "2 minutes, 59 seconds");
/// assert_eq!(UptimeFull::from(3599_u32),     "59 minutes, 59 seconds");
/// assert_eq!(UptimeFull::from(3600_u32),     "1 hour");
/// assert_eq!(UptimeFull::from(3601_u32),     "1 hour, 1 second");
/// assert_eq!(UptimeFull::from(3602_u32),     "1 hour, 2 seconds");
/// assert_eq!(UptimeFull::from(3660_u32),     "1 hour, 1 minute");
/// assert_eq!(UptimeFull::from(3720_u32),     "1 hour, 2 minutes");
/// assert_eq!(UptimeFull::from(86399_u32),    "23 hours, 59 minutes, 59 seconds");
/// assert_eq!(UptimeFull::from(86400_u32),    "1 day");
/// assert_eq!(UptimeFull::from(86401_u32),    "1 day, 1 second");
/// assert_eq!(UptimeFull::from(86402_u32),    "1 day, 2 seconds");
/// assert_eq!(UptimeFull::from(86460_u32),    "1 day, 1 minute");
/// assert_eq!(UptimeFull::from(86520_u32),    "1 day, 2 minutes");
/// assert_eq!(UptimeFull::from(90000_u32),    "1 day, 1 hour");
/// assert_eq!(UptimeFull::from(93600_u32),    "1 day, 2 hours");
/// assert_eq!(UptimeFull::from(604799_u32),   "6 days, 23 hours, 59 minutes, 59 seconds");
/// assert_eq!(UptimeFull::from(604800_u32),   "7 days");
/// assert_eq!(UptimeFull::from(2678400_u32),  "1 month");
/// assert_eq!(UptimeFull::from(3283199_u32),  "1 month, 6 days, 23 hours, 59 minutes, 59 seconds");
/// assert_eq!(UptimeFull::from(5356800_u32),  "2 months");
/// assert_eq!(UptimeFull::from(31536000_u32), "1 year");
/// assert_eq!(UptimeFull::from(63072000_u32), "2 years");
/// assert_eq!(
///     UptimeFull::from(u32::MAX),
///     "136 years, 2 months, 8 days, 6 hours, 28 minutes, 15 seconds",
/// );
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UptimeFull(pub(super) u32, pub(super) Str<{ UptimeFull::MAX_LEN }>);

impl_math!(UptimeFull, u32);
impl_traits!(UptimeFull, u32);

//---------------------------------------------------------------------------------------------------- Constants
impl UptimeFull {
	/// ```rust
	/// # use readable::*;
	/// let time = "--- years, -- months, -- days, -- hours, -- minutes, -- seconds";
	/// assert_eq!(time.len(), UptimeFull::MAX_LEN);
	/// ```
	pub const MAX_LEN: usize = 63;

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(UptimeFull::UNKNOWN, 0);
	/// assert_eq!(UptimeFull::UNKNOWN, "(unknown)");
	/// ```
	pub const UNKNOWN: Self = Self(0, Str::from_static_str("(unknown)"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(UptimeFull::ZERO, 0);
	/// assert_eq!(UptimeFull::ZERO, "0 seconds");
	/// ```
	pub const ZERO: Self = Self(0, Str::from_static_str("0 seconds"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(UptimeFull::SECOND, 1);
	/// assert_eq!(UptimeFull::SECOND, "1 second");
	/// ```
	pub const SECOND: Self = Self(1, Str::from_static_str("1 second"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(UptimeFull::MINUTE, 60);
	/// assert_eq!(UptimeFull::MINUTE, "1 minute");
	/// ```
	pub const MINUTE: Self = Self(60, Str::from_static_str("1 minute"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(UptimeFull::HOUR, 3600);
	/// assert_eq!(UptimeFull::HOUR, "1 hour");
	/// ```
	pub const HOUR: Self = Self(3600, Str::from_static_str("1 hour"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(UptimeFull::DAY, 86400);
	/// assert_eq!(UptimeFull::DAY, "1 day");
	/// ```
	pub const DAY: Self = Self(86400, Str::from_static_str("1 day"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(UptimeFull::MONTH, 86400);
	/// assert_eq!(UptimeFull::MONTH, "1 month");
	/// ```
	pub const MONTH: Self = Self(86400, Str::from_static_str("1 month"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(UptimeFull::YEAR, 31_536_000);
	/// assert_eq!(UptimeFull::YEAR, "1 year");
	/// ```
	pub const YEAR: Self = Self(31_536_000, Str::from_static_str("1 year"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(UptimeFull::MAX, u32::MAX);
	/// assert_eq!(UptimeFull::MAX, "136 years, 2 months, 8 days, 6 hours, 28 minutes, 15 seconds");
	/// ```
	pub const MAX: Self = Self(u32::MAX, Str::from_static_str("136 years, 2 months, 8 days, 6 hours, 28 minutes, 15 seconds"));
}

//---------------------------------------------------------------------------------------------------- Pub Impl
impl UptimeFull {
	impl_common!(u32);
	impl_const!();
	impl_usize!();

	#[inline]
	#[must_use]
	/// ```rust
	/// # use readable::*;
	/// assert!(UptimeFull::UNKNOWN.is_unknown());
	/// assert!(!UptimeFull::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		matches!(*self, Self::UNKNOWN)
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl UptimeFull {
	#[inline]
	fn plural(
		s: &mut Str<{ Self::MAX_LEN }>,
		name: &'static str,
		value: u32,
		started: &mut bool,
	) {
		if value > 0 {
			if *started {
				s.push_str_panic(", ");
			}
			s.push_str_panic(itoa!(value));
			s.push_str_panic(" ");
			s.push_str_panic(name);
			if value > 1 {
				s.push_str_panic("s");
			}
			*started = true;
		}
	}

	fn from_priv(secs: u32) -> Self {
		// #[cfg(feature = "inline_time")]
		// if secs <= 3660 {
		// 	// SAFETEE:
		// 	// Cast `u64` to `u16` is safe because it's under 65_535.
		// 	return Self(secs, CompactString::new_inline(readable_inlined_time::inlined(secs as u16)))
		// }

		if secs == 0 {
			return Self::ZERO;
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
// Implementation Macro.
macro_rules! impl_u {
	($($u:ty),* $(,)?) => { $(
		impl From<$u> for UptimeFull {
			#[inline]
			fn from(u: $u) -> Self {
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for UptimeFull {
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
		impl From<$u> for UptimeFull {
			#[inline]
			fn from(u: $u) -> Self {
				handle_over_u32!(u, $u);
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for UptimeFull {
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
		impl From<$int> for UptimeFull {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for UptimeFull {
			#[inline]
			fn from(int: &$int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
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
		impl From<$int> for UptimeFull {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				handle_over_u32!(int, $int);
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for UptimeFull {
			#[inline]
			fn from(int: &$int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
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
		impl From<$float> for UptimeFull {
			#[inline]
			fn from(float: $float) -> Self {
				return_bad_float!(float, Self::UNKNOWN, Self::UNKNOWN);
				if float.is_sign_negative() {
					return Self::UNKNOWN;
				}
				handle_over_u32!(float, $float);
				Self::from_priv(float as u32)
			}
		}
		impl From<&$float> for UptimeFull {
			#[inline]
			fn from(float: &$float) -> Self {
				return_bad_float!(float, Self::UNKNOWN, Self::UNKNOWN);
				if float.is_sign_negative() {
					return Self::UNKNOWN;
				}
				handle_over_u32!(*float, $float);
				Self::from_priv(*float as u32)
			}
		}
	}
}
impl_f!(f32);
impl_f!(f64);

//---------------------------------------------------------------------------------------------------- Other Uptime Impl.
macro_rules! impl_from_time {
	($this:ty => $($other:ty),* $(,)?) => { $(
		impl From<$other> for $this {
			#[inline]
			fn from(from: $other) -> Self {
				if from.is_unknown() {
					Self::UNKNOWN
				} else {
					Self::from_priv(from.inner())
				}
			}
		}
		impl From<&$other> for $this {
			#[inline]
			fn from(from: &$other) -> Self {
				if from.is_unknown() {
					Self::UNKNOWN
				} else {
					Self::from_priv(from.inner())
				}
			}
		}
	)*}
}
impl_from_time!(UptimeFull => Uptime, Htop, TimeUnit);

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for UptimeFull {
	#[inline]
	fn from(duration: std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Duration> for UptimeFull {
	#[inline]
	fn from(duration: &std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<std::time::Instant> for UptimeFull {
	#[inline]
	fn from(instant: std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Instant> for UptimeFull {
	#[inline]
	fn from(instant: &std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<UptimeFull> for std::time::Duration {
	#[inline]
	fn from(value: UptimeFull) -> Self {
		Self::from_secs(value.inner().into())
	}
}

impl From<&UptimeFull> for std::time::Duration {
	#[inline]
	fn from(value: &UptimeFull) -> Self {
		Self::from_secs(value.inner().into())
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn all_ints() {
		let mut f = 1_u64;
		while f < (u64::from(UptimeFull::MAX.0)) {
			let t = UptimeFull::from(f);
			println!("t: {t}, f: {f}");
			assert_eq!(t, f as u32);
			f *= 10;
		}
	}

	#[test]
	fn over() {
		assert_ne!(UptimeFull::from(u32::MAX),                UptimeFull::UNKNOWN);
		assert_eq!(UptimeFull::from(u64::from(u32::MAX) + 1), UptimeFull::UNKNOWN);
		assert_eq!(UptimeFull::from(u64::MAX),                UptimeFull::UNKNOWN);
		assert_eq!(UptimeFull::from(f64::MAX),                UptimeFull::UNKNOWN);
		assert_eq!(UptimeFull::from(f32::MAX),                UptimeFull::UNKNOWN);
	}

	#[test]
	fn special() {
		assert_eq!(UptimeFull::from(f32::NAN),          UptimeFull::UNKNOWN);
		assert_eq!(UptimeFull::from(f32::INFINITY),     UptimeFull::UNKNOWN);
		assert_eq!(UptimeFull::from(f32::NEG_INFINITY), UptimeFull::UNKNOWN);
		assert_eq!(UptimeFull::from(f64::NAN),          UptimeFull::UNKNOWN);
		assert_eq!(UptimeFull::from(f64::INFINITY),     UptimeFull::UNKNOWN);
		assert_eq!(UptimeFull::from(f64::NEG_INFINITY), UptimeFull::UNKNOWN);
	}
}
