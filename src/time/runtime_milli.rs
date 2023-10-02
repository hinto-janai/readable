//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::time::{Runtime,RuntimePad,RuntimeUnion};
use crate::macros::{
	impl_common,impl_const,
	impl_traits,return_bad_float,
	impl_usize,impl_math,impl_impl_math,
};
use crate::time::{
	ZERO_RUNTIME_F32,
	SECOND_RUNTIME_F32,
	MINUTE_RUNTIME_F32,
	HOUR_RUNTIME_F32,
	MAX_RUNTIME_F32,
};

//---------------------------------------------------------------------------------------------------- Constants (Public)
/// The max length of [`RuntimeMilli`]'s string.
pub const MAX_LEN_RUNTIME_MILLI: usize = 12;

/// [`str`] returned when using [`RuntimeMilli::unknown`]
pub const UNKNOWN_RUNTIME_MILLI: &str = "??:??:??.???";

/// [`str`] returned when using [`RuntimeMilli::zero`]
pub const ZERO_RUNTIME_MILLI: &str = "00:00:00.000";

/// [`str`] returned when using [`RuntimeMilli::second`]
pub const SECOND_RUNTIME_MILLI: &str = "00:00:01.000";

/// [`str`] returned when using [`RuntimeMilli::minute`]
pub const MINUTE_RUNTIME_MILLI: &str = "00:01:00.000";

/// [`str`] returned when using [`RuntimeMilli::hour`]
pub const HOUR_RUNTIME_MILLI: &str = "01:00:00.000";

/// [`str`] for the max time [`RuntimeMilli`] can handle
pub const MAX_RUNTIME_MILLI: &str = "99:59:59.000";

//---------------------------------------------------------------------------------------------------- RuntimeMilli
/// [`RuntimePad`], but with milliseconds
///
/// This is the exact same type as [`RuntimePad`], except, the
/// milliseconds are included, which makes this type `4` bytes bigger.
///
/// ```rust
/// # use readable::time::*;
/// let runtime_full = RuntimePad::minute();
/// assert_eq!(runtime_full, "00:01:00"); // seconds is lowest unit
///
/// let runtime_milli = RuntimeMilli::minute();
/// assert_eq!(runtime_milli, "00:01:00.000"); // millisecond is lowest unit
/// ```
///
/// [`RuntimeMilli::from`] input can be:
/// - [`u8`]
/// - [`u16`]
/// - [`f64`]
/// - [`u64`]
/// - [`u128`]
/// - [`usize`]
/// - [`f32`]
/// - [`f64`]
/// - [`std::time::Duration`]
/// - [`std::time::Instant`]
/// - Other [`Runtime`] types
///
/// Integer inputs are presumed to be in _seconds._
///
/// ## Errors
/// The max input is `359999` seconds, or: `99:59:59`.
///
/// If the input is larger than [`MAX_RUNTIME_MILLI`], [`Self::unknown()`] is returned.
///
/// ## Copy
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 8 byte array buffer, literally: [`Str<8>`].
///
/// Since the max valid runtime is: `99:59:59` (8 characters, `359999` seconds), an 8 byte
/// buffer is used and enables this type to have [`Copy`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
/// ```rust
/// # use readable::RuntimeMilli;
/// let a = RuntimeMilli::from(100_000.0);
///
/// // Copy 'a', use 'b'.
/// let b = a;
/// assert_eq!(b, 100_000.0);
///
/// // We can still use 'a'
/// assert_eq!(a, 100_000.0);
/// ```
///
/// ## Size
/// ```rust
/// # use readable::time::*;
/// assert_eq!(std::mem::size_of::<RuntimeMilli>(), 20);
/// ```
///
/// ## Exceptions
/// Inputting [`f64::NAN`], [`f64::INFINITY`], [`f64::NEG_INFINITY`] or the [`f32`] variants returns errors
///
/// ## Math
/// These operators are overloaded. They will always output a new [`Self`]:
/// - `Add +`
/// - `Sub -`
/// - `Div /`
/// - `Mul *`
/// - `Rem %`
///
/// They can either be:
/// - Combined with another [`Self`]: `RuntimeMilli::from(1) + RuntimeMilli::from(1)`
/// - Or with the inner number itself: `RuntimeMilli::from(1) + 1`
///
/// ```rust
/// # use readable::*;
/// let n = RuntimeMilli::from(f32::MAX) + f32::MAX;
/// assert!(n == RuntimeMilli::unknown());
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::RuntimeMilli;
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
pub struct RuntimeMilli(pub(super) f32, pub(super) Str<MAX_LEN_RUNTIME_MILLI>);

crate::time::runtime::impl_runtime! {
	self  = RuntimeMilli,
	len   = MAX_LEN_RUNTIME_MILLI,
	union = as_str_milli,

	other = Runtime,
	other = RuntimePad,
}
impl_math!(RuntimeMilli, f32);
impl_traits!(RuntimeMilli, f32);

