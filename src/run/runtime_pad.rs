//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::run::{Runtime,RuntimeMilli,RuntimeUnion};
use crate::macros::{
	impl_common,impl_const,
	impl_traits,return_bad_float,
	impl_usize,impl_math,impl_impl_math,
};

//---------------------------------------------------------------------------------------------------- RuntimePad
/// [`Runtime`] but always full length and padded with zeros
///
/// This is the exact same type as [`Runtime`], except, the
/// numbers will _always_ be padded with `0`'s.
///
/// ```rust
/// # use readable::*;
/// let runtime = Runtime::minute();
/// assert_eq!(runtime, "1:00"); // hour left out, minute not padded
///
/// let runtime_zero = RuntimePad::minute();
/// assert_eq!(runtime_zero, "00:01:00"); // always includes everything
///
/// let runtime = Runtime::hour();
/// assert_eq!(runtime, "1:00:00");
///
/// let runtime_zero = RuntimePad::hour();
/// assert_eq!(runtime_zero, "01:00:00");
/// ```
///
/// ## Size
/// [`Str<8>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<RuntimePad>(), 16);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::RuntimePad;
/// // Always round down.
/// assert_eq!(RuntimePad::from(11.1111), "00:00:11");
/// assert_eq!(RuntimePad::from(11.9999), "00:00:11");
///
/// assert_eq!(RuntimePad::from(111.111), "00:01:51");
/// assert_eq!(RuntimePad::from(111.999), "00:01:51");
///
/// assert_eq!(RuntimePad::from(11111.1), "03:05:11");
/// assert_eq!(RuntimePad::from(11111.9), "03:05:11");
///
/// assert_eq!(RuntimePad::from(0.0), "00:00:00");
/// assert_eq!(RuntimePad::from(1.0), "00:00:01");
/// assert_eq!(RuntimePad::from(1.9), "00:00:01");
/// assert_eq!(RuntimePad::from(2.0), "00:00:02");
///
/// assert_eq!(RuntimePad::from(f32::NAN),      "??:??:??");
/// assert_eq!(RuntimePad::from(f64::INFINITY), "??:??:??");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct RuntimePad(pub(super) f32, pub(super) Str<{ RuntimePad::MAX_LEN }>);

crate::run::runtime::impl_runtime! {
	self  = RuntimePad,
	len   = RuntimeMilli::MAX_LEN,
	union = as_str_pad,

	other = Runtime,
	other = RuntimeMilli,
}
impl_math!(RuntimePad, f32);
impl_traits!(RuntimePad, f32);

//---------------------------------------------------------------------------------------------------- RuntimePad Constants
impl RuntimePad {
	/// The max length of [`RuntimePad`]'s string.
	pub const MAX_LEN: usize = 8;

	/// [`f32`] returned when calling [`RuntimePad::zero`]
	pub const ZERO_F32: f32 = 0.0;

	/// [`f32`] returned when calling [`RuntimePad::second`]
	pub const SECOND_F32: f32 = 1.0;

	/// [`f32`] returned when calling [`RuntimePad::minute`]
	pub const MINUTE_F32: f32 = 60.0;

	/// [`f32`] returned when calling [`RuntimePad::hour`]
	pub const HOUR_F32: f32 = 3600.0;

	/// [`f32`] returned when calling [`RuntimePad::day`]
	pub const DAY_F32: f32 = 86400.0;

	/// Input greater to [`RuntimePad`] will make it return [`Self::MAX`]
	pub const MAX_F32: f32 = 359999.0;

	/// Returned when using [`RuntimePad::unknown`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::UNKNOWN, 0.0);
	/// assert_eq!(RuntimePad::UNKNOWN, "??:??:??");
	/// ```
	pub const UNKNOWN: Self = Self(Self::ZERO_F32, Str::from_static_str("??:??:??"));

	/// Returned when using [`RuntimePad::zero`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::ZERO, 0.0);
	/// assert_eq!(RuntimePad::ZERO, "00:00:00");
	/// ```
	pub const ZERO: Self = Self(Self::ZERO_F32, Str::from_static_str("00:00:00"));

	/// Returned when using [`RuntimePad::second`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::SECOND, 1.0);
	/// assert_eq!(RuntimePad::SECOND, "00:00:01");
	/// ```
	pub const SECOND: Self = Self(Self::SECOND_F32, Str::from_static_str("00:00:01"));

