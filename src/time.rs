//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use std::fmt::Write;

//---------------------------------------------------------------------------------------------------- Time
/// Human-readable [`std::time::Duration`].
///
/// **The input is always assumed to be in seconds.**
///
/// The inner fields are `(f64, String)` but they are not public.
///
/// [`From`] input can be:
/// - [`std::time::Duration`]
/// - [`u8`], [`u16`], [`u32`], [`u64`], [`usize`]
/// - [`f32`], [`f64`]
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
/// # Exceptions
/// | Exception                                     | [`String`] Output |
/// |-----------------------------------------------|-------------------|
/// | [`f32::NAN`] & [`f64::NAN`]                   | `NaN`
/// | [`f32::INFINITY`] & [`f64::INFINITY`]         | `∞`
/// | [`f32::NEG_INFINITY`] & [`f64::NEG_INFINITY`] | `-∞`
///
/// # Credit
/// This code is forked from `https://docs.rs/humantime`, edited to remove sub-second time, change spacing and some words.

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Time(f64, String);

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
				Self::from(number as f64)
			}
		}
	}
}
impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(usize);
impl_number!(f32);

impl From<std::time::Duration> for Time {
	fn from(duration: std::time::Duration) -> Self {
		Self::from(duration.as_secs_f64())
	}
}

impl From<&std::time::Duration> for Time {
	fn from(duration: &std::time::Duration) -> Self {
		Self::from(duration.as_secs_f64())
	}
}

