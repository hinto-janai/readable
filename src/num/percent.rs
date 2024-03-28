//---------------------------------------------------------------------------------------------------- Use
use crate::macros::{
    impl_common, impl_const, impl_impl_math, impl_isize, impl_math, impl_traits, impl_usize,
    return_bad_float, str_i64, str_u64,
};
use crate::num::constants::{INFINITY, NAN};
use crate::str::Str;
use compact_str::format_compact;

//---------------------------------------------------------------------------------------------------- Percent
/// Human readable percentage.
///
/// [`Percent::from`] input can be:
/// - [`u8`], [`u16`], [`u32`]
/// - [`i8`], [`i16`], [`i32`]
/// - [`f32`], [`f64`]
///
/// The default [`Percent::from`] implementation will print `2` decimal numbers.
///
/// Anything lower than `0.01` is rounded down to `0.00`.
///
/// This can be changed by using different functions when initially
/// creating the [`Percent`], or converting an existing [`Percent`], for example:
///
/// ```rust
/// # use readable::num::Percent;
/// let f0 = Percent::new_0(3.0);
/// let f2 = Percent::from(3.0);
/// let f3 = Percent::new_3(3.0);
/// let f4 = Percent::new_4(3.0);
///
/// assert!(f0 == "3%");
/// assert!(f2 == "3.00%");
/// assert!(f3 == "3.000%");
/// assert!(f4 == "3.0000%");
///```
///
/// ## Size
/// [`Str<20>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::num::*;
/// assert_eq!(std::mem::size_of::<Percent>(), 32);
/// ```
///
/// ## Copy
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 22 byte array string, literally: [`Str<22>`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also either a [`String`].
/// ```rust
/// # use readable::num::Percent;
/// let a = Percent::from(100_000.0);
///
/// // Copy 'a', use 'b'.
/// let b = a;
/// assert!(b == 100_000.0);
///
/// // We can still use 'a'
/// assert!(a == 100_000.0);
/// ```
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a [`CompactString`](https://docs.rs/compact_str) so that any string 24 bytes (12 bytes on 32-bit) or less are _stack_ allocated instead of _heap_ allocated.
///
/// The documentation will still refer to the inner string as a `String`. Anything returned will also be a `String`.
///
/// ## Float Errors
/// - Inputting [`f64::NAN`], [`f64::INFINITY`], [`f64::NEG_INFINITY`] or the [`f32`] variants returns errors
///
/// ## Math
/// These operators are overloaded. They will always output a new [`Self`]:
/// - `Add +`
/// - `Sub -`
/// - `Div /`
/// - `Mul *`
/// - `Rem %`
///
/// They can either be:
/// - Combined with another [`Self`]: `Percent::from(1.0) + Percent::from(1.0)`
/// - Or with the inner number itself: `Percent::from(1.0) + 1.0`
///
/// They also have the same `panic!()` behavior on overflow as the normal ones, because internally,
/// it is just calling `.inner() $OPERATOR $NUMBER`.
///
/// ```rust
/// # use readable::num::*;
/// assert_eq!(Percent::from(10.0) + 10.0, Percent::from(20.0));
/// assert_eq!(Percent::from(10.0) - 10.0, Percent::from(0.0));
/// assert_eq!(Percent::from(10.0) / 10.0, Percent::from(1.0));
/// assert_eq!(Percent::from(10.0) * 10.0, Percent::from(100.0));
/// assert_eq!(Percent::from(10.0) % 10.0, Percent::from(0.0));
/// ```
/// Overflow example (floats don't panic in this case):
/// ```rust
/// # use readable::num::*;
/// let n = Percent::from(f64::MAX) + f64::MAX;
/// assert!(n.is_unknown());
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::num::Percent;
/// assert_eq!(Percent::ZERO,    "0.00%");
/// assert_eq!(Percent::UNKNOWN, "?.??%");
///
/// assert_eq!(Percent::from(0.001),   "0.00%");
/// assert_eq!(Percent::from(0.1),     "0.10%");
/// assert_eq!(Percent::from(1.0),     "1.00%");
/// assert_eq!(Percent::from(100.0),   "100.00%");
/// assert_eq!(Percent::from(1_000.0), "1,000.00%");
///
/// assert_eq!(Percent::from(1_u32),      "1.00%");
/// assert_eq!(Percent::from(1_000_u32),  "1,000.00%");
/// assert_eq!(Percent::from(10_000_u32), "10,000.00%");
///
/// assert_eq!(Percent::from(-1_i32),      "-1.00%");
/// assert_eq!(Percent::from(-1_000_i32),  "-1,000.00%");
/// assert_eq!(Percent::from(-10_000_i32), "-10,000.00%");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Percent(f64, Str<{ Percent::MAX_LEN }>);

const LEN: usize = 22; // 14 decimal point accuracy + 8 extra chars

impl_math!(Percent, f64);
impl_traits!(Percent, f64);

//---------------------------------------------------------------------------------------------------- Percent Constants
impl Percent {
    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Percent::ZERO, 0.0);
    /// assert_eq!(Percent::ZERO, "0.00%");
    /// ```
    pub const ZERO: Self = Self(0.0, Str::from_static_str("0.00%"));

    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Percent::NAN, "NaN");
    /// assert!(Percent::NAN.is_nan());
    /// ```
    pub const NAN: Self = Self(f64::NAN, Str::from_static_str(NAN));

    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Percent::INFINITY, "inf");
    /// assert!(Percent::INFINITY.is_infinite());
    /// ```
    pub const INFINITY: Self = Self(f64::INFINITY, Str::from_static_str(INFINITY));

    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Percent::UNKNOWN, 0.0);
    /// assert_eq!(Percent::UNKNOWN, "?.??%");
    /// ```
    pub const UNKNOWN: Self = Self(0.0, Str::from_static_str("?.??%"));

    /// The maximum string length of a [`Percent`].
    ///
    /// ```rust
    /// # use readable::num::*;
    /// assert_eq!(Percent::MAX_LEN, 22);
    /// ```
    pub const MAX_LEN: usize = LEN;
}

