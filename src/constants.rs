//---------------------------------------------------------------------------------------------------- Use
use crate::{
	int::Int,
	unsigned::Unsigned,
	float::Float,
	percent::Percent,
	runtime::Runtime,
	date::Date,
};

//---------------------------------------------------------------------------------------------------- Constants
/// The separator character. This looks like: `1,000`
pub const COMMA: u8 = b',';

/// The max length the inner buffer within [`Unsigned`] or [`Int`] can be.
///
/// [`u64::MAX`] == `"18_446_744_073_709_551_615".len()` == `26`
///
/// [`i64::MIN`] == `"-9,223,372,036,854,775,808".len()` == `26`
pub const MAX_BUF_LEN: usize = 26;

/// Returned when encountering a [`f32::NAN`] or [`f64::NAN`]
pub const NAN: &str = "NaN";

/// Returned when encountering an `INFINITY` variant of an `f32/f64`.
pub const INFINITY: &str = "∞";

/// Returned when using [`Int::zero`] or [`Unsigned::zero`]
pub const ZERO: &str = "0";

/// Returned when using an `*::unknown()` function
pub const UNKNOWN: &str = "???";

/// Returned when using [`Float::unknown`]
pub const UNKNOWN_FLOAT: &str = "?.???";

/// Returned when using [`Percent::unknown`]
pub const UNKNOWN_PERCENT: &str = "?.??%";

/// Returned when using [`Runtime::unknown`]
pub const UNKNOWN_RUNTIME: &str = "?:??";

/// UTF-8 byte encoding of [`UNKNOWN`]
///
/// ```rust
/// # use readable::*;
/// assert!(UNKNOWN.as_bytes()[..3] == UNKNOWN_NUM_BUFFER[..3]);
/// ```
pub const UNKNOWN_NUM_BUFFER: [u8; 26] = [63, 63, 63, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

/// UTF-8 byte encoding of [`UNKNOWN_DATE`], aka: `?:??`
///
/// ```rust
/// # use readable::*;
/// assert!(UNKNOWN_RUNTIME.as_bytes()[..4] == UNKNOWN_RUNTIME_BUFFER[..4]);
/// ```
pub const UNKNOWN_RUNTIME_BUFFER: [u8; 8] = [63, 58, 63, 63, 0, 0, 0, 0];

/// Returned when using [`Date::unknown`]
pub const UNKNOWN_DATE: &str = "????-??-??";

/// UTF-8 byte encoding of [`UNKNOWN_DATE`], aka: `????-??-??`
///
/// ```rust
/// # use readable::*;
/// assert!(UNKNOWN_DATE.as_bytes() == UNKNOWN_DATE_BUFFER);
/// ```
pub const UNKNOWN_DATE_BUFFER: [u8; 10] = [63, 63, 63, 63, 45, 63, 63, 45, 63, 63];

/// Returned when using [`Unsigned::zero`] or [`Int::zero`]
pub const ZERO_NUM: &str = "0";
/// UTF-8 byte encoding of [`ZERO_NUM`] for [`Unsigned`]
///
/// ```rust
/// # use readable::*;
/// assert!(ZERO_NUM.as_bytes()[0] == ZERO_BUFFER[0]);
/// ```
pub const ZERO_NUM_BUFFER: [u8; 26] = [48, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

/// Returned when using [`Float::zero`]
pub const ZERO_FLOAT: &str = "0.000";

/// Returned when using [`Percent::zero`]
pub const ZERO_PERCENT: &str = "0.00%";

/// Returned when using [`Runtime::zero`]
pub const ZERO_RUNTIME: &str = "0:00";
/// UTF-8 byte encoding of [`ZERO_RUNTIME`]
///
/// ```rust
/// # use readable::*;
/// assert!(ZERO_RUNTIME.as_bytes()[..3] == ZERO_RUNTIME_BUFFER[..3]);
/// ```
pub const ZERO_RUNTIME_BUFFER: [u8; 8] = [48, 58, 48, 48, 0, 0, 0, 0];

/// Returned when using [`Runtime::second`]
pub const SECOND_RUNTIME: &str = "0:01";
/// UTF-8 byte encoding of [`SECOND_RUNTIME`]
///
/// ```rust
/// # use readable::*;
/// assert!(SECOND_RUNTIME.as_bytes()[..4] == SECOND_RUNTIME_BUFFER[..4]);
/// ```
pub const SECOND_RUNTIME_BUFFER: [u8; 8] = [48, 58, 48, 49, 0, 0, 0, 0];

/// Returned when using [`Runtime::minute`]
pub const MINUTE_RUNTIME: &str = "1:00";
/// UTF-8 byte encoding of [`MINUTE_RUNTIME`]
///
/// ```rust
/// # use readable::*;
/// assert!(MINUTE_RUNTIME.as_bytes()[..4] == MINUTE_RUNTIME_BUFFER[..4]);
/// ```
pub const MINUTE_RUNTIME_BUFFER: [u8; 8] = [49, 58, 48, 48, 0, 0, 0, 0];

/// Returned when using [`Runtime::hour`]
pub const HOUR_RUNTIME: &str = "1:00:00";
/// UTF-8 byte encoding of [`HOUR_RUNTIME`]
///
/// ```rust
/// # use readable::*;
/// assert!(HOUR_RUNTIME.as_bytes()[..7] == HOUR_RUNTIME_BUFFER[..7]);
/// ```
pub const HOUR_RUNTIME_BUFFER: [u8; 8] = [49, 58, 48, 48, 58, 48, 48, 0];

/// Returned when calling [`Runtime::zero`]
pub const ZERO_RUNTIME_U32: u32 = 0;
/// Returned when calling [`Runtime::second`]
pub const SECOND_RUNTIME_U32: u32 = 1;
/// Returned when calling [`Runtime::minute`]
pub const MINUTE_RUNTIME_U32: u32 = 60;
/// Returned when calling [`Runtime::hour`]
pub const HOUR_RUNTIME_U32: u32 = 3600;

/// The max input to [`Runtime`] before it overflows and returns [`UNKNOWN_RUNTIME`]
pub const MAX_RUNTIME_U32: u32 = 359999;

/// The text [`Runtime`] will return [`UNKNOWN_RUNTIME`]
pub const MAX_RUNTIME: &str = "99:59:59";
/// UTF-8 byte encoding of [`MAX_RUNTIME`]
///
/// ```rust
/// # use readable::*;
/// assert!(MAX_RUNTIME.as_bytes() == MAX_RUNTIME_BUFFER);
/// ```
pub const MAX_RUNTIME_BUFFER: [u8; 8] = [57, 57, 58, 53, 57, 58, 53, 57];