//---------------------------------------------------------------------------------------------------- Impl
impl RuntimeMilli {
	impl_common!(f32);
	impl_const!();

	#[inline]
	/// ```rust
	/// # use readable::RuntimeMilli;
	/// assert_eq!(RuntimeMilli::unknown(), 0.0);
	/// assert_eq!(RuntimeMilli::unknown(), "??:??:??.???");
	/// ```
	pub const fn unknown() -> Self {
		Self(ZERO_RUNTIME_F32, Str::from_static_str(UNKNOWN_RUNTIME_MILLI))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeMilli;
	/// assert_eq!(RuntimeMilli::zero(), 0.0);
	/// assert_eq!(RuntimeMilli::zero(), "00:00:00.000");
	/// ```
	pub const fn zero() -> Self {
		Self(ZERO_RUNTIME_F32, Str::from_static_str(ZERO_RUNTIME_MILLI))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeMilli;
	/// assert_eq!(RuntimeMilli::second(), 1.0);
	/// assert_eq!(RuntimeMilli::second(), "00:00:01.000");
	/// assert_eq!(RuntimeMilli::second(), RuntimeMilli::from(1.0));
	/// ```
	pub const fn second() -> Self {
		Self(SECOND_RUNTIME_F32, Str::from_static_str(SECOND_RUNTIME_MILLI))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeMilli;
	/// assert_eq!(RuntimeMilli::minute(), 60.0);
	/// assert_eq!(RuntimeMilli::minute(), "00:01:00.000");
	/// assert_eq!(RuntimeMilli::minute(), RuntimeMilli::from(60.0));
	/// ```
	pub const fn minute() -> Self {
		Self(MINUTE_RUNTIME_F32, Str::from_static_str(MINUTE_RUNTIME_MILLI))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeMilli;
	/// assert_eq!(RuntimeMilli::hour(), 3600.0);
	/// assert_eq!(RuntimeMilli::hour(), "01:00:00.000");
	/// assert_eq!(RuntimeMilli::hour(), RuntimeMilli::from(3600.0));
	/// ```
	pub const fn hour() -> Self {
		Self(HOUR_RUNTIME_F32, Str::from_static_str(HOUR_RUNTIME_MILLI))
	}

	#[inline]
	/// ```rust
	/// # use readable::RuntimeMilli;
	/// assert_eq!(RuntimeMilli::max(), 359999.0);
	/// assert_eq!(RuntimeMilli::max(), "99:59:59.000");
	/// assert_eq!(RuntimeMilli::max(), RuntimeMilli::from(359999.0));
	/// ```
	pub const fn max() -> Self {
		Self(MAX_RUNTIME_F32, Str::from_static_str(MAX_RUNTIME_MILLI))
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
		let mut buf = [0; MAX_LEN_RUNTIME_MILLI];
		Self::format(
			&mut buf,
			h as u8,
			m as u8,
			s as u8,
			(1000.0 * s.fract()).round() as u16,
		);

		Self(runtime, unsafe { Str::from_raw(MAX_LEN_RUNTIME_MILLI as u8, buf) })
	}

	#[inline]
	pub(super) fn priv_from_inner(runtime: f32) -> Option<(f32, f32, f32)> {
		// Zero MAX_LEN_RUNTIME_MILLIgth.
		if runtime <= 0.0 {
			return Some((0.0, 0.0, 0.0));
		}

		// Return unknown if over max.
		if runtime > MAX_RUNTIME_F32 {
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
	fn format(buf: &mut [u8; MAX_LEN_RUNTIME_MILLI], hour: u8, min: u8, sec: u8, milli: u16) {
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
			std::str::from_utf8(&b).unwrap()
		}

		let mut buf = [0; MAX_LEN_RUNTIME_MILLI];
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
		for i in 0..MAX_RUNTIME_F32 as u32 {
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
		while f < MAX_RUNTIME_F32 {
			let rt = RuntimeMilli::from(f);
			println!("rt: {} - f: {}", rt, f);
			assert_eq!(rt, f);
			f += 0.1;
		}
	}

	#[test]
	fn overflow_float() {
		assert_eq!(RuntimeMilli::from(MAX_RUNTIME_F32 + 1.0), 0.0);
		assert_eq!(RuntimeMilli::from(MAX_RUNTIME_F32 + 1.0), RuntimeMilli::unknown());
	}

	#[test]
	fn overflow_uint() {
		assert_eq!(RuntimeMilli::from(MAX_RUNTIME_F32 + 1.0), 0.0);
		assert_eq!(RuntimeMilli::from(MAX_RUNTIME_F32 + 1.0), RuntimeMilli::unknown());
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
