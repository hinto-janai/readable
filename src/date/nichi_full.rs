//---------------------------------------------------------------------------------------------------- Use
use crate::date::free::{ok, ok_year};
#[allow(unused_imports)]
use crate::date::Nichi;
use crate::macros::{impl_common, impl_const, impl_traits};
use crate::str::Str; // docs

//---------------------------------------------------------------------------------------------------- `NichiFull`
/// Same as [`Nichi`] but with fully specified words
///
/// ```rust
/// # use readable::date::*;
/// let nichi = NichiFull::new(2020, 12, 25).unwrap();
/// assert_eq!(nichi, "Friday, December 25th, 2020");
/// assert_eq!(nichi, (2020, 12, 25));
/// ```
///
/// ## Size
/// [`Str<30>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::date::*;
/// assert_eq!(std::mem::size_of::<NichiFull>(), 36);
/// ```
///
/// ## Examples
/// ```rust
/// # use readable::date::*;
/// assert_eq!(NichiFull::new(1776, 7, 4).unwrap(),   "Thursday, July 4th, 1776");
/// assert_eq!(NichiFull::new(2017, 3, 3).unwrap(),   "Friday, March 3rd, 2017");
/// assert_eq!(NichiFull::new(1999, 12, 25).unwrap(), "Saturday, December 25th, 1999");
/// assert_eq!(NichiFull::new(2018, 4, 25).unwrap(),  "Wednesday, April 25th, 2018");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NichiFull((u16, u8, u8), Str<{ NichiFull::MAX_LEN }>);

impl_traits!(NichiFull, (u16, u8, u8));

//---------------------------------------------------------------------------------------------------- NichiFull Constants
impl NichiFull {
    /// The maximum string length of a [`NichiFull`].
    /// ```rust
    /// # use readable::date::*;
    /// assert_eq!(NichiFull::from_str("Saturday, September 31st, 9998").unwrap().len(), NichiFull::MAX_LEN);
    /// ```
    pub const MAX_LEN: usize = 30;

    /// Returns a [`Self`] with the date values set to `(0, 0, 0)`
    ///
    /// This is the exact same as [`Self::UNKNOWN`].
    ///
    /// ```rust
    /// # use readable::date::*;
    /// assert_eq!(NichiFull::ZERO, (0, 0, 0));
    /// assert_eq!(NichiFull::ZERO, "???");
    /// assert_eq!(NichiFull::ZERO, NichiFull::UNKNOWN);
    /// ```
    pub const ZERO: Self = Self::UNKNOWN;

    /// Returned on error situations.
    ///
    /// ```rust
    /// # use readable::date::*;
    /// assert_eq!(NichiFull::UNKNOWN, (0, 0, 0));
    /// assert_eq!(NichiFull::UNKNOWN, "???");
    /// ```
    pub const UNKNOWN: Self = Self((0, 0, 0), Str::from_static_str("???"));
}

//---------------------------------------------------------------------------------------------------- NichiFull impl
impl NichiFull {
    impl_common!((u16, u8, u8));
    impl_const!();

    // Common functions.
    #[inline]
    #[must_use]
    /// Returns a [`Self`] with the date values set to `(0, 0, 0)`
    ///
    /// The [`String`] is set to [`Self::UNKNOWN`].
    pub const fn unknown() -> Self {
        Self::UNKNOWN
    }

    #[inline]
    #[must_use]
    /// Same as [`Self::unknown`]
    ///
    /// ```rust
    /// # use readable::date::*;
    /// assert_eq!(NichiFull::zero(), NichiFull::unknown());
    /// ```
    pub const fn zero() -> Self {
        Self::unknown()
    }

    #[inline]
    #[must_use]
    /// Return the inner year (1000-9999)
    pub const fn year(&self) -> u16 {
        self.0 .0
    }

    #[inline]
    #[must_use]
    /// Return the inner month (1-12)
    pub const fn month(&self) -> u8 {
        self.0 .1
    }

    #[inline]
    #[must_use]
    /// Return the inner day (1-31)
    pub const fn day(&self) -> u8 {
        self.0 .2
    }

    #[inline]
    #[must_use]
    /// Calculate the weekday
    ///
    /// ```rust
    /// # use readable::date::*;
    /// // Christmas in 1999 was on a Saturday.
    /// assert_eq!(
    ///     NichiFull::new(1999, 12, 25).unwrap().weekday().as_str(),
    ///     "Saturday"
    /// );
    /// ```
    pub const fn weekday(&self) -> nichi::Weekday {
        #[allow(clippy::cast_possible_wrap)]
        nichi::Date::weekday_raw(self.year() as i16, self.month(), self.day())
    }

