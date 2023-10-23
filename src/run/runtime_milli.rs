//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::run::{Runtime,RuntimePad,RuntimeUnion};
use crate::macros::{
	impl_common,impl_const,
	impl_traits,impl_math,impl_impl_math,
};

//---------------------------------------------------------------------------------------------------- RuntimeMilli
/// [`RuntimePad`] but with milliseconds
///
/// This is the exact same type as [`RuntimePad`], except, the
/// milliseconds are included, which makes this type `4` bytes bigger.
///
/// ```rust
/// # use readable::*;
/// let runtime_full = RuntimePad::minute();
/// assert_eq!(runtime_full, "00:01:00"); // seconds is lowest unit
///
/// let runtime_milli = RuntimeMilli::minute();
/// assert_eq!(runtime_milli, "00:01:00.000"); // millisecond is lowest unit
/// ```
///
/// ## Size
/// [`Str<12>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<RuntimeMilli>(), 20);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::*;
/// // Always round down.
/// assert_eq!(RuntimeMilli::from(11.111), "00:00:11.111");
/// assert_eq!(RuntimeMilli::from(11.999), "00:00:11.999");
///
/// assert_eq!(RuntimeMilli::from(111.111), "00:01:51.111");
/// assert_eq!(RuntimeMilli::from(111.999), "00:01:51.999");
///
/// assert_eq!(RuntimeMilli::from(11111.1), "03:05:11.100");
/// assert_eq!(RuntimeMilli::from(11111.9), "03:05:11.900");
///
/// assert_eq!(RuntimeMilli::from(0.0), "00:00:00.000");
/// assert_eq!(RuntimeMilli::from(1.5), "00:00:01.500");
/// assert_eq!(RuntimeMilli::from(1.9), "00:00:01.900");
/// assert_eq!(RuntimeMilli::from(2.34), "00:00:02.340");
///
/// assert_eq!(RuntimeMilli::from(f32::NAN),      "??:??:??.???");
/// assert_eq!(RuntimeMilli::from(f64::INFINITY), "??:??:??.???");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct RuntimeMilli(pub(super) f32, pub(super) Str<{ RuntimeMilli::MAX_LEN }>);

crate::run::runtime::impl_runtime! {
	self  = RuntimeMilli,
	len   = RuntimeMilli::MAX_LEN,
	union = as_str_milli,

	other = Runtime,
	other = RuntimePad,
}
impl_math!(RuntimeMilli, f32);
impl_traits!(RuntimeMilli, f32);

//---------------------------------------------------------------------------------------------------- RuntimeMilli Constants
impl RuntimeMilli {
	/// The max length of [`RuntimeMilli`]'s string.
	pub const MAX_LEN: usize = 12;

	/// [`f32`] returned when calling [`RuntimeMilli::zero`]
	pub const ZERO_F32: f32 = 0.0;

	/// [`f32`] returned when calling [`RuntimeMilli::second`]
	pub const SECOND_F32: f32 = 1.0;

	/// [`f32`] returned when calling [`RuntimeMilli::minute`]
	pub const MINUTE_F32: f32 = 60.0;

	/// [`f32`] returned when calling [`RuntimeMilli::hour`]
	pub const HOUR_F32: f32 = 3600.0;

	/// [`f32`] returned when calling [`RuntimeMilli::day`]
	pub const DAY_F32: f32 = 86400.0;

	/// Input greater to [`RuntimeMilli`] will make it return [`Self::MAX`]
	pub const MAX_F32: f32 = 359999.0;

	/// Returned when using [`RuntimeMilli::unknown`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::UNKNOWN, 0.0);
	/// assert_eq!(RuntimeMilli::UNKNOWN, "??:??:??.???");
	/// ```
	pub const UNKNOWN: Self = Self(Self::ZERO_F32, Str::from_static_str("??:??:??.???"));

	/// Returned when using [`RuntimeMilli::zero`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::ZERO, 0.0);
	/// assert_eq!(RuntimeMilli::ZERO, "00:00:00.000");
	/// ```
	pub const ZERO: Self = Self(Self::ZERO_F32, Str::from_static_str("00:00:00.000"));

