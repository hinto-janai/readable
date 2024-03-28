//---------------------------------------------------------------------------------------------------- Use
use crate::itoa;
use crate::macros::{
    handle_over_u32, impl_common, impl_const, impl_impl_math, impl_math, impl_traits, impl_usize,
    return_bad_float,
};
use crate::str::Str;
#[cfg(feature = "time")]
use crate::time::TimeUnit;
use crate::up::{Htop, UptimeFull};

//---------------------------------------------------------------------------------------------------- Uptime
/// Human-readable uptime
///
/// This formats numbers into an "uptime"-style time format,
/// suffixed with a single letter indicated the unit.
///
/// ## Size
/// [`Str<29>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::up::*;
/// assert_eq!(std::mem::size_of::<Uptime>(), 36);
/// ```
///
/// ## Warning
/// This stylizes both `minute` and `month` as `m`, thus:
/// ```rust
/// # use readable::up::*;
/// assert_eq!(Uptime::MINUTE, "1m");
/// assert_eq!(Uptime::MONTH,  "1m");
/// ```
///
/// Although, their inner number will be different and context may make it more clear:
/// ```
/// # use readable::up::*;
/// assert_eq!(Uptime::MINUTE.inner(), 60);
/// assert_eq!(Uptime::MONTH.inner(),  2678400);
///
/// assert_eq!(Uptime::MINUTE + 3601, "1h, 1m, 1s");
/// assert_eq!(Uptime::MONTH + 3661,  "1m, 1h, 1m, 1s");
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::up::Uptime;
/// assert_eq!(Uptime::from(0_u32),        "0s");
/// assert_eq!(Uptime::from(1_u32),        "1s");
/// assert_eq!(Uptime::from(2_u32),        "2s");
/// assert_eq!(Uptime::from(59_u32),       "59s");
/// assert_eq!(Uptime::from(60_u32),       "1m");
/// assert_eq!(Uptime::from(61_u32),       "1m, 1s");
/// assert_eq!(Uptime::from(62_u32),       "1m, 2s");
/// assert_eq!(Uptime::from(120_u32),      "2m");
/// assert_eq!(Uptime::from(121_u32),      "2m, 1s");
/// assert_eq!(Uptime::from(122_u32),      "2m, 2s");
/// assert_eq!(Uptime::from(179_u32),      "2m, 59s");
/// assert_eq!(Uptime::from(3599_u32),     "59m, 59s");
/// assert_eq!(Uptime::from(3600_u32),     "1h");
/// assert_eq!(Uptime::from(3601_u32),     "1h, 1s");
/// assert_eq!(Uptime::from(3602_u32),     "1h, 2s");
/// assert_eq!(Uptime::from(3660_u32),     "1h, 1m");
/// assert_eq!(Uptime::from(3720_u32),     "1h, 2m");
/// assert_eq!(Uptime::from(86399_u32),    "23h, 59m, 59s");
/// assert_eq!(Uptime::from(86400_u32),    "1d");
/// assert_eq!(Uptime::from(86401_u32),    "1d, 1s");
/// assert_eq!(Uptime::from(86402_u32),    "1d, 2s");
/// assert_eq!(Uptime::from(86460_u32),    "1d, 1m");
/// assert_eq!(Uptime::from(86520_u32),    "1d, 2m");
/// assert_eq!(Uptime::from(90000_u32),    "1d, 1h");
/// assert_eq!(Uptime::from(93600_u32),    "1d, 2h");
/// assert_eq!(Uptime::from(604799_u32),   "6d, 23h, 59m, 59s");
/// assert_eq!(Uptime::from(604800_u32),   "7d");
/// assert_eq!(Uptime::from(2678400_u32),  "1m");
/// assert_eq!(Uptime::from(3283199_u32),  "1m, 6d, 23h, 59m, 59s");
/// assert_eq!(Uptime::from(5356800_u32),  "2m");
/// assert_eq!(Uptime::from(31536000_u32), "1y");
/// assert_eq!(Uptime::from(63072000_u32), "2y");
/// println!("{}", Uptime::from(u32::MAX));
/// assert_eq!(
///     Uptime::from(u32::MAX),
///     "136y, 2m, 8d, 6h, 28m, 15s",
/// );
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Uptime(pub(super) u32, pub(super) Str<{ Uptime::MAX_LEN }>);

impl_math!(Uptime, u32);
impl_traits!(Uptime, u32);

//---------------------------------------------------------------------------------------------------- Constants
impl Uptime {
    /// ```rust
    /// # use readable::up::*;
    /// let time = "---y, --m, --d, --h, --m, --s";
    /// assert_eq!(time.len(), Uptime::MAX_LEN);
    /// ```
    pub const MAX_LEN: usize = 29;

