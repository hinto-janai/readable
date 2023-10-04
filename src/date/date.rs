//---------------------------------------------------------------------------------------------------- Use
use compact_str::format_compact;
use regex::Regex;
use once_cell::sync::Lazy;
use crate::str::Str;
use crate::itoa;
use crate::macros::{
	impl_traits,impl_common,
	impl_const,
};

//---------------------------------------------------------------------------------------------------- Constants
/// Returned when using [`Date::unknown`] or error situations.
pub const UNKNOWN_DATE: &str = "????-??-??";

/// The separator character for [`Date`].
pub const DASH: u8 = b'-';

/// ```rust
/// assert_eq!(readable::date::UNKNOWN_DATE.len(), 10);
/// ```
pub const MAX_LEN_DATE: usize = 10;

//---------------------------------------------------------------------------------------------------- Regexes
// Length of the input string
// determines which regex we use.

// `Y`  (Year)  == always `4` length (1000-9999)
// `M`  (Month) == `1` length (1-9)
// `MM` (Month) == `2` length (10-12)
// `D`  (Day)   == `1` length (1-9)
// `DD` (Day)   == `2` length (10-31)

// Number only matches.
//
// 4  == Y
// 5  == YM    || MY
// 6  == YMM   || YMD   || MDY  || DMY
// 7  == YMMD  || YMDD  || MMDY || MDDY || DMMY || DDMY
// 8  == YMMDD || MMDDY || DDMMY

// Separator matches.
// 6  == Y.M     || M.Y
// 7  == Y.MM    || MM.Y
// 8  == Y.M.D   || M.D.Y
// 9  == Y.MM.D  || Y.M.DD  || MM.D.Y || M.DD.Y || D.MM.Y || DD.M.Y
// 10 == Y.MM.DD || MM.DD.Y || DD.MM.Y

// Number (+space) checker.
static NUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d{4}|[0-9][0-9][0-9][0-9]+)$").unwrap());

// First `4` characters are a valid year.
static YEAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}.*$").unwrap());

// Number only - `YearMonthDay`
static YM_NUM:    Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}[1-9].*$").unwrap());
static YMM_NUM:   Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}([0][1-9]|1[012]).*$").unwrap());
static YMD_NUM:   Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}[1-9][1-9].*$").unwrap());
static YMMD_NUM:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}(0[1-9]|1[012])[1-9].*$").unwrap());
static YMDD_NUM:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}[1-9](0[1-9]|[12][0-9]|30|31).*$").unwrap());
static YMMDD_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}(0[1-9]|1[012])(0[1-9]|[12][0-9]|30|31).*$").unwrap());

// Number only - `MonthDayYear`
static MY_NUM:    Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9][1-9]\d{3}.*$").unwrap());
static MDY_NUM:   Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9][1-9][1-9]\d{3}.*$").unwrap());
static MMDY_NUM:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0[1-9]|1[012])[1-9][1-9]\d{3}.*$").unwrap());
static MDDY_NUM:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9](0[1-9]|[12][0-9]|30|31)[1-9]\d{3}.*$").unwrap());
static MMDDY_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0[1-9]|1[012])(0[1-9]|[12][0-9]|30|31)[1-9]\d{3}.*$").unwrap());

// Number only - `DayMonthYear`
static DMY_NUM:   Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9][1-9][1-9]\d{3}.*$").unwrap());
static DDMY_NUM:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0[1-9]|[12][0-9]|3[01])[1-9][1-9]\d{3}.*$").unwrap());
static DMMY_NUM:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9](0[1-9]|1[012])[1-9]\d{3}.*$").unwrap());
static DDMMY_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0[1-9]|[12][0-9]|30|31)(0[1-9]|1[012])[1-9]\d{3}.*$").unwrap());

// Separated - `YEAR MONTH DAY`
static YM:    Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}\D[1-9].*$").unwrap());
static YMM:   Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}\D(0[1-9]|1[012]).*$").unwrap());
static YMD:   Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}\D[1-9]\D[1-9].*$").unwrap());
static YMMD:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}\D(0[1-9]|1[012])\D[1-9].*$").unwrap());
static YMDD:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}\D[1-9]\D(0[1-9]|[12][0-9]|30|31).*$").unwrap());
static YMMDD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\d{3}\D(0[1-9]|1[012])\D(0[1-9]|[12][0-9]|30|31).*$").unwrap());

// Separated - `MONTH DAY YEAR`
static MY:    Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\D[1-9]\d{3}.*$").unwrap());
static MMY:   Lazy<Regex> = Lazy::new(|| Regex::new(r"^([0][1-9]|1[012])\D[1-9]\d{3}.*$").unwrap());
static MDY:   Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\D[1-9]\D[1-9]\d{3}.*$").unwrap());
static MMDY:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0[1-9]|1[012])\D[1-9]\D[1-9]\d{3}.*$").unwrap());
static MDDY:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\D(0[1-9]|[12][0-9]|30|31)\D[1-9]\d{3}.*$").unwrap());
static MMDDY: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0[1-9]|1[012])\D(0[1-9]|[12][0-9]|30|31)\D[1-9]\d{3}.*$").unwrap());

// Separated - `DAY MONTH YEAR`
static DMY:   Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\D[1-9]\D[1-9]\d{3}.*$").unwrap());
static DDMY:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0[1-9]|[12][0-9]|3[01])\D[1-9]\D[1-9]\d{3}.*$").unwrap());
static DMMY:  Lazy<Regex> = Lazy::new(|| Regex::new(r"^[1-9]\D(0[1-9]|1[012])\D[1-9]\d{3}.*$").unwrap());
static DDMMY: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0[1-9]|[12][0-9]|30|31)\D(0[1-9]|1[012])\D[1-9]\d{3}.*$").unwrap());

