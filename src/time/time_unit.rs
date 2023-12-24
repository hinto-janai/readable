//---------------------------------------------------------------------------------------------------- Use
use crate::up::{Uptime,Htop,UptimeFull};
use crate::time::{Time,Military};
use crate::macros::{
	return_bad_float,impl_impl_math,impl_math,
};
use crate::num::Unsigned;

//---------------------------------------------------------------------------------------------------- Use
/// Unit of time
///
/// Unlike most `readable` types, this struct is not a string;
/// it is a utility type that takes seconds as input and calculates
/// the different units of time from it.
///
/// For example:
/// ```rust
/// # use readable::time::*;
/// assert_eq!(TimeUnit::from(1).seconds(), 1);
/// assert_eq!(TimeUnit::from(60).minutes(), 1);
/// assert_eq!(TimeUnit::from(3600).hours(), 1);
/// assert_eq!(TimeUnit::from(86400).days(), 1);
/// assert_eq!(TimeUnit::from(86400 * 7).weeks(), 1);
/// assert_eq!(TimeUnit::from(86400 * 31).months(), 1);
/// assert_eq!(TimeUnit::from(86400 * 365).years(), 1);
/// ```
///
/// If a unit overflows it will carry over to the lower unit, for example:
/// ```rust
/// # use readable::time::*;
/// let unit = TimeUnit::from(
///     31536000 + // 1 year
///     5356800  + // 2 months
///     1814400  + // 3 weeks
///     345600   + // 4 days
///     18000    + // 5 hours
///     360      + // 6 minutes
///     7          // 7 seconds
/// );
///
/// assert_eq!(unit.years(),   1);
/// assert_eq!(unit.months(),  2);
/// assert_eq!(unit.weeks(),   3);
/// assert_eq!(unit.days(),    4);
/// assert_eq!(unit.hours(),   5);
/// assert_eq!(unit.minutes(), 6);
/// assert_eq!(unit.seconds(), 7);
///
/// // Total amount of seconds.
/// assert_eq!(unit.inner(), 39071167);
/// ```
///
/// ## Size
/// ```rust
/// # use readable::time::*;
/// assert_eq!(std::mem::size_of::<TimeUnit>(), 12);
/// ```
///
/// ## Uptime & Conversion
/// Like the other `readable::time` types, [`TimeUnit`] implements [`Uptime`]
/// and can be losslessly convert from/into other `readable::time` types, even
/// maintaining `unknown` variants:
///
/// ```rust
/// # use readable::{time::*,up::*};
/// // Uptime
/// let time = Uptime::from(86461);
/// assert_eq!(time, "1d, 1m, 1s");
///
/// // TimeUnit
/// let unit = TimeUnit::from(time);
/// assert_eq!(unit.inner(),   86461);
/// assert_eq!(unit.years(),   0);
/// assert_eq!(unit.months(),  0);
/// assert_eq!(unit.weeks(),   0);
/// assert_eq!(unit.days(),    1);
/// assert_eq!(unit.hours(),   0);
/// assert_eq!(unit.minutes(), 1);
/// assert_eq!(unit.seconds(), 1);
///
/// // Maintain the `unknown` variant.
/// let time: Uptime   = Uptime::UNKNOWN;
/// let unit: TimeUnit = TimeUnit::from(time);
/// assert!(unit.is_unknown());
/// let time: Uptime = Uptime::from(unit);
/// assert!(time.is_unknown());
/// ```
///
/// ## Naive time
/// Like the othear `readable::time` types, [`TimeUnit`] naively assumes that:
/// 1. Each day is `86400` seconds
/// 2. Each month is `31` days
/// 3. Each year is `365` days
///
/// This is incorrect as not all months are 31 days long and leap years exist.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TimeUnit {
	unknown: bool,
	inner: u32,
	years: u8,
	months: u8,
	weeks: u8,
	days: u8,
	hours: u8,
	minutes: u8,
	seconds: u8,
}