    /// ```rust
    /// # use readable::up::*;
    /// assert_eq!(Uptime::UNKNOWN, 0);
    /// assert_eq!(Uptime::UNKNOWN, "(unknown)");
    /// ```
    pub const UNKNOWN: Self = Self(0, Str::from_static_str("(unknown)"));

    /// ```rust
    /// # use readable::up::*;
    /// assert_eq!(Uptime::ZERO, 0);
    /// assert_eq!(Uptime::ZERO, "0s");
    /// ```
    pub const ZERO: Self = Self(0, Str::from_static_str("0s"));

    /// ```rust
    /// # use readable::up::*;
    /// assert_eq!(Uptime::SECOND, 1);
    /// assert_eq!(Uptime::SECOND, "1s");
    /// ```
    pub const SECOND: Self = Self(1, Str::from_static_str("1s"));

    /// ```rust
    /// # use readable::up::*;
    /// assert_eq!(Uptime::MINUTE, 60);
    /// assert_eq!(Uptime::MINUTE, "1m");
    /// ```
    pub const MINUTE: Self = Self(60, Str::from_static_str("1m"));

    /// ```rust
    /// # use readable::up::*;
    /// assert_eq!(Uptime::HOUR, 3600);
    /// assert_eq!(Uptime::HOUR, "1h");
    /// ```
    pub const HOUR: Self = Self(3600, Str::from_static_str("1h"));

    /// ```rust
    /// # use readable::up::*;
    /// assert_eq!(Uptime::DAY, 86400);
    /// assert_eq!(Uptime::DAY, "1d");
    /// ```
    pub const DAY: Self = Self(86400, Str::from_static_str("1d"));

    /// ```rust
    /// # use readable::up::*;
    /// assert_eq!(Uptime::MONTH, 2678400);
    /// assert_eq!(Uptime::MONTH, "1m");
    /// ```
    pub const MONTH: Self = Self(2678400, Str::from_static_str("1m"));

    /// ```rust
    /// # use readable::up::*;
    /// assert_eq!(Uptime::YEAR, 31536000);
    /// assert_eq!(Uptime::YEAR, "1y");
    /// ```
    pub const YEAR: Self = Self(31536000, Str::from_static_str("1y"));

    /// ```rust
    /// # use readable::up::*;
    /// assert_eq!(Uptime::MAX, u32::MAX);
    /// assert_eq!(Uptime::MAX, "136y, 2m, 8d, 6h, 28m, 15s");
    /// ```
    pub const MAX: Self = Self(u32::MAX, Str::from_static_str("136y, 2m, 8d, 6h, 28m, 15s"));
}

//---------------------------------------------------------------------------------------------------- Pub Impl
impl Uptime {
    impl_common!(u32);
    impl_const!();
    impl_usize!();

    #[inline]
    #[must_use]
    /// ```rust
    /// # use readable::up::*;
    /// assert!(Uptime::UNKNOWN.is_unknown());
    /// assert!(!Uptime::ZERO.is_unknown());
    /// ```
    pub const fn is_unknown(&self) -> bool {
        matches!(*self, Self::UNKNOWN)
    }
}

//---------------------------------------------------------------------------------------------------- Private impl
impl Uptime {
    #[inline]
    fn plural(s: &mut Str<{ Self::MAX_LEN }>, name: &'static str, value: u32, started: &mut bool) {
        if value > 0 {
            if *started {
                s.push_str_panic(", ");
            }
            s.push_str_panic(itoa!(value));
            s.push_str_panic(name);
            *started = true;
        }
    }

    fn from_priv(secs: u32) -> Self {
        // #[cfg(feature = "inline_time")]
        // if secs <= 3660 {
        // 	// SAFETEE:
        // 	// Cast `u64` to `u16` is safe because it's under 65_535.
        // 	return Self(secs, CompactString::new_inline(readable_inlined_time::inlined(secs as u16)))
        // }

        if secs == 0 {
            return Self::ZERO;
        }

        let years = secs / 31_536_000; // 365 days
        let ydays = secs % 31_536_000;
        let months = ydays / 2_678_400; // 31 days
        let mdays = ydays % 2_678_400;
        let days = mdays / 86400;
        let day_secs = mdays % 86400;
        let hours = day_secs / 3600;
        let minutes = day_secs % 3600 / 60;
        let seconds = day_secs % 60;

        let started = &mut false;
        let mut string = Str::new();
        let s = &mut string;
        Self::plural(s, "y", years, started);
        Self::plural(s, "m", months, started);
        Self::plural(s, "d", days, started);
        Self::plural(s, "h", hours, started);
        Self::plural(s, "m", minutes, started);
        Self::plural(s, "s", seconds, started);

        Self(secs, string)
    }
}

