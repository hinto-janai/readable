//---------------------------------------------------------------------------------------------------- Use
use crate::num::{
	Unsigned,Int,Float,
};

//---------------------------------------------------------------------------------------------------- Other
/// The number separator character.
///
/// ```rust
/// # use readable::*;
/// let x = Unsigned::from(1000_u16);
/// assert_eq!(x, "1,000");
///
/// let x = Int::from(-1000_i16);
/// assert_eq!(x, "-1,000");
///
/// let x = Float::from(1000.0_f32);
/// assert_eq!(x, "1,000.000");
///
/// let x = Percent::from(1000_u16);
/// assert_eq!(x, "1,000.00%");
/// ```
pub const COMMA: u8 = b',';

//---------------------------------------------------------------------------------------------------- Number (Unsigned + Int)
/// The max length (byte and `str`-wise) [`Unsigned`] or [`Int`] can be.
///
/// ```rust
/// # use readable::num::*;
/// assert_eq!( 18_446_744_073_709_551_615, u64::MAX);
/// assert_eq!("18,446,744,073,709,551,615".len(), MAX_LEN_NUM);
///
/// assert_eq!( -9_223_372_036_854_775_808, i64::MIN);
/// assert_eq!("-9,223,372,036,854,775,808".len(), MAX_LEN_NUM);
/// ```
pub const MAX_LEN_NUM: usize = 26;

/// Returned when using [`Unsigned::zero()`] and [`Int::zero()`]
/// ```
pub const ZERO_NUM: &str = "0";

/// Returned when using [`Unsigned::unknown()`] and [`Int::unknown()`]
pub const UNKNOWN_NUM: &str = "???";

/// Returned when using [`Unsigned::max()`]
/// ```rust
/// # use readable::num::*;
/// assert_eq!(Unsigned::from(u64::MAX), MAX_UNSIGNED);
/// ```
pub const MAX_UNSIGNED: &str = "18,446,744,073,709,551,615";

/// Returned when using [`Int::max()`]
/// ```rust
/// # use readable::num::*;
/// assert_eq!(Int::from(i64::MAX), MAX_INT);
/// ```
pub const MAX_INT: &str = "9,223,372,036,854,775,807";

/// Returned when using [`Int::min()`]
/// ```rust
/// # use readable::num::*;
/// assert_eq!(Int::from(i64::MIN), MIN_INT);
/// ```
pub const MIN_INT: &str = "-9,223,372,036,854,775,808";

//---------------------------------------------------------------------------------------------------- Float/Percent
/// The string returned when encountering a [`f32::NAN`] or [`f64::NAN`]
pub const NAN: &str = "NaN";

/// The string returned when encountering an `INFINITY` variant of an `f32/f64`.
pub const INFINITY: &str = "inf";

/// The string returned when encountering an `INFINITY` variant of an `f32/f64`.
pub const NEG_INFINITY: &str = "-inf";

/// Returned when using [`Float::zero()`]
pub const ZERO_FLOAT: &str = "0.000";

/// Returned when using [`Percent::zero()`]
pub const ZERO_PERCENT: &str = "0.00%";

/// Returned when using [`Float::unknown()`]
pub const UNKNOWN_FLOAT: &str = "?.???";

/// Returned when using [`Percent::unknown()`]
pub const UNKNOWN_PERCENT: &str = "?.??%";