//---------------------------------------------------------------------------------------------------- Functions.
#[inline(always)]
const fn ok_year(y: u16) -> bool {
	y >= 1000 && y <= 9999
}

#[inline(always)]
const fn ok_month(m: u8) -> bool {
	m >= 1 && m <= 12
}

#[inline(always)]
const fn ok_day(d: u8) -> bool {
	d >= 1 && d <= 31
}

#[inline(always)]
const fn ok(y:u16, m: u8, d: u8) -> bool {
	ok_year(y) && ok_month(m) && ok_day(d)
}

//---------------------------------------------------------------------------------------------------- `Date`
/// A _recent_ date that is in `YEAR-MONTH-DAY` format, similar to [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
///
/// [`Date`] differs from [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) in that:
/// - It only allows years from `1000-9999`
/// - It allows months and days to be truncated (e.g `2010` is a valid [`Date`])
/// - It is _very_ lenient when parsing strings
///
/// The inner "integer" type is a tuple of: `(u16, u8, u8)` representing the `(Year, Month, Day)`
///
/// Any value being `0` means it is invalid, akin to a [`None`]:
/// ```rust
/// # use readable::Date;
/// let a = Date::from_str("2020-12").unwrap();
///
/// assert_eq!(a, "2020-12");
/// assert_eq!(a, (2020, 12, 0));
/// ```
///
/// - The year must be `1000-9999`
/// - The month must be `1-12`
/// - The day must be `1-31`
///
/// Example:
/// ```
/// # use readable::Date;
/// let (y, m, d) = (2020_u16, 12_u8, 1_u8);
///
/// let d1 = Date::from_ymd(y, m, d).unwrap();
/// let d2 = Date::from_ym(y, m).unwrap();
/// let d3 = Date::from_y(y).unwrap();
///
/// assert_eq!(d1, "2020-12-01");
/// assert_eq!(d2, "2020-12");
/// assert_eq!(d3, "2020");
///```
///
/// ## String parsing and format
/// To parse an arbitrary string into a [`Date`], use: [`Date::from_str`].
///
/// Although [`Date`] will always internally be `YYYY-MM-DD`, the input string can be any of these formats:
/// ```rust
/// # use readable::Date;
/// assert_eq!(Date::from_str("2022-12-31").unwrap(), "2022-12-31"); // YYYY-MM-DD
/// assert_eq!(Date::from_str("2022-01-01").unwrap(), "2022-01-01"); // YYYY-M-D
/// assert_eq!(Date::from_str("2022-12").unwrap(),    "2022-12");    // YYYY-MM
/// assert_eq!(Date::from_str("2022-1").unwrap(),     "2022-01");    // YYYY-M
/// assert_eq!(Date::from_str("2022").unwrap(),       "2022");       // YYYY
/// assert_eq!(Date::from_str("12-31-2022").unwrap(), "2022-12-31"); // MM-DD-YYYY
/// assert_eq!(Date::from_str("1-31-2022").unwrap(),  "2022-01-31"); // M-DD-YYYY
/// assert_eq!(Date::from_str("12-1-2022").unwrap(),  "2022-12-01"); // MM-D-YYYY
/// assert_eq!(Date::from_str("1-5-2022").unwrap(),   "2022-01-05"); // M-D-YYYY
/// assert_eq!(Date::from_str("12-2022").unwrap(),    "2022-12");    // MM-YYYY
/// assert_eq!(Date::from_str("1-2022").unwrap(),     "2022-01");    // M-YYYY
/// assert_eq!(Date::from_str("31-12-2022").unwrap(), "2022-12-31"); // DD-MM-YYYY
/// assert_eq!(Date::from_str("31-1-2022").unwrap(),  "2022-01-31"); // DD-M-YYYY
///
/// // This one is ambiguous, `Date` will always assume M-D-YYYY over D-M-YYYY
/// assert_eq!(Date::from_str("3-1-2022").unwrap(), "2022-03-01");
/// ```
///
/// You can input a string that is _just_ numbers, or separated by a single byte, e.g:
/// ```rust
/// # use readable::Date;
/// let dates = [
///     Date::from_str("20201231").unwrap(),
///     Date::from_str("2020-12-31").unwrap(),
///     Date::from_str("2020/12/31").unwrap(),
///     Date::from_str("2020.12.31").unwrap(),
///     Date::from_str("2020_12_31").unwrap(),
///     Date::from_str("2020 12 31").unwrap(),
/// ];
///
/// for date in dates {
///     assert_eq!(date, (2020, 12, 31));
///     assert_eq!(date, "2020-12-31");
/// }
/// ```
/// **Warning:** be aware that many `UTF-8` characters are _not_ a single byte in length.
///
/// The separator character doesn't need to be `-` and it doesn't need to exist at all:
/// ```rust
/// # use readable::Date;
/// assert_eq!(Date::from_str("20221231").unwrap(), "2022-12-31"); // YYYYMMDD
/// assert_eq!(Date::from_str("202212").unwrap(),   "2022-12");    // YYYYMM
/// assert_eq!(Date::from_str("2022").unwrap(),     "2022");       // YYYY
/// assert_eq!(Date::from_str("12312022").unwrap(), "2022-12-31"); // MMDDYYYY
///
/// // Some dates are ambiguous (122001 could be 2001-12 or 1220-01).
/// // See further below for more examples.
/// assert_eq!(Date::from_str("129000").unwrap(), "9000-01-02"); // MDYYYY
/// ```
///
/// Given an ambiguous date, the parsing function will prioritize:
///
/// - `YEAR-MONTH-DAY`
/// - `MONTH-DAY-YEAR`
/// - `DAY-MONTH-YEAR`
///
/// Example:
/// ```rust
/// # use readable::Date;
/// // This could be:
/// //   - 1111-11-11 (YMD)
/// //   - 11-11-1111 (MDY)
/// //   - 11-11-1111 (DMY)
/// let ambiguous = "11111111";
/// // Although, we prioritize YMD.
/// assert_eq!(Date::from_str(ambiguous).unwrap(), "1111-11-11");
///
/// // This could be:
/// //   - MDY
/// //   - DMY
/// let ambiguous = "12-12-1111";
/// // We prioritize MDY over DMY.
/// assert_eq!(Date::from_str(ambiguous).unwrap(), "1111-12-12");
///
/// // This cannot be MDY, so it must be DMY.
/// let dmy = "13-11-1111";
/// assert_eq!(Date::from_str(dmy).unwrap(), "1111-11-13");
/// ```
///
/// Some errors can occur during string parsing:
/// - Year is not in-between `1000-9999`
/// - Month is not in-between `1-12`
/// - Day is not in-between `1-31`
///
/// ## Trailing Characters
/// [`Date`] is very lenient when parsing strings, as it will ignore trailing
/// characters if there is a valid match in the first characters, for example:
/// ```rust
/// # use readable::Date;
/// // This is an invalid year (10,000), although the first 4 characters
/// // extracted _are_ a valid year (1000), so this gets a pass.
/// assert_eq!(Date::from_str("10000-57-99").unwrap(), "1000");
///
/// // This is convenient when parsing bad data that
/// // may have un-related trailing characters.
/// assert_eq!(Date::from_str("1000bad-data").unwrap(), "1000"); // but we can still parse it.
/// ```
///
/// This leniency causes [`Date`] to parse some incorrect strings,
/// even if it plainly looks incorrect (for convenience sake):
/// ```rust
/// # use readable::Date;
/// // trailing 0 is ignored, year 1000 is extracted
/// let d1 = Date::from_str("10000").unwrap();
/// // 32nd day is ignored, year.month is extracted
/// let d2 = Date::from_str("2022.12.32").unwrap();
/// // `2/32` is ignored, but `3` is a valid month,
/// // so both the year & month 3 is extracted
/// let d3 = Date::from_str("2000/32/32").unwrap();
/// // random trailing data is ignored
/// let d4 = Date::from_str("2000/12/25aaaaaa").unwrap();
///
/// assert_eq!(d1, "1000");
/// assert_eq!(d2, "2022-12");
/// assert_eq!(d3, "2000-03");
/// assert_eq!(d4, "2000-12-25");
/// ```
///
/// ## Size
/// [`Str<10>`] is used internally to represent the string.
///
/// ```rust
/// # use readable::*;
/// assert_eq!(std::mem::size_of::<Date>(), 16);
/// ```
///
/// ## Copy
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 10 byte array buffer, literally: [`Str<10>`].
///
/// Since the max valid date is: `9999-12-31` (10 characters), a 10 byte
/// buffer is used and enables this type to have [`Copy`].
///
/// The documentation will still refer to the inner buffer as a [`String`]. Anything returned will also be a [`String`].
/// ```rust
/// # use readable::Date;
/// let a = Date::from_str("2014-04-22").unwrap();
///
/// // Copy 'a', use 'b'.
/// let b = a;
/// assert_eq!(b, "2014-04-22");
///
/// // We can still use 'a'
/// assert_eq!(a, "2014-04-22");
/// ```
#[cfg_attr(feature = "serde",derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode",derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Date((u16, u8, u8), Str<MAX_LEN_DATE>);

