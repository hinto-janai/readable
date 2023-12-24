//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::run::{Runtime,RuntimeMilli,RuntimeUnion};
use crate::macros::{
	impl_common,impl_const,
	impl_traits,impl_usize,
	impl_math,impl_impl_math,
};

//---------------------------------------------------------------------------------------------------- RuntimePad
/// [`Runtime`] but always full length and padded with zeros
///
/// This is the exact same type as [`Runtime`], except, the
/// numbers will _always_ be padded with `0`'s.
///
/// ```rust
/// # use readable::run::*;
/// let runtime = Runtime::MINUTE;
/// assert_eq!(runtime, "1:00"); // hour left out, minute not padded
///
/// let runtime_zero = RuntimePad::MINUTE;
/// assert_eq!(runtime_zero, "00:01:00"); // always includes everything
///
/// let runtime = Runtime::HOUR;
/// assert_eq!(runtime, "1:00:00");
///
/// let runtime_zero = RuntimePad::HOUR;
/// assert_eq!(runtime_zero, "01:00:00");
/// ```
///
/// ## Size
/// [`Str<8>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::run::*;
/// assert_eq!(std::mem::size_of::<RuntimePad>(), 16);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::run::*;
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
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
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

	/// [`f32`] inside of [`RuntimePad::ZERO`]
	pub const ZERO_F32: f32 = 0.0;

	/// [`f32`] inside of [`RuntimePad::SECOND`]
	pub const SECOND_F32: f32 = 1.0;

	/// [`f32`] inside of [`RuntimePad::MINUTE`]
	pub const MINUTE_F32: f32 = 60.0;

	/// [`f32`] inside of [`RuntimePad::HOUR`]
	pub const HOUR_F32: f32 = 3600.0;

	/// [`f32`] inside of [`RuntimePad::DAY`]
	pub const DAY_F32: f32 = 86400.0;

	/// Input greater to [`RuntimePad`] will make it return [`Self::MAX`]
	pub const MAX_F32: f32 = 359999.0;

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimePad::UNKNOWN, 0.0);
	/// assert_eq!(RuntimePad::UNKNOWN, "??:??:??");
	/// ```
	pub const UNKNOWN: Self = Self(Self::ZERO_F32, Str::from_static_str("??:??:??"));

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimePad::ZERO, 0.0);
	/// assert_eq!(RuntimePad::ZERO, "00:00:00");
	/// ```
	pub const ZERO: Self = Self(Self::ZERO_F32, Str::from_static_str("00:00:00"));

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimePad::SECOND, 1.0);
	/// assert_eq!(RuntimePad::SECOND, "00:00:01");
	/// ```
	pub const SECOND: Self = Self(Self::SECOND_F32, Str::from_static_str("00:00:01"));

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimePad::MINUTE, 60.0);
	/// assert_eq!(RuntimePad::MINUTE, "00:01:00");
	/// ```
	pub const MINUTE: Self = Self(Self::MINUTE_F32, Str::from_static_str("00:01:00"));

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimePad::HOUR, 3600.0);
	/// assert_eq!(RuntimePad::HOUR, "01:00:00");
	/// ```
	pub const HOUR: Self = Self(Self::HOUR_F32, Str::from_static_str("01:00:00"));

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimePad::DAY, 86400.0);
	/// assert_eq!(RuntimePad::DAY, "24:00:00");
	/// ```
	pub const DAY: Self = Self(Self::DAY_F32, Str::from_static_str("24:00:00"));

	/// ```rust
	/// # use readable::run::*;
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
	#[must_use]
	/// Dynamically format [`Self`] as a [`Runtime`].
	///
	/// As [`RuntimePad`] is a superset of [`Runtime`], it can
	/// cut off a few characters and format itself as [`Runtime`].
	///
	/// This branches a maximum of 4 times and does not allocate anything.
	///
	/// ```rust
	/// # use readable::run::*;
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
	#[must_use]
	/// ```rust
	/// # use readable::run::*;
	/// assert!(RuntimePad::UNKNOWN.is_unknown());
	/// assert!(!RuntimePad::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		matches!(self.1.as_bytes(), b"??:??:??")
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
			return Self::UNKNOWN;
		};

		if (h, m, s) == (0.0, 0.0, 0.0) {
			return Self::ZERO;
		}

		let (hours, minutes, seconds) = (h as u32, m as u32, s as u32);

		// Format.
		let mut buf = [0; Self::MAX_LEN];
		Self::format(&mut buf, hours, minutes, seconds);

		// SAFETY: we know the str len
		Self(runtime, unsafe { Str::from_raw(buf, Self::MAX_LEN as u8) })
	}

	#[inline]
	// 0 Padding for `hh:mm:ss` according to `RuntimePad` rules.
	fn format(buf: &mut [u8; Self::MAX_LEN], hour: u32, min: u32, sec: u32) {
		const Z: u8 = b'0';
		const C: u8 = b':';

		debug_assert!(hour < 100);
		debug_assert!(min < 60);
		debug_assert!(sec < 60);

		// Colons are always in the same position.
		buf[2] = C;
		buf[5] = C;

		let mut h = crate::toa::ItoaTmp::new();
		let mut m = crate::toa::ItoaTmp::new();
		let mut s = crate::toa::ItoaTmp::new();
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
			std::str::from_utf8(b).unwrap()
		}

		let buf = &mut [0; RuntimePad::MAX_LEN];

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
			println!("rt: {rt} - i: {i}");
			assert_eq!(rt.inner() as u32, i);
			assert_eq!(rt.inner() as u32, i);
			println!("{rt}");
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
		assert_eq!(RuntimePad::from(RuntimePad::MAX_F32 + 1.0), RuntimePad::UNKNOWN);
	}

	#[test]
	fn overflow_uint() {
		assert_eq!(RuntimePad::from(RuntimePad::MAX_F32 + 1.0), 0.0);
		assert_eq!(RuntimePad::from(RuntimePad::MAX_F32 + 1.0), RuntimePad::UNKNOWN);
	}

	#[test]
	fn special() {
		assert_eq!(RuntimePad::from(f32::NAN),          RuntimePad::UNKNOWN);
		assert_eq!(RuntimePad::from(f32::INFINITY),     RuntimePad::UNKNOWN);
		assert_eq!(RuntimePad::from(f32::NEG_INFINITY), RuntimePad::UNKNOWN);
		assert_eq!(RuntimePad::from(f64::NAN),          RuntimePad::UNKNOWN);
		assert_eq!(RuntimePad::from(f64::INFINITY),     RuntimePad::UNKNOWN);
		assert_eq!(RuntimePad::from(f64::NEG_INFINITY), RuntimePad::UNKNOWN);
	}

	#[test]
	#[cfg(feature = "serde")]
	fn serde() {
		let this: RuntimePad = RuntimePad::from(111.999);
		let json = serde_json::to_string(&this).unwrap();
		assert_eq!(json, r#"[111.999,"00:01:51"]"#);

		let this: RuntimePad = serde_json::from_str(&json).unwrap();
		assert_eq!(this, 111.999);
		assert_eq!(this, "00:01:51");

		// Bad bytes.
		assert!(serde_json::from_str::<RuntimePad>(&"---").is_err());
	}

	#[test]
	#[cfg(feature = "bincode")]
	fn bincode() {
		let this: RuntimePad = RuntimePad::from(111.999);
		let config = bincode::config::standard();
		let bytes = bincode::encode_to_vec(&this, config).unwrap();

		let this: RuntimePad = bincode::decode_from_slice(&bytes, config).unwrap().0;
		assert_eq!(this, 111.999);
		assert_eq!(this, "00:01:51");
	}

	#[test]
	#[cfg(feature = "bincode")]
	fn borsh() {
		let this: RuntimePad = RuntimePad::from(111.999);
		let bytes = borsh::to_vec(&this).unwrap();

		let this: RuntimePad = borsh::from_slice(&bytes).unwrap();
		assert_eq!(this, 111.999);
		assert_eq!(this, "00:01:51");

		// Bad bytes.
		assert!(borsh::from_slice::<RuntimePad>(b"bad .-;[]124/ bytes").is_err());
	}
}