//---------------------------------------------------------------------------------------------------- Macros
// Implements `new_X` functions.
macro_rules! impl_new {
    ( $num:tt ) => {
        paste::item! {
            #[doc = "Same as [`Percent::from`] but with `" $num "` floating point."]
            #[must_use]
            pub fn [<new_ $num>](f: f64) -> Self {
                return_bad_float!(f, Self::NAN, Self::INFINITY);

                let fract = &format_compact!(concat!("{:.", $num, "}"), f.fract())[2..];
                let string = format_compact!("{}.{}%", str_u64!(f as u64), fract);
                if string.len() > Self::MAX_LEN {
                    Self::UNKNOWN
                } else {
                    let mut s = Str::new();
                    s.push_str_panic(string);
                    Self(f, s)
                }
            }
        }
    };
}

//---------------------------------------------------------------------------------------------------- Percent Impl
impl Percent {
    impl_common!(f64);
    impl_const!();
    impl_usize!();
    impl_isize!();

    #[inline]
    #[must_use]
    /// Calls [`f64::is_nan`].
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    #[inline]
    #[must_use]
    /// Calls [`f64::is_infinite`].
    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    #[inline]
    #[must_use]
    /// ```rust
    /// # use readable::num::*;
    /// assert!(Percent::UNKNOWN.is_unknown());
    /// assert!(!Percent::ZERO.is_unknown());
    /// ```
    pub const fn is_unknown(&self) -> bool {
        matches!(self.as_str().as_bytes(), b"?.??%")
    }

    #[inline]
    #[must_use]
    /// Same as [`Self::from`] but with no floating point on the inner [`String`].
    ///
    /// The inner [`f64`] stays the same as the input.
    ///
    /// This does not round _up_ or _down_, it completely ignores the floating point.
    ///
    /// ## Examples
    /// | Input  | String Output |
    /// |--------|---------------|
    /// | 0.0    | `0%`
    /// | 50.123 | `50%`
    /// | 100.1  | `100%`
    pub fn new_0(f: f64) -> Self {
        return_bad_float!(f, Self::NAN, Self::INFINITY);
        let string = format_compact!("{}%", str_u64!(f as u64));
        if string.len() > Self::MAX_LEN {
            Self::UNKNOWN
        } else {
            let mut s = Str::new();
            s.push_str_panic(string);
            Self(f, s)
        }
    }

    impl_new!(1);
    seq_macro::seq!(N in 3..=14 {
        impl_new!(N);
    });
}

// Implementation Macro.
macro_rules! impl_u {
	($( $number:ty ),*) => {
		$(
			impl From<$number> for Percent {
				#[inline]
				fn from(number: $number) -> Self {
					let string = format_compact!("{}.00%", str_u64!(number as u64));
					if string.len() > Self::MAX_LEN {
						Self::UNKNOWN
					} else {
						let mut s = Str::new();
						s.push_str_panic(string);
						Self(number as f64, s)
					}
				}
			}
		)*
	}
}
impl_u!(u8, u16, u32, u64, usize);

// Implementation Macro.
macro_rules! impl_i {
	($( $number:ty ),*) => {
		$(
			impl From<$number> for Percent {
				#[inline]
				fn from(number: $number) -> Self {
					let string = format_compact!("{}.00%", str_i64!(number as i64));
					if string.len() > Self::MAX_LEN {
						Self::UNKNOWN
					} else {
						let mut s = Str::new();
						s.push_str_panic(string);
						Self(number as f64, s)
					}
				}
			}
		)*
	}
}
impl_i!(i8, i16, i32, i64, isize);

impl From<f32> for Percent {
    #[inline]
    fn from(f: f32) -> Self {
        return_bad_float!(f, Self::NAN, Self::INFINITY);
        #[allow(clippy::cast_lossless)]
        Self::from(f as f64)
    }
}

