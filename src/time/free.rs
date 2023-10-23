//---------------------------------------------------------------------------------------------------- Ok
#[inline]
/// Get the current system UNIX timestamp
///
/// The returned value is how many seconds has passed since `UNIX_EPOCH`
///
/// This will return `0` if the underlying system call fails.
pub fn unix() -> u64 {
	use std::time::{SystemTime, UNIX_EPOCH};

	match SystemTime::now().duration_since(UNIX_EPOCH) {
		Ok(unix) => unix.as_secs(),
		_ => 0,
	}
}

#[inline]
/// Get the clock time of a UNIX timestamp
///
/// The input must be a UNIX timestamp.
///
/// The returned `u64` will represent how many seconds has
/// passed on the day corresponding to that timestamp.
///
/// The output is guaranteed to be in the range of `0..=86399`.
///
/// ```rust
/// # use readable::{time::*,date::*};
/// // October 20th 2023 - 10:18:30 PM
/// const TIME: u64 = 1697840310;
///
/// let seconds = unix_clock(TIME);
/// assert_eq!(seconds, 80310);
///
/// let (h, m, s) = secs_to_clock(seconds);
/// // 10:18:30 PM.
/// assert_eq!((h, m, s), (22, 18, 30))
/// ```
pub const fn unix_clock(seconds_after_unix_epoch: u64) -> u32 {
	(seconds_after_unix_epoch % 86400) as _
}

#[inline]
/// Convert seconds to `hours`, `minutes` and `seconds`.
///
/// - The seconds returned is guaranteed to be `0..=59`
/// - The minutes returned is guaranteed to be `0..=59`
/// - The hours returned can be over `23`, as this is not a clock function,
/// see [`secs_to_clock`] for clock-like behavior that wraps around on `24`
///
/// ```rust
/// # use readable::{time::*,date::*};
/// // 59 seconds.
/// assert_eq!(secs_to_hms(59), (0, 0, 59));
///
/// // 1 minute.
/// assert_eq!(secs_to_hms(60), (0, 1, 0));
///
/// // 59 minutes, 59 seconds.
/// assert_eq!(secs_to_hms(3599), (0, 59, 59));
///
/// // 1 hour.
/// assert_eq!(secs_to_hms(3600), (1, 0, 0));
///
/// // 23 hours, 59 minutes, 59 seconds.
/// assert_eq!(secs_to_hms(86399), (23, 59, 59));
///
/// // 24 hours.
/// assert_eq!(secs_to_hms(86400), (24, 0, 0));
/// ```
pub const fn secs_to_hms(seconds: u64) -> (u64, u8, u8) {
	let hours   = seconds / 3600;
	let minutes = (seconds % 3600) / 60;
	let seconds = (seconds % 3600) % 60;

	debug_assert!(minutes < 60);
	debug_assert!(seconds < 60);

	(hours, minutes as u8, seconds as u8)
}

#[inline]
/// Convert seconds to clock time, `hours`, `minutes` and `seconds`.
///
/// This is the same as [`secs_to_hms`] except it will wrap around,
/// e.g, `24:00:00` would turn into `00:00:00`.
///
/// - The seconds returned is guaranteed to be `0..=59`
/// - The minutes returned is guaranteed to be `0..=59`
/// - The hours returned is guaranteed to be `0..=23`
///
/// ```rust
/// # use readable::{time::*,date::*};
/// // 59 seconds.
/// assert_eq!(secs_to_clock(59), (0, 0, 59));
///
/// // 1 minute.
/// assert_eq!(secs_to_clock(60), (0, 1, 0));
///
/// // 59 minutes, 59 seconds.
/// assert_eq!(secs_to_clock(3599), (0, 59, 59));
///
/// // 1 hour.
/// assert_eq!(secs_to_clock(3600), (1, 0, 0));
///
/// // 23 hours, 59 minutes, 59 seconds.
/// assert_eq!(secs_to_clock(86399), (23, 59, 59));
///
/// // 24 hours (wraps back)
/// assert_eq!(secs_to_clock(86400), (0, 0, 0));
///
/// // 24 hours, 59 minutes, 59 seconds (wraps back)
/// assert_eq!(secs_to_clock(89999), (0, 59, 59));
/// ```
pub const fn secs_to_clock(seconds: u32) -> (u8, u8, u8) {
	let seconds = seconds % 86400;
	let (h,m,s) = secs_to_hms(seconds as u64);

	debug_assert!(h < 24);
	debug_assert!(m < 60);
	debug_assert!(s < 60);

	(h as u8, m, s)
}

//---------------------------------------------------------------------------------------------------- Time
#[inline]
/// Get the current system time in the system's timezone
///
/// The returned value is the total amount of seconds passed in the current day.
///
/// This is guaranteed to return a value between `0..=86399`
///
/// This will return `0` if the underlying system call fails.
pub fn time() -> u32 {
	let now = chrono::offset::Local::now().time();
	(now.hour() * 3600) + (now.minute() * 60) + now.second()
}

#[inline]
/// Get the current system time in the UTC timezone
///
/// The returned value is the total amount of seconds passed in the current day.
///
/// This is guaranteed to return a value between `0..=86399`
pub fn time_utc() -> u32 {
	unix_clock(chrono::offset::Local::now().timestamp() as u64)
}

//---------------------------------------------------------------------------------------------------- DateTime
use chrono::Timelike;
#[allow(unused_imports)] // docs
use crate::date::date;
#[inline]
/// Combines [`date()`] and [`time()`]
///
/// This returns the system's current (`year`, `month`, `day`, `seconds_passed_today`).
///
/// The seconds passed represents how many seconds
/// have passed in the current day, the same as [`time`].
pub fn datetime() -> (i16, u8, u8, u32) {
	let now  = chrono::offset::Local::now();
	let (y,m,d) = nichi::Date::from_unix(now.timestamp() as i128).inner();
	let time = now.time();
	let seconds = (time.hour() * 3600) + (time.minute() * 60) + time.second();

	(y, m, d, seconds)
}

#[inline]
/// [`datetime()`] but in the UTC timezone
pub fn datetime_utc() -> (i16, u8, u8, u32) {
	let now  = chrono::offset::Utc::now();
	let (y,m,d) = nichi::Date::from_unix(now.timestamp() as i128).inner();
	let time = now.time();
	let seconds = (time.hour() * 3600) + (time.minute() * 60) + time.second();

	(y, m, d, seconds)
}
