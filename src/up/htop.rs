//---------------------------------------------------------------------------------------------------- Use
use crate::up::{Uptime,UptimeFull};
use crate::time::TimeUnit;
use crate::run::RuntimePad;
use crate::str::Str;
use crate::macros::{
	return_bad_float,impl_common,
	impl_const,impl_impl_math,impl_math,
	impl_usize,impl_traits,handle_over_u32,
};
use crate::itoa;

//---------------------------------------------------------------------------------------------------- Htop
/// [`htop`](https://github.com/htop-dev/htop)-style uptime formatting
///
/// This formats numbers into an "uptime"-style time format,
/// following the `htop` style (as of [this commit](https://github.com/htop-dev/htop/blob/ca41c25642a3ba26f001091189163da240923772/UptimeMeter.c)) exactly.
///
/// (Technically, `htop` uses a C `int` which will overflow at [`i32::MAX`] seconds
/// where as [`Htop`] uses a [`u32`] internally which will return [`Htop::UNKNOWN`] after [`u32::MAX`] seconds)
///
/// ## Formatting
/// The `htop`-style is exactly the same as [`RuntimePad`]...
/// ```text
/// 00:45:25
/// ```
/// ...until after 24 hours, at which point, a day count will prefix the runtime:
/// ```text
/// 1 day, 00:45:25
/// ```
/// When multiple days are involved, it is pluralized:
/// ```text
/// 2 days, 00:45:25
/// ```
///
/// After surpassing `100` days, a `(!)` will suffix the day count:
/// ```text
/// 101 days(!), 00:00:00
/// ```
///
/// ## Size
/// [`Str<23>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<Htop>(), 28);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::Htop;
/// assert_eq!(Htop::from(0_u32),       "00:00:00");
/// assert_eq!(Htop::from(1_u32),       "00:00:01");
/// assert_eq!(Htop::from(2_u32),       "00:00:02");
/// assert_eq!(Htop::from(59_u32),      "00:00:59");
/// assert_eq!(Htop::from(60_u32),      "00:01:00");
/// assert_eq!(Htop::from(3599_u32),    "00:59:59");
/// assert_eq!(Htop::from(3600_u32),    "01:00:00");
/// assert_eq!(Htop::from(86399_u32),   "23:59:59");
/// assert_eq!(Htop::from(86400_u32),   "1 day, 00:00:00");
/// assert_eq!(Htop::from(86401_u32),   "1 day, 00:00:01");
/// assert_eq!(Htop::from(90000_u32),   "1 day, 01:00:00");
/// assert_eq!(Htop::from(604799_u32),  "6 days, 23:59:59");
/// assert_eq!(Htop::from(2678400_u32), "31 days, 00:00:00");
/// assert_eq!(Htop::from(8553600_u32), "99 days, 00:00:00");
/// assert_eq!(Htop::from(8640000_u32), "100 days, 00:00:00");
/// assert_eq!(Htop::from(8726400_u32), "101 days(!), 00:00:00");
/// assert_eq!(Htop::from(u32::MAX),    "49710 days(!), 06:28:15");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Htop(pub(super) u32, pub(super) Str<{ Htop::MAX_LEN }>);

impl_math!(Htop, u32);
impl_traits!(Htop, u32);