impl_traits!(Date, (u16, u8, u8));

//---------------------------------------------------------------------------------------------------- Date impl
impl Date {
	impl_common!((u16, u8, u8));
	impl_const!();

	// Common functions.
	#[inline]
	/// Returns a [`Self`] with the date values set to `(0, 0, 0)`
	///
	/// The [`String`] is set to [`UNKNOWN_DATE`].
	pub const fn unknown() -> Self {
		Self((0, 0, 0), Str::from_static_str(UNKNOWN_DATE))
	}

	#[inline]
	/// Same as [`Self::unknown`]
	pub const fn zero() -> Self {
		Self((0, 0, 0), Str::from_static_str(UNKNOWN_DATE))
	}

	#[inline]
	/// Return the inner year (1000-9999)
	pub const fn year(&self) -> u16 {
		self.0.0
	}

	#[inline]
	/// Return the inner month (1-12)
	pub const fn month(&self) -> u8 {
		self.0.1
	}

	#[inline]
	/// Return the inner day (1-31)
	pub const fn day(&self) -> u8 {
		self.0.2
	}

	#[inline]
	/// Returns `true` if the inner year is valid.
	/// ```rust
	/// # use readable::Date;
	/// let a = Date::from_y(2022).unwrap();
	/// let b = Date::unknown();
	///
	/// assert!(a.ok_year());
	/// assert!(!b.ok_year());
	/// ```
	pub const fn ok_year(&self) -> bool {
		ok_year(self.0.0)
	}

	#[inline]
	/// Returns `true` if the inner month is valid.
	/// ```rust
	/// # use readable::Date;
	/// let a = Date::from_ym(2022, 12).unwrap();
	/// let b = Date::unknown();
	///
	/// assert!(a.ok_month());
	/// assert!(!b.ok_month());
	/// ```
	pub const fn ok_month(&self) -> bool {
		ok_month(self.0.1)
	}

