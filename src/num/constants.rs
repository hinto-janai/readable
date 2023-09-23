//---------------------------------------------------------------------------------------------------- Private
/// UTF-8 byte encoding of [`UNKNOWN`]
pub(super) const UNKNOWN_NUM_BUFFER: [u8; 26] = [63, 63, 63, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

/// UTF-8 byte encoding of [`ZERO`] for [`Unsigned`]
pub(super) const ZERO_NUM_BUFFER: [u8; 26] = [48, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

/// The max length the inner buffer within [`Unsigned`] or [`Int`] can be.
pub(super) const MAX_BUF_LEN: usize = 26;

//---------------------------------------------------------------------------------------------------- Public
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

/// The string returned when encountering a [`f32::NAN`] or [`f64::NAN`]
pub const NAN: &str = "NaN";

/// The string returned when encountering an `INFINITY` variant of an `f32/f64`.
pub const INFINITY: &str = "∞";

/// Returned when using [`Int::zero`] or [`Unsigned::zero`]
pub const ZERO: &str = "0";

/// Returned when using [`Float::zero`]
pub const ZERO_FLOAT: &str = "0.000";

/// Returned when using [`Percent::zero`]
pub const ZERO_PERCENT: &str = "0.00%";

/// Returned when using an `*::unknown()` function
pub const UNKNOWN: &str = "???";

/// Returned when using [`Float::unknown`]
pub const UNKNOWN_FLOAT: &str = "?.???";

/// Returned when using [`Percent::unknown`]
pub const UNKNOWN_PERCENT: &str = "?.??%";

//---------------------------------------------------------------------------------------------------- Tests
mod tests {
	use crate::*;
	use super::*;

	#[test]
	fn unknown_num_buffer() {
		assert_eq!(UNKNOWN.as_bytes()[..3], UNKNOWN_NUM_BUFFER[..3]);
	}

	#[test]
	fn zero_num_buffer() {
		assert_eq!(ZERO.as_bytes()[0], ZERO_NUM_BUFFER[0]);
	}


	#[test]
	fn max_buf_len() {
		let x = Unsigned::from(u64::MAX);
		assert_eq!(x.as_str().len(), MAX_BUF_LEN);
		let x = Int::from(i64::MIN);
		assert_eq!(x.as_str().len(), MAX_BUF_LEN);
	}
}
