//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use std::fmt::Write;

//---------------------------------------------------------------------------------------------------- Time
/// Human-readable [`std::time::Duration`].
///
/// **The input is always assumed to be in seconds.**
///
/// The inner fields are `(u64, String)` but they are not public.
///
/// [`From`] input can be:
/// - [`std::time::Duration`]
/// - [`u8`], [`u16`], [`u32`], [`u64`], [`usize`]
///
/// The lowest unit is `second`, the highest is `year`, and `week` is skipped in favor of `7 days`.
///
/// # Examples
/// | Input      | [`String`] Output  |
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
pub struct Time(u64, String);

impl std::fmt::Display for Time {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", &self.1.as_str())
	}
}

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
		let mut string = String::with_capacity(8);
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
	fn plural(string: &mut String, started: &mut bool, name: &str, value: u64) {
		if value > 0 {
			if *started {
				string.write_str(", ");
			}
			write!(string, "{} {}", value, name);
			if value > 1 {
				string.write_str("s");
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
	/// [`Clone`]'s and returns the inner [`String`].
	pub fn to_string(&self) -> String {
		self.1.clone()
	}

	#[inline]
	/// Returns the inner [`u64`].
	pub fn to_u64(&self) -> u64 {
		self.0
	}

	#[inline]
	/// Consumes [`Self]`, returning the inner [`String`].
	pub fn into_string(self) -> String {
		self.1
	}

	#[inline]
	/// Consumes [`Self`], returning the inner [`u64`] and [`String`].
	pub fn into_raw(self) -> (u64, String) {
		(self.0, self.1)
	}

	#[inline]
	/// Return `(0, "0 seconds")`.
	pub fn zero() -> Self {
		Self(0, String::from("0 seconds"))
	}

	#[inline]
	/// Return `(1, "1 second")`.
	pub fn second() -> Self {
		Self(1, String::from("1 second"))
	}

	#[inline]
	/// Return `(60, "1 minute")`.
	pub fn minute() -> Self {
		Self(60, String::from("1 minute"))
	}

	#[inline]
	/// Return `(3600, "1 hour")`.
	pub fn hour() -> Self {
		Self(3600, String::from("1 hour"))
	}

	#[inline]
	/// Return `(86400, "1 day")`.
	pub fn day() -> Self {
		Self(86400, String::from("1 day"))
	}

	#[inline]
	/// Return `(2630016, "1 month")`.
	pub fn month() -> Self {
		Self(2630016, String::from("1 month"))
	}

	#[inline]
	/// Return `(31557600, "1 year")`.
	pub fn year() -> Self {
		Self(31557600, String::from("1 year"))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn time() {
		use std::time::Duration;
		assert!(Time::from(Duration::from_secs(0)).to_string()        == "0 seconds");
		assert!(Time::from(Duration::from_secs(1)).to_string()        == "1 second");
		assert!(Time::from(Duration::from_secs(2)).to_string()        == "2 seconds");
		assert!(Time::from(Duration::from_secs(59)).to_string()       == "59 seconds");
		assert!(Time::from(Duration::from_secs(60)).to_string()       == "1 minute");
		assert!(Time::from(Duration::from_secs(61)).to_string()       == "1 minute, 1 second");
		assert!(Time::from(Duration::from_secs(62)).to_string()       == "1 minute, 2 seconds");
		assert!(Time::from(Duration::from_secs(120)).to_string()      == "2 minutes");
		assert!(Time::from(Duration::from_secs(121)).to_string()      == "2 minutes, 1 second");
		assert!(Time::from(Duration::from_secs(122)).to_string()      == "2 minutes, 2 seconds");
		assert!(Time::from(Duration::from_secs(179)).to_string()      == "2 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(3599)).to_string()     == "59 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(3600)).to_string()     == "1 hour");
		assert!(Time::from(Duration::from_secs(3601)).to_string()     == "1 hour, 1 second");
		assert!(Time::from(Duration::from_secs(3602)).to_string()     == "1 hour, 2 seconds");
		assert!(Time::from(Duration::from_secs(3660)).to_string()     == "1 hour, 1 minute");
		assert!(Time::from(Duration::from_secs(3720)).to_string()     == "1 hour, 2 minutes");
		assert!(Time::from(Duration::from_secs(86399)).to_string()    == "23 hours, 59 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(86400)).to_string()    == "1 day");
		assert!(Time::from(Duration::from_secs(86401)).to_string()    == "1 day, 1 second");
		assert!(Time::from(Duration::from_secs(86402)).to_string()    == "1 day, 2 seconds");
		assert!(Time::from(Duration::from_secs(86460)).to_string()    == "1 day, 1 minute");
		assert!(Time::from(Duration::from_secs(86520)).to_string()    == "1 day, 2 minutes");
		assert!(Time::from(Duration::from_secs(90000)).to_string()    == "1 day, 1 hour");
		assert!(Time::from(Duration::from_secs(93600)).to_string()    == "1 day, 2 hours");
		assert!(Time::from(Duration::from_secs(604799)).to_string()   == "6 days, 23 hours, 59 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(604800)).to_string()   == "7 days");
		assert!(Time::from(Duration::from_secs(2630016)).to_string()  == "1 month");
		assert!(Time::from(Duration::from_secs(3234815)).to_string()  == "1 month, 6 days, 23 hours, 59 minutes, 59 seconds");
		assert!(Time::from(Duration::from_secs(5260032)).to_string()  == "2 months");
		assert!(Time::from(Duration::from_secs(31557600)).to_string() == "1 year");
		assert!(Time::from(Duration::from_secs(63115200)).to_string() == "2 years");
		assert_eq!(
			Time::from(Duration::from_secs(18446744073709551615)).to_string(),
			"584542046090 years, 7 months, 15 days, 17 hours, 5 minutes, 3 seconds",
		);
	}
}