	#[inline]
	/// Returns `true` if the inner day is valid.
	/// ```rust
	/// # use readable::Date;
	/// let a = Date::from_ymd(2022, 12, 31).unwrap();
	/// let b = Date::unknown();
	///
	/// assert!(a.ok_day());
	/// assert!(!b.ok_day());
	/// ```
	pub const fn ok_day(&self) -> bool {
		ok_day(self.0.2)
	}

	#[inline]
	/// Returns `true` if the inner `(year, month, day)` are all valid.
	/// ```rust
	/// # use readable::Date;
	/// let a = Date::from_ymd(2022, 12, 31).unwrap();
	/// let b = Date::unknown();
	///
	/// assert!(a.ok());
	/// assert!(!b.ok());
	/// ```
	pub const fn ok(&self) -> bool {
		ok(self.0.0, self.0.1, self.0.2)
	}

	#[inline]
	/// Parse a [`u16`] for a year.
	///
	/// ## Errors
	/// - The year must be in-between `1000-9999`
	///
	/// If an [`Err`] is returned, it will contain a [`Date`]
	/// set with [`UNKNOWN_DATE`] which looks like: `????-??-??`.
	pub fn from_y(year: u16) -> Result<Self, Self> {
		if ok_year(year) {
			Ok(Self::priv_y_num(year))
		} else {
			Err(Self::unknown())
		}
	}

	#[inline]
	/// Parse [`u16`], [`u8`] for a year and month.
	///
	/// ## Errors
	/// - The year must be in-between `1000-9999`
	/// - The month must be in-between `1-12`
	///
	/// If an [`Err`] is returned, it will contain a [`Date`]
	/// set with [`UNKNOWN_DATE`] which looks like: `????-??-??`.
	pub fn from_ym(year: u16, month: u8) -> Result<Self, Self> {
		if ok_year(year) && ok_month(month) {
			Ok(Self::priv_ym_num(year, month))
		} else {
			Err(Self::unknown())
		}
	}

	#[inline]
	/// Parse [`u16`], [`u8`], [`u8`] for a year, month and day.
	///
	/// ## Errors
	/// - The year must be in-between `1000-9999`
	/// - The month must be in-between `1-12`
	/// - The day must be in-between `1-31`
	///
	/// If an [`Err`] is returned, it will contain a [`Date`]
	/// set with [`UNKNOWN_DATE`] which looks like: `????-??-??`.
	pub fn from_ymd(year: u16, month: u8, day: u8) -> Result<Self, Self> {
		if ok(year, month, day) {
			Ok(Self::priv_ymd_num(year, month, day))
		} else {
			Err(Self::unknown())
		}
	}

	#[inline]
	/// Same as [`Self::from_y`] but silently errors
	///
	/// ## Errors
	/// - The year must be in-between `1000-9999`
	///
	/// [`UNKNOWN_DATE`] will be returned silently if an error occurs.
	pub fn from_y_silent(year: u16) -> Self {
		if ok_year(year) {
			Self::priv_y_num(year)
		} else {
			Self::unknown()
		}
	}

	#[inline]
	/// Same as [`Self::from_ym`] but silently errors
	///
	/// ## Errors
	/// - The year must be in-between `1000-9999`
	/// - The month must be in-between `1-12`
	///
	/// [`UNKNOWN_DATE`] will be returned silently if an error occurs.
	pub fn from_ym_silent(year: u16, month: u8) -> Self {
		if ok_year(year) && ok_month(month) {
			Self::priv_ym_num(year, month)
		} else {
			Self::unknown()
		}
	}

	#[inline]
	/// Same as [`Self::from_ymd`] but silently errors
	///
	/// ## Errors
	/// - The year must be in-between `1000-9999`
	/// - The month must be in-between `1-12`
	/// - The day must be in-between `1-31` or [`Err`] is returned.
	///
	/// [`UNKNOWN_DATE`] will be returned silently if an error occurs.
	pub fn from_ymd_silent(year: u16, month: u8, day: u8) -> Self {
		if ok(year, month, day) {
			Self::priv_ymd_num(year, month, day)
		} else {
			Self::unknown()
		}
	}

	#[inline]
	#[allow(clippy::should_implement_trait)] // i don't want to `use std::str::FromStr` everytime.
	/// Parse arbitrary strings for a date.
	///
	/// If the complete date cannot be parsed, this function will
	/// attempt to extract as much as it can, which may lead to
	/// surprising results. Read [`Date`]'s documentation for more info.
	///
	/// Example:
	/// ```rust
	/// # use readable::Date;
	/// // Parsed as `YYYY-M` (2022-9)
	/// let a = Date::from_str("2022-99-99").unwrap();
	/// // Parsed as `YYYY-MM` (2022-03)
	/// let b = Date::from_str("2022-03-32").unwrap();
	/// // Parsed as `YYYY-M` (2022-3)
	/// let c = Date::from_str("2022-32-00").unwrap();
	/// // Parsed as `YYYY` (2022)
	/// let d = Date::from_str("2022-00-31").unwrap();
	///
	/// assert_eq!(a, (2022, 9, 0));
	/// assert_eq!(b, (2022, 3, 0));
	/// assert_eq!(c, (2022, 3, 0));
	/// assert_eq!(d, (2022, 0, 0));
	/// ```
	///
	/// If an [`Err`] is returned, it will contain a [`Date`]
	/// set with [`UNKNOWN_DATE`] which looks like: `????-??-??`.
	///
	/// ## Examples:
	/// ```rust
	/// # use readable::Date;
	/// let a = Date::from_str("2022-3-31").unwrap();
	/// assert!(a == "2022-03-31");
	///
	/// ```
	pub fn from_str(string: &str) -> Result<Self, Self> {
		Self::priv_from_str(string)
	}