	/// Returned when using [`RuntimeMilli::second`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::SECOND, 1.0);
	/// assert_eq!(RuntimeMilli::SECOND, "00:00:01.000");
	/// ```
	pub const SECOND: Self = Self(Self::SECOND_F32, Str::from_static_str("00:00:01.000"));

	/// Returned when using [`RuntimeMilli::minute`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::MINUTE, 60.0);
	/// assert_eq!(RuntimeMilli::MINUTE, "00:01:00.000");
	/// ```
	pub const MINUTE: Self = Self(Self::MINUTE_F32, Str::from_static_str("00:01:00.000"));

	/// Returned when using [`RuntimeMilli::hour`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::HOUR, 3600.0);
	/// assert_eq!(RuntimeMilli::HOUR, "01:00:00.000");
	/// ```
	pub const HOUR: Self = Self(Self::HOUR_F32, Str::from_static_str("01:00:00.000"));

	/// Returned when using [`RuntimeMilli::day`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::DAY, 86400.0);
	/// assert_eq!(RuntimeMilli::DAY, "24:00:00.000");
	/// ```
	pub const DAY: Self = Self(Self::DAY_F32, Str::from_static_str("24:00:00.000"));

	/// Returned when using [`RuntimeMilli::max`]
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::MAX, 359999.0);
	/// assert_eq!(RuntimeMilli::MAX, "99:59:59.000");
	/// ```
	pub const MAX: Self = Self(Self::MAX_F32, Str::from_static_str("99:59:59.000"));
}

//---------------------------------------------------------------------------------------------------- Impl
macro_rules! impl_as_str_runtime_inner {
	($self:expr) => {{
		let u = $self.0 as u32;

		// 00:0x:00
		let (offset, end) = if u < 600 {
			(4, 4)
		// 00:x0:00
		} else if u < 3600 {
			(3, 5)
		// 0x:00:00
		} else if u < 36000 {
			(1, 7)
		// x0:00:00
		} else {
			debug_assert!(u >= 36000);
			(0, 8)
		};

		// SAFETY:
		// We are manually calculating where the start and
		// end bounds of this `str` is. It is just numbers
		// and colons so this is always UTF8.
		// SAFETY, we trust the buffer.
		unsafe {
			let slice = std::slice::from_raw_parts(
				$self.1.as_ptr().offset(offset),
				end,
			);
			std::str::from_utf8_unchecked(slice)
		}
	}};
}
pub(super) use impl_as_str_runtime_inner;

//---------------------------------------------------------------------------------------------------- Impl
impl RuntimeMilli {
	impl_common!(f32);
	impl_const!();

