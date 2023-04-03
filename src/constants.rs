//---------------------------------------------------------------------------------------------------- Constants
// The locale numbers are formatting in is English, which looks like: [1,000]
pub(crate) const LOCALE: num_format::Locale = num_format::Locale::en;

/// Returned when using an `*_unknown()` function.
pub const UNKNOWN:      &str = "???";

/// Returned when encountering a [`f64::NAN`].
pub const NAN:          &str = "NaN";

/// Returned when encountering a [`f64::INFINITY`] or [`f64::NEG_INFINITY`].
pub const INFINITY:     &str = "∞";