	#[inline]
	/// Same as [`Date::from_str`] but silently returns an [`UNKNOWN_DATE`]
	/// on error that isn't wrapped in a [`Result::Err`].
	pub fn from_str_silent(string: &str) -> Self {
		match Self::priv_from_str(string) {
			Ok(s)  => s,
			Err(s) => s,
		}
	}

	#[inline]
	fn priv_from_str(s: &str) -> Result<Self, Self> {
		let len = s.len();

		// // If feature enabled, match on all possible
		// // `YYYY-MM-DD` strings between `1900-2100`.
		// #[cfg(feature = "inline_date")]
		// if len == 10 {
		// 	if let Some(date) = readable_inlined_date::inlined(string.as_bytes()) {
		// 		let (y, m, d, bytes) = date;
		// 		return Ok(Self((y, m, d), Buffer::from_unchecked(&bytes)));
		// 	}
		// }

		// Return `YYYY`.
		if len == 4 {
			match s.parse::<u16>() {
				// If the string is 4 characters long, but is less than 1000,
				// there must be leading zeros
				Ok(y) if ok_year(y) => return Ok(Self::priv_y(s)),
				_     => return Err(Self::unknown()),
			}
		}

		// Less than the minimum year (1000).
		if len < 4 {
			return Err(Self::unknown());
		}

		// SAFETY:
		// If the regex matches, the number and the positions of where
		// they are in the `str` UTF-8 byte array _should_ be valid,
		// so `parse().unwrap()` and indexing will only `panic!()`
		// if the regexes I've made are faulty themselves (sorry).

		// If input is just numbers...
		if NUM.is_match(s) {
			match len {
				// YM || MY
				5 => {
					if YM_NUM.is_match(s) {
						let y = &s[..4];
						let m = &s[4..];
						return Ok(Self::priv_ym(y, m));
					} else if MY_NUM.is_match(s) {
						let m = &s[..1];
						let y = &s[1..];
						return Ok(Self::priv_ym(y, m));
					} else if YEAR.is_match(s) {
						let y = &s[..4];
						return Ok(Self::priv_y(y));
					}
				}

				// YMM || YMD || MDY || DMY
				6 => {
					if YMM_NUM.is_match(s) {
						let y = &s[..4];
						let m = &s[4..];
						return Ok(Self::priv_ym(y, m));
					} else if YMD_NUM.is_match(s) {
						let y = &s[..4];
						let m = &s[4..5];
						let d = &s[5..];
						return Ok(Self::priv_ymd(y, m, d));
					} else if MDY_NUM.is_match(s) {
						let m = &s[..1];
						let d = &s[1..2];
						let y = &s[2..];
						return Ok(Self::priv_ymd(y, m, d));
					} else if DMY_NUM.is_match(s) {
						let d = &s[..1];
						let m = &s[1..2];
						let y = &s[2..];
						return Ok(Self::priv_ymd(y, m, d));
					} else if YEAR.is_match(s) {
						let y = &s[..4];
						return Ok(Self::priv_y(y));
					}
				},

				// YMMD || YMDD || MMDY || MDDY || DMMY || DDMY
				7 => {
					if YMMD_NUM.is_match(s) {
						let y = &s[..4];
						let m = &s[4..6];
						let d = &s[6..];
						return Ok(Self::priv_ymd(y, m, d));
					} else if YMDD_NUM.is_match(s) {
						let y = &s[..4];
						let m = &s[4..5];
						let d = &s[5..];
						return Ok(Self::priv_ymd(y, m, d));
					} else if MMDY_NUM.is_match(s) {
						let m = &s[..2];
						let d = &s[2..3];
						let y = &s[3..];
						return Ok(Self::priv_ymd(y, m, d));
					} else if MDDY_NUM.is_match(s) {
						let m = &s[..1];
						let d = &s[1..3];
						let y = &s[3..];
						return Ok(Self::priv_ymd(y, m, d));
					} else if DMMY_NUM.is_match(s) {
						let d = &s[..1];
						let m = &s[1..3];
						let y = &s[3..];
						return Ok(Self::priv_ymd(y, m, d));
					} else if DDMY_NUM.is_match(s) {
						let d = &s[..2];
						let m = &s[2..3];
						let y = &s[3..];
						return Ok(Self::priv_ymd(y, m, d));
					} else if YEAR.is_match(s) {
						let y = &s[..4];
						return Ok(Self::priv_y(y));
					}
				},

				// YMMDD || MMDDY || DDMMY
				_ => {
					if YMMDD_NUM.is_match(s) {
						let y = &s[..4];
						let m = &s[4..6];
						let d = &s[6..8];
						return Ok(Self::priv_ymd(y, m, d));
					} else if MMDDY_NUM.is_match(s) {
						let m = &s[..2];
						let d = &s[2..4];
						let y = &s[4..8];
						return Ok(Self::priv_ymd(y, m, d));
					} else if DDMMY_NUM.is_match(s) {
						let d = &s[..2];
						let m = &s[2..4];
						let y = &s[4..8];
						return Ok(Self::priv_ymd(y, m, d));
					} else if YEAR.is_match(s) {
						let y = &s[..4];
						return Ok(Self::priv_y(y));
					}
				},

			}
		}

		// If input is separated...
		match len {
			// Y.M || M.Y
			6 => {
				if YM.is_match(s) {
					let y = &s[..4];
					let m = &s[5..];
					return Ok(Self::priv_ym(y, m));
				} else if MY.is_match(s) {
					let m = &s[..1];
					let y = &s[2..];
					return Ok(Self::priv_ym(y, m));
				} else if YEAR.is_match(s) {
					let y = &s[..4];
					return Ok(Self::priv_y(y));
				}
			},

			// Y.MM || MM.Y
			7 => {
				if YMM.is_match(s) {
					let y = &s[..4];
					let m = &s[5..];
					return Ok(Self::priv_ym(y, m));
				} else if MMY.is_match(s) {
					let m = &s[..2];
					let y = &s[3..];
					return Ok(Self::priv_ym(y, m));
				// Fallback, try to at least parse YEAR + MONTH or at least YEAR.
				} else if YM.is_match(s) {
					let y = &s[..4];
					let m = &s[5..6];
					return Ok(Self::priv_ym(y, m));
				} else if YEAR.is_match(s) {
					let y = &s[..4];
					return Ok(Self::priv_y(y));
				}
			},

			// Y.M.D || M.D.Y || D.M.Y
			8 => {
				if YMD.is_match(s) {
					let y = &s[..4];
					let m = &s[5..6];
					let d = &s[7..];
					return Ok(Self::priv_ymd(y, m, d));
				} else if MDY.is_match(s) {
					let m = &s[..1];
					let d = &s[2..3];
					let y = &s[4..];
					return Ok(Self::priv_ymd(y, m, d));
				} else if DMY.is_match(s) {
					let d = &s[..1];
					let m = &s[2..3];
					let y = &s[4..];
					return Ok(Self::priv_ymd(y, m, d));
				// Fallback, try to at least parse YEAR + MONTH or at least YEAR.
				} else if YMM.is_match(s) {
					let y = &s[..4];
					let m = &s[5..7];
					return Ok(Self::priv_ym(y, m));
				} else if YM.is_match(s) {
					let y = &s[..4];
					let m = &s[5..6];
					return Ok(Self::priv_ym(y, m));
				} else if YEAR.is_match(s) {
					let y = &s[..4];
					return Ok(Self::priv_y(y));
				}
			},

			// Y.MM.D || Y.M.DD || MM.D.Y || M.DD.Y || D.MM.Y || DD.M.Y
			9 => {
				if YMMD.is_match(s) {
					let y = &s[..4];
					let m = &s[5..7];
					return Ok(Self::priv_ym(y, m));
				} else if YMDD.is_match(s) {
					let y = &s[..4];
					let m = &s[5..6];
					let d = &s[7..];
					return Ok(Self::priv_ymd(y, m, d));
				} else if MMDY.is_match(s) {
					let m = &s[..2];
					let d = &s[3..4];
					let y = &s[5..];
					return Ok(Self::priv_ymd(y, m, d));
				} else if MDDY.is_match(s) {
					let m = &s[..1];
					let d = &s[2..4];
					let y = &s[5..];
					return Ok(Self::priv_ymd(y, m, d));
				} else if DMMY.is_match(s) {
					let d = &s[..1];
					let m = &s[2..4];
					let y = &s[5..];
					return Ok(Self::priv_ymd(y, m, d));
				} else if DDMY.is_match(s) {
					let d = &s[..2];
					let m = &s[3..4];
					let y = &s[5..];
					return Ok(Self::priv_ymd(y, m, d));
				// Fallback, try to at least parse YEAR + MONTH or at least YEAR.
				} else if YMM.is_match(s) {
					let y = &s[..4];
					let m = &s[5..7];
					return Ok(Self::priv_ym(y, m));
				} else if YM.is_match(s) {
					let y = &s[..4];
					let m = &s[5..6];
					return Ok(Self::priv_ym(y, m));
				} else if YEAR.is_match(s) {
					let y = &s[..4];
					return Ok(Self::priv_y(y));
				}
			},

			// Y.MM.DD || MM.DD.Y || DD.MM.Y
			_ => {
				if YMMDD.is_match(s) {
					let y = &s[..4];
					let m = &s[5..7];
					let d = &s[8..10];
					return Ok(Self::priv_ymd(y, m, d));
				} else if MMDDY.is_match(s) {
					let m = &s[..2];
					let d = &s[3..5];
					let y = &s[6..10];
					return Ok(Self::priv_ymd(y, m, d));
				} else if DDMMY.is_match(s) {
					let d = &s[..2];
					let m = &s[3..5];
					let y = &s[6..10];
					return Ok(Self::priv_ymd(y, m, d));
				// Fallback, try to at least parse YEAR + MONTH or at least YEAR.
				} else if YMM.is_match(s) {
					let y = &s[..4];
					let m = &s[5..7];
					return Ok(Self::priv_ym(y, m));
				} else if YM.is_match(s) {
					let y = &s[..4];
					let m = &s[5..6];
					return Ok(Self::priv_ym(y, m));
				} else if YEAR.is_match(s) {
					let y = &s[..4];
					return Ok(Self::priv_y(y));
				}
			},
		}

		// Give up.
		Err(Date::unknown())
	}
}