	/// Returned when using [`RuntimePad::minute`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::MINUTE, 60.0);
	/// assert_eq!(RuntimePad::MINUTE, "00:01:00");
	/// ```
	pub const MINUTE: Self = Self(Self::MINUTE_F32, Str::from_static_str("00:01:00"));

	/// Returned when using [`RuntimePad::hour`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::HOUR, 3600.0);
	/// assert_eq!(RuntimePad::HOUR, "01:00:00");
	/// ```
	pub const HOUR: Self = Self(Self::HOUR_F32, Str::from_static_str("01:00:00"));

	/// Returned when using [`RuntimePad::day`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::DAY, 86400.0);
	/// assert_eq!(RuntimePad::DAY, "24:00:00");
	/// ```
	pub const DAY: Self = Self(Self::DAY_F32, Str::from_static_str("24:00:00"));

	/// Returned when using [`RuntimePad::max`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::MAX, 359999.0);
	/// assert_eq!(RuntimePad::MAX, "99:59:59");
	/// ```
	pub const MAX: Self = Self(Self::MAX_F32, Str::from_static_str("99:59:59"));
}

//---------------------------------------------------------------------------------------------------- Impl
impl RuntimePad {
	impl_common!(f32);
	impl_const!();
	impl_usize!();

	#[inline]
	/// Dynamically format [`Self`] as a [`Runtime`].
	///
	/// As [`RuntimePad`] is a superset of [`Runtime`], it can
	/// cut off a few characters and format itself as [`Runtime`].
	///
	/// This branches a maximum of 4 times and does not allocate anything.
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::from(0.0).as_str_runtime(),     "0:00");
	/// assert_eq!(RuntimePad::from(59.0).as_str_runtime(),    "0:59");
	/// assert_eq!(RuntimePad::from(599.0).as_str_runtime(),   "9:59");
	/// assert_eq!(RuntimePad::from(3599.0).as_str_runtime(),  "59:59");
	/// assert_eq!(RuntimePad::from(35999.0).as_str_runtime(), "9:59:59");
	/// assert_eq!(RuntimePad::from(36000.0).as_str_runtime(), "10:00:00");
	/// ```
	pub const fn as_str_runtime(&self) -> &str {
		crate::run::runtime_milli::impl_as_str_runtime_inner!(self)
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::unknown(), RuntimePad::UNKNOWN);
	/// ```
	pub const fn unknown() -> Self {
		Self::UNKNOWN
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::zero(), RuntimePad::ZERO);
	/// ```
	pub const fn zero() -> Self {
		Self::ZERO
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::second(), RuntimePad::SECOND);
	/// ```
	pub const fn second() -> Self {
		Self::SECOND
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::minute(), RuntimePad::MINUTE);
	/// ```
	pub const fn minute() -> Self {
		Self::MINUTE
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::hour(), RuntimePad::HOUR);
	/// ```
	pub const fn hour() -> Self {
		Self::HOUR
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::day(), RuntimePad::DAY);
	/// ```
	pub const fn day() -> Self {
		Self::DAY
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimePad::max(), RuntimePad::MAX);
	/// ```
	pub const fn max() -> Self {
		Self::MAX
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(RuntimePad::UNKNOWN.is_unknown());
	/// assert!(!RuntimePad::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		match self.1.as_bytes() {
			b"??:??:??" => true,
			_ => false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl RuntimePad {
	#[allow(unreachable_code)]
	// Private function used in float `From`.
	//
	// INVARIANT:
	// `handle_float!()` should be
	// called before this function.
	pub(super) fn priv_from(runtime: f32) -> Self {
		let Some((h, m, s)) = Runtime::priv_from_inner(runtime) else {
			return Self::unknown();
		};

		if (h, m, s) == (0.0, 0.0, 0.0) {
			return Self::zero();
		}

		let (hours, minutes, seconds) = (h as u32, m as u32, s as u32);

		// Format.
		let mut buf = [0; Self::MAX_LEN];
		Self::format(&mut buf, hours, minutes, seconds);

		Self(runtime, unsafe { Str::from_raw(buf, Self::MAX_LEN as u8) })
	}

	#[inline]
	// 0 Padding for `hh:mm:ss` according to `RuntimePad` rules.
	fn format(buf: &mut [u8; Self::MAX_LEN], hour: u32, min: u32, sec: u32) {
		debug_assert!(hour < 100);
		debug_assert!(min < 60);
		debug_assert!(sec < 60);

		const Z: u8 = b'0';
		const C: u8 = b':';
		// Colons are always in the same position.
		buf[2] = C;
		buf[5] = C;

		let mut h = crate::ItoaTmp::new();
		let mut m = crate::ItoaTmp::new();
		let mut s = crate::ItoaTmp::new();
		let h = h.format(hour).as_bytes();
		let m = m.format(min).as_bytes();
		let s = s.format(sec).as_bytes();

		if h.len() == 1 {
			buf[0] = Z;
			buf[1] = h[0];
		} else {
			buf[0] = h[0];
			buf[1] = h[1];
		}

		if m.len() == 1 {
			buf[3] = Z;
			buf[4] = m[0];
		} else {
			buf[3] = m[0];
			buf[4] = m[1];
		}

		if s.len() == 1 {
			buf[6] = Z;
			buf[7] = s[0];
		} else {
			buf[6] = s[0];
			buf[7] = s[1];
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _format_hms() {
		fn s(b: &[u8]) -> &str {
			std::str::from_utf8(&b).unwrap()
		}

		let mut buf = [0; RuntimePad::MAX_LEN];
		let buf = &mut buf;

		// 0:0:0
		RuntimePad::format(buf, 1, 1, 1);
		assert_eq!(s(buf), "01:01:01");

		// 0:00:0
		RuntimePad::format(buf, 1, 10, 1);
		assert_eq!(s(buf), "01:10:01");

		// 0:0:00
		RuntimePad::format(buf, 1, 1, 10);
		assert_eq!(s(buf), "01:01:10");

		// 0:00:00
		RuntimePad::format(buf, 1, 10, 10);
		assert_eq!(s(buf), "01:10:10");

		// 00:0:0
		RuntimePad::format(buf, 10, 1, 1);
		assert_eq!(s(buf), "10:01:01");

		// 00:00:0
		RuntimePad::format(buf, 10, 10, 1);
		assert_eq!(s(buf), "10:10:01");

		// 00:0:00
		RuntimePad::format(buf, 10, 1, 10);
		assert_eq!(s(buf), "10:01:10");

		// 00:00:00
		RuntimePad::format(buf, 10, 10, 10);
		assert_eq!(s(buf), "10:10:10");

		// 0:0
		RuntimePad::format(buf, 0, 1, 1);
		assert_eq!(s(buf), "00:01:01");

		// 00:0
		RuntimePad::format(buf, 0, 10, 1);
		assert_eq!(s(buf), "00:10:01");

		// 0:00
		RuntimePad::format(buf, 0, 1, 10);
		assert_eq!(s(buf), "00:01:10");

		// 00:00
		RuntimePad::format(buf, 0, 10, 10);
		assert_eq!(s(buf), "00:10:10");
	}

	#[test]
	fn all_uint() {
		for i in 0..RuntimePad::MAX_F32 as u32 {
			let rt = RuntimePad::from(i);
			println!("rt:{} - i: {}", rt, i);
			assert_eq!(rt.inner() as u32, i);
			assert_eq!(rt.inner() as u32, i);
			println!("{}", rt);
		}
	}

	#[test]
	fn all_floats() {
		let mut f = 0.0;
		while f <= RuntimePad::MAX_F32 {
			let rt = RuntimePad::from(f);
			println!("rt: {} - f: {}", rt.inner(), f);
			assert_eq!(rt, f);
			f += 0.1;
		}
	}

	#[test]
	fn overflow_float() {
		assert_eq!(RuntimePad::from(RuntimePad::MAX_F32 + 1.0), 0.0);
		assert_eq!(RuntimePad::from(RuntimePad::MAX_F32 + 1.0), RuntimePad::unknown());
	}

	#[test]
	fn overflow_uint() {
		assert_eq!(RuntimePad::from(RuntimePad::MAX_F32 + 1.0), 0.0);
		assert_eq!(RuntimePad::from(RuntimePad::MAX_F32 + 1.0), RuntimePad::unknown());
	}

	#[test]
	fn special() {
		assert_eq!(RuntimePad::from(f32::NAN),          RuntimePad::unknown());
		assert_eq!(RuntimePad::from(f32::INFINITY),     RuntimePad::unknown());
		assert_eq!(RuntimePad::from(f32::NEG_INFINITY), RuntimePad::unknown());
		assert_eq!(RuntimePad::from(f64::NAN),          RuntimePad::unknown());
		assert_eq!(RuntimePad::from(f64::INFINITY),     RuntimePad::unknown());
		assert_eq!(RuntimePad::from(f64::NEG_INFINITY), RuntimePad::unknown());
	}
}