    #[inline]
    #[must_use]
    /// Create a [`Self`] using [`nichi`]'s date type
    pub fn from_nichi(nichi: nichi::Date) -> Self {
        let (y, m, d) = nichi.inner();
        Self::priv_from(y as u16, m, d)
    }

    #[inline]
    /// Parse [`u16`], [`u8`], [`u8`] for a year, month and day.
    ///
    /// ## Errors
    /// - The year must be in-between `1000-9999`
    /// - The month must be in-between `1-12`
    /// - The day must be in-between `1-31`
    /// If an [`Err`] is returned, it will contain a [`NichiFull`] set with [`Self::UNKNOWN`].
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, Self> {
        if ok(year, month, day) {
            Ok(Self::priv_from(year, month, day))
        } else {
            Err(Self::unknown())
        }
    }

    #[inline]
    #[must_use]
    /// Same as [`Self::new`] but silently errors
    ///
    /// ## Errors
    /// - The year must be in-between `1000-9999`
    /// - The month must be in-between `1-12`
    /// - The day must be in-between `1-31` or [`Err`] is returned.
    ///
    /// [`Self::UNKNOWN`] will be returned silently if an error occurs.
    pub fn new_silent(year: u16, month: u8, day: u8) -> Self {
        if ok(year, month, day) {
            Self::priv_from(year, month, day)
        } else {
            Self::unknown()
        }
    }

    #[inline]
    // Private function for serde.
    fn __new_silent(t: (u16, u8, u8)) -> Self {
        Self::new_silent(t.0, t.1, t.2)
    }

    #[inline]
    /// Create a [`Self`] from a UNIX timestamp
    ///
    /// This creates a [`Self`] by taking UNIX timestamp as input.
    ///
    /// (Seconds after `January 1st, 1970 UTC`).
    //
    /// ```rust
    /// # use readable::date::*;
    /// let nichi = NichiFull::from_unix(1698019200).unwrap();
    /// println!("{nichi}");
    /// assert_eq!(nichi, "Monday, October 23rd, 2023");
    /// assert_eq!(nichi, (2023, 10, 23));
    /// ```
    ///
    /// ## Errors
    /// This returns a [`Self::UNKNOWN`] wrapped in an [`Err`] if the given
    /// `unix_timestamp` is a date with a year larger than `9999` or less than `1000`.
    ///
    /// ```rust,should_panic
    /// # use readable::date::*;
    /// // Would be `12732-1-28`.
    /// NichiFull::from_unix(339618217000).unwrap();
    /// ```
    pub fn from_unix(unix_timestamp: u64) -> Result<Self, Self> {
        let nichi = nichi::Date::from_unix(i128::from(unix_timestamp));
        let year = nichi.year().inner() as u16;
        if ok_year(year) {
            Ok(Self::priv_from(
                year,
                nichi.month().inner(),
                nichi.day().inner(),
            ))
        } else {
            Err(Self::unknown())
        }
    }