//---------------------------------------------------------------------------------------------------- Date impl (private)
impl Date {
	// INVARIANT:
	// The inputs _must_ be correct.
	// Private functions for construction.
	//
	// The callers are responsible for giving:
	// - A year slice that is always `4` length
	// - A month slice that is always `1` or `2` length
	// - A day slice that is always `1` or `2` length
	#[inline]
	fn priv_y(year: &str) -> Self {
		debug_assert_eq!(year.len(), 4);
		let y = year.parse::<u16>().unwrap();
		Self::priv_y_num(y)
	}
	#[inline]
	fn priv_ym(year: &str, month: &str) -> Self {
		debug_assert_eq!(year.len(), 4);
		debug_assert!(month.len() <= 2);
		debug_assert!(month.len() >= 1);
		let y = year.parse::<u16>().unwrap();
		let m = month.parse::<u8>().unwrap();
		Self::priv_ym_num(y, m)
	}
	#[inline]
	fn priv_ymd(year: &str, month: &str, day: &str) -> Self {
		debug_assert_eq!(year.len(), 4);
		debug_assert!(month.len() <= 2);
		debug_assert!(month.len() >= 1);
		debug_assert!(day.len() <= 2);
		debug_assert!(day.len() >= 1);
		let y = year.parse::<u16>().unwrap();
		let m = month.parse::<u8>().unwrap();
		let d = day.parse::<u8>().unwrap();
		Self::priv_ymd_num(y, m, d)
	}