	#[inline]
	/// Dynamically format [`Self`] as a [`Runtime`].
	///
	/// As [`RuntimeMilli`] is a superset of [`Runtime`], it can
	/// cut off a few characters and format itself as [`Runtime`].
	///
	/// This branches a maximum of 4 times and does not allocate anything.
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::from(0.0).as_str_runtime(),     "0:00");
	/// assert_eq!(RuntimeMilli::from(59.0).as_str_runtime(),    "0:59");
	/// assert_eq!(RuntimeMilli::from(599.0).as_str_runtime(),   "9:59");
	/// assert_eq!(RuntimeMilli::from(3599.0).as_str_runtime(),  "59:59");
	/// assert_eq!(RuntimeMilli::from(35999.0).as_str_runtime(), "9:59:59");
	/// assert_eq!(RuntimeMilli::from(36000.0).as_str_runtime(), "10:00:00");
	/// ```
	pub const fn as_str_runtime(&self) -> &str {
		impl_as_str_runtime_inner!(self)
	}

	#[inline]
	/// Dynamically format [`Self`] as a [`RuntimePad`].
	///
	/// As [`RuntimeMilli`] is a superset of [`RuntimePad`], it can
	/// cut off 4 characters (`.xxx`) and format itself as [`RuntimePad`].
	///
	/// This does not allocate anything.
	///
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::from(0.0).as_str_pad(),     "00:00:00");
	/// assert_eq!(RuntimeMilli::from(59.0).as_str_pad(),    "00:00:59");
	/// assert_eq!(RuntimeMilli::from(599.0).as_str_pad(),   "00:09:59");
	/// assert_eq!(RuntimeMilli::from(3599.0).as_str_pad(),  "00:59:59");
	/// assert_eq!(RuntimeMilli::from(35999.0).as_str_pad(), "09:59:59");
	/// assert_eq!(RuntimeMilli::from(36000.0).as_str_pad(), "10:00:00");
	/// ```
	pub const fn as_str_pad(&self) -> &str {
		// 7 is the last index containing
		// a number, 8 is the `.` then milliseconds.
		const END: usize = 8;

		// SAFETY, we trust the buffer.
		unsafe {
			let slice = std::slice::from_raw_parts(
				self.1.as_ptr(),
				END,
			);
			std::str::from_utf8_unchecked(slice)
		}
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::unknown(), RuntimeMilli::UNKNOWN);
	/// ```
	pub const fn unknown() -> Self {
		Self::UNKNOWN
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::zero(), RuntimeMilli::ZERO);
	/// ```
	pub const fn zero() -> Self {
		Self::ZERO
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::second(), RuntimeMilli::SECOND);
	/// ```
	pub const fn second() -> Self {
		Self::SECOND
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::minute(), RuntimeMilli::MINUTE);
	/// ```
	pub const fn minute() -> Self {
		Self::MINUTE
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::hour(), RuntimeMilli::HOUR);
	/// ```
	pub const fn hour() -> Self {
		Self::HOUR
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::day(), RuntimeMilli::DAY);
	/// ```
	pub const fn day() -> Self {
		Self::DAY
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeMilli::max(), RuntimeMilli::MAX);
	/// ```
	pub const fn max() -> Self {
		Self::MAX
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert!(RuntimeMilli::UNKNOWN.is_unknown());
	/// assert!(!RuntimeMilli::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		match self.1.as_bytes() {
			b"??:??:??.???" => true,
			_ => false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl RuntimeMilli {
	#[allow(unreachable_code)]
	#[inline]
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

		// println!("h: {}, m: {}, s: {}, mm: {}", h as u8, m as u8, s as u8, (1_000.0 * s.fract()).round() as u16);

		// Format.
		let mut buf = [0; Self::MAX_LEN];
		Self::format(
			&mut buf,
			h as u8,
			m as u8,
			s as u8,
			(1000.0 * s.fract()).round() as u16,
		);

		Self(runtime, unsafe { Str::from_raw(buf, Self::MAX_LEN as u8) })
	}

	#[inline]
	pub(super) fn priv_from_inner(runtime: f32) -> Option<(f32, f32, f32)> {
		// Zero Self::MAXgth_LEN.
		if runtime <= 0.0 {
			return Some((0.0, 0.0, 0.0));
		}

		// Return unknown if over max.
		if runtime > Self::MAX_F32 {
			return None;
		}

	    let mut hours = (runtime / 60.0) / 60.0;

		let mut minutes = ((runtime / 60.0) % 60.0) + hours.fract();
		// Add remainders.
		if minutes >= 60.0 {
			hours += 1.0;
			minutes -= 60.0;
		}

		let mut seconds = (runtime % 60.0) + minutes.fract();
		// Add remainders.
		if seconds >= 60.0 {
			minutes += 1.0;
			seconds -= 60.0;
			if minutes >= 60.0 {
				hours += 1.0;
				minutes -= 60.0;
			}
		}

		if hours >= 100.0 {
			hours = 99.0;
		}


		Some((hours, minutes, seconds))
	}

	#[inline]
	// 0 Padding for `hh:mm:ss` according to `RuntimeMilli` rules.
	fn format(buf: &mut [u8; Self::MAX_LEN], hour: u8, min: u8, sec: u8, milli: u16) {
		debug_assert!(hour < 100);
		debug_assert!(min < 60);
		debug_assert!(sec < 60);

		const Z: u8 = b'0';
		const C: u8 = b':';
		// Colons are always in the same position.
		buf[2] = C;
		buf[5] = C;
		buf[8] = b'.';

		let mut h = crate::ItoaTmp::new();
		let mut m = crate::ItoaTmp::new();
		let mut s = crate::ItoaTmp::new();
		let mut i = crate::ItoaTmp::new();
		let h = h.format(hour).as_bytes();
		let m = m.format(min).as_bytes();
		let s = s.format(sec).as_bytes();
		let i = i.format(milli).as_bytes();

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

		match i.len() {
			1 => {
				buf[9] = Z;
				buf[10] = Z;
				buf[11] = i[0];
			},
			2 => {
				buf[9] = Z;
				buf[10] = i[0];
				buf[11] = i[1];
			},
			_ => {
				buf[9] = i[0];
				buf[10] = i[1];
				buf[11] = i[2];
			},
		}
	}
}

