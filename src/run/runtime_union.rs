//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::run::{Runtime,RuntimePad,RuntimeMilli};
use crate::macros::{
	impl_common,impl_const,
	impl_traits,return_bad_float,
	impl_usize,impl_math,impl_impl_math,
};
use crate::run::{
	UNKNOWN_RUNTIME,
	UNKNOWN_RUNTIME_PAD,
	UNKNOWN_RUNTIME_MILLI,
	ZERO_RUNTIME,
	ZERO_RUNTIME_PAD,
	ZERO_RUNTIME_MILLI,
	SECOND_RUNTIME,
	SECOND_RUNTIME_PAD,
	SECOND_RUNTIME_MILLI,
	MINUTE_RUNTIME,
	MINUTE_RUNTIME_PAD,
	MINUTE_RUNTIME_MILLI,
	HOUR_RUNTIME,
	HOUR_RUNTIME_PAD,
	HOUR_RUNTIME_MILLI,
	MAX_RUNTIME,
	MAX_RUNTIME_PAD,
	MAX_RUNTIME_MILLI,
	MAX_LEN_RUNTIME,
	MAX_LEN_RUNTIME_MILLI,
	ZERO_RUNTIME_F32,
	SECOND_RUNTIME_F32,
	MINUTE_RUNTIME_F32,
	HOUR_RUNTIME_F32,
	MAX_RUNTIME_F32,
};

//---------------------------------------------------------------------------------------------------- RuntimeUnion
/// All [`Runtime`] types combined
///
/// This is a combination of all [`Runtime`] types so that you don't need to carry around
/// all the different `Runtime`'s when you want to display the same runtime in different ways.
///
/// Since [`RuntimeMilli`] is also a superset of all [`Runtime`]'s, this type's
/// existence may be confusing, although it is a `size` vs `computation` trade-off.
///
/// [`RuntimeMilli`] and [`RuntimePad`] dynamically compute their subset strings
/// based off how large the internal float is, which include multiple branches.
///
/// [`RuntimeUnion`] just stores the final results so there is no computation.
///
/// However, the computation is very small so you should usually just use [`RuntimeMilli`].
///
/// ```rust
/// # use readable::*;
/// let runtime_union = RuntimeUnion::from(65.555);
///
/// // We can display regular `Runtime`
/// assert_eq!(runtime_union.as_str(), "1:05");
///
/// // Or `RuntimePad`
/// assert_eq!(runtime_union.as_str_pad(), "00:01:05");
///
/// // Or `RuntimeMilli`
/// assert_eq!(runtime_union.as_str_milli(), "00:01:05.555");
/// ```
///
/// ## Size
/// Two strings are stored internally:
/// - A [`Str<8>`] for the regular [`Runtime`] formatted string
/// - A [`Str<12>`] for the [`RuntimeMilli`] formatted string
///
/// Since [`RuntimePad`] is a strict subset of [`RuntimeMilli`], we don't need to store it.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<RuntimeUnion>(), 36);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::*;
/// // Always round down.
/// assert_eq!(RuntimeUnion::from(11.111).as_str_milli(), "00:00:11.111");
/// assert_eq!(RuntimeUnion::from(11.999).as_str_milli(), "00:00:11.999");
///
/// assert_eq!(RuntimeUnion::from(111.111).as_str_pad(), "00:01:51");
/// assert_eq!(RuntimeUnion::from(111.999).as_str_pad(), "00:01:51");
///
/// assert_eq!(RuntimeUnion::from(11111.1).as_str(), "3:05:11");
/// assert_eq!(RuntimeUnion::from(11111.9).as_str(), "3:05:11");
///
/// assert_eq!(RuntimeUnion::from(f32::NAN).as_str(),               "?:??");
/// assert_eq!(RuntimeUnion::from(f64::INFINITY).as_str_pad(),      "??:??:??");
/// assert_eq!(RuntimeUnion::from(f32::NEG_INFINITY).as_str_milli(), "??:??:??.???");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct RuntimeUnion {
	pub(super) float: f32,
	pub(super) runtime: Str<MAX_LEN_RUNTIME>,
	pub(super) runtime_pad: Str<MAX_LEN_RUNTIME>,
	pub(super) runtime_milli: Str<MAX_LEN_RUNTIME_MILLI>,
}

crate::run::runtime::impl_runtime! {
	self  = RuntimeUnion,
	other = Runtime,
	other = RuntimePad,
	other = RuntimeMilli,
}

impl_math!(RuntimeUnion, f32);

