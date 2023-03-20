//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Constants
// The locale numbers are formatting in is English, which looks like: [1,000]
const LOCALE: num_format::Locale = num_format::Locale::en;
pub const UNKNOWN:      &str = "???";
/// Returned when encountering a [`f64::NAN`].
pub const NAN:          &str = "NaN";

/// Returned when encountering a [`f64::INFINITY`].
pub const INFINITY:     &str = "∞";

/// Returned when encountering a [`f64::NEG_INFINITY`].
pub const NEG_INFINITY: &str = "-∞";

//---------------------------------------------------------------------------------------------------- Float
/// Human readable float.
///
/// Takes a floating point number as input and returns a ready-to-[`print!()`] [`Float`].
///
/// The inner fields are `(f64, String)` but they are not public.
///
/// The default [`From`] implementation will print `3` decimal numbers.
///
/// This can be changed by using different functions when initially
/// creating the [`Float`], or converting an existing [`Float`], for example:
/// ```
/// let f2 = Float::new_2_point(3.0);
/// let f6 = Float::new_6_point(3.0);
/// let f9 = Float::new_9_point(f3.to_f64);
///
/// println!("{}\n{}\n{}", f3, f6, f9);
///
/// > 3.000
/// > 3.000000
/// > 3.000000000
///```
///
/// All conversions take into account:
/// - [`f64::NAN`]
/// - [`f64::INFINITY`]
/// - [`f64::NEG_INFINITY`]
///
/// and will produce the output:
/// - `NaN`
/// - `∞`
/// - `-∞`
///
/// To disable these checks, (you are _sure_ you don't have NaN's), enable the `ignore_nan_inf` feature flag.
///
/// # Examples
/// | Input              | [`String`] Output |
/// |--------------------|-------------------|
/// | `0.0`              | `0.000`
/// | `1234.571`         | `1,234.571`
/// | `1234.571`         | `1,234.571000`
/// | `99.123` (percent) | `99.12%`
/// | `0.001` (percent)  | `0%`

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Float(f64, String);

impl std::fmt::Display for Float {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", &self.1.as_str())
	}
}

// "Handle NaN/Infinite" Macro.
macro_rules! handle_nan {
	($float:ident) => {
		#[cfg(not(feature = "ignore_nan_inf"))]
		if $float == f64::NAN {
			return Self($float, String::from("NaN"))
		} else if $float == f64::INFINITY {
			return Self($float, String::from("∞"))
		} else if $float == f64::NEG_INFINITY {
			return Self($float, String::from("-∞"))
		}
	}
}
pub(crate) use handle_nan;

impl Float {
	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of `0.0`.
	///
	/// The [`String`] is set to `0.000`.
	pub fn zero() -> Self {
		Self(0.0, String::from("0.000"))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to `???`.
	pub fn unknown() -> Self {
		Self(f64::NAN, String::from(UNKNOWN))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::NAN`].
	///
	/// The [`String`] is set to `NaN`.
	pub fn nan() -> Self {
		Self(f64::NAN, String::from(NAN))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::INFINITY`].
	///
	/// The [`String`] is set to `∞`.
	pub fn inf() -> Self {
		Self(f64::INFINITY, String::from(INFINITY))
	}

	#[inline]
	/// Returns a [`Self`] with the [`f64`] value of [`f64::INFINITY`].
	///
	/// The [`String`] is set to `-∞`.
	pub fn neg_inf() -> Self {
		Self(f64::INFINITY, String::from(NEG_INFINITY))
	}

	#[inline]
	/// Truncates to `2` floating point and appends a `%`.
	///
	/// Anything lower than `0.01` is rounded down to `0.00`.
	///
	/// ## Examples
	/// | Input | String Output |
	/// |-------|---------------|
	/// | 0.0   | `0.00%`
	/// | 0.001 | `0.00%`
	/// | 0.01  | `0.01%`
	/// | 0.1   | `0.10%`
	/// | 1.0   | `1.00%`
	/// | 50.0  | `50.00%`
	/// | 100.0 | `100.00%`
	/// | 150.0 | `150.00%`
	/// | 1000.0 | `1,000.00%`
	/// | 250000.0 | `250,000.00%`
	pub fn percent(f: f64) -> Self {
		handle_nan!(f);

		if f < 0.01 {
			Self(0.0, String::from("0.00%"))
		} else if f >= 1000.0 {
			let fract = &format!("{}", f)[2..4];
			Self(f, format!("{}.{}%", f as u64, fract))
		} else {
			Self(f, format!("{:.2}%", f))
		}
	}

	#[inline]
	/// Return a borrowed [`str`] without consuming [`Self`].
	pub fn as_str(&self) -> &str {
		self.1.as_str()
	}

	#[inline]
	/// Returns a [`Clone`] of the inner [`String`].
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
	/// Same as [`Self::from`] but with no floating point on the inner [`String`].
	///
	/// The inner [`f64`] stays the same as the input.
	///
	/// This does not round _up_ or _down_, it completely ignores the floating point.
	///
	/// ## Examples
	/// | Input  | String Output |
	/// |--------|---------------|
	/// | 0.0    | `0`
	/// | 50.123 | `50`
	/// | 100.1  | `100`
	pub fn new_0_point(f: f64) -> Self {
		handle_nan!(f);

		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}", buf.as_str().to_string()))
	}