impl_math!(TimeUnit, u32);

//---------------------------------------------------------------------------------------------------- Constants
impl TimeUnit {
	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::UNKNOWN.inner(),   0);
	/// assert_eq!(TimeUnit::UNKNOWN.years(),   0);
	/// assert_eq!(TimeUnit::UNKNOWN.months(),  0);
	/// assert_eq!(TimeUnit::UNKNOWN.weeks(),   0);
	/// assert_eq!(TimeUnit::UNKNOWN.days(),    0);
	/// assert_eq!(TimeUnit::UNKNOWN.hours(),   0);
	/// assert_eq!(TimeUnit::UNKNOWN.minutes(), 0);
	/// assert_eq!(TimeUnit::UNKNOWN.seconds(), 0);
	/// assert_eq!(TimeUnit::UNKNOWN, TimeUnit::from(-1));
	/// assert_eq!(TimeUnit::UNKNOWN, TimeUnit::from(u64::MAX));
	/// assert_eq!(TimeUnit::UNKNOWN, TimeUnit::from(f32::NAN));
	/// assert!(TimeUnit::UNKNOWN.is_unknown());
	/// ```
	pub const UNKNOWN: Self = Self { inner: 0, unknown: true, years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 };
		/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::ZERO.inner(), 0);
	/// assert_eq!(TimeUnit::ZERO, TimeUnit::from(0));
	/// ```
	pub const ZERO: Self = Self { inner: 0, unknown: false, years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 };

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::SECOND.inner(), 1);
	/// assert_eq!(TimeUnit::SECOND.seconds(), 1);
	/// assert_eq!(TimeUnit::SECOND, TimeUnit::from(1));
	/// ```
	pub const SECOND: Self = Self { inner: 1, unknown: false, years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 1 };

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::MINUTE.inner(), 60);
	/// assert_eq!(TimeUnit::MINUTE.minutes(), 1);
	/// assert_eq!(TimeUnit::MINUTE, TimeUnit::from(60));
	/// ```
	pub const MINUTE: Self = Self { inner: 60, unknown: false, years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 1, seconds: 0 };

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::HOUR.inner(), 3600);
	/// assert_eq!(TimeUnit::HOUR.hours(), 1);
	/// assert_eq!(TimeUnit::HOUR, TimeUnit::from(3600));
	/// ```
	pub const HOUR: Self = Self { inner: 3600, unknown: false, years: 0, months: 0, weeks: 0, days: 0, hours: 1, minutes: 0, seconds: 0 };

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::DAY.inner(), 86400);
	/// assert_eq!(TimeUnit::DAY.days(), 1);
	/// assert_eq!(TimeUnit::DAY, TimeUnit::from(86400));
	/// ```
	pub const DAY: Self = Self { inner: 86400, unknown: false, years: 0, months: 0, weeks: 0, days: 1, hours: 0, minutes: 0, seconds: 0 };

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::WEEK.inner(), 604800);
	/// assert_eq!(TimeUnit::WEEK.weeks(), 1);
	/// assert_eq!(TimeUnit::WEEK, TimeUnit::from(604800));
	/// ```
	pub const WEEK: Self = Self { inner: 604800, unknown: false, years: 0, months: 0, weeks: 1, days: 0, hours: 0, minutes: 0, seconds: 0 };

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::MONTH.inner(), 2678400);
	/// assert_eq!(TimeUnit::MONTH.months(), 1);
	/// assert_eq!(TimeUnit::MONTH, TimeUnit::from(2678400));
	/// ```
	pub const MONTH: Self = Self { inner: 2678400, unknown: false, years: 0, months: 1, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 };

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::YEAR.inner(), 31536000);
	/// assert_eq!(TimeUnit::YEAR.years(), 1);
	/// assert_eq!(TimeUnit::YEAR, TimeUnit::from(31536000));
	/// ```
	pub const YEAR: Self = Self { inner: 31536000, unknown: false, years: 1, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 };

	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::MAX.inner(),   u32::MAX);
	/// assert_eq!(TimeUnit::MAX.years(),   136);
	/// assert_eq!(TimeUnit::MAX.months(),  2);
	/// assert_eq!(TimeUnit::MAX.weeks(),   1);
	/// assert_eq!(TimeUnit::MAX.days(),    1);
	/// assert_eq!(TimeUnit::MAX.hours(),   6);
	/// assert_eq!(TimeUnit::MAX.minutes(), 28);
	/// assert_eq!(TimeUnit::MAX.seconds(), 15);
	/// assert_eq!(TimeUnit::MAX, TimeUnit::from(u32::MAX));
	/// ```
	pub const MAX: Self = Self { inner: u32::MAX, unknown: false, years: 136, months: 2, weeks: 1, days: 1, hours: 6, minutes: 28, seconds: 15 };
}