// ---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _format_hms() {
		fn s(b: &[u8]) -> &str {
			std::str::from_utf8(b).unwrap()
		}

		let mut buf = [0; RuntimeMilli::MAX_LEN];
		let buf = &mut buf;

		// 0:0:0
		RuntimeMilli::format(buf, 1, 1, 1, 555);
		assert_eq!(s(buf), "01:01:01.555");

		// 0:00:0
		RuntimeMilli::format(buf, 1, 10, 1, 123);
		assert_eq!(s(buf), "01:10:01.123");

		// 0:0:00
		RuntimeMilli::format(buf, 1, 1, 10, 111);
		assert_eq!(s(buf), "01:01:10.111");

		// 0:00:00
		RuntimeMilli::format(buf, 1, 10, 10, 33);
		assert_eq!(s(buf), "01:10:10.033");

		// 00:0:0
		RuntimeMilli::format(buf, 10, 1, 1, 1);
		assert_eq!(s(buf), "10:01:01.001");

		// 00:00:0
		RuntimeMilli::format(buf, 10, 10, 1, 11);
		assert_eq!(s(buf), "10:10:01.011");

		// 00:0:00
		RuntimeMilli::format(buf, 10, 1, 10, 999);
		assert_eq!(s(buf), "10:01:10.999");

		// 00:00:00
		RuntimeMilli::format(buf, 10, 10, 10, 512);
		assert_eq!(s(buf), "10:10:10.512");

		// 0:0
		RuntimeMilli::format(buf, 0, 1, 1, 100);
		assert_eq!(s(buf), "00:01:01.100");

		// 00:0
		RuntimeMilli::format(buf, 0, 10, 1, 101);
		assert_eq!(s(buf), "00:10:01.101");

		// 0:00
		RuntimeMilli::format(buf, 0, 1, 10, 2);
		assert_eq!(s(buf), "00:01:10.002");

		// 00:00
		RuntimeMilli::format(buf, 0, 10, 10, 3);
		assert_eq!(s(buf), "00:10:10.003");
	}

	#[test]
	fn all_uint() {
		for i in 0..RuntimeMilli::MAX_F32 as u32 {
			let rt = RuntimeMilli::from(i);
			println!("rt:{} - i: {}", rt, i);
			assert_eq!(rt.inner() as u32, i);
			assert_eq!(rt.inner() as u32, i);
			println!("{}", rt);
		}
	}

	#[test]
	fn all_floats() {
		let mut f = 1.0;
		while f < RuntimeMilli::MAX_F32 {
			let rt = RuntimeMilli::from(f);
			println!("rt: {} - f: {}", rt, f);
			assert_eq!(rt, f);
			f += 0.1;
		}
	}

	#[test]
	fn overflow_float() {
		assert_eq!(RuntimeMilli::from(RuntimeMilli::MAX_F32 + 1.0), 0.0);
		assert_eq!(RuntimeMilli::from(RuntimeMilli::MAX_F32 + 1.0), RuntimeMilli::unknown());
	}

	#[test]
	fn overflow_uint() {
		assert_eq!(RuntimeMilli::from(RuntimeMilli::MAX_F32 + 1.0), 0.0);
		assert_eq!(RuntimeMilli::from(RuntimeMilli::MAX_F32 + 1.0), RuntimeMilli::unknown());
	}

	#[test]
	fn special() {
		assert_eq!(RuntimeMilli::from(f32::NAN),          RuntimeMilli::unknown());
		assert_eq!(RuntimeMilli::from(f32::INFINITY),     RuntimeMilli::unknown());
		assert_eq!(RuntimeMilli::from(f32::NEG_INFINITY), RuntimeMilli::unknown());
		assert_eq!(RuntimeMilli::from(f64::NAN),          RuntimeMilli::unknown());
		assert_eq!(RuntimeMilli::from(f64::INFINITY),     RuntimeMilli::unknown());
		assert_eq!(RuntimeMilli::from(f64::NEG_INFINITY), RuntimeMilli::unknown());
	}
}