//---------------------------------------------------------------------------------------------------- "u*" impl
// Implementation Macro.
macro_rules! impl_u {
	($($u:ty),* $(,)?) => { $(
		impl From<$u> for Uptime {
			#[inline]
			fn from(u: $u) -> Self {
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for Uptime {
			#[inline]
			fn from(u: &$u) -> Self {
				Self::from_priv(*u as u32)
			}
		}
	)*}
}
impl_u!(u8, u16, u32);
#[cfg(not(target_pointer_width = "64"))]
impl_u!(usize);

macro_rules! impl_u_over {
	($($u:ty),* $(,)?) => { $(
		impl From<$u> for Uptime {
			#[inline]
			fn from(u: $u) -> Self {
				handle_over_u32!(u, $u);
				Self::from_priv(u as u32)
			}
		}
		impl From<&$u> for Uptime {
			#[inline]
			fn from(u: &$u) -> Self {
				handle_over_u32!(*u, $u);
				Self::from_priv(*u as u32)
			}
		}
	)*}
}

impl_u_over!(u64, u128);
#[cfg(target_pointer_width = "64")]
impl_u_over!(usize);

//---------------------------------------------------------------------------------------------------- i* impl
macro_rules! impl_int {
	($($int:ty),* $(,)?) => { $(
		impl From<$int> for Uptime {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for Uptime {
			#[inline]
			fn from(int: &$int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				Self::from_priv(*int as u32)
			}
		}
	)*}
}
impl_int!(i8, i16, i32);
#[cfg(not(target_pointer_width = "64"))]
impl_u!(isize);

macro_rules! impl_int_over {
	($($int:ty),* $(,)?) => { $(
		impl From<$int> for Uptime {
			#[inline]
			fn from(int: $int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				handle_over_u32!(int, $int);
				Self::from_priv(int as u32)
			}
		}
		impl From<&$int> for Uptime {
			#[inline]
			fn from(int: &$int) -> Self {
				if int.is_negative() {
					return Self::UNKNOWN;
				}
				handle_over_u32!(*int, $int);
				Self::from_priv(*int as u32)
			}
		}
	)*}
}
impl_int_over!(i64, i128);
#[cfg(target_pointer_width = "64")]
impl_u_over!(isize);

//---------------------------------------------------------------------------------------------------- "f" impl
macro_rules! impl_f {
    ($float:ty) => {
        impl From<$float> for Uptime {
            #[inline]
            fn from(float: $float) -> Self {
                return_bad_float!(float, Self::UNKNOWN, Self::UNKNOWN);
                if float.is_sign_negative() {
                    return Self::UNKNOWN;
                }
                handle_over_u32!(float, $float);
                Self::from_priv(float as u32)
            }
        }
        impl From<&$float> for Uptime {
            #[inline]
            fn from(float: &$float) -> Self {
                return_bad_float!(float, Self::UNKNOWN, Self::UNKNOWN);
                if float.is_sign_negative() {
                    return Self::UNKNOWN;
                }
                handle_over_u32!(*float, $float);
                Self::from_priv(*float as u32)
            }
        }
    };
}
impl_f!(f32);
impl_f!(f64);

//---------------------------------------------------------------------------------------------------- Other Uptime Impl.
macro_rules! impl_from_time {
	($this:ty => $($other:ty),* $(,)?) => { $(
		impl From<$other> for $this {
			#[inline]
			fn from(from: $other) -> Self {
				if from.is_unknown() {
					Self::UNKNOWN
				} else {
					Self::from_priv(from.inner())
				}
			}
		}
		impl From<&$other> for $this {
			#[inline]
			fn from(from: &$other) -> Self {
				if from.is_unknown() {
					Self::UNKNOWN
				} else {
					Self::from_priv(from.inner())
				}
			}
		}
	)*}
}
impl_from_time!(Uptime => UptimeFull, Htop);
#[cfg(feature = "time")]
impl_from_time!(Uptime => TimeUnit);

//---------------------------------------------------------------------------------------------------- Trait Impl
impl From<std::time::Duration> for Uptime {
    #[inline]
    fn from(duration: std::time::Duration) -> Self {
        let u = duration.as_secs();
        handle_over_u32!(u, u64);
        Self::from_priv(u as u32)
    }
}

impl From<&std::time::Duration> for Uptime {
    #[inline]
    fn from(duration: &std::time::Duration) -> Self {
        let u = duration.as_secs();
        handle_over_u32!(u, u64);
        Self::from_priv(u as u32)
    }
}

impl From<std::time::Instant> for Uptime {
    #[inline]
    fn from(instant: std::time::Instant) -> Self {
        let u = instant.elapsed().as_secs();
        handle_over_u32!(u, u64);
        Self::from_priv(u as u32)
    }
}

impl From<&std::time::Instant> for Uptime {
    #[inline]
    fn from(instant: &std::time::Instant) -> Self {
        let u = instant.elapsed().as_secs();
        handle_over_u32!(u, u64);
        Self::from_priv(u as u32)
    }
}

impl From<Uptime> for std::time::Duration {
    #[inline]
    fn from(value: Uptime) -> Self {
        Self::from_secs(value.inner().into())
    }
}

impl From<&Uptime> for std::time::Duration {
    #[inline]
    fn from(value: &Uptime) -> Self {
        Self::from_secs(value.inner().into())
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_ints() {
        let mut f = 1_u64;
        while f < (u64::from(Uptime::MAX.0)) {
            let t = Uptime::from(f);
            println!("t: {t}, f: {f}");
            assert_eq!(t, f as u32);
            f *= 10;
        }
    }

    #[test]
    fn over() {
        assert_ne!(Uptime::from(u32::MAX), Uptime::UNKNOWN);
        assert_eq!(Uptime::from(u64::from(u32::MAX) + 1), Uptime::UNKNOWN);
        assert_eq!(Uptime::from(u64::MAX), Uptime::UNKNOWN);
        assert_eq!(Uptime::from(f64::MAX), Uptime::UNKNOWN);
        assert_eq!(Uptime::from(f32::MAX), Uptime::UNKNOWN);
    }

    #[test]
    fn special() {
        assert_eq!(Uptime::from(f32::NAN), Uptime::UNKNOWN);
        assert_eq!(Uptime::from(f32::INFINITY), Uptime::UNKNOWN);
        assert_eq!(Uptime::from(f32::NEG_INFINITY), Uptime::UNKNOWN);
        assert_eq!(Uptime::from(f64::NAN), Uptime::UNKNOWN);
        assert_eq!(Uptime::from(f64::INFINITY), Uptime::UNKNOWN);
        assert_eq!(Uptime::from(f64::NEG_INFINITY), Uptime::UNKNOWN);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        let this: Uptime = Uptime::from(3283199_u32);
        let json = serde_json::to_string(&this).unwrap();
        assert_eq!(json, r#"[3283199,"1m, 6d, 23h, 59m, 59s"]"#);

        let this: Uptime = serde_json::from_str(&json).unwrap();
        assert_eq!(this, 3283199_u32);
        assert_eq!(this, "1m, 6d, 23h, 59m, 59s");

        // Bad bytes.
        assert!(serde_json::from_str::<Uptime>(&"---").is_err());

        // Unknown.
        let json = serde_json::to_string(&Uptime::UNKNOWN).unwrap();
        assert_eq!(json, r#"[0,"(unknown)"]"#);
        assert!(serde_json::from_str::<Uptime>(&json).unwrap().is_unknown());
    }

    #[test]
    #[cfg(feature = "bincode")]
    fn bincode() {
        let this: Uptime = Uptime::from(3283199_u32);
        let config = bincode::config::standard();
        let bytes = bincode::encode_to_vec(&this, config).unwrap();

        let this: Uptime = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert_eq!(this, 3283199_u32);
        assert_eq!(this, "1m, 6d, 23h, 59m, 59s");

        // Unknown.
        let bytes = bincode::encode_to_vec(&Uptime::UNKNOWN, config).unwrap();
        let this: Uptime = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert!(this.is_unknown());
    }

    #[test]
    #[cfg(feature = "borsh")]
    fn borsh() {
        let this: Uptime = Uptime::from(3283199_u32);
        let bytes = borsh::to_vec(&this).unwrap();

        let this: Uptime = borsh::from_slice(&bytes).unwrap();
        assert_eq!(this, 3283199_u32);
        assert_eq!(this, "1m, 6d, 23h, 59m, 59s");

        // Bad bytes.
        assert!(borsh::from_slice::<Uptime>(b"bad .-;[]124/ bytes").is_err());

        // Unknown.
        let bytes = borsh::to_vec(&Uptime::UNKNOWN).unwrap();
        let this: Uptime = borsh::from_slice(&bytes).unwrap();
        assert!(this.is_unknown());
    }
}