//---------------------------------------------------------------------------------------------------- Pub Impl
impl TimeUnit {
}

//---------------------------------------------------------------------------------------------------- Construction Impl
impl TimeUnit {
	#[inline]
	#[must_use]
	/// Create a new [`TimeUnit`] from seconds as input.
	///
	/// This will divide and use the remainder to calculate each unit,
	/// for example `62` as input would lead to `1 minute` and `2 seconds`.
	///
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::from(62);
	/// assert_eq!(unit.minutes(), 1);
	/// assert_eq!(unit.seconds(), 2);
	/// ```
	pub const fn new(secs: u32) -> Self {
		if secs == 0 {
			return Self::ZERO;
		}

		let years     = secs / 31_536_000;  // 365 days
		let years_rem = secs % 31_536_000;

		let months     = years_rem / 2_678_400;  // 31 days
		let months_rem = years_rem % 2_678_400;

		let weeks     = months_rem / 604_800; // 7 days
		let weeks_rem = months_rem % 604_800;

		let days     = weeks_rem / 86_400; // 24 hours
		let days_rem = weeks_rem % 86_400;

		let hours     = days_rem / 3_600; // 60 minutes
		let hours_rem = days_rem % 3_600;

		let minutes = hours_rem / 60; // 60 seconds
		let seconds = hours_rem % 60;

		Self {
			unknown: false,
			inner: secs,
			years: years as u8,
			months: months as u8,
			weeks: weeks as u8,
			days: days as u8,
			hours: hours as u8,
			minutes: minutes as u8,
			seconds: seconds as u8,
		}
	}

	#[inline]
	#[must_use]
	/// Create [`Self`] with minutes as input
	///
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::from_minutes(1);
	/// assert_eq!(unit.inner(), 60);
	/// ```
	///
	/// ## Maximum Input
	/// The maximum input is `71_582_788` minutes before this function saturates.
	pub const fn from_minutes(minutes: u32) -> Self { Self::new(minutes.saturating_mul(60)) }

	#[inline]
	#[must_use]
	/// Create [`Self`] with hours as input
	///
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::from_hours(1);
	/// assert_eq!(unit.inner(), 3_600);
	/// ```
	///
	/// ## Maximum Input
	/// The maximum input is `1_193_046` hours before this function saturates.
	pub const fn from_hours(hours: u32) -> Self { Self::new(hours.saturating_mul(3_600)) }

	#[inline]
	#[must_use]
	/// Create [`Self`] with days as input
	///
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::from_days(1);
	/// assert_eq!(unit.inner(), 86_400);
	/// ```
	///
	/// ## Maximum Input
	/// The maximum input is `49_710` days before this function saturates.
	pub const fn from_days(days: u16) -> Self { Self::new((days as u32).saturating_mul(86_400)) }

	#[inline]
	#[must_use]
	/// Create [`Self`] with weeks as input
	///
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::from_weeks(1);
	/// assert_eq!(unit.inner(), 604_800);
	/// ```
	///
	/// ## Maximum Input
	/// The maximum input is `7_101` weeks before this function saturates.
	pub const fn from_weeks(weeks: u16) -> Self { Self::new((weeks as u32).saturating_mul(604_800)) }