	#[inline]
	/// Create a new [`Self`]  but with `1` floating point.
	pub fn new_1_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..3];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `2` floating point.
	pub fn new_2_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..4];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `4` floating point.
	pub fn new_4_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..6];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `5` floating point.
	pub fn new_5_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..7];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `6` floating point.
	pub fn new_6_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..8];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `7` floating point.
	pub fn new_7_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..9];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `8` floating point.
	pub fn new_8_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..10];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `9` floating point.
	pub fn new_9_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..11];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `10` floating point.
	pub fn new_10_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..12];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `11` floating point.
	pub fn new_11_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..13];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `12` floating point.
	pub fn new_12_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..14];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `13` floating point.
	pub fn new_13_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..15];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Create a new [`Self`]  but with `14` floating point.
	pub fn new_14_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..16];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Same as [`Self::percent`] but with no floating point on the inner [`String`].
	///
	/// The inner [`f64`] stays the same as the input.
	///
	/// This does not round _up_ or _down_, it completely ignores the floating point.
	///
	/// ## Examples
	/// | Input  | String Output |
	/// |--------|---------------|
	/// | 0.0    | `0%`
	/// | 50.123 | `50%`
	/// | 100.1  | `100%`
	pub fn percent_0_point(f: f64) -> Self {
		handle_nan!(f);

		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}%", buf.as_str().to_string()))
	}

	#[inline]
	/// Same as [`Self::percent`] but with `1` floating point.
	pub fn percent_1_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..3];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}%", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Same as [`Self::percent`] but with `2` floating point.
	pub fn percent_2_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..4];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}%", buf.as_str().to_string(), fract))
	}

	#[inline]
	/// Same as [`Self::percent`] but with `4` floating point.
	pub fn percent_4_point(f: f64) -> Self {
		handle_nan!(f);

		let fract = &format!("{}", f)[2..6];
		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(f as u64), &LOCALE);

		Self(f, format!("{}.{}%", buf.as_str().to_string(), fract))
	}
}

// Implementation Macro.
macro_rules! impl_number {
	($number:ty) => {
		impl From<$number> for Float {
			#[inline]
			fn from(number: $number) -> Self {
				let mut buf = num_format::Buffer::new();
				buf.write_formatted(&(number as u64), &LOCALE);
				Self(number as f64, format!("{}.000", buf.as_str().to_string()))
			}
		}
	}
}
impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(usize);
impl From<f32> for Float {
	#[inline]
	fn from(number: f32) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		if number == f32::NAN {
			return Self(number as f64, String::from(NAN))
		} else if number == f32::INFINITY {
			return Self(number as f64, String::from(INFINITY))
		} else if number == f32::NEG_INFINITY {
			return Self(number as f64, String::from(NEG_INFINITY))
		}

		let fract = &format!("{}", number)[2..5];

		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(number as u32), &LOCALE);
		Self(number as f64, format!("{}.{}", buf.as_str().to_string(), fract))
	}
}

impl From<f64> for Float {
	#[inline]
	fn from(number: f64) -> Self {
		#[cfg(not(feature = "ignore_nan_inf"))]
		if number == f64::NAN {
			return Self(number, String::from(NAN))
		} else if number == f64::INFINITY {
			return Self(number, String::from(INFINITY))
		} else if number == f64::NEG_INFINITY {
			return Self(number, String::from(NEG_INFINITY))
		}

		let fract = &format!("{}", number)[2..5];

		let mut buf = num_format::Buffer::new();
		buf.write_formatted(&(number as u64), &LOCALE);
		Self(number, format!("{}.{}", buf.as_str().to_string(), fract))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}