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
/// [`From`] input can be:
/// - [`std::time::Duration`]
/// - [`u8`], [`u16`], [`u32`], [`u64`], [`usize`]
///
/// The lowest unit is `second`, the highest is `year`, and `week` is skipped in favor of `7 days`.
///
/// # Examples
/// | Input      | Output             |
/// |------------|--------------------|
/// | 0          | `0 seconds`
/// | 1          | `1 second`
/// | 59         | `59 seconds`
/// | 3599       | `59 minutes, 59 seconds`
/// | 86399      | `23 hours, 59 minutes, 59 seconds`
/// | 604799     | `6 days, 23 hours, 59 minutes, 59 seconds`
/// | 3234815    | `1 month, 6 days, 23 hours, 59 minutes, 59 seconds`
/// | 63115200   | `2 years`
///
/// # Credit
/// This code is forked from `https://docs.rs/humantime`, edited to remove sub-second time, change spacing and some words.

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time(u64, CompactString);

impl_traits!(Time, u64);

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

impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(usize);

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

impl From<u64> for Time {
	fn from(seconds: u64) -> Self {
		if seconds == 0 {
			return Self::zero()
		}

		let years = seconds / 31_557_600;  // 365.25d
		let ydays = seconds % 31_557_600;
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

		Self(seconds, string)
	}
}

impl Time {
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
	/// Return a borrowed [`str`] without consuming [`Self`].
	pub fn as_str(&self) -> &str {
		self.1.as_str()
	}

	#[inline]
	/// Returns the inner [`u64`].
	pub fn u64(&self) -> u64 {
		self.0
	}

	#[inline]
	#[cfg(target_pointer_width = "64")]
	/// Returns the inner [`u64`] as a [`usize`].
	///
	/// # Notes
	/// This function is only available on 64-bit platforms.
	pub fn usize(&self) -> usize {
		self.0 as usize
	}

	#[inline]
	/// Consumes [`Self]`, returning the inner [`String`].
	pub fn into_string(self) -> String {
		self.1.into_string()
	}

	#[inline]
	/// Consumes [`Self`], returning the inner [`u64`] and [`String`].
	pub fn into_raw(self) -> (u64, String) {
		(self.0, self.1.into_string())
	}

	#[inline]
	/// Return `(0, "0 seconds")`.
	pub fn zero() -> Self {
		Self(0, CompactString::new("0 seconds"))
	}

	#[inline]
	/// Return `(1, "1 second")`.
	pub fn second() -> Self {
		Self(1, CompactString::new("1 second"))
	}

	#[inline]
	/// Return `(60, "1 minute")`.
	pub fn minute() -> Self {
		Self(60, CompactString::new("1 minute"))
	}

	#[inline]
	/// Return `(3600, "1 hour")`.
	pub fn hour() -> Self {
		Self(3600, CompactString::new("1 hour"))
	}

	#[inline]
	/// Return `(86400, "1 day")`.
	pub fn day() -> Self {
		Self(86400, CompactString::new("1 day"))
	}

	#[inline]
	/// Return `(2630016, "1 month")`.
	pub fn month() -> Self {
		Self(2630016, CompactString::new("1 month"))
	}

	#[inline]
	/// Return `(31557600, "1 year")`.
	pub fn year() -> Self {
		Self(31557600, CompactString::new("1 year"))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn time() {
		use std::time::Duration;
		assert!(Time::from(Duration::from_secs(0))        == "0 seconds");
		assert!(Time::from(Duration::from_secs(1))        == "1 second");
		assert!(Time::from(Duration::from_secs(2))        == "2 seconds");
		assert!(Time::from(Duration::from_secs(59))       == "59 seconds");
		assert!(Time::from(Duration::from_secs(60))       == "1 minute");
		assert!(Time::from(Duration::from_secs(61))       == "1 minute, 1 second");
		assert!(Time::from(Duration::from_secs(62))       == "1 minute, 2 seconds");
		assert!(Time::from(Duration::from_secs(120))      == "2 minutes");
		assert!(Time::from(Duration::from_secs(121))      == "2 minutes, 1 second");
		assert!(Time::from(Duration::from_secs(122))      == "2 minutes, 2 seconds");
		assert!(Time::from(Duration::from_secs(179))      == "2 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(3599))     == "59 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(3600))     == "1 hour");
		assert!(Time::from(Duration::from_secs(3601))     == "1 hour, 1 second");
		assert!(Time::from(Duration::from_secs(3602))     == "1 hour, 2 seconds");
		assert!(Time::from(Duration::from_secs(3660))     == "1 hour, 1 minute");
		assert!(Time::from(Duration::from_secs(3720))     == "1 hour, 2 minutes");
		assert!(Time::from(Duration::from_secs(86399))    == "23 hours, 59 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(86400))    == "1 day");
		assert!(Time::from(Duration::from_secs(86401))    == "1 day, 1 second");
		assert!(Time::from(Duration::from_secs(86402))    == "1 day, 2 seconds");
		assert!(Time::from(Duration::from_secs(86460))    == "1 day, 1 minute");
		assert!(Time::from(Duration::from_secs(86520))    == "1 day, 2 minutes");
		assert!(Time::from(Duration::from_secs(90000))    == "1 day, 1 hour");
		assert!(Time::from(Duration::from_secs(93600))    == "1 day, 2 hours");
		assert!(Time::from(Duration::from_secs(604799))   == "6 days, 23 hours, 59 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(604800))   == "7 days");
		assert!(Time::from(Duration::from_secs(2630016))  == "1 month");
		assert!(Time::from(Duration::from_secs(3234815))  == "1 month, 6 days, 23 hours, 59 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(5260032))  == "2 months");
		assert!(Time::from(Duration::from_secs(31557600)) == "1 year");
		assert!(Time::from(Duration::from_secs(63115200)) == "2 years");
		assert_eq!(
			Time::from(Duration::from_secs(18446744073709551615)),
			"584542046090 years, 7 months, 15 days, 17 hours, 5 minutes, 3 seconds",
		);
	}
}