	#[inline]
	#[must_use]
	/// Create [`Self`] with months as input
	///
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::from_months(1);
	/// assert_eq!(unit.inner(), 2_678_400);
	/// ```
	///
	/// ## Maximum Input
	/// The maximum input is `1_603` months before this function saturates.
	pub const fn from_months(months: u16) -> Self { Self::new((months as u32).saturating_mul(2_678_400)) }

	#[inline]
	#[must_use]
	/// Create [`Self`] with years as input
	///
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::from_years(1);
	/// assert_eq!(unit.inner(), 31_536_000);
	/// ```
	///
	/// ## Maximum Input
	/// The maximum input is `136` years before this function saturates.
	pub const fn from_years(years: u8) -> Self { Self::new((years as u32).saturating_mul(31_536_000)) }

	#[inline]
	#[must_use]
	/// Create a new [`TimeUnit`] from a variety of input
	///
	/// This multiplies and combines all the input.
	///
	/// ## Maximum Input
	/// If the total _second_ count of all the inputs combined exceeds [`u32::MAX`]
	/// then this function will saturate and return [`TimeUnit::MAX`].
	///
	/// ## Examples
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::new_variety(
	///     0, // years   (0s)
	///     1, // months  (2678400s)
	///     2, // weeks   (1209600s)
	///     3, // days    (259200s)
	///     4, // hours   (14400s)
	///     5, // minutes (300s)
	///     6, // seconds (6s)
	/// );
	///
	/// // Total second count: 4,161,906 seconds
	/// assert_eq!(unit.inner(), 4_161_906);
	/// assert_eq!(unit.years(),   0);
	/// assert_eq!(unit.months(),  1);
	/// assert_eq!(unit.weeks(),   2);
	/// assert_eq!(unit.days(),    3);
	/// assert_eq!(unit.hours(),   4);
	/// assert_eq!(unit.minutes(), 5);
	/// assert_eq!(unit.seconds(), 6);
	/// ```
	///
	/// Example of saturating inputs.
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::new_variety(
	///     172,   // years
	///     134,   // months
	///     22,    // weeks
	///     32575, // days
	///     46,    // hours
	///     5123,  // minutes
	///     54,    // seconds
	/// );
	///
	/// assert_eq!(unit, TimeUnit::MAX);
	/// ```
	pub const fn new_variety(
		years:   u8,
		months:  u16,
		weeks:   u16,
		days:    u16,
		hours:   u32,
		minutes: u32,
		seconds: u32,
	) -> Self {
		let mut inner: u32 = 0;

		inner = inner.saturating_add((years as u32).saturating_mul(31_536_000));
		inner = inner.saturating_add((months as u32).saturating_mul(2_678_400));
		inner = inner.saturating_add((weeks as u32).saturating_mul(604_800));
		inner = inner.saturating_add((days as u32).saturating_mul(86_400));
		inner = inner.saturating_add(hours.saturating_mul(3_600));
		inner = inner.saturating_add(minutes.saturating_mul(60));
		inner = inner.saturating_add(seconds);

		Self::new(inner)
	}

	#[inline]
	#[must_use]
	/// Returns the internal structure.
	///
	/// A tuple is returned mirroring the internal structure of [`TimeUnit`], going from left-to-right:
	/// - [`bool`] - If this [`TimeUnit`] is unknown or not ([`TimeUnit::is_unknown()`])
	/// - [`u32`] - The total amount of seconds ([`TimeUnit::inner()`])
	/// - [`u8`] - [`TimeUnit::years()`]
	/// - [`u8`] - [`TimeUnit::months()`]
	/// - [`u8`] - [`TimeUnit::weeks()`]
	/// - [`u8`] - [`TimeUnit::days()`]
	/// - [`u8`] - [`TimeUnit::hours()`]
	/// - [`u8`] - [`TimeUnit::minutes()`]
	/// - [`u8`] - [`TimeUnit::seconds()`]
	///
	/// # Example
	/// ```rust
	/// # use readable::time::*;
	/// let (
	///     unknown,
	///     inner,
	///     years,
	///     months,
	///     weeks,
	///     days,
	///     hours,
	///     minutes,
	///     seconds,
	/// ) = TimeUnit::from(39071167).into_raw();
	///
	/// assert_eq!(unknown, false);
	/// assert_eq!(inner,   39071167);
	/// assert_eq!(years,   1);
	/// assert_eq!(months,  2);
	/// assert_eq!(weeks,   3);
	/// assert_eq!(days,    4);
	/// assert_eq!(hours,   5);
	/// assert_eq!(minutes, 6);
	/// assert_eq!(seconds, 7);
	/// ```
	pub const fn into_raw(self) -> (bool, u32, u8, u8, u8, u8, u8, u8, u8) {
		(
			self.unknown,
			self.inner,
			self.years,
			self.months,
			self.weeks,
			self.days,
			self.hours,
			self.minutes,
			self.seconds,
		)
	}

	#[inline]
	#[must_use]
	/// Same as [`TimeUnit::into_raw()`] but does not destruct `self`
	pub const fn to_raw(&self) -> (bool, u32, u8, u8, u8, u8, u8, u8, u8) {
		(
			self.unknown,
			self.inner,
			self.years,
			self.months,
			self.weeks,
			self.days,
			self.hours,
			self.minutes,
			self.seconds,
		)
	}

	#[inline]
	#[must_use]
	/// An unknown [`TimeUnit`] can be created on irregular input (negative integer, NaN float, etc)
	/// or if it was converted from a different `readable::time` type that was unknown.
	///
	/// This function checks if `self` is unknown.
	///
	/// Although all inner numbers are all set to `0`,
	/// a flag is set internally such that:
	/// ```rust
	/// # use readable::time::*;
	/// assert!(TimeUnit::ZERO != TimeUnit::UNKNOWN);
	/// ```
	///
	/// # Examples
	/// ```rust
	/// # use readable::{time::*,up::*};
	/// assert!(TimeUnit::UNKNOWN.is_unknown());
	/// assert!(TimeUnit::from(Uptime::UNKNOWN).is_unknown());
	/// assert!(TimeUnit::from(f32::NAN).is_unknown());
	/// assert!(TimeUnit::from(-1).is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool { self.unknown }

	#[inline]
	#[must_use]
	/// Returns the _total_ amount of seconds this [`TimeUnit`] represents.
	///
	/// ```rust
	/// # use readable::time::*;
	/// let unit = TimeUnit::from(123);
	///
	/// assert_eq!(unit.minutes(), 2);
	/// assert_eq!(unit.seconds(), 3);
	/// assert_eq!(unit.inner(), 123);
	/// ```
	pub const fn inner(&self) -> u32 { self.inner }

	#[inline]
	#[must_use]
	/// Returns the remaining amount of years this [`TimeUnit`] represents.
	///
	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::from(86400 * 364).years(), 0);
	/// assert_eq!(TimeUnit::from(86400 * 365).years(), 1);
	/// ```
	pub const fn years(&self) -> u8 { self.years }

	#[inline]
	#[must_use]
	/// Returns the remaining amount of months this [`TimeUnit`] represents.
	///
	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::from(86400 * 30).months(), 0);
	/// assert_eq!(TimeUnit::from(86400 * 31).months(), 1);
	/// ```
	pub const fn months(&self) -> u8 { self.months }
	#[inline]
	#[must_use]
	/// Returns the remaining amount of weeks this [`TimeUnit`] represents.
	///
	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::from(86400 * 6).weeks(), 0);
	/// assert_eq!(TimeUnit::from(86400 * 7).weeks(), 1);
	/// ```
	pub const fn weeks(&self) -> u8 { self.weeks }

	#[inline]
	#[must_use]
	/// Returns the remaining amount of days this [`TimeUnit`] represents.
	///
	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::from(86399).days(), 0);
	/// assert_eq!(TimeUnit::from(86400).days(), 1);
	/// ```
	pub const fn days(&self) -> u8 { self.days }

	#[inline]
	#[must_use]
	/// Returns the remaining amount of hours this [`TimeUnit`] represents.
	///
	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::from(3599).hours(), 0);
	/// assert_eq!(TimeUnit::from(3600).hours(), 1);
	/// ```
	pub const fn hours(&self) -> u8 { self.hours }

	#[inline]
	#[must_use]
	/// Returns the remaining amount of minutes this [`TimeUnit`] represents.
	///
	/// ```rust
	/// # use readable::time::*;
	/// assert_eq!(TimeUnit::from(59).minutes(), 0);
	/// assert_eq!(TimeUnit::from(60).minutes(), 1);
	/// ```
	pub const fn minutes(&self) -> u8 { self.minutes }

	#[inline]
	#[must_use]
	/// Returns the remaining amount of seconds this [`TimeUnit`] represents.
	///
	/// This is the _remaining_ amount of seconds, not the _total_ amount of seconds.
	/// ```rust
	/// # use readable::time::*;
	/// // `0` is returned since `60` is == `1 minute`.
	/// assert_eq!(TimeUnit::from(60).seconds(), 0);
	/// assert_eq!(TimeUnit::from(60).minutes(), 1);
	/// assert_eq!(TimeUnit::from(60).inner(), 60);
	///
	/// // `1` is returned since there's 1 remaining second.
	/// assert_eq!(TimeUnit::from(61).seconds(), 1);
	/// assert_eq!(TimeUnit::from(61).minutes(), 1);
	/// assert_eq!(TimeUnit::from(61).inner(),   61);
	/// ```
	pub const fn seconds(&self) -> u8 { self.seconds }
}

