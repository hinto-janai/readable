//---------------------------------------------------------------------------------------------------- Use
use crate::macros::{
    handle_over_u32, impl_common, impl_const, impl_impl_math, impl_math, impl_traits, impl_usize,
};
#[cfg(feature = "num")]
use crate::num::Unsigned;
use crate::str::Str;
use crate::time::{Time, TimeUnit};

//---------------------------------------------------------------------------------------------------- Military
/// Military time - `23:59:59`
///
/// This formats seconds into "military"-style 24-hour based `HH:MM:SS` formatting.
///
/// An overflowing input will wrap back around (like a real clock), e.g:
/// ```rust
/// # use readable::time::*;
/// // 23 hours, 59 minutes, 59 seconds.
/// assert_eq!(Military::from(86399), "23:59:59");
///
/// // 1 day (wraps).
/// assert_eq!(Military::from(86400), "00:00:00");
///
/// // 1 day and 1 second (wraps).
/// assert_eq!(Military::from(86401), "00:00:01");
/// ```
///
/// ## Size
/// [`Str<7>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::time::*;
/// assert_eq!(std::mem::size_of::<Military>(), 16);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::time::*;
/// assert_eq!(Military::from(0),         "00:00:00");
/// assert_eq!(Military::from(1),         "00:00:01");
/// assert_eq!(Military::from(10),        "00:00:10");
/// assert_eq!(Military::from(60),        "00:01:00");
/// assert_eq!(Military::from(3599),      "00:59:59");
/// assert_eq!(Military::from(3600),      "01:00:00");
/// assert_eq!(Military::from(3600 * 2),  "02:00:00");
/// assert_eq!(Military::from(3600 * 3),  "03:00:00");
/// assert_eq!(Military::from(3600 * 4),  "04:00:00");
/// assert_eq!(Military::from(3600 * 5),  "05:00:00");
/// assert_eq!(Military::from(3600 * 6),  "06:00:00");
/// assert_eq!(Military::from(3600 * 7),  "07:00:00");
/// assert_eq!(Military::from(3600 * 8),  "08:00:00");
/// assert_eq!(Military::from(3600 * 9),  "09:00:00");
/// assert_eq!(Military::from(3600 * 10), "10:00:00");
/// assert_eq!(Military::from(3600 * 11), "11:00:00");
/// assert_eq!(Military::from(3600 * 12), "12:00:00");
/// assert_eq!(Military::from(3600 * 13), "13:00:00");
/// assert_eq!(Military::from(3600 * 14), "14:00:00");
/// assert_eq!(Military::from(3600 * 15), "15:00:00");
/// assert_eq!(Military::from(3600 * 16), "16:00:00");
/// assert_eq!(Military::from(3600 * 17), "17:00:00");
/// assert_eq!(Military::from(3600 * 18), "18:00:00");
/// assert_eq!(Military::from(3600 * 19), "19:00:00");
/// assert_eq!(Military::from(3600 * 20), "20:00:00");
/// assert_eq!(Military::from(3600 * 21), "21:00:00");
/// assert_eq!(Military::from(3600 * 22), "22:00:00");
/// assert_eq!(Military::from(3600 * 23), "23:00:00");
/// assert_eq!(Military::from(3600 * 24), "00:00:00");
/// assert_eq!(Military::from((3600 * 24) + 1),    "00:00:01");
/// assert_eq!(Military::from((3600 * 24) + 60),   "00:01:00");
/// assert_eq!(Military::from((3600 * 24) + 3599), "00:59:59");
/// assert_eq!(Military::from((3600 * 24) + 1830), "00:30:30");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Military(pub(super) u32, pub(super) Str<{ Military::MAX_LEN }>);

impl_traits!(Military, u32);
impl_math!(Military, u32);

//---------------------------------------------------------------------------------------------------- Military Constants
impl Military {
    /// The max length of [`Military`]'s string.
    /// ```rust
    /// # use readable::time::*;
    /// assert_eq!("10:10:10".len(), Military::MAX_LEN);
    /// ```
    pub const MAX_LEN: usize = 8;

    /// ```rust
    /// # use readable::time::*;
    /// assert_eq!(Military::UNKNOWN, 0);
    /// assert_eq!(Military::UNKNOWN, "??:??:??");
    /// ```
    pub const UNKNOWN: Self = Self(0, Str::from_static_str("??:??:??"));

    /// ```rust
    /// # use readable::time::*;
    /// assert_eq!(Military::ZERO, 0);
    /// assert_eq!(Military::ZERO, "00:00:00");
    /// ```
    pub const ZERO: Self = Self(0, Str::from_static_str("00:00:00"));