	#[inline]
	fn priv_y_num(y: u16) -> Self {
		let mut buf = [0_u8; MAX_LEN_DATE];
		Self::format_year(&mut buf, itoa!(y));
		// SAFETY: we're manually creating a `Str`.
		// This is okay because we filled the bytes
		// and know the length.
		let string = unsafe { Str::from_raw(buf, 4) };

		Self((y, 0, 0), string)
	}

	#[inline]
	fn priv_ym_num(y: u16, m: u8) -> Self {
		let mut buf = [0_u8; MAX_LEN_DATE];
		let b = &mut buf;

		Self::format_year(b, itoa!(y));
		b[4] = DASH;
		Self::format_month(b, Self::match_month(m));

		// SAFETY: we're manually creating a `Str`.
		// This is okay because we filled the bytes
		// and know the length.
		let string = unsafe { Str::from_raw(buf, 7) };

		Self((y, m, 0), string)
	}

	#[inline]
	fn priv_ymd_num(y: u16, m: u8, d: u8) -> Self {
		let mut buf = [0_u8; MAX_LEN_DATE];
		let b = &mut buf;

		Self::format_year(b, itoa!(y));
		b[4] = DASH;
		Self::format_month(b, Self::match_month(m));
		b[7] = DASH;
		Self::format_day(b, Self::match_day(d));

		// SAFETY: we're manually creating a `Str`.
		// This is okay because we filled the bytes
		// and know the length.
		let string = unsafe { Str::from_raw(buf, MAX_LEN_DATE as u8) };

		Self((y, m, d), string)
	}

	#[inline]
	// Format `YYYY`.
	fn format_year(buf: &mut [u8; MAX_LEN_DATE], year: &str) {
		buf[..4].copy_from_slice(year.as_bytes());
	}

	#[inline]
	// Pad month if needed.
	fn format_month(buf: &mut [u8; MAX_LEN_DATE], month: &str) {
		let m = month.as_bytes();

		debug_assert!(m.len() >= 1);
		debug_assert!(m.len() <= 2);

		if m.len() == 1 {
			buf[5] = b'0';
			buf[6] = m[0];
		} else {
			buf[5] = m[0];
			buf[6] = m[1];
		}
	}

	#[inline]
	// Pad day if needed.
	fn format_day(buf: &mut [u8; MAX_LEN_DATE], day: &str) {
		let d = day.as_bytes();

		debug_assert!(d.len() >= 1);
		debug_assert!(d.len() <= 2);

		if d.len() == 1 {
			buf[8] = b'0';
			buf[9] = d[0];
		} else {
			buf[8] = d[0];
			buf[9] = d[1];
		}
	}