//---------------------------------------------------------------------------------------------------- "u*" impl
macro_rules! handle_over_u32 {
	($value:expr, $type:ty) => {
		if $value > (u32::MAX as $type) {
			return Self::UNKNOWN;
		}
	};
}

//---------------------------------------------------------------------------------------------------- "u*" impl
// Implementation Macro.
macro_rules! impl_u {
	($($u:ty),* $(,)?) => { $(
		impl From<$u> for TimeUnit {
			#[inline]
			fn from(u: $u) -> Self {
				Self::new(u as u32)
			}
		}
		impl From<&$u> for TimeUnit {
			#[inline]
			fn from(u: &$u) -> Self {
				Self::new(*u as u32)
			}
		}
	)*}
}
impl_u!(u8,u16,u32);
#[cfg(not(target_pointer_width = "64"))]
impl_u!(usize);

macro_rules! impl_u_over {
	($($u:ty),* $(,)?) => { $(
		impl From<$u> for TimeUnit {
			#[inline]
			fn from(u: $u) -> Self {
				handle_over_u32!(u, $u);
				Self::new(u as u32)
			}
		}
		impl From<&$u> for TimeUnit {
			#[inline]
			fn from(u: &$u) -> Self {
				handle_over_u32!(*u, $u);
				Self::new(*u as u32)
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
		impl From<$int> for TimeUnit {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				Self::new(int as u32)
			}
		}
		impl From<&$int> for TimeUnit {
			#[inline]
			fn from(int: &$int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				Self::new(*int as u32)
			}
		}
	)*}
}
impl_int!(i8,i16,i32);
#[cfg(not(target_pointer_width = "64"))]
impl_u!(isize);

macro_rules! impl_int_over {
	($($int:ty),* $(,)?) => { $(
		impl From<$int> for TimeUnit {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				handle_over_u32!(int, $int);
				Self::new(int as u32)
			}
		}
		impl From<&$int> for TimeUnit {
			#[inline]
			fn from(int: &$int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				handle_over_u32!(*int, $int);
				Self::new(*int as u32)
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
		impl From<$float> for TimeUnit {
			#[inline]
			fn from(float: $float) -> Self {
				return_bad_float!(float, Self::UNKNOWN, Self::UNKNOWN);
				if float.is_sign_negative() {
					return Self::UNKNOWN;
				}
				handle_over_u32!(float, $float);
				Self::new(float as u32)
			}
		}
		impl From<&$float> for TimeUnit {
			#[inline]
			fn from(float: &$float) -> Self {
				return_bad_float!(float, Self::UNKNOWN, Self::UNKNOWN);
				if float.is_sign_negative() {
					return Self::UNKNOWN;
				}
				handle_over_u32!(*float, $float);
				Self::new(*float as u32)
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
					Self::new(from.inner() as u32)
				}
			}
		}
		impl From<&$other> for $this {
			#[inline]
			fn from(from: &$other) -> Self {
				if from.is_unknown() {
					Self::UNKNOWN
				} else {
					Self::new(from.inner() as u32)
				}
			}
		}
	)*}
}
impl_from_time!(TimeUnit => UptimeFull, Htop, Uptime, Time, Military, Unsigned);

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for TimeUnit {
	#[inline]
	fn from(duration: std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::new(u as u32)
	}
}

impl From<&std::time::Duration> for TimeUnit {
	#[inline]
	fn from(duration: &std::time::Duration) -> Self {
		let u = duration.as_secs();
		handle_over_u32!(u, u64);
		Self::new(u as u32)
	}
}

impl From<std::time::Instant> for TimeUnit {
	#[inline]
	fn from(instant: std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::new(u as u32)
	}
}

impl From<&std::time::Instant> for TimeUnit {
	#[inline]
	fn from(instant: &std::time::Instant) -> Self {
		let u = instant.elapsed().as_secs();
		handle_over_u32!(u, u64);
		Self::new(u as u32)
	}
}

impl From<TimeUnit> for std::time::Duration {
	#[inline]
	fn from(value: TimeUnit) -> Self {
		Self::from_secs(value.inner().into())
	}
}

impl From<&TimeUnit> for std::time::Duration {
	#[inline]
	fn from(value: &TimeUnit) -> Self {
		Self::from_secs(value.inner().into())
	}
}

impl std::default::Default for TimeUnit {
	#[inline]
	/// Calls [`Self::ZERO`]
	fn default() -> Self {
		Self::ZERO
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[cfg(feature = "serde")]
	fn serde() {
		let this: TimeUnit = TimeUnit::from(39071167);
		let json = serde_json::to_string(&this).unwrap();
		assert_eq!(
			json,
			r#"{"unknown":false,"inner":39071167,"years":1,"months":2,"weeks":3,"days":4,"hours":5,"minutes":6,"seconds":7}"#
		);

		let this: TimeUnit = serde_json::from_str(&json).unwrap();
		assert_eq!(this.inner(), 39071167);

		// Bad bytes.
		assert!(serde_json::from_str::<TimeUnit>(&"---").is_err());
	}

	#[test]
	#[cfg(feature = "bincode")]
	fn bincode() {
		let this: TimeUnit = TimeUnit::from(39071167);
		let config = bincode::config::standard();
		let bytes = bincode::encode_to_vec(&this, config).unwrap();

		let this: TimeUnit = bincode::decode_from_slice(&bytes, config).unwrap().0;
		assert_eq!(this.inner(), 39071167);
	}

	#[test]
	#[cfg(feature = "borsh")]
	fn borsh() {
		let this: TimeUnit = TimeUnit::from(39071167);
		let bytes = borsh::to_vec(&this).unwrap();

		let this: TimeUnit = borsh::from_slice(&bytes).unwrap();
		assert_eq!(this.inner(), 39071167);

		// Bad bytes.
		assert!(borsh::from_slice::<TimeUnit>(b"bad .-;[]124/ bytes").is_err());
	}
}