    /// ```rust
    /// # use readable::time::*;
    /// assert_eq!(Military::MAX, 86399);
    /// assert_eq!(Military::MAX, "23:59:59");
    /// ```
    pub const MAX: Self = Self(86399, Str::from_static_str("23:59:59"));
}

//---------------------------------------------------------------------------------------------------- Impl
impl Military {
    impl_common!(u32);
    impl_const!();
    impl_usize!();

    #[inline]
    #[must_use]
    /// Create a [`Self`] from seconds
    ///
    /// This behaves the exact same way as the [`From`]
    /// implementation, although this function is `const`.
    ///
    /// ```rust
    /// # use readable::time::*;
    /// let from:    Military = Military::from(86399);
    /// const CONST: Military = Military::new(86399);
    ///
    /// assert_eq!(from,  "23:59:59");
    /// assert_eq!(CONST, "23:59:59");
    /// assert_eq!(from, CONST);
    /// ```
    pub const fn new(total_seconds: u32) -> Self {
        Self::priv_from(total_seconds)
    }

    #[inline]
    #[must_use]
    /// Create a [`Self`] with specified `hours`, `minutes`, and `seconds`
    ///
    /// This takes hours, minutes, and seconds and will convert the
    /// total military into a [`Military`] (maintaing the normal wrapping behavior).
    ///
    /// A value being left as `None` is equal to `0`.
    ///
    /// ```rust
    /// # use readable::time::*;
    /// let military = Military::new_specified(
    ///     3,  // hours
    ///     21, // minutes
    ///     55, // seconds
    /// );
    /// assert_eq!(military, "03:21:55");
    ///
    /// // Overflowing to PM.
    /// let military = Military::new_specified(13, 21, 0);
    /// assert_eq!(military, "13:21:00");
    ///
    /// // Wrapping back around.
    /// let military = Military::new_specified(25, 1, 1);
    /// assert_eq!(military, "01:01:01");
    /// ```
    pub const fn new_specified(hours: u8, minutes: u8, seconds: u8) -> Self {
        Self::priv_from((seconds as u32) + (minutes as u32 * 60) + (hours as u32 * 3600))
    }

    #[inline]
    #[must_use]
    /// ```rust
    /// # use readable::time::*;
    /// assert!(Military::UNKNOWN.is_unknown());
    /// assert!(!Military::ZERO.is_unknown());
    /// ```
    pub const fn is_unknown(&self) -> bool {
        matches!(self.1.as_bytes(), b"??:??:??")
    }
}

//---------------------------------------------------------------------------------------------------- Private impl
impl Military {
    pub(super) const fn priv_from(total_seconds: u32) -> Self {
        const C: u8 = b':';

        let total_seconds = total_seconds % 86400;

        if total_seconds == 0 {
            return Self::ZERO;
        }

        let (hours, minutes, seconds) = crate::time::secs_to_clock(total_seconds);

        // Format.
        let h = Self::str_hour(hours);
        let m = Time::str_0_59(minutes);
        let s = Time::str_0_59(seconds);

        let buf: [u8; Self::MAX_LEN] = [h[0], h[1], C, m[0], m[1], C, s[0], s[1]];

        // SAFETY: we know the str len
        Self(total_seconds, unsafe {
            Str::from_raw(buf, Self::MAX_LEN as u8)
        })
    }

    #[inline]
    // INVARIANT: input must be 0..=23
    const fn str_hour(u: u8) -> &'static [u8] {
        match u {
            0 => b"00",
            1 => b"01",
            2 => b"02",
            3 => b"03",
            4 => b"04",
            5 => b"05",
            6 => b"06",
            7 => b"07",
            8 => b"08",
            9 => b"09",
            10 => b"10",
            11 => b"11",
            12 => b"12",
            13 => b"13",
            14 => b"14",
            15 => b"15",
            16 => b"16",
            17 => b"17",
            18 => b"18",
            19 => b"19",
            20 => b"20",
            21 => b"21",
            22 => b"22",
            23 => b"23",
            _ => unreachable!(),
        }
    }
}

//---------------------------------------------------------------------------------------------------- Floats
macro_rules! impl_f {
    ($from:ty) => {
        impl From<$from> for Military {
            #[inline]
            fn from(f: $from) -> Self {
                $crate::macros::return_bad_float!(f, Self::UNKNOWN, Self::UNKNOWN);

                Self::priv_from(f as u32)
            }
        }
        impl From<&$from> for Military {
            #[inline]
            fn from(f: &$from) -> Self {
                $crate::macros::return_bad_float!(f, Self::UNKNOWN, Self::UNKNOWN);

                Self::priv_from(*f as u32)
            }
        }
    };
}
impl_f!(f32);
impl_f!(f64);

