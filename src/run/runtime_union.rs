//---------------------------------------------------------------------------------------------------- Use
use crate::str::Str;
use crate::run::{Runtime,RuntimePad,RuntimeMilli};
use crate::macros::{
	impl_math,impl_impl_math,
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
/// # use readable::run::*;
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
/// 3 strings are stored internally:
/// - A [`Str<8>`] for the [`Runtime`] formatted string
/// - A [`Str<8>`] for the [`RuntimePad`] formatted string
/// - A [`Str<12>`] for the [`RuntimeMilli`] formatted string
///
/// ```rust
/// # use readable::run::*;
/// assert_eq!(std::mem::size_of::<RuntimeUnion>(), 36);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::run::*;
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
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct RuntimeUnion {
	pub(super) float: f32,
	pub(super) runtime: Str<{ Runtime::MAX_LEN }>,
	pub(super) runtime_pad: Str<{ RuntimePad::MAX_LEN }>,
	pub(super) runtime_milli: Str<{ RuntimeMilli::MAX_LEN }>,
}

crate::run::runtime::impl_runtime! {
	self  = RuntimeUnion,
	other = Runtime,
	other = RuntimePad,
	other = RuntimeMilli,
}

impl_math!(RuntimeUnion, f32);

//---------------------------------------------------------------------------------------------------- RuntimeUnion Constants
impl RuntimeUnion {
	/// [`f32`] inside of [`RuntimeUnion::ZERO`]
	pub const ZERO_F32: f32 = 0.0;

	/// [`f32`] inside of [`RuntimeUnion::SECOND`]
	pub const SECOND_F32: f32 = 1.0;

	/// [`f32`] inside of [`RuntimeUnion::MINUTE`]
	pub const MINUTE_F32: f32 = 60.0;

	/// [`f32`] inside of [`RuntimeUnion::HOUR`]
	pub const HOUR_F32: f32 = 3600.0;

	/// [`f32`] inside of [`RuntimeUnion::DAY`]
	pub const DAY_F32: f32 = 86400.0;

	/// Input greater to [`RuntimeUnion`] will make it return [`Self::MAX`]
	pub const MAX_F32: f32 = 359999.0;

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::UNKNOWN,                0.0);
	/// assert_eq!(RuntimeUnion::UNKNOWN.as_str(),       "?:??");
	/// assert_eq!(RuntimeUnion::UNKNOWN.as_str_pad(),   "??:??:??");
	/// assert_eq!(RuntimeUnion::UNKNOWN.as_str_milli(), "??:??:??.???");
	/// ```
	pub const UNKNOWN: Self = Self {
		float: Runtime::ZERO_F32,
		runtime: Runtime::UNKNOWN.1,
		runtime_pad: RuntimePad::UNKNOWN.1,
		runtime_milli: RuntimeMilli::UNKNOWN.1,
	};

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::ZERO,                0.0);
	/// assert_eq!(RuntimeUnion::ZERO.as_str(),       "0:00");
	/// assert_eq!(RuntimeUnion::ZERO.as_str_pad(),   "00:00:00");
	/// assert_eq!(RuntimeUnion::ZERO.as_str_milli(), "00:00:00.000");
	/// assert_eq!(RuntimeUnion::ZERO, RuntimeUnion::from(0.0));
	/// ```
	pub const ZERO: Self = Self {
		float: Runtime::ZERO_F32,
		runtime: Runtime::ZERO.1,
		runtime_pad: RuntimePad::ZERO.1,
		runtime_milli: RuntimeMilli::ZERO.1,
	};

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::SECOND,                1.0);
	/// assert_eq!(RuntimeUnion::SECOND.as_str(),       "0:01");
	/// assert_eq!(RuntimeUnion::SECOND.as_str_pad(),   "00:00:01");
	/// assert_eq!(RuntimeUnion::SECOND.as_str_milli(), "00:00:01.000");
	/// assert_eq!(RuntimeUnion::SECOND, RuntimeUnion::from(1.0));
	/// ```
	pub const SECOND: Self = Self {
		float: Runtime::SECOND_F32,
		runtime: Runtime::SECOND.1,
		runtime_pad: RuntimePad::SECOND.1,
		runtime_milli: RuntimeMilli::SECOND.1,
	};

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::MINUTE,                60.0);
	/// assert_eq!(RuntimeUnion::MINUTE.as_str(),       "1:00");
	/// assert_eq!(RuntimeUnion::MINUTE.as_str_pad(),   "00:01:00");
	/// assert_eq!(RuntimeUnion::MINUTE.as_str_milli(), "00:01:00.000");
	/// assert_eq!(RuntimeUnion::MINUTE, RuntimeUnion::from(60.0));
	/// ```
	pub const MINUTE: Self = Self {
		float: Runtime::MINUTE_F32,
		runtime: Runtime::MINUTE.1,
		runtime_pad: RuntimePad::MINUTE.1,
		runtime_milli: RuntimeMilli::MINUTE.1,
	};

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::HOUR,                3600.0);
	/// assert_eq!(RuntimeUnion::HOUR.as_str(),       "1:00:00");
	/// assert_eq!(RuntimeUnion::HOUR.as_str_pad(),   "01:00:00");
	/// assert_eq!(RuntimeUnion::HOUR.as_str_milli(), "01:00:00.000");
	/// assert_eq!(RuntimeUnion::HOUR, RuntimeUnion::from(3600.0));
	/// ```
	pub const HOUR: Self = Self {
		float: Runtime::HOUR_F32,
		runtime: Runtime::HOUR.1,
		runtime_pad: RuntimePad::HOUR.1,
		runtime_milli: RuntimeMilli::HOUR.1,
	};

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::DAY,                86400.0);
	/// assert_eq!(RuntimeUnion::DAY.as_str(),       "24:00:00");
	/// assert_eq!(RuntimeUnion::DAY.as_str_pad(),   "24:00:00");
	/// assert_eq!(RuntimeUnion::DAY.as_str_milli(), "24:00:00.000");
	/// assert_eq!(RuntimeUnion::DAY, RuntimeUnion::from(86400.0));
	/// ```
	pub const DAY: Self = Self {
		float: Runtime::DAY_F32,
		runtime: Runtime::DAY.1,
		runtime_pad: RuntimePad::DAY.1,
		runtime_milli: RuntimeMilli::DAY.1,
	};

	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::MAX,                359999.0);
	/// assert_eq!(RuntimeUnion::MAX.as_str(),       "99:59:59");
	/// assert_eq!(RuntimeUnion::MAX.as_str_pad(),   "99:59:59");
	/// assert_eq!(RuntimeUnion::MAX.as_str_milli(), "99:59:59.000");
	/// assert_eq!(RuntimeUnion::MAX, RuntimeUnion::from(359999.0));
	/// ```
	pub const MAX: Self = Self {
		float: Runtime::MAX_F32,
		runtime: Runtime::MAX.1,
		runtime_pad: RuntimePad::MAX.1,
		runtime_milli: RuntimeMilli::MAX.1,
	};
}