//---------------------------------------------------------------------------------------------------- Constants
impl Htop {
	/// ```rust
	/// # use readable::*;
	/// let time = "49710 days(!), 06:28:15";
	/// assert_eq!(time.len(), Htop::MAX_LEN);
	/// ```
	pub const MAX_LEN: usize = 23;

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::UNKNOWN, 0);
	/// assert_eq!(Htop::UNKNOWN, "(unknown)");
	/// ```
	pub const UNKNOWN: Self = Self(0, Str::from_static_str("(unknown)"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::ZERO, 0);
	/// assert_eq!(Htop::ZERO, "00:00:00");
	/// assert_eq!(Htop::ZERO, Htop::from(0));
	/// ```
	pub const ZERO: Self = Self(0, Str::from_static_str("00:00:00"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::SECOND, 1);
	/// assert_eq!(Htop::SECOND, "00:00:01");
	/// assert_eq!(Htop::SECOND, Htop::from(1));
	/// ```
	pub const SECOND: Self = Self(1, Str::from_static_str("00:00:01"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::MINUTE, 60);
	/// assert_eq!(Htop::MINUTE, "00:01:00");
	/// assert_eq!(Htop::MINUTE, Htop::from(60));
	/// ```
	pub const MINUTE: Self = Self(60, Str::from_static_str("00:01:00"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::HOUR, 3600);
	/// assert_eq!(Htop::HOUR, "01:00:00");
	/// assert_eq!(Htop::HOUR, Htop::from(3600));
	/// ```
	pub const HOUR: Self = Self(3600, Str::from_static_str("01:00:00"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::DAY, 86400);
	/// assert_eq!(Htop::DAY, "1 day, 00:00:00");
	/// assert_eq!(Htop::DAY, Htop::from(86400));
	/// ```
	pub const DAY: Self = Self(86400, Str::from_static_str("1 day, 00:00:00"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::MONTH, 2678400);
	/// assert_eq!(Htop::MONTH, "31 days, 00:00:00");
	/// assert_eq!(Htop::MONTH, Htop::from(2678400));
	/// ```
	pub const MONTH: Self = Self(2678400, Str::from_static_str("31 days, 00:00:00"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::YEAR, 31536000);
	/// assert_eq!(Htop::YEAR, "365 days(!), 00:00:00");
	/// assert_eq!(Htop::YEAR, Htop::from(31536000));
	/// ```
	pub const YEAR: Self = Self(31536000, Str::from_static_str("365 days(!), 00:00:00"));

	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::MAX, u32::MAX);
	/// assert_eq!(Htop::MAX, "49710 days(!), 06:28:15");
	/// assert_eq!(Htop::MAX, Htop::from(u32::MAX));
	/// ```
	pub const MAX: Self = Self(u32::MAX, Str::from_static_str("49710 days(!), 06:28:15"));
}

//---------------------------------------------------------------------------------------------------- Pub Impl
impl Htop {
	impl_common!(u32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::zero(), Htop::ZERO);
	/// ```
	pub const fn zero() -> Self {
		Self::ZERO
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::second(), Htop::SECOND);
	/// ```
	pub const fn second() -> Self {
		Self::SECOND
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::minute(), Htop::MINUTE);
	/// ```
	pub const fn minute() -> Self {
		Self::MINUTE
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::hour(), Htop::HOUR);
	/// ```
	pub const fn hour() -> Self {
		Self::HOUR
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::day(), Htop::DAY);
	/// ```
	pub const fn day() -> Self {
		Self::DAY
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::month(), Htop::MONTH);
	/// ```
	pub const fn month() -> Self {
		Self::MONTH
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::year(), Htop::YEAR);
	/// ```
	pub const fn year() -> Self {
		Self::YEAR
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::max(), Htop::MAX);
	/// ```
	pub const fn max() -> Self {
		Self::MAX
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(Htop::unknown(), Htop::UNKNOWN);
	/// ```
	pub const fn unknown() -> Self {
		Self::UNKNOWN
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(Htop::UNKNOWN.is_unknown());
	/// assert!(!Htop::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		match *self {
			Self::UNKNOWN => true,
			_ => false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl Htop {
	#[inline]
	fn from_priv(secs: u32) -> Self {
		if secs == 0 {
			return Self::zero();
		}

		let days = secs / 86400;
		let mut string = Str::new();

		if days > 0 {
			string.push_str_panic(itoa!(days));

			if days > 100 {
				string.push_str_panic(" days(!), ");
			} else if days > 1 {
				string.push_str_panic(" days, ");
			} else if days == 1 {
				string.push_str_panic(" day, ");
			}
		};

		let runtime = RuntimePad::from(secs % 86400);
		string.push_str_panic(runtime);

		Self(secs, string)
	}
}

//---------------------------------------------------------------------------------------------------- Other Uptime Impl.
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
impl_from_time!(Htop => Uptime, UptimeFull, TimeUnit);

//---------------------------------------------------------------------------------------------------- "u*" impl
// Implementation Macro.
macro_rules! impl_u {
	($($u:ty),* $(,)?) => { $(
		impl From<$u> for Htop {
			#[inline]
			fn from(u: $u) -> Self {
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for Htop {
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
		impl From<$u> for Htop {
			#[inline]
			fn from(u: $u) -> Self {
				handle_over_u32!(u, $u);
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for Htop {
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
		impl From<$int> for Htop {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::unknown();
				}
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for Htop {
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
		impl From<$int> for Htop {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::unknown();
				}
				handle_over_u32!(int, $int);
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for Htop {
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
		impl From<$float> for Htop {
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
		impl From<&$float> for Htop {
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

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for Htop {
	#[inline]
	fn from(duration: std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Duration> for Htop {
	#[inline]
	fn from(duration: &std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<std::time::Instant> for Htop {
	#[inline]
	fn from(instant: std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<&std::time::Instant> for Htop {
	#[inline]
	fn from(instant: &std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::from_priv(u as u32)
	}
}

impl From<Htop> for std::time::Duration {
	#[inline]
	fn from(value: Htop) -> Self {
		std::time::Duration::from_secs(value.inner() as u64)
	}
}

impl From<&Htop> for std::time::Duration {
	#[inline]
	fn from(value: &Htop) -> Self {
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
		while f < (Htop::MAX.0 as u64) {
			let t = Htop::from(f);
			println!("t: {t}, f: {f}");
			assert_eq!(t, f as u32);
			f *= 10;
		}
	}

	#[test]
	fn over() {
		assert_ne!(Htop::from(u32::MAX),            Htop::unknown());
		assert_eq!(Htop::from(u32::MAX as u64 + 1), Htop::unknown());
		assert_eq!(Htop::from(u64::MAX),            Htop::unknown());
		assert_eq!(Htop::from(f64::MAX),            Htop::unknown());
		assert_eq!(Htop::from(f32::MAX),            Htop::unknown());
	}

	#[test]
	fn special() {
		assert_eq!(Htop::from(f32::NAN),          Htop::unknown());
		assert_eq!(Htop::from(f32::INFINITY),     Htop::unknown());
		assert_eq!(Htop::from(f32::NEG_INFINITY), Htop::unknown());
		assert_eq!(Htop::from(f64::NAN),          Htop::unknown());
		assert_eq!(Htop::from(f64::INFINITY),     Htop::unknown());
		assert_eq!(Htop::from(f64::NEG_INFINITY), Htop::unknown());
	}
}