impl From<f64> for Time {
	fn from(seconds: f64) -> Self {
		if seconds == 0.0 {
			return Self::zero()
		}

		// Handle NaN/Inf.
		crate::float::handle_nan!(seconds);

		let years = seconds / 31_557_600.0;  // 365.25d
		let ydays = seconds % 31_557_600.0;
		let months = ydays / 2_630_016.0;  // 30.44d
		let mdays = ydays % 2_630_016.0;
		let days = mdays / 86400.0;
		let day_secs = mdays % 86400.0;
		let hours = day_secs / 3600.0;
		let minutes = day_secs % 3600.0 / 60.0;
		let seconds = day_secs % 60.0;

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
	/// Returns the inner [`f64`].
	pub fn to_f64(&self) -> f64 {
		self.0
	}

	#[inline]
	/// Consumes [`Self]`, returning the inner [`String`].
	pub fn into_string(self) -> String {
		self.1
	}

	#[inline]
	/// Consumes [`Self`], returning the inner [`f64`] and [`String`].
	pub fn into_raw(self) -> (f64, String) {
		(self.0, self.1)
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to `???`.
	pub fn unknown() -> Self {
		Self(f64::NAN, String::from(crate::float::UNKNOWN))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to `NaN`.
	pub fn nan() -> Self {
		Self(f64::NAN, String::from(crate::float::NAN))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::INFINITY`].
	///
	/// The [`String`] is set to `∞`.
	pub fn inf() -> Self {
		Self(f64::INFINITY, String::from(crate::float::INFINITY))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::INFINITY`].
	///
	/// The [`String`] is set to `-∞`.
	pub fn neg_inf() -> Self {
		Self(f64::INFINITY, String::from(crate::float::NEG_INFINITY))
	}

	#[inline]
	/// Return `(0.0, "0 seconds")`.
	pub fn zero() -> Self {
		Self(0.0, String::from("0 seconds"))
	}

	#[inline]
	/// Return `(1.0, "1 second")`.
	pub fn second() -> Self {
		Self(1.0, String::from("1 second"))
	}

	#[inline]
	/// Return `(60.0, "1 minute")`.
	pub fn minute() -> Self {
		Self(60.0, String::from("1 minute"))
	}

	#[inline]
	/// Return `(3600.0, "1 hour")`.
	pub fn hour() -> Self {
		Self(3600.0, String::from("1 hour"))
	}

	#[inline]
	/// Return `(86400.0, "1 day")`.
	pub fn day() -> Self {
		Self(86400.0, String::from("1 day"))
	}

	#[inline]
	/// Return `(2630016.0, "1 month")`.
	pub fn month() -> Self {
		Self(2630016.0, String::from("1 month"))
	}

	#[inline]
	/// Return `(31557600.0, "1 year")`.
	pub fn year() -> Self {
		Self(31557600.0, String::from("1 year"))
	}

	#[inline]
	fn plural(string: &mut String, started: &mut bool, name: &str, value: f64) {
		if value > 0.0 {
			if *started {
				string.write_str(", ");
			}
			write!(string, "{} {}", value, name);
			if value > 1.0 {
				string.write_str("s");
			}
			*started = true;
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	#[test]
	fn human_time() {
		use std::time::Duration;

		assert!(Time::into_human(Duration::from_secs(0)).to_string()        == "0 seconds");
		assert!(Time::into_human(Duration::from_secs(1)).to_string()        == "1 second");
		assert!(Time::into_human(Duration::from_secs(2)).to_string()        == "2 seconds");
		assert!(Time::into_human(Duration::from_secs(59)).to_string()       == "59 seconds");
		assert!(Time::into_human(Duration::from_secs(60)).to_string()       == "1 minute");
		assert!(Time::into_human(Duration::from_secs(61)).to_string()       == "1 minute, 1 second");
		assert!(Time::into_human(Duration::from_secs(62)).to_string()       == "1 minute, 2 seconds");
		assert!(Time::into_human(Duration::from_secs(120)).to_string()      == "2 minutes");
		assert!(Time::into_human(Duration::from_secs(121)).to_string()      == "2 minutes, 1 second");
		assert!(Time::into_human(Duration::from_secs(122)).to_string()      == "2 minutes, 2 seconds");
		assert!(Time::into_human(Duration::from_secs(179)).to_string()      == "2 minutes, 59 seconds");
		assert!(Time::into_human(Duration::from_secs(3599)).to_string()     == "59 minutes, 59 seconds");
		assert!(Time::into_human(Duration::from_secs(3600)).to_string()     == "1 hour");
		assert!(Time::into_human(Duration::from_secs(3601)).to_string()     == "1 hour, 1 second");
		assert!(Time::into_human(Duration::from_secs(3602)).to_string()     == "1 hour, 2 seconds");
		assert!(Time::into_human(Duration::from_secs(3660)).to_string()     == "1 hour, 1 minute");
		assert!(Time::into_human(Duration::from_secs(3720)).to_string()     == "1 hour, 2 minutes");
		assert!(Time::into_human(Duration::from_secs(86399)).to_string()    == "23 hours, 59 minutes, 59 seconds");
		assert!(Time::into_human(Duration::from_secs(86400)).to_string()    == "1 day");
		assert!(Time::into_human(Duration::from_secs(86401)).to_string()    == "1 day, 1 second");
		assert!(Time::into_human(Duration::from_secs(86402)).to_string()    == "1 day, 2 seconds");
		assert!(Time::into_human(Duration::from_secs(86460)).to_string()    == "1 day, 1 minute");
		assert!(Time::into_human(Duration::from_secs(86520)).to_string()    == "1 day, 2 minutes");
		assert!(Time::into_human(Duration::from_secs(90000)).to_string()    == "1 day, 1 hour");
		assert!(Time::into_human(Duration::from_secs(93600)).to_string()    == "1 day, 2 hours");
		assert!(Time::into_human(Duration::from_secs(604799)).to_string()   == "6 days, 23 hours, 59 minutes, 59 seconds");
		assert!(Time::into_human(Duration::from_secs(604800)).to_string()   == "7 days");
		assert!(Time::into_human(Duration::from_secs(2630016)).to_string()  == "1 month");
		assert!(Time::into_human(Duration::from_secs(3234815)).to_string()  == "1 month, 6 days, 23 hours, 59 minutes, 59 seconds");
		assert!(Time::into_human(Duration::from_secs(5260032)).to_string()  == "2 months");
		assert!(Time::into_human(Duration::from_secs(31557600)).to_string() == "1 year");
		assert!(Time::into_human(Duration::from_secs(63115200)).to_string() == "2 years");
		assert_eq!(
			Time::into_human(Duration::from_secs(18446744073709551615)).to_string(),
			"584542046090 years, 7 months, 15 days, 17 hours, 5 minutes, 3 seconds",
		);
	}
}