//---------------------------------------------------------------------------------------------------- RuntimeUnion Impl
impl RuntimeUnion {
	#[inline]
	/// Returns the inner number.
	pub const fn inner(&self) -> f32 {
		self.float
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeUnion::from(65.555).as_str(), "1:05");
	/// ```
	pub const fn as_str(&self) -> &str {
		self.runtime.as_str()
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeUnion::from(65.555).as_str_pad(), "00:01:05");
	/// ```
	pub const fn as_str_pad(&self) -> &str {
		self.runtime_pad.as_str()
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeUnion::from(65.555).as_str_milli(), "00:01:05.555");
	/// ```
	pub const fn as_str_milli(&self) -> &str {
		self.runtime_milli.as_str()
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeUnion::unknown(),                0.0);
	/// assert_eq!(RuntimeUnion::unknown().as_str(),       "?:??");
	/// assert_eq!(RuntimeUnion::unknown().as_str_pad(),   "??:??:??");
	/// assert_eq!(RuntimeUnion::unknown().as_str_milli(), "??:??:??.???");
	/// ```
	pub const fn unknown() -> Self {
		Self {
			float: ZERO_RUNTIME_F32,
			runtime: Str::from_static_str(UNKNOWN_RUNTIME),
			runtime_pad: Str::from_static_str(UNKNOWN_RUNTIME_PAD),
			runtime_milli: Str::from_static_str(UNKNOWN_RUNTIME_MILLI),
		}
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeUnion::zero(),                0.0);
	/// assert_eq!(RuntimeUnion::zero().as_str(),       "0:00");
	/// assert_eq!(RuntimeUnion::zero().as_str_pad(),   "00:00:00");
	/// assert_eq!(RuntimeUnion::zero().as_str_milli(), "00:00:00.000");
	/// assert_eq!(RuntimeUnion::zero(), RuntimeUnion::from(0.0));
	/// ```
	pub const fn zero() -> Self {
		Self {
			float: ZERO_RUNTIME_F32,
			runtime: Str::from_static_str(ZERO_RUNTIME),
			runtime_pad: Str::from_static_str(ZERO_RUNTIME_PAD),
			runtime_milli: Str::from_static_str(ZERO_RUNTIME_MILLI),
		}
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeUnion::second(),                1.0);
	/// assert_eq!(RuntimeUnion::second().as_str(),       "0:01");
	/// assert_eq!(RuntimeUnion::second().as_str_pad(),   "00:00:01");
	/// assert_eq!(RuntimeUnion::second().as_str_milli(), "00:00:01.000");
	/// assert_eq!(RuntimeUnion::second(), RuntimeUnion::from(1.0));
	/// ```
	pub const fn second() -> Self {
		Self {
			float: SECOND_RUNTIME_F32,
			runtime: Str::from_static_str(SECOND_RUNTIME),
			runtime_pad: Str::from_static_str(SECOND_RUNTIME_PAD),
			runtime_milli: Str::from_static_str(SECOND_RUNTIME_MILLI),
		}
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeUnion::minute(),                60.0);
	/// assert_eq!(RuntimeUnion::minute().as_str(),       "1:00");
	/// assert_eq!(RuntimeUnion::minute().as_str_pad(),   "00:01:00");
	/// assert_eq!(RuntimeUnion::minute().as_str_milli(), "00:01:00.000");
	/// assert_eq!(RuntimeUnion::minute(), RuntimeUnion::from(60.0));
	/// ```
	pub const fn minute() -> Self {
		Self {
			float: MINUTE_RUNTIME_F32,
			runtime: Str::from_static_str(MINUTE_RUNTIME),
			runtime_pad: Str::from_static_str(MINUTE_RUNTIME_PAD),
			runtime_milli: Str::from_static_str(MINUTE_RUNTIME_MILLI),
		}
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeUnion::hour(),                3600.0);
	/// assert_eq!(RuntimeUnion::hour().as_str(),       "1:00:00");
	/// assert_eq!(RuntimeUnion::hour().as_str_pad(),   "01:00:00");
	/// assert_eq!(RuntimeUnion::hour().as_str_milli(), "01:00:00.000");
	/// assert_eq!(RuntimeUnion::hour(), RuntimeUnion::from(3600.0));
	/// ```
	pub const fn hour() -> Self {
		Self {
			float: HOUR_RUNTIME_F32,
			runtime: Str::from_static_str(HOUR_RUNTIME),
			runtime_pad: Str::from_static_str(HOUR_RUNTIME_PAD),
			runtime_milli: Str::from_static_str(HOUR_RUNTIME_MILLI),
		}
	}

	#[inline]
	/// ```rust
	/// # use readable::*;
	/// assert_eq!(RuntimeUnion::max(),                359999.0);
	/// assert_eq!(RuntimeUnion::max().as_str(),       "99:59:59");
	/// assert_eq!(RuntimeUnion::max().as_str_pad(),   "99:59:59");
	/// assert_eq!(RuntimeUnion::max().as_str_milli(), "99:59:59.000");
	/// assert_eq!(RuntimeUnion::max(), RuntimeUnion::from(359999.0));
	/// ```
	pub const fn max() -> Self {
		Self {
			float: MAX_RUNTIME_F32,
			runtime: Str::from_static_str(MAX_RUNTIME),
			runtime_pad: Str::from_static_str(MAX_RUNTIME_PAD),
			runtime_milli: Str::from_static_str(MAX_RUNTIME_MILLI),
		}
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl RuntimeUnion {
	#[allow(unreachable_code)]
	#[inline]
	// Private function used in float `From`.
	fn priv_from(float: f32) -> Self {
		let runtime = Runtime::priv_from(float);
		if runtime == UNKNOWN_RUNTIME {
			return Self::unknown();
		}

		let runtime_pad = RuntimePad::priv_from(float);
		if runtime_pad == UNKNOWN_RUNTIME_PAD {
			return Self::unknown();
		}

		let runtime_milli = RuntimeMilli::priv_from(float);
		if runtime_milli == UNKNOWN_RUNTIME_MILLI {
			return Self::unknown();
		}

		Self {
			float,
			runtime: runtime.1,
			runtime_pad: runtime_pad.1,
			runtime_milli: runtime_milli.1,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Trait impl
impl PartialEq<f32> for RuntimeUnion {
	#[inline]
	fn eq(&self, other: &f32) -> bool {
		self.float == *other
	}
}

impl PartialEq<RuntimeUnion> for f32 {
	#[inline]
	fn eq(&self, other: &RuntimeUnion) -> bool {
		*self == other.float
	}
}

impl PartialEq<f32> for &RuntimeUnion {
	#[inline]
	fn eq(&self, other: &f32) -> bool {
		self.float == *other
	}
}

impl PartialEq<&RuntimeUnion> for f32 {
	#[inline]
	fn eq(&self, other: &&RuntimeUnion) -> bool {
		*self == other.float
	}
}