//---------------------------------------------------------------------------------------------------- RuntimeUnion Impl
impl RuntimeUnion {
	#[inline]
	#[must_use]
	/// Returns the inner number.
	pub const fn inner(&self) -> f32 {
		self.float
	}

	#[inline]
	#[must_use]
	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::from(65.555).as_str(), "1:05");
	/// ```
	pub const fn as_str(&self) -> &str {
		self.runtime.as_str()
	}

	#[inline]
	#[must_use]
	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::from(65.555).as_str_pad(), "00:01:05");
	/// ```
	pub const fn as_str_pad(&self) -> &str {
		self.runtime_pad.as_str()
	}

	#[inline]
	#[must_use]
	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::from(65.555).as_str_milli(), "00:01:05.555");
	/// ```
	pub const fn as_str_milli(&self) -> &str {
		self.runtime_milli.as_str()
	}

	#[inline]
	#[must_use]
	/// Creates an identical [`Runtime`] without consuming [`Self`]
	///
	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::from(65.555).to_runtime(), Runtime::from(65.555));
	/// ```
	pub const fn to_runtime(&self) -> Runtime {
		Runtime(self.float, self.runtime)
	}

	#[inline]
	#[must_use]
	/// Creates an identical [`RuntimePad`] without consuming [`Self`]
	///
	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::from(65.555).to_pad(), RuntimePad::from(65.555));
	/// ```
	pub const fn to_pad(&self) -> RuntimePad {
		RuntimePad(self.float, self.runtime_pad)
	}

	#[inline]
	#[must_use]
	/// Creates an identical [`RuntimeMilli`] without consuming [`Self`]
	///
	/// ```rust
	/// # use readable::run::*;
	/// assert_eq!(RuntimeUnion::from(65.555).to_milli(), RuntimeMilli::from(65.555));
	/// ```
	pub const fn to_milli(&self) -> RuntimeMilli {
		RuntimeMilli(self.float, self.runtime_milli)
	}

	#[inline]
	#[must_use]
	/// Deconstructs [`Self`] and returns the [`Runtime`] variants
	///
	/// ```rust
	/// # use readable::run::*;
	/// let (r, p, m) = RuntimeUnion::from(65.555).into_inner();
	///
	/// assert_eq!(r, Runtime::from(65.555));
	/// assert_eq!(p, RuntimePad::from(65.555));
	/// assert_eq!(m, RuntimeMilli::from(65.555));
	/// ```
	pub const fn into_inner(self) -> (Runtime, RuntimePad, RuntimeMilli) {
		(
			Runtime(self.float, self.runtime),
			RuntimePad(self.float, self.runtime_pad),
			RuntimeMilli(self.float, self.runtime_milli),
		)
	}

	#[inline]
	#[must_use]
	/// ```rust
	/// # use readable::run::*;
	/// assert!(RuntimeUnion::UNKNOWN.is_unknown());
	/// assert!(!RuntimeUnion::ZERO.is_unknown());
	/// ```
	pub const fn is_unknown(&self) -> bool {
		let bytes = (
			self.runtime.as_bytes(),
			self.runtime_pad.as_bytes(),
			self.runtime_milli.as_bytes(),
		);
		matches!(bytes, (b"?:??", b"??:??:??", b"??:??:??.???"))
	}
}

//---------------------------------------------------------------------------------------------------- Private impl
impl RuntimeUnion {
	#[allow(unreachable_code)]
	#[inline]
	// Private function used in float `From`.
	fn priv_from(float: f32) -> Self {
		let runtime = Runtime::priv_from(float);
		if runtime == Runtime::UNKNOWN {
			return Self::UNKNOWN;
		}

		let runtime_pad = RuntimePad::priv_from(float);
		if runtime_pad == RuntimePad::UNKNOWN {
			return Self::UNKNOWN;
		}

		let runtime_milli = RuntimeMilli::priv_from(float);
		if runtime_milli == RuntimeMilli::UNKNOWN {
			return Self::UNKNOWN;
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