    #[inline]
    #[must_use]
    /// Same as [`Self::from_unix`] but silently returns a [`Self::UNKNOWN`]
    /// on error that isn't wrapped in a [`Result::Err`].
    pub fn from_unix_silent(unix_timestamp: u64) -> Self {
        match Self::from_unix(unix_timestamp) {
            Ok(s) | Err(s) => s,
        }
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    /// Get the corresponding UNIX timestamp of [`Self`]
    ///
    /// ```rust
    /// # use readable::date::*;
    /// let nichi = NichiFull::from_unix(1698019200).unwrap();
    /// assert_eq!(nichi.as_unix(), 1698019200);
    /// ```
    pub const fn as_unix(&self) -> u64 {
        nichi::Date::new(self.year() as i16, self.month(), self.day()).as_unix() as u64
    }

    #[inline]
    #[allow(clippy::should_implement_trait)] // i don't want to `use std::str::FromStr` everytime.
    /// Parse arbitrary strings for a date.
    ///
    /// ## Errors
    /// - The year must be `1000..=9999`
    /// - The month must be at least the first 3 letters of the month in english (`oct`, `Dec`, `SEP`, etc)
    /// - The day must be a number, either optionally with a leading `0` or suffixed by `th`, `rd`, `nd`, `st` (but not both, e.g, `3rd` is OK, `03` is OK, `03rd` is INVALID)
    ///
    /// The order of the `year`, `month`, and `day` do not matter:
    /// ```rust
    /// # use readable::date::*;
    /// let december_25th_2010 = NichiFull::new(2010, 12, 25).unwrap();
    /// assert_eq!(NichiFull::from_str("dec 25 2010").unwrap(), december_25th_2010);
    /// assert_eq!(NichiFull::from_str("2010 dec 25").unwrap(), december_25th_2010);
    /// assert_eq!(NichiFull::from_str("2010 25th Dec").unwrap(), december_25th_2010);
    /// assert_eq!(NichiFull::from_str("25TH 2010 DEC").unwrap(), december_25th_2010);
    /// ```
    ///
    /// Infinite amount of separator characters are allowed:
    /// ```rust
    /// # use readable::date::*;
    /// let december_25th_2010 = NichiFull::new(2010, 12, 25).unwrap();
    /// assert_eq!(NichiFull::from_str("dec-25 ...       2010").unwrap(), december_25th_2010);
    /// ```
    ///
    /// This function is extremely leniant, as long as some resemblance of a
    /// calendar date is in the input string, it will parse it out:
    /// ```rust
    /// # use readable::date::*;
    /// //                                             Year 2010
    /// //                                   25th day      |
    /// //                          December     |         |
    /// //                             |         |         |
    /// assert_eq!( //                 v         v         v
    ///     NichiFull::from_str("----fasdf decBR wef 25 a - >.a2010a...aa").unwrap(),
    ///     NichiFull::new(2010, 12, 25).unwrap(),
    /// );
    /// ```
    ///
    /// ## ISO 8601 (like)
    /// This function also parses `ISO 8601`-like dates.
    ///
    /// The `year`, `month`, and `day` must be available in that order.
    ///
    /// A single separator character must exist, although it does not need to be `-`.
    ///
    /// ```rust
    /// # use readable::date::*;
    /// let nichi = NichiFull::new(2010, 2, 2).unwrap();
    /// assert_eq!(NichiFull::from_str("2010.02.02").unwrap(), nichi);
    /// assert_eq!(NichiFull::from_str("2010/2/2").unwrap(),   nichi);
    /// assert_eq!(NichiFull::from_str("2010_02_2").unwrap(),  nichi);
    /// assert_eq!(NichiFull::from_str("2010 2 02").unwrap(),  nichi);
    /// ```
    ///
    /// ## Panic
    /// If the input to this function is not ASCII (or 1 byte per character), it may panic.
    ///
    /// ## Examples
    /// ```rust
    /// # use readable::date::*;
    /// let december_25th_2010 = NichiFull::new(2010, 12, 25).unwrap();
    ///
    /// assert_eq!(NichiFull::from_str("dec, 25, 2010").unwrap(),        december_25th_2010);
    /// assert_eq!(NichiFull::from_str("dec 25 2010").unwrap(),          december_25th_2010);
    /// assert_eq!(NichiFull::from_str("Dec 25th 2010").unwrap(),        december_25th_2010);
    /// assert_eq!(NichiFull::from_str("DEC 25TH 2010").unwrap(),        december_25th_2010);
    /// assert_eq!(NichiFull::from_str("DEC-25th-2010").unwrap(),        december_25th_2010);
    /// assert_eq!(NichiFull::from_str("2010.dec.25").unwrap(),          december_25th_2010);
    /// assert_eq!(NichiFull::from_str("2010, 25th, Dec").unwrap(),      december_25th_2010);
    /// assert_eq!(NichiFull::from_str("2010 december 25th").unwrap(),   december_25th_2010);
    /// assert_eq!(NichiFull::from_str("2010, DECEMBER, 25th").unwrap(), december_25th_2010);
    /// assert_eq!(NichiFull::from_str("DECEMBER 25th 2010").unwrap(),   december_25th_2010);
    /// assert_eq!(NichiFull::from_str("December 25th, 2010").unwrap(),  december_25th_2010);
    ///
    /// let april_3rd_1000 = NichiFull::new(1000, 4, 3).unwrap();
    /// assert_eq!(NichiFull::from_str("apr, 3, 1000").unwrap(),     april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("apr 03 1000").unwrap(),      april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("Apr 3rd 1000").unwrap(),     april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("APR 3RD 1000").unwrap(),     april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("APR-3RD-1000").unwrap(),     april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("1000.apr.03").unwrap(),      april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("1000, 3rd, Apr").unwrap(),   april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("1000 april 3rd").unwrap(),   april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("1000, APRIL, 3RD").unwrap(), april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("APRIL 3rd 1000").unwrap(),   april_3rd_1000);
    /// assert_eq!(NichiFull::from_str("April 3rd, 1000").unwrap(),  april_3rd_1000);
    /// ```
    pub fn from_str(string: &str) -> Result<Self, Self> {
        Self::priv_from_str(string)
    }

    #[inline]
    #[must_use]
    /// Same as [`NichiFull::from_str`] but silently returns an [`Self::UNKNOWN`]
    /// on error that isn't wrapped in a [`Result::Err`].
    pub fn from_str_silent(string: &str) -> Self {
        match Self::priv_from_str(string) {
            Ok(s) | Err(s) => s,
        }
    }

    #[inline]
    fn priv_from_str(s: &str) -> Result<Self, Self> {
        #[allow(clippy::option_if_let_else)]
        match nichi::Date::from_str(s) {
            Some(nichi) => {
                let (y, m, d) = nichi.inner();
                Ok(Self::priv_from(y as u16, m, d))
            }
            None => Err(Self::unknown()),
        }
    }

    #[inline]
    #[must_use]
    /// ```rust
    /// # use readable::date::*;
    /// assert!(NichiFull::UNKNOWN.is_unknown());
    /// ```
    pub const fn is_unknown(&self) -> bool {
        matches!(*self, Self::UNKNOWN)
    }
}

//---------------------------------------------------------------------------------------------------- NichiFull impl (private)
impl NichiFull {
    // INVARIANT: inputs must be valid.
    #[inline]
    pub(super) fn priv_from(y: u16, m: u8, d: u8) -> Self {
        let mut buf = [0_u8; Self::MAX_LEN];

        #[allow(clippy::cast_possible_wrap)]
        let nichi = nichi::Date::new(y as i16, m, d);

        // Mon, Fri, Sat, etc
        let weekday = nichi.weekday().as_str().as_bytes();
        let len = weekday.len();
        buf[..len].copy_from_slice(weekday);
        buf[len] = b',';
        buf[len + 1] = b' ';

        let month = nichi.month().as_str().as_bytes();
        let idx = len + 2;
        let len2 = month.len() + idx;
        buf[idx..len2].copy_from_slice(month);
        buf[len2] = b' ';

        let day = nichi.day().as_str_num_ordinal().as_bytes();
        let idx = len2 + 1;
        let len3 = day.len() + idx;
        buf[idx..len3].copy_from_slice(day);
        buf[len3] = b',';
        buf[len3 + 1] = b' ';

        let mut year = crate::toa::Itoa64::new();
        let year = year.format_str(y).as_bytes();
        buf[len3 + 2] = year[0];
        buf[len3 + 3] = year[1];
        buf[len3 + 4] = year[2];
        buf[len3 + 5] = year[3];

        // SAFETY: we're manually creating a `Str`.
        // This is okay because we filled the bytes
        // and know the length.
        let string = unsafe { Str::from_raw(buf, (len3 + 6) as u8) };
        Self((y, m, d), string)
    }
}

//---------------------------------------------------------------------------------------------------- Impl
impl TryFrom<(u16, u8, u8)> for NichiFull {
    type Error = Self;
    #[inline]
    // Calls [`Self::new`].
    fn try_from(value: (u16, u8, u8)) -> Result<Self, Self> {
        Self::new(value.0, value.1, value.2)
    }
}

impl From<nichi::Date> for NichiFull {
    fn from(value: nichi::Date) -> Self {
        Self::from_nichi(value)
    }
}

impl From<crate::date::Date> for NichiFull {
    fn from(value: crate::date::Date) -> Self {
        if value.ok() {
            let (y, m, d) = value.inner();
            Self::priv_from(y, m, d)
        } else {
            Self::unknown()
        }
    }
}

impl From<crate::date::Nichi> for NichiFull {
    fn from(value: crate::date::Nichi) -> Self {
        if value.is_unknown() {
            Self::unknown()
        } else {
            let (y, m, d) = value.inner();
            Self::priv_from(y, m, d)
        }
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    //-------------------------------------------------------------------------------- NichiFull tests.
    const EXPECTED: (u16, u8, u8) = (2020, 12, 25);
    const EXPECTED_STR: &str = "Friday, December 25th, 2020";

    #[test]
    fn invalid_years() {
        assert_eq!(NichiFull::from_str_silent("0"), NichiFull::unknown());
        assert_eq!(NichiFull::from_str_silent("100"), NichiFull::unknown());
        assert_eq!(NichiFull::from_str_silent("010"), NichiFull::unknown());
        assert_eq!(NichiFull::from_str_silent("0010"), NichiFull::unknown());
        assert_eq!(NichiFull::from_str_silent("0100"), NichiFull::unknown());
        assert_eq!(NichiFull::from_str_silent("999"), NichiFull::unknown());
        assert_eq!(NichiFull::from_str_silent("0999"), NichiFull::unknown());
    }

    #[test]
    fn invalid_dates() {
        assert_eq!(
            NichiFull::from_str_silent("12-25-0100"),
            NichiFull::unknown()
        );
        assert_eq!(NichiFull::from_str_silent("01001225"), NichiFull::unknown());
        assert_eq!(
            NichiFull::from_str_silent("25-12-0100"),
            NichiFull::unknown()
        );
        assert_eq!(NichiFull::from_str_silent("01000"), NichiFull::unknown());
        assert_eq!(NichiFull::from_str_silent("010000"), NichiFull::unknown());
        assert_eq!(NichiFull::from_str_silent("0100000"), NichiFull::unknown());
    }

    #[test]
    fn from_str_ymd() {
        assert_eq!(NichiFull::from_str("2020-12-25").unwrap(), EXPECTED);
        assert_eq!(NichiFull::from_str("2020-12-25").unwrap(), EXPECTED_STR);
        assert_eq!(NichiFull::from_str("2020 12 25").unwrap(), EXPECTED);
        assert_eq!(NichiFull::from_str("2020 12 25").unwrap(), EXPECTED_STR);
        assert_eq!(NichiFull::from_str("2020/12/25").unwrap(), EXPECTED);
        assert_eq!(NichiFull::from_str("2020/12/25").unwrap(), EXPECTED_STR);
        assert_eq!(NichiFull::from_str("2020.12.25").unwrap(), EXPECTED);
        assert_eq!(NichiFull::from_str("2020.12.25").unwrap(), EXPECTED_STR);
        assert_eq!(NichiFull::from_str("2020_12_25").unwrap(), EXPECTED);
        assert_eq!(NichiFull::from_str("2020_12_25").unwrap(), EXPECTED_STR);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        let this: NichiFull = NichiFull::try_from((2024, 1, 1)).unwrap();
        let json = serde_json::to_string(&this).unwrap();
        assert_eq!(json, r#"[[2024,1,1],"Monday, January 1st, 2024"]"#);

        let this: NichiFull = serde_json::from_str(&json).unwrap();
        assert_eq!(this, (2024, 1, 1));
        assert_eq!(this, "Monday, January 1st, 2024");

        // Bad bytes.
        assert!(serde_json::from_str::<NichiFull>(&"---").is_err());

        // Unknown.
        let json = serde_json::to_string(&NichiFull::UNKNOWN).unwrap();
        assert_eq!(json, r#"[[0,0,0],"???"]"#);
        assert!(serde_json::from_str::<NichiFull>(&json)
            .unwrap()
            .is_unknown());
    }

    #[test]
    #[cfg(feature = "bincode")]
    fn bincode() {
        let this: NichiFull = NichiFull::try_from((2024, 1, 1)).unwrap();
        let config = bincode::config::standard();
        let bytes = bincode::encode_to_vec(&this, config).unwrap();

        let this: NichiFull = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert_eq!(this, (2024, 1, 1));
        assert_eq!(this, "Monday, January 1st, 2024");

        // Unknown.
        let bytes = bincode::encode_to_vec(&NichiFull::UNKNOWN, config).unwrap();
        let this: NichiFull = bincode::decode_from_slice(&bytes, config).unwrap().0;
        assert!(this.is_unknown());
    }

    #[test]
    #[cfg(feature = "borsh")]
    fn borsh() {
        let this: NichiFull = NichiFull::try_from((2024, 1, 1)).unwrap();
        let bytes = borsh::to_vec(&this).unwrap();

        let this: NichiFull = borsh::from_slice(&bytes).unwrap();
        assert_eq!(this, (2024, 1, 1));
        assert_eq!(this, "Monday, January 1st, 2024");

        // Bad bytes.
        assert!(borsh::from_slice::<NichiFull>(b"bad .-;[]124/ bytes").is_err());

        // Unknown.
        let bytes = borsh::to_vec(&NichiFull::UNKNOWN).unwrap();
        let this: NichiFull = borsh::from_slice(&bytes).unwrap();
        assert!(this.is_unknown());
    }
}
