//---------------------------------------------------------------------------------------------------- Use

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

//---------------------------------------------------------------------------------------------------- Float/Percent
/// The string returned when encountering a [`f32::NAN`] or [`f64::NAN`]
pub const NAN: &str = "NaN";

/// The string returned when encountering an `INFINITY` variant of an `f32/f64`.
pub const INFINITY: &str = "inf";