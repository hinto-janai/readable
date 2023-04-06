//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use compact_str::{format_compact,CompactString};
use std::fmt::Write;
use crate::macros::*;

//---------------------------------------------------------------------------------------------------- Time
/// Human-readable [`std::time::Duration`].
///
/// **The input is always assumed to be in seconds.**
///
/// [`Time::from`] input can be:
/// - [`u8`], [`u16`], [`u32`], [`u64`], [`usize`]
/// - [`std::time::Duration`], [`std::time::Instant`]
///
/// The lowest unit is `second`, the highest is `year`, and `week` is skipped in favor of `7 days`.
///
/// ## Performance
/// [`Clone`] may be expensive.
/// ```rust,compile_fail
/// # use readable::Time;
/// let a = Time::from(100.0);
///
/// // Move 'a'
/// let b = a;
///
/// // We can't use 'a', it moved into 'b'.
/// // We must `.clone()`.
/// assert!(a == 100.0);
/// ```
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a [`CompactString`](https://docs.rs/compact_str) so that any string 24 bytes (12 bytes on 32-bit) or less are _stack_ allocated instead of _heap_ allocated.
///
/// The documentation will still refer to the inner string as a `String`. Anything returned will also be a `String`.
///
/// ## Examples
/// ```rust
/// # use readable::Time;
/// assert!(Time::from(0_u64)        == "0 seconds");
/// assert!(Time::from(1_u64)        == "1 second");
/// assert!(Time::from(2_u64)        == "2 seconds");
/// assert!(Time::from(59_u64)       == "59 seconds");
/// assert!(Time::from(60_u64)       == "1 minute");
/// assert!(Time::from(61_u64)       == "1 minute, 1 second");
/// assert!(Time::from(62_u64)       == "1 minute, 2 seconds");
/// assert!(Time::from(120_u64)      == "2 minutes");
/// assert!(Time::from(121_u64)      == "2 minutes, 1 second");
/// assert!(Time::from(122_u64)      == "2 minutes, 2 seconds");
/// assert!(Time::from(179_u64)      == "2 minutes, 59 seconds");
/// assert!(Time::from(3599_u64)     == "59 minutes, 59 seconds");
/// assert!(Time::from(3600_u64)     == "1 hour");
/// assert!(Time::from(3601_u64)     == "1 hour, 1 second");
/// assert!(Time::from(3602_u64)     == "1 hour, 2 seconds");
/// assert!(Time::from(3660_u64)     == "1 hour, 1 minute");
/// assert!(Time::from(3720_u64)     == "1 hour, 2 minutes");
/// assert!(Time::from(86399_u64)    == "23 hours, 59 minutes, 59 seconds");
/// assert!(Time::from(86400_u64)    == "1 day");
/// assert!(Time::from(86401_u64)    == "1 day, 1 second");
/// assert!(Time::from(86402_u64)    == "1 day, 2 seconds");
/// assert!(Time::from(86460_u64)    == "1 day, 1 minute");
/// assert!(Time::from(86520_u64)    == "1 day, 2 minutes");
/// assert!(Time::from(90000_u64)    == "1 day, 1 hour");
/// assert!(Time::from(93600_u64)    == "1 day, 2 hours");
/// assert!(Time::from(604799_u64)   == "6 days, 23 hours, 59 minutes, 59 seconds");
/// assert!(Time::from(604800_u64)   == "7 days");
/// assert!(Time::from(2630016_u64)  == "1 month");
/// assert!(Time::from(3234815_u64)  == "1 month, 6 days, 23 hours, 59 minutes, 59 seconds");
/// assert!(Time::from(5260032_u64)  == "2 months");
/// assert!(Time::from(31557600_u64) == "1 year");
/// assert!(Time::from(63115200_u64) == "2 years");
/// assert_eq!(
///     Time::from(u64::MAX),
///     "584542046090 years, 7 months, 15 days, 17 hours, 5 minutes, 3 seconds",
/// );
/// ```
/// # Credit
/// This code is forked from `https://docs.rs/humantime`, edited to remove sub-second time, change spacing and some words.

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time(u64, CompactString);

// Implementation Macro.
macro_rules! impl_number {
	($number:ty) => {
		impl From<$number> for Time {
			#[inline]
			fn from(number: $number) -> Self {
				Self::from(number as u64)
			}
		}
	}
}

impl_traits!(Time, u64);
impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(usize);

impl Time {
	impl_common!(u64);
	impl_usize!();

	#[inline]
	fn plural(string: &mut CompactString, started: &mut bool, name: &str, value: u64) {
		if value > 0 {
			if *started {
				string.push_str(", ");
			}
			write!(string, "{} {}", value, name);
			if value > 1 {
				string.push('s');
			}
			*started = true;
		}
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert!(Time::zero() == 0_u64);
	/// assert!(Time::zero() == "0 seconds");
	/// ```
	pub fn zero() -> Self {
		Self(0, CompactString::new("0 seconds"))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert!(Time::second() == 1_u64);
	/// assert!(Time::second() == "1 second");
	/// ```
	pub fn second() -> Self {
		Self(1, CompactString::new("1 second"))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert!(Time::minute() == 60_u64);
	/// assert!(Time::minute() == "1 minute");
	/// ```
	pub fn minute() -> Self {
		Self(60, CompactString::new("1 minute"))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert!(Time::hour() == 3600_u64);
	/// assert!(Time::hour() == "1 hour");
	/// ```
	pub fn hour() -> Self {
		Self(3600, CompactString::new("1 hour"))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert!(Time::day() == 86400_u64);
	/// assert!(Time::day() == "1 day");
	/// ```
	pub fn day() -> Self {
		Self(86400, CompactString::new("1 day"))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert!(Time::month() == 2630016_u64);
	/// assert!(Time::month() == "1 month");
	/// ```
	pub fn month() -> Self {
		Self(2630016, CompactString::new("1 month"))
	}

	#[inline]
	/// ```rust
	/// # use readable::Time;
	/// assert!(Time::year() == 31557600_u64);
	/// assert!(Time::year() == "1 year");
	/// ```
	pub fn year() -> Self {
		Self(31557600, CompactString::new("1 year"))
	}
}

impl From<std::time::Duration> for Time {
	fn from(duration: std::time::Duration) -> Self {
		Self::from(duration.as_secs())
	}
}

impl From<&std::time::Duration> for Time {
	fn from(duration: &std::time::Duration) -> Self {
		Self::from(duration.as_secs())
	}
}

impl From<std::time::Instant> for Time {
	fn from(instant: std::time::Instant) -> Self {
		Self::from(instant.elapsed().as_secs())
	}
}

impl From<&std::time::Instant> for Time {
	fn from(duration: &std::time::Instant) -> Self {
		Self::from(instant.elapsed().as_secs())
	}
}

impl From<u64> for Time {
	fn from(secs: u64) -> Self {
		if secs == 0 {
			return Self::zero()
		}

		let years = secs / 31_557_600;  // 365.25d
		let ydays = secs % 31_557_600;
		let months = ydays / 2_630_016;  // 30.44d
		let mdays = ydays % 2_630_016;
		let days = mdays / 86400;
		let day_secs = mdays % 86400;
		let hours = day_secs / 3600;
		let minutes = day_secs % 3600 / 60;
		let seconds = day_secs % 60;

		let started = &mut false;
		let mut string = CompactString::with_capacity(8);
		Self::plural(&mut string, started, "year", years);
		Self::plural(&mut string, started, "month", months);
		Self::plural(&mut string, started, "day", days);
		Self::plural(&mut string, started, "hour", hours);
		Self::plural(&mut string, started, "minute", minutes);
		Self::plural(&mut string, started, "second", seconds);

		Self(secs, string)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
}