//---------------------------------------------------------------------------------------------------- uint
macro_rules! impl_u {
    ($from:ty) => {
        impl From<$from> for Military {
            #[inline]
            fn from(seconds: $from) -> Self {
                Self::priv_from(seconds as u32)
            }
        }
        impl From<&$from> for Military {
            #[inline]
            fn from(seconds: &$from) -> Self {
                Self::from(*seconds)
            }
        }
    };
}
impl_u!(u8);
impl_u!(u16);
impl_u!(u32);
impl_u!(u64);
impl_u!(u128);
impl_u!(usize);

//---------------------------------------------------------------------------------------------------- Int
macro_rules! impl_i {
    ($from:ty) => {
        impl From<$from> for Military {
            #[inline]
            fn from(seconds: $from) -> Self {
                if seconds.is_negative() {
                    return Self::UNKNOWN;
                }
                Self::priv_from(seconds as u32)
            }
        }
        impl From<&$from> for Military {
            #[inline]
            fn from(seconds: &$from) -> Self {
                if seconds.is_negative() {
                    return Self::UNKNOWN;
                }
                Self::priv_from(*seconds as u32)
            }
        }
    };
}
impl_i!(i8);
impl_i!(i16);
impl_i!(i32);
impl_i!(i64);
impl_i!(i128);
impl_i!(isize);

//---------------------------------------------------------------------------------------------------- Other
macro_rules! impl_other {
	($($from:ty),* $(,)?) => {
		$(
			impl From<$from> for Military {
				#[inline]
				fn from(other: $from) -> Self {
					if other.is_unknown() {
						return Self::UNKNOWN;
					}
					Self::priv_from(other.inner() as u32)
				}
			}
			impl From<&$from> for Military {
				#[inline]
				fn from(other: &$from) -> Self {
					if other.is_unknown() {
						return Self::UNKNOWN;
					}
					Self::priv_from(other.inner() as u32)
				}
			}
		)*
	}
}
impl_other!(Time, TimeUnit);
#[cfg(feature = "num")]
impl_other!(Unsigned);

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for Military {
    #[inline]
    fn from(duration: std::time::Duration) -> Self {
        let u = duration.as_secs();
        handle_over_u32!(u, u64);
        Self::new(u as u32)
    }
}

impl From<&std::time::Duration> for Military {
    #[inline]
    fn from(duration: &std::time::Duration) -> Self {
        let u = duration.as_secs();
        handle_over_u32!(u, u64);
        Self::new(u as u32)
    }
}

impl From<Military> for std::time::Duration {
    #[inline]
    fn from(value: Military) -> Self {
        Self::from_secs(value.inner().into())
    }
}

impl From<&Military> for std::time::Duration {
    #[inline]
    fn from(value: &Military) -> Self {
        Self::from_secs(value.inner().into())
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        let this: Military = Military::from(3599);
        let json = serde_json::to_string(&this).unwrap();
        assert_eq!(json, r#"[3599,"00:59:59"]"#);

        let this: Military = serde_json::from_str(&json).unwrap();
        assert_eq!(this, 3599);
        assert_eq!(this, "00:59:59");

        // Bad bytes.
        assert!(serde_json::from_str::<Military>(&"---").is_err());

        let json = serde_json::to_string(&Military::UNKNOWN).unwrap();
        assert_eq!(json, r#"[0,"??:??:??"]"#);
        assert!(serde_json::from_str::<Military>(&json)
            .unwrap()
            .is_unknown());
    }

    #[test]
    #[cfg(feature = "bincode")]
    fn bincode() {
        let this: Military = Military::from(3599);
        let config = bincode::config::standard();
        let bytes = bincode::encode_to_vec(&this, config).unwrap();

        let this: Military = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert_eq!(this, 3599);
        assert_eq!(this, "00:59:59");

        // Unknown.
        let bytes = bincode::encode_to_vec(&Military::UNKNOWN, config).unwrap();
        let this: Military = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert!(this.is_unknown());
    }

    #[test]
    #[cfg(feature = "borsh")]
    fn borsh() {
        let this: Military = Military::from(3599);
        let bytes = borsh::to_vec(&this).unwrap();

        let this: Military = borsh::from_slice(&bytes).unwrap();
        assert_eq!(this, 3599);
        assert_eq!(this, "00:59:59");

        // Bad bytes.
        assert!(borsh::from_slice::<Military>(b"bad .-;[]124/ bytes").is_err());

        // Unknown.
        let bytes = borsh::to_vec(&Military::UNKNOWN).unwrap();
        let this: Military = borsh::from_slice(&bytes).unwrap();
        assert!(this.is_unknown());
    }
}