impl From<f64> for Percent {
    #[inline]
    fn from(f: f64) -> Self {
        return_bad_float!(f, Self::NAN, Self::INFINITY);

        let fract = &format_compact!("{:.2}", f.fract())[2..];
        let string = format_compact!("{}.{}%", str_u64!(f as u64), fract);
        if string.len() > Self::MAX_LEN {
            Self::UNKNOWN
        } else {
            let mut s = Str::new();
            s.push_str_panic(string);
            Self(f, s)
        }
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special() {
        assert_eq!(Percent::ZERO, "0.00%");
        assert_eq!(Percent::UNKNOWN, "?.??%");
        assert_eq!(Percent::NAN, NAN);
        assert_eq!(Percent::INFINITY, INFINITY);

        assert_eq!(Percent::from(0.0), "0.00%");
        assert_eq!(Percent::from(f64::NAN), NAN);
        assert_eq!(Percent::from(f64::INFINITY), INFINITY);
        assert_eq!(Percent::from(f64::NEG_INFINITY), INFINITY);
    }

    #[test]
    fn percent() {
        assert_eq!(Percent::from(0.0), "0.00%");
        assert_eq!(Percent::from(0.001), "0.00%");
        assert_eq!(Percent::from(0.1), "0.10%");
        assert_eq!(Percent::from(1.0), "1.00%");
        assert_eq!(Percent::from(50.0), "50.00%");
        assert_eq!(Percent::from(100.0), "100.00%");
        assert_eq!(Percent::from(150.0), "150.00%");
        assert_eq!(Percent::from(1_000.0), "1,000.00%");
        assert_eq!(Percent::from(250_000.0), "250,000.00%");
    }

    #[test]
    fn percent_dot() {
        assert_eq!(Percent::new_1(0.0), "0.0%");
        assert_eq!(Percent::new_1(1_000.123_4), "1,000.1%");
        assert_eq!(Percent::new_3(1_000.123_4), "1,000.123%");
        assert_eq!(Percent::new_4(1_000.123_4), "1,000.1234%");

        assert_eq!(Percent::new_1(0.1), "0.1%");
        assert_eq!(Percent::new_1(10_000.123_4), "10,000.1%");
        assert_eq!(Percent::new_3(100_000.123_4), "100,000.123%");
        assert_eq!(Percent::new_4(1_000_000.123_4), "1,000,000.1234%");
    }

    #[test]
    fn from_unsigned() {
        assert_eq!(Percent::from(1_u32), "1.00%");
        assert_eq!(Percent::from(1_000_u32), "1,000.00%");
        assert_eq!(Percent::from(10_000_u32), "10,000.00%");
        assert_eq!(Percent::from(100_000_u32), "100,000.00%");
        assert_eq!(Percent::from(1_000_000_u32), "1,000,000.00%");
    }

    #[test]
    fn from_int() {
        assert_eq!(Percent::from(-1_i32), "-1.00%");
        assert_eq!(Percent::from(-1_000_i32), "-1,000.00%");
        assert_eq!(Percent::from(-10_000_i32), "-10,000.00%");
        assert_eq!(Percent::from(-100_000_i32), "-100,000.00%");
        assert_eq!(Percent::from(-1_000_000_i32), "-1,000,000.00%");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        let this: Percent = Percent::from(1.0);
        let json = serde_json::to_string(&this).unwrap();
        assert_eq!(json, r#"[1.0,"1.00%"]"#);

        let this: Percent = serde_json::from_str(&json).unwrap();
        assert_eq!(this, 1.0);
        assert_eq!(this, "1.00%");

        // Bad bytes.
        assert!(serde_json::from_str::<Percent>(&"---").is_err());

        // Unknown.
        let json = serde_json::to_string(&Percent::UNKNOWN).unwrap();
        assert_eq!(json, r#"[0.0,"?.??%"]"#);
        assert!(serde_json::from_str::<Percent>(&json).unwrap().is_unknown());
    }

    #[test]
    #[cfg(feature = "bincode")]
    fn bincode() {
        let this: Percent = Percent::from(1.0);
        let config = bincode::config::standard();
        let bytes = bincode::encode_to_vec(&this, config).unwrap();

        let this: Percent = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert_eq!(this, 1.0);
        assert_eq!(this, "1.00%");

        // Unknown.
        let bytes = bincode::encode_to_vec(&Percent::UNKNOWN, config).unwrap();
        let this: Percent = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert!(this.is_unknown());
    }

    #[test]
    #[cfg(feature = "borsh")]
    fn borsh() {
        let this: Percent = Percent::from(1.0);
        let bytes = borsh::to_vec(&this).unwrap();

        let this: Percent = borsh::from_slice(&bytes).unwrap();
        assert_eq!(this, 1.0);
        assert_eq!(this, "1.00%");

        // Bad bytes.
        assert!(borsh::from_slice::<Percent>(b"bad .-;[]124/ bytes").is_err());

        // Unknown.
        let bytes = borsh::to_vec(&Percent::UNKNOWN).unwrap();
        let this: Percent = borsh::from_slice(&bytes).unwrap();
        assert!(this.is_unknown());
    }
}
