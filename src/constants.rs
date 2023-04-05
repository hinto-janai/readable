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
/// The locale numbers are formatted in (English). This looks like: `1,000`
const LOCALE: num_format::Locale = num_format::Locale::en;

/// Returned when using an `*::unknown()` function
pub const UNKNOWN: &str = "???";

/// Returned when using [`Float::unknown`]
pub const UNKNOWN_FLOAT: &str = "?.???";

/// Returned when using [`Percent::unknown`]
pub const UNKNOWN_PERCENT: &str = "?.??%";

/// Returned when using [`Runtime::unknown`]
pub const UNKNOWN_RUNTIME: &str = "?:??";

/// Returned when using [`Date::unknown`]
pub const UNKNOWN_DATE: &str = "????-??-??";

/// UTF-8 byte encoding of [`UNKNOWN_DATE`], aka: `????-??-??`
///
/// ```rust
///	# use readable::*;
/// assert!(UNKNOWN_DATE.as_bytes() == UNKNOWN_DATE_BUFFER);
/// ```
pub const UNKNOWN_DATE_BUFFER: [u8; 10] = [63, 63, 63, 63, 45, 63, 63, 45, 63, 63];

/// Returned when encountering a [`f32::NAN`] or [`f64::NAN`]
pub const NAN: &str = "NaN";

/// Returned when encountering an `INFINITY` variant of an `f32/f64`.
pub const INFINITY: &str = "∞";

/// Returned when using [`Int::zero`] or [`Unsigned::zero`]
pub const ZERO: &str = "0";

/// Returned when using [`Float::zero`]
pub const ZERO_FLOAT: &str = "0.000";

/// Returned when using [`Percent::zero`]
pub const ZERO_PERCENT: &str = "0.00%";

/// Returned when using [`Runtime::zero`]
pub const ZERO_RUNTIME: &str = "0:00";