	#[inline]
	/// INVARIANT: input must be 1..=12
	const fn match_month(m: u8) -> &'static str {
		debug_assert!(m >= 1);
		debug_assert!(m <= 12);
		match m {
			1  => "1",
			2  => "2",
			3  => "3",
			4  => "4",
			5  => "5",
			6  => "6",
			7  => "7",
			8  => "8",
			9  => "9",
			10 => "10",
			11 => "11",
			12 => "12",
			_ => unreachable!(),
		}
	}

	#[inline]
	/// INVARIANT: input must be 1..=31
	const fn match_day(d: u8) -> &'static str {
		debug_assert!(d >= 1);
		debug_assert!(d <= 31);
		match d {
			1  => "1",
			2  => "2",
			3  => "3",
			4  => "4",
			5  => "5",
			6  => "6",
			7  => "7",
			8  => "8",
			9  => "9",
			10 => "10",
			11 => "11",
			12 => "12",
			13 => "13",
			14 => "14",
			15 => "15",
			16 => "16",
			17 => "17",
			18 => "18",
			19 => "19",
			20 => "20",
			21 => "21",
			22 => "22",
			23 => "23",
			24 => "24",
			25 => "25",
			26 => "26",
			27 => "27",
			28 => "28",
			29 => "29",
			30 => "30",
			31 => "31",
			_ => unreachable!(),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use std::cmp::Ordering;

	//-------------------------------------------------------------------------------- Date tests.
	const EXPECTED: (u16, u8, u8) = (2020, 12, 25);
	const EXPECTED_STR: &str      = "2020-12-25";

	#[test]
	fn cmp() {
		let a = Date::from_str("2020-12-01").unwrap();
		let b = Date::from_str("2020-12-01").unwrap();
		let c = Date::from_str("2020-12").unwrap();
		let d = Date::from_str("2020-01").unwrap();
		assert_eq!(a.cmp(&b), Ordering::Equal);
		assert_eq!(a.cmp(&c), Ordering::Greater);
		assert_eq!(a.cmp(&d), Ordering::Greater);

		for i in 1..12 {
			let s = format_compact!("2020-{:0>2}-01",i);
			let b = Date::from_str(&s).unwrap();
			assert_eq!(a.cmp(&b), Ordering::Greater);
		}
		for i in 2..32 {
			let s = format_compact!("2020-12-{:0>2}",i);
			let b = Date::from_str(&s).unwrap();
			assert_eq!(a.cmp(&b), Ordering::Less);
		}
		for i in 2021..9999 {
			let s = format_compact!("{}-12-01",i);
			let b = Date::from_str(&s).unwrap();
			assert_eq!(a.cmp(&b), Ordering::Less);
		}
	}

	fn variety(start: u16, end: u16) {
		for y in start..end {
			for m in 1..12 {
				for d in 1..31 {
					Date::from_str(&format_compact!("{y}{m}{d}")).unwrap();
					Date::from_str(&format_compact!("{m}{d}{y}")).unwrap();
					Date::from_str(&format_compact!("{d}{m}{y}")).unwrap();
					Date::from_str(&format_compact!("{y}-{m}-{d}")).unwrap();
					Date::from_str(&format_compact!("{m}-{d}-{y}")).unwrap();
					Date::from_str(&format_compact!("{d}-{m}-{y}")).unwrap();
				}
			}
		}
	}
	#[test]
	fn variety_1() { variety(1000, 2000); }
	#[test]
	fn variety_2() { variety(2000, 3000); }
	#[test]
	fn variety_3() { variety(3000, 4000); }
	#[test]
	fn variety_4() { variety(4000, 5000); }
	#[test]
	fn variety_5() { variety(5000, 6000); }
	#[test]
	fn variety_6() { variety(6000, 7000); }
	#[test]
	fn variety_7() { variety(7000, 8000); }
	#[test]
	fn variety_8() { variety(8000, 9000); }
	#[test]
	fn variety_9() { variety(9000, 10_000); }

	#[test]
	fn year() {
		for i in 1000..10_000 {
			assert!(Date::from_str(&format_compact!("{i}")).unwrap() == (i, 0, 0));
		}
	}

	#[test]
	fn invalid_years() {
		assert_eq!(Date::from_str_silent("0"),    Date::unknown());
		assert_eq!(Date::from_str_silent("100"),  Date::unknown());
		assert_eq!(Date::from_str_silent("010"),  Date::unknown());
		assert_eq!(Date::from_str_silent("0010"), Date::unknown());
		assert_eq!(Date::from_str_silent("0100"), Date::unknown());
		assert_eq!(Date::from_str_silent("999"),  Date::unknown());
		assert_eq!(Date::from_str_silent("0999"), Date::unknown());
	}

	#[test]
	fn invalid_dates() {
		assert_eq!(Date::from_str_silent("12-25-0100"), Date::unknown());
		assert_eq!(Date::from_str_silent("01001225") ,  Date::unknown());
		assert_eq!(Date::from_str_silent("25-12-0100"), Date::unknown());
		assert_eq!(Date::from_str_silent("01000"),      Date::unknown());
		assert_eq!(Date::from_str_silent("010000"),     Date::unknown());
		assert_eq!(Date::from_str_silent("0100000"),    Date::unknown());
	}

	#[test]
	fn from_str_ymd() {
		assert_eq!(Date::from_str("2020-12-25").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("2020-12-25").unwrap(), EXPECTED_STR);
		assert_eq!(Date::from_str("2020 12 25").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("2020 12 25").unwrap(), EXPECTED_STR);
		assert_eq!(Date::from_str("20201225").unwrap(),   EXPECTED);
		assert_eq!(Date::from_str("20201225").unwrap(),   EXPECTED_STR);
		assert_eq!(Date::from_str("2020/12/25").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("2020/12/25").unwrap(), EXPECTED_STR);
		assert_eq!(Date::from_str("2020.12.25").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("2020.12.25").unwrap(), EXPECTED_STR);
		assert_eq!(Date::from_str("2020_12_25").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("2020_12_25").unwrap(), EXPECTED_STR);
	}

	#[test]
	fn from_str_mdy() {
		assert_eq!(Date::from_str("12-25-2020").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("12-25-2020").unwrap(), EXPECTED_STR);
		assert_eq!(Date::from_str("12 25 2020").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("12 25 2020").unwrap(), EXPECTED_STR);
		assert_eq!(Date::from_str("12252020").unwrap()  , EXPECTED);
		assert_eq!(Date::from_str("12252020").unwrap()  , EXPECTED_STR);
		assert_eq!(Date::from_str("12/25/2020").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("12/25/2020").unwrap(), EXPECTED_STR);
		assert_eq!(Date::from_str("12.25.2020").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("12.25.2020").unwrap(), EXPECTED_STR);
		assert_eq!(Date::from_str("12_25_2020").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("12_25_2020").unwrap(), EXPECTED_STR);
	}

	#[test]
	fn from_str_dmy() {
		assert_eq!(Date::from_str("25-12-2020").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("25 12 2020").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("25122020").unwrap()  , EXPECTED);
		assert_eq!(Date::from_str("25/12/2020").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("25.12.2020").unwrap(), EXPECTED);
		assert_eq!(Date::from_str("25_12_2020").unwrap(), EXPECTED);
	}
}
