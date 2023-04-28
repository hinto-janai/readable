//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

use crate::macros::*;
use crate::constants::*;
use compact_str::{format_compact,CompactString};
use std::num::TryFromIntError;
use regex::Regex;

//---------------------------------------------------------------------------------------------------- Regexes
lazy_static::lazy_static! {
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
	pub(crate) static ref NUM: Regex = Regex::new(r"^(\d{4}|[0-9][0-9][0-9][0-9][0-9]+)$").unwrap();

	// First `4` characters are a valid year.
	pub(crate) static ref YEAR: Regex = Regex::new(r"^\d{4}.*$").unwrap();

	// Number only - `YearMonthDay`
	pub(crate) static ref YM_NUM:    Regex = Regex::new(r"^[1-9]\d{3}[1-9]$").unwrap();
	pub(crate) static ref YMM_NUM:   Regex = Regex::new(r"^[1-9]\d{3}([0][1-9]|1[012])$").unwrap();
	pub(crate) static ref YMD_NUM:   Regex = Regex::new(r"^[1-9]\d{3}[1-9][1-9]$").unwrap();
	pub(crate) static ref YMMD_NUM:  Regex = Regex::new(r"^[1-9]\d{3}(0[1-9]|1[012])[1-9]$").unwrap();
	pub(crate) static ref YMDD_NUM:  Regex = Regex::new(r"^[1-9]\d{3}[1-9](0[1-9]|[12][0-9]|30|31)$").unwrap();
	pub(crate) static ref YMMDD_NUM: Regex = Regex::new(r"^[1-9]\d{3}(0[1-9]|1[012])(0[1-9]|[12][0-9]|30|31)$").unwrap();

	// Number only - `MonthDayYear`
	pub(crate) static ref MY_NUM:    Regex = Regex::new(r"^[1-9]\d{4}$").unwrap();
	pub(crate) static ref MMY_NUM:   Regex = Regex::new(r"^([0][1-9]|1[012])\d{4}$").unwrap();
	pub(crate) static ref MDY_NUM:   Regex = Regex::new(r"^[1-9][1-9]\d{4}$").unwrap();
	pub(crate) static ref MMDY_NUM:  Regex = Regex::new(r"^(0[1-9]|1[012])[1-9]\d{4}$").unwrap();
	pub(crate) static ref MDDY_NUM:  Regex = Regex::new(r"^[1-9](0[1-9]|[12][0-9]|30|31)\d{4}$").unwrap();
	pub(crate) static ref MMDDY_NUM: Regex = Regex::new(r"^(0[1-9]|1[012])(0[1-9]|[12][0-9]|30|31)\d{4}$").unwrap();

	// Number only - `DayMonthYear`
	pub(crate) static ref DMY_NUM:   Regex = Regex::new(r"^[1-9][1-9]\d{4}$").unwrap();
	pub(crate) static ref DDMY_NUM:  Regex = Regex::new(r"^(0[1-9]|[12][0-9]|3[01])[1-9]\d{4}$").unwrap();
	pub(crate) static ref DMMY_NUM:  Regex = Regex::new(r"^[1-9](0[1-9]|1[012])\d{4}$").unwrap();
	pub(crate) static ref DDMMY_NUM: Regex = Regex::new(r"^(0[1-9]|[12][0-9]|30|31)(0[1-9]|1[012])\d{4}$").unwrap();

	// Separated - `YEAR MONTH DAY`
	pub(crate) static ref YM:    Regex = Regex::new(r"^[1-9]\d{3}\D[1-9]$").unwrap();
	pub(crate) static ref YMM:   Regex = Regex::new(r"^[1-9]\d{3}\D([0][1-9]|1[012])$").unwrap();
	pub(crate) static ref YMD:   Regex = Regex::new(r"^[1-9]\d{3}\D[1-9]\D[1-9]$").unwrap();
	pub(crate) static ref YMMD:  Regex = Regex::new(r"^[1-9]\d{3}\D(0[1-9]|1[012])\D[1-9]$").unwrap();
	pub(crate) static ref YMDD:  Regex = Regex::new(r"^[1-9]\d{3}\D[1-9]\D(0[1-9]|[12][0-9]|30|31)$").unwrap();
	pub(crate) static ref YMMDD: Regex = Regex::new(r"^[1-9]\d{3}\D(0[1-9]|1[012])\D(0[1-9]|[12][0-9]|30|31)$").unwrap();

	// Separated - `MONTH DAY YEAR`
	pub(crate) static ref MY:    Regex = Regex::new(r"^[1-9]\D\d{4}$").unwrap();
	pub(crate) static ref MMY:   Regex = Regex::new(r"^([0][1-9]|1[012])\D\d{4}$").unwrap();
	pub(crate) static ref MDY:   Regex = Regex::new(r"^[1-9]\D[1-9]\D\d{4}$").unwrap();
	pub(crate) static ref MMDY:  Regex = Regex::new(r"^(0[1-9]|1[012])\D[1-9]\D\d{4}$").unwrap();
	pub(crate) static ref MDDY:  Regex = Regex::new(r"^[1-9]\D(0[1-9]|[12][0-9]|30|31)\D\d{4}$").unwrap();
	pub(crate) static ref MMDDY: Regex = Regex::new(r"^(0[1-9]|1[012])\D(0[1-9]|[12][0-9]|30|31)\D\d{4}$").unwrap();

	// Separated - `DAY MONTH YEAR`
	pub(crate) static ref DMY:   Regex = Regex::new(r"^[1-9]\D[1-9]\D\d{4}$").unwrap();
	pub(crate) static ref DDMY:  Regex = Regex::new(r"^(0[1-9]|[12][0-9]|3[01])\D[1-9]\D\d{4}$").unwrap();
	pub(crate) static ref DMMY:  Regex = Regex::new(r"^[1-9]\D(0[1-9]|1[012])\D\d{4}$").unwrap();
	pub(crate) static ref DDMMY: Regex = Regex::new(r"^(0[1-9]|[12][0-9]|30|31)\D(0[1-9]|1[012])\D\d{4}$").unwrap();

}

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
/// A _recent_ date that is human readable date in `YEAR-MONTH-DAY` format
///
/// The inner "integer" type is a tuple of: `(u16, u8, u8)` representing the `(Year, Month, Day)`
///
/// Any value being `0` means it is invalid:
/// ```rust
/// # use readable::Date;
/// let a = Date::from_str("2020-12").unwrap();
///
/// assert!(a == (2020, 12, 0));
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
/// assert!(d1 == "2020-12-01");
/// assert!(d2 == "2020-12");
/// assert!(d3 == "2020");
///```
///
/// ## String parsing and format
/// To parse an abitrary string into a [`Date`], use: [`Date::from_str`].
///
/// You can input a string that is _just_ numbers, or separated by a single byte, e.g:
///
/// ```rust
/// # use readable::Date;
/// let dates = [
///     Date::from_str("2020-12-31").unwrap(),
///     Date::from_str("2020/12/31").unwrap(),
///     Date::from_str("2020.12.31").unwrap(),
///     Date::from_str("2020_12_31").unwrap(),
///     Date::from_str("2020 12 31").unwrap(),
///     Date::from_str("20201231").unwrap(),
/// ];
///
/// for date in dates {
///     assert!(date == (2020, 12, 31));
///     assert!(date == "2020-12-31");
/// }
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
/// assert!(Date::from_str(ambiguous).unwrap() == "1111-11-11");
///
/// // This could be:
/// //   - MDY
/// //   - DMY
/// let ambiguous = "12-12-1111";
/// // We prioritize MDY over DMY.
/// assert!(Date::from_str(ambiguous).unwrap() == "1111-12-12");
///
/// // This cannot be MDY, so it must be DMY.
/// let dmy = "13-11-1111";
/// assert!(Date::from_str(dmy).unwrap() == "1111-11-13");
/// ```
///
/// Some errors can occur during string parsing:
///
/// - Year is less than `1000`, a signed number, or greater than [`u16::MAX`]
/// - Month is not in-between `1-12`
/// - Day is not in-between `1-31`
///
/// Good Example:
/// ```rust
/// # use readable::Date;
/// let d1 = Date::from_str("2020-12-31").unwrap();
/// let d2 = Date::from_str("11_30_2012").unwrap();
/// let d3 = Date::from_str("1980.5").unwrap();
///
/// assert!(d1 == "2020-12-31");
/// assert!(d2 == "2012-11-30");
/// assert!(d3 == "1980-05");
/// ```
///
/// Bad Example:
/// ```rust,should_panic
/// # use readable::Date;
/// let d1 = Date::from_str("10000-57-99").unwrap();
/// let d2 = Date::from_str("2022.31.31").unwrap();
/// let d3 = Date::from_str("-1231").unwrap();
/// ```
///
/// ## Inlining
/// If the feature flag `inline_date` is enabled, inputs that are in `YYYY-MM-DD` format
/// that range from year `1900-2100` will cause [`Date::from_str`] to match on inlined static bytes.
///
/// **Warning:** This feature is disabled by default. While it increases speed,
/// it also _heavily_ increases build time and binary size.
///
/// ## Cloning
/// [`Copy`] is available.
///
/// The actual string used internally is not a [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
/// but a 10 byte array buffer, literally: `[u8; 10]`.
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
/// assert!(b == "2014-04-22");
///
/// // We can still use 'a'
/// assert!(a == "2014-04-22");
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Date((u16, u8, u8), Buffer);

impl Date {
	impl_common!((u16, u8, u8));
	impl_const!();
	impl_buffer!(MAX_BUF_LEN, UNKNOWN_DATE_BUFFER, UNKNOWN_DATE.len());

	// INVARIANT:
	// The inputs _must_ be correct.
	// Private functions for construction.
	#[inline]
	fn priv_y(y: u16) -> Self {
		let s = format_compact!("{y}");
		Self((y, 0, 0), Buffer::from_4_unchecked(s.as_bytes()))
	}
	#[inline]
	fn priv_ym(y: u16, m: u8) -> Self {
		let s = format_compact!("{y}-{m:0>2}");
		Self((y, m, 0), Buffer::from_unchecked(s.as_bytes()))
	}
	#[inline]
	fn priv_ymd(y: u16, m: u8, d: u8) -> Self {
		let s = format_compact!("{y}-{m:0>2}-{d:0>2}");
		Self((y, m, d), Buffer::from_unchecked(s.as_bytes()))
	}

	// Common functions.
	#[inline]
	/// Returns a [`Self`] with the date values set to `(0, 0, 0)`
	///
	/// The [`String`] is set to [`UNKNOWN_DATE`].
	pub const fn unknown() -> Self {
		Self((0, 0, 0), Buffer::unknown())
	}

	#[inline]
	/// Same as [`Self::unknown`]
	pub const fn zero() -> Self {
		Self((0, 0, 0), Buffer::unknown())
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
			Ok(Self::priv_y(year))
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
			Ok(Self::priv_ym(year, month))
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
			Ok(Self::priv_ymd(year, month, day))
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
			Self::priv_y(year)
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
			Self::priv_ym(year, month)
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
			Self::priv_ymd(year, month, day)
		} else {
			Self::unknown()
		}
	}

	/// Parse arbitrary strings for a date.
	///
	/// If the complete date cannot be parsed, this function will
	/// attempt to extract at least the year, e.g:
	/// ```rust
	/// # use readable::Date;
	/// let a = Date::from_str("2022-99-99").unwrap();
	/// let b = Date::from_str("2022-03-32").unwrap();
	/// let c = Date::from_str("2022-32-00").unwrap();
	/// let d = Date::from_str("2022-00-31").unwrap();
	///
	/// assert!(a == "2022");
	/// assert!(b == "2022");
	/// assert!(c == "2022");
	/// assert!(d == "2022");
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

	/// Same as [`Date::from_str`] but silently returns an [`UNKNOWN_DATE`]
	/// on error that isn't wrapped in a [`Result::Err`].
	pub fn from_str_silent(string: &str) -> Self {
		match Self::priv_from_str(string) {
			Ok(s)  => s,
			Err(s) => s,
		}
	}

	#[inline(always)]
	fn priv_from_str(string: &str) -> Result<Self, Self> {
		let len = string.len();

		// If feature enabled, match on all possible
		// `YYYY-MM-DD` strings between `1900-2100`.
		#[cfg(feature = "inline_date")]
		if len == 10 {
			if let Some(date) = readable_inlined_date::inlined(string.as_bytes()) {
				let (y, m, d, bytes) = date;
				return Ok(Self((y, m, d), Buffer::from_unchecked(&bytes)));
			}
		}

		// Return `YYYY`.
		if len == 4 {
			match string.parse::<u16>() {
				Ok(y) => return Ok(Self::priv_y(y)),
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
		if NUM.is_match(&string) {
			match len {
				// YM || MY
				5 => {
					if YM_NUM.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						let m = string[4..].parse::<u8>().unwrap();
						return Ok(Self::priv_ym(y, m));
					} else if MY_NUM.is_match(&string) {
						let m = string[..1].parse::<u8>().unwrap();
						let y = string[1..].parse::<u16>().unwrap();
						return Ok(Self::priv_ym(y, m));
					} else if YEAR.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						return Ok(Self::priv_y(y));
					}
				}

				// YMM || YMD || MDY || DMY
				6 => {
					if YMM_NUM.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						let m = string[4..].parse::<u8>().unwrap();
						return Ok(Self::priv_ym(y, m));
					} else if YMD_NUM.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						let m = string[4..5].parse::<u8>().unwrap();
						let d = string[5..].parse::<u8>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if MDY_NUM.is_match(&string) {
						let m = string[..1].parse::<u8>().unwrap();
						let d = string[1..2].parse::<u8>().unwrap();
						let y = string[2..].parse::<u16>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if DMY_NUM.is_match(&string) {
						let d = string[..1].parse::<u8>().unwrap();
						let m = string[1..2].parse::<u8>().unwrap();
						let y = string[2..].parse::<u16>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if YEAR.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						return Ok(Self::priv_y(y));
					}
				},

				// YMMD || YMDD || MMDY || MDDY || DMMY || DDMY
				7 => {
					if YMMD_NUM.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						let m = string[4..5].parse::<u8>().unwrap();
						let d = string[6..].parse::<u8>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if YMDD_NUM.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						let m = string[4..5].parse::<u8>().unwrap();
						let d = string[5..].parse::<u8>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if MMDY_NUM.is_match(&string) {
						let m = string[..2].parse::<u8>().unwrap();
						let d = string[2..3].parse::<u8>().unwrap();
						let y = string[3..].parse::<u16>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if MDDY_NUM.is_match(&string) {
						let m = string[..1].parse::<u8>().unwrap();
						let d = string[1..3].parse::<u8>().unwrap();
						let y = string[3..].parse::<u16>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if DMMY_NUM.is_match(&string) {
						let d = string[..1].parse::<u8>().unwrap();
						let m = string[1..3].parse::<u8>().unwrap();
						let y = string[3..].parse::<u16>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if DDMY_NUM.is_match(&string) {
						let d = string[..2].parse::<u8>().unwrap();
						let m = string[2..3].parse::<u8>().unwrap();
						let y = string[3..].parse::<u16>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if YEAR.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						return Ok(Self::priv_y(y));
					}
				},

				// YMMDD || MMDDY || DDMMY
				8 => {
					if YMMDD_NUM.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						let m = string[4..6].parse::<u8>().unwrap();
						let d = string[6..].parse::<u8>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if MMDDY_NUM.is_match(&string) {
						let m = string[..2].parse::<u8>().unwrap();
						let d = string[2..4].parse::<u8>().unwrap();
						let y = string[4..].parse::<u16>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if DDMMY_NUM.is_match(&string) {
						let d = string[..2].parse::<u8>().unwrap();
						let m = string[2..4].parse::<u8>().unwrap();
						let y = string[4..].parse::<u16>().unwrap();
						return Ok(Self::priv_ymd(y, m, d));
					} else if YEAR.is_match(&string) {
						let y = string[..4].parse::<u16>().unwrap();
						return Ok(Self::priv_y(y));
					}
				},

				_ => return Err(Self::unknown()),
			}
		}

		// If input is separated...
		match len {
			// Y.M || M.Y
			6 => {
				if YM.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					let m = string[5..].parse::<u8>().unwrap();
					return Ok(Self::priv_ym(y, m));
				} else if MY.is_match(&string) {
					let m = string[..1].parse::<u8>().unwrap();
					let y = string[2..].parse::<u16>().unwrap();
					return Ok(Self::priv_ym(y, m));
				} else if YEAR.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					return Ok(Self::priv_y(y));
				}
			},

			// Y.MM || MM.Y
			7 => {
				if YMM.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					let m = string[5..].parse::<u8>().unwrap();
					return Ok(Self::priv_ym(y, m));
				} else if MMY.is_match(&string) {
					let m = string[..2].parse::<u8>().unwrap();
					let y = string[3..].parse::<u16>().unwrap();
					return Ok(Self::priv_ym(y, m));
				} else if YEAR.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					return Ok(Self::priv_y(y));
				}
			},

			// Y.M.D || M.D.Y || D.M.Y
			8 => {
				if YMD.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					let m = string[5..6].parse::<u8>().unwrap();
					let d = string[7..].parse::<u8>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if MDY.is_match(&string) {
					let m = string[..1].parse::<u8>().unwrap();
					let d = string[2..3].parse::<u8>().unwrap();
					let y = string[4..].parse::<u16>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if DMY.is_match(&string) {
					let d = string[..1].parse::<u8>().unwrap();
					let m = string[2..3].parse::<u8>().unwrap();
					let y = string[4..].parse::<u16>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if YEAR.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					return Ok(Self::priv_y(y));
				}
			},

			// Y.MM.D || Y.M.DD || MM.D.Y || M.DD.Y || D.MM.Y || DD.M.Y
			9 => {
				if YMMD.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					let m = string[5..7].parse::<u8>().unwrap();
					return Ok(Self::priv_ym(y, m));
				} else if YMDD.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					let m = string[5..6].parse::<u8>().unwrap();
					let d = string[7..].parse::<u8>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if MMDY.is_match(&string) {
					let m = string[..2].parse::<u8>().unwrap();
					let d = string[3..4].parse::<u8>().unwrap();
					let y = string[5..].parse::<u16>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if MDDY.is_match(&string) {
					let m = string[..1].parse::<u8>().unwrap();
					let d = string[2..4].parse::<u8>().unwrap();
					let y = string[5..].parse::<u16>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if DMMY.is_match(&string) {
					let d = string[..1].parse::<u8>().unwrap();
					let m = string[2..4].parse::<u8>().unwrap();
					let y = string[5..].parse::<u16>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if DDMY.is_match(&string) {
					let d = string[..2].parse::<u8>().unwrap();
					let m = string[3..4].parse::<u8>().unwrap();
					let y = string[5..].parse::<u16>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if YEAR.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					return Ok(Self::priv_y(y));
				}
			},

			// Y.MM.DD || MM.DD.Y || DD.MM.Y
			10 => {
				if YMMDD.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					let m = string[5..7].parse::<u8>().unwrap();
					let d = string[8..].parse::<u8>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if MMDDY.is_match(&string) {
					let m = string[..2].parse::<u8>().unwrap();
					let d = string[3..5].parse::<u8>().unwrap();
					let y = string[6..].parse::<u16>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if DDMMY.is_match(&string) {
					let d = string[..2].parse::<u8>().unwrap();
					let m = string[3..5].parse::<u8>().unwrap();
					let y = string[6..].parse::<u16>().unwrap();
					return Ok(Self::priv_ymd(y, m, d));
				} else if YEAR.is_match(&string) {
					let y = string[..4].parse::<u16>().unwrap();
					return Ok(Self::priv_y(y));
				}
			},

			_ => return Err(Date::unknown()),
		}

		// Give up.
		Err(Date::unknown())
	}
}

impl_traits!(Date, (u16, u8, u8));

//---------------------------------------------------------------------------------------------------- Date Buffer.
// "9999-12-31".len() == 10
const MAX_BUF_LEN: usize = 10;

buffer!(MAX_BUF_LEN, UNKNOWN_DATE_BUFFER, UNKNOWN_DATE.len());

impl Buffer {
	#[inline]
	// INVARIANT:
	// Assumes input is `4` bytes.
	fn from_4_unchecked(byte: &[u8]) -> Self {
		let mut buf = [0_u8; 10];
		buf[..4].copy_from_slice(&byte[..4]);

		Self {
			buf,
			len: 4,
		}
	}

	#[inline]
	// INVARIANT:
	// Assumes input is `5-10` bytes.
	fn from_unchecked(byte: &[u8]) -> Self {
		let len = byte.len();

		let mut buf = [0_u8; 10];
		match len {
			5  => buf[..5].copy_from_slice(&byte[..5]),
			6  => buf[..6].copy_from_slice(&byte[..6]),
			7  => buf[..7].copy_from_slice(&byte[..7]),
			8  => buf[..8].copy_from_slice(&byte[..8]),
			9  => buf[..9].copy_from_slice(&byte[..9]),
			10 => buf[..10].copy_from_slice(&byte[..10]),
			_  => unreachable!(),
		};

		Self {
			buf,
			len,
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
		assert!(a.cmp(&b) == Ordering::Equal);
		assert!(a.cmp(&c) == Ordering::Greater);
		assert!(a.cmp(&d) == Ordering::Greater);

		for i in 1..12 {
			let s = format_compact!("2020-{:0>2}-01", i);
			let b = Date::from_str(&s).unwrap();
			assert!(a.cmp(&b) == Ordering::Greater);
		}
		for i in 2..32 {
			let s = format_compact!("2020-12-{:0>2}", i);
			let b = Date::from_str(&s).unwrap();
			assert!(a.cmp(&b) == Ordering::Less);
		}
		for i in 2021..9999 {
			let s = format_compact!("{}-12-01", i);
			let b = Date::from_str(&s).unwrap();
			assert!(a.cmp(&b) == Ordering::Less);
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
	fn from_str_ymd() {
		assert!(Date::from_str("2020-12-25").unwrap() == EXPECTED);
		assert!(Date::from_str("2020-12-25").unwrap() == EXPECTED_STR);
		assert!(Date::from_str("2020 12 25").unwrap() == EXPECTED);
		assert!(Date::from_str("2020 12 25").unwrap() == EXPECTED_STR);
		assert!(Date::from_str("20201225").unwrap()   == EXPECTED);
		assert!(Date::from_str("20201225").unwrap()   == EXPECTED_STR);
		assert!(Date::from_str("2020/12/25").unwrap() == EXPECTED);
		assert!(Date::from_str("2020/12/25").unwrap() == EXPECTED_STR);
		assert!(Date::from_str("2020.12.25").unwrap() == EXPECTED);
		assert!(Date::from_str("2020.12.25").unwrap() == EXPECTED_STR);
		assert!(Date::from_str("2020_12_25").unwrap() == EXPECTED);
		assert!(Date::from_str("2020_12_25").unwrap() == EXPECTED_STR);
	}

	#[test]
	fn from_str_mdy() {
		assert!(Date::from_str("12-25-2020").unwrap() == EXPECTED);
		assert!(Date::from_str("12-25-2020").unwrap() == EXPECTED_STR);
		assert!(Date::from_str("12 25 2020").unwrap() == EXPECTED);
		assert!(Date::from_str("12 25 2020").unwrap() == EXPECTED_STR);
		assert!(Date::from_str("12252020").unwrap()   == EXPECTED);
		assert!(Date::from_str("12252020").unwrap()   == EXPECTED_STR);
		assert!(Date::from_str("12/25/2020").unwrap() == EXPECTED);
		assert!(Date::from_str("12/25/2020").unwrap() == EXPECTED_STR);
		assert!(Date::from_str("12.25.2020").unwrap() == EXPECTED);
		assert!(Date::from_str("12.25.2020").unwrap() == EXPECTED_STR);
		assert!(Date::from_str("12_25_2020").unwrap() == EXPECTED);
		assert!(Date::from_str("12_25_2020").unwrap() == EXPECTED_STR);
	}

	#[test]
	fn from_str_dmy() {
		assert!(Date::from_str("25-12-2020").unwrap() == EXPECTED);
		assert!(Date::from_str("25 12 2020").unwrap() == EXPECTED);
		assert!(Date::from_str("25122020").unwrap()   == EXPECTED);
		assert!(Date::from_str("25/12/2020").unwrap() == EXPECTED);
		assert!(Date::from_str("25.12.2020").unwrap() == EXPECTED);
		assert!(Date::from_str("25_12_2020").unwrap() == EXPECTED);
	}

	//-------------------------------------------------------------------------------- Regex tests.
	//-------------------------------------------------------------------------------- `YearMonthDay`
	const SEPARATORS: [char; 16] = ['-', ' ', '_', '.', '/', '\\', '+', '^', '@', '|', ',', ':', ';', '\'', '"', 'x'];

	#[test]
	#[ignore]
	fn regex_num() {
		for y in 0..1000 {
			assert!(!NUM.is_match(&format_compact!("{y}")));
			assert!(!NUM.is_match(&format_compact!("{y} ")));
			assert!(!NUM.is_match(&format_compact!("{y}  ")));
		}
		for y in 1000..1_000_000 {
			assert!(!NUM.is_match(&format_compact!(" {y}")));
			assert!(!NUM.is_match(&format_compact!("{y} ")));
			assert!(!NUM.is_match(&format_compact!(" {y} ")));
			assert!(NUM.is_match(&format_compact!("{y}")));
		}
	}

	#[test]
	#[ignore]
	fn regex_year() {
		for y in 0..1000 {
			assert!(!YEAR.is_match(&format_compact!("{y}")));
			assert!(!YEAR.is_match(&format_compact!("{y} ")));
			assert!(!YEAR.is_match(&format_compact!("{y}  ")));
		}
		for y in 1000..10_000 {
			assert!(!YEAR.is_match(&format_compact!(" {y}")));
			assert!(YEAR.is_match(&format_compact!("{y}")));
			assert!(YEAR.is_match(&format_compact!("{y} ")));
			assert!(YEAR.is_match(&format_compact!("{y}  ")));
		}
	}


	#[test]
	#[ignore]
	fn regex_ym_num() {
		for y in 0..1000 {
			for m in 1..10 {
				assert!(!YM_NUM.is_match(&format_compact!("{y}{m}")));
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				assert!(YM_NUM.is_match(&format_compact!("{y}{m}")));
			}
		}
		for m in 1..10 {
			assert!(!YM_NUM.is_match(&format_compact!("10000{m}")));
		}
	}

	#[test]
	#[ignore]
	fn regex_ymm_num() {
		for y in 0..1000 {
			for m in 1..10 {
				assert!(!YMM_NUM.is_match(&format_compact!("{y}{m:0>2}")));
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				assert!(YMM_NUM.is_match(&format_compact!("{y}{m:0>2}")));
			}
		}
		for m in 1..10 {
			assert!(!YMM_NUM.is_match(&format_compact!("10000{m:0>2}")));
		}
	}

	#[test]
	#[ignore]
	fn regex_ymd_num() {
		for y in 0..1000 {
			for m in 1..10 {
				for d in 1..10 {
					assert!(!YMD_NUM.is_match(&format_compact!("{y}{m}{d}")));
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				for d in 1..10 {
					assert!(YMD_NUM.is_match(&format_compact!("{y}{m}{d}")));
				}
			}
		}
		for m in 1..10 {
			for d in 1..10 {
				assert!(!YMD_NUM.is_match(&format_compact!("10000{m}{d}")));
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ymmd_num() {
		for y in 1000..10_000 {
			for m in 1..13 {
				for d in 1..10 {
					assert!(YMMD_NUM.is_match(&format_compact!("{y}{m:0>2}{d}")));
				}
				for d in 10..32 {
					assert!(!YMMD_NUM.is_match(&format_compact!("{y}{m:0>2}{d}")));
				}
			}
		}
		for m in 1..13 {
			for d in 1..32 {
				assert!(!YMMD_NUM.is_match(&format_compact!("10000{m:0>2}{d}")));
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ymdd_num() {
		for y in 1000..10_000 {
			for m in 1..10 {
				for d in 1..32 {
					assert!(YMDD_NUM.is_match(&format_compact!("{y}{m}{d:0>2}")));
				}
				for d in 32..99 {
					assert!(!YMDD_NUM.is_match(&format_compact!("{y}{m}{d:0>2}")));
				}
			}
		}
		for m in 1..13 {
			for d in 1..32 {
				assert!(!YMDD_NUM.is_match(&format_compact!("10000{m}{d:0>2}")));
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ymmdd_num() {
		for y in 1000..10_000 {
			for m in 1..13 {
				for d in 1..32 {
					assert!(YMMDD_NUM.is_match(&format_compact!("{y}{m:0>2}{d:0>2}")));
				}
				for d in 32..99 {
					assert!(!YMMDD_NUM.is_match(&format_compact!("{y}{m:0>2}{d:0>2}")));
				}
			}
		}
		for m in 0..99 {
			for d in 0..99 {
				assert!(!YMMDD_NUM.is_match(&format_compact!("10000{m:0>2}{d:0>2}")));
			}
		}
	}

	//-------------------------------------------------------------------------------- `YEAR MONTH DAY`
	#[test]
	#[ignore]
	fn regex_ym() {
		assert!(YM.is_match(&format_compact!("2022-1")));
		assert!(!YM.is_match(&format_compact!("202201")));
		for y in 0..1000 {
			for m in 1..10 {
				for s in SEPARATORS {
					assert!(!YM.is_match(&format_compact!("{y}{s}{m}")));
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				for s in SEPARATORS {
					assert!(YM.is_match(&format_compact!("{y}{s}{m}")));
				}
			}
		}
		for m in 1..10 {
			for s in SEPARATORS {
				assert!(!YM.is_match(&format_compact!("10000{s}{m}")));
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ymm() {
		assert!(YMM.is_match(&format_compact!("2022-12")));
		assert!(!YMM.is_match(&format_compact!("2022012")));
		for y in 0..1000 {
			for m in 1..13 {
				for s in SEPARATORS {
					assert!(!YMM.is_match(&format_compact!("{y}{s}{m:0>2}")));
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..13 {
				for s in SEPARATORS {
					assert!(YMM.is_match(&format_compact!("{y}{s}{m:0>2}")));
				}
			}
		}
		for m in 1..13 {
			for s in SEPARATORS {
				assert!(!YMM.is_match(&format_compact!("10000{s}{m:0>2}")));
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ymd() {
		assert!(YMD.is_match(&format_compact!("2022-1-1")));
		assert!(!YMD.is_match(&format_compact!("20220101")));
		for y in 0..1000 {
			for m in 1..10 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(!YMD.is_match(&format_compact!("{y}{s}{m}{s}{d}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(YMD.is_match(&format_compact!("{y}{s}{m}{s}{d}")));
					}
				}
			}
		}
		for m in 1..10 {
			for d in 1..10 {
				for s in SEPARATORS {
					assert!(!YMD.is_match(&format_compact!("10000{s}{m}{s}{d}")));
				}
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ymmd() {
		assert!(YMMD.is_match(&format_compact!("2022-12-1")));
		assert!(!YMMD.is_match(&format_compact!("202201201")));
		for y in 0..1000 {
			for m in 1..13 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(!YMMD.is_match(&format_compact!("{y}{s}{m:0>2}{s}{d}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..13 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(YMMD.is_match(&format_compact!("{y}{s}{m:0>2}{s}{d}")));
					}
				}
			}
		}
		for m in 1..13 {
			for d in 1..10 {
				for s in SEPARATORS {
					assert!(!YMMD.is_match(&format_compact!("10000{s}{m:0>2}{s}{d}")));
				}
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ymdd() {
		assert!(YMDD.is_match(&format_compact!("2022-1-31")));
		assert!(!YMDD.is_match(&format_compact!("2022-1031")));
		assert!(!YMDD.is_match(&format_compact!("202201031")));
		for y in 0..1000 {
			for m in 1..10 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(!YMDD.is_match(&format_compact!("{y}{s}{m}{s}{d:0>2}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(YMDD.is_match(&format_compact!("{y}{s}{m}{s}{d:0>2}")));
					}
				}
			}
		}
		for m in 1..10 {
			for d in 1..32 {
				for s in SEPARATORS {
					assert!(!YMDD.is_match(&format_compact!("10000{s}{m}{s}{d:0>2}")));
				}
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ymmdd() {
		assert!(YMMDD.is_match(&format_compact!("2022-12-31")));
		assert!(!YMMDD.is_match(&format_compact!("2022012-31")));
		assert!(!YMMDD.is_match(&format_compact!("2022012031")));
		for y in 0..1000 {
			for m in 1..13 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(!YMMDD.is_match(&format_compact!("{y}{s}{m:0>2}{s}{d:0>2}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..13 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(YMMDD.is_match(&format_compact!("{y}{s}{m:0>2}{s}{d:0>2}")));
					}
				}
			}
		}
		for m in 1..13 {
			for d in 1..32 {
				for s in SEPARATORS {
					assert!(!YMMDD.is_match(&format_compact!("10000{s}{m:0>2}{s}{d:0>2}")));
				}
			}
		}
	}

	//-------------------------------------------------------------------------------- `MONTH DAY YEAR`
	#[test]
	#[ignore]
	fn regex_my() {
		assert!(MY.is_match("1.2020"));
		assert!(!MY.is_match("1202020"));
		assert!(!MY.is_match("12.2020"));
		assert!(!MY.is_match("13.2020"));
		for y in 0..1000 {
			for m in 1..10 {
				for s in SEPARATORS {
					assert!(!MY.is_match(&format_compact!("{m}{s}{y}")));
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				for s in SEPARATORS {
					assert!(MY.is_match(&format_compact!("{m}{s}{y}")));
				}
			}
		}
		for m in 1..10 {
			for s in SEPARATORS {
				assert!(!MY.is_match(&format_compact!("{m}{s}10000")));
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_mmy() {
		assert!(MMY.is_match("01.2020"));
		assert!(MMY.is_match("12.2020"));
		assert!(!MMY.is_match("13.2020"));
		assert!(!MMY.is_match("1202020"));
		for y in 0..1000 {
			for m in 1..13 {
				for s in SEPARATORS {
					assert!(!MMY.is_match(&format_compact!("{m:0>2}{s}{y}")));
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..13 {
				for s in SEPARATORS {
					assert!(MMY.is_match(&format_compact!("{m:0>2}{s}{y}")));
				}
			}
		}
		for m in 1..13 {
			for s in SEPARATORS {
				assert!(!MMY.is_match(&format_compact!("{m:0>2}{s}10000")));
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_mdy() {
		assert!(MDY.is_match("9.9.2020"));
		assert!(!MDY.is_match("0.0.2020"));
		assert!(!MDY.is_match("12012.2020"));
		assert!(!MDY.is_match("13.12.2020"));
		for y in 0..1000 {
			for m in 1..10 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(!MDY.is_match(&format_compact!("{m}{s}{d}{s}{y}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(MDY.is_match(&format_compact!("{m}{s}{d}{s}{y}")));
					}
				}
			}
		}
		for m in 1..10 {
			for d in 1..10 {
				for s in SEPARATORS {
					assert!(!MDY.is_match(&format_compact!("{m}{s}{d}{s}10000")));
				}
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_mmdy() {
		assert!(MMDY.is_match("12.9.2020"));
		assert!(MMDY.is_match("01.9.2020"));
		assert!(!MMDY.is_match("00.1.2020"));
		assert!(!MMDY.is_match("13.12.2020"));
		assert!(!MMDY.is_match("12012.2020"));
		for y in 0..1000 {
			for m in 1..13 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(!MMDY.is_match(&format_compact!("{m:0>2}{s}{d}{s}{y}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..13 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(MMDY.is_match(&format_compact!("{m:0>2}{s}{d}{s}{y}")));
					}
				}
			}
		}
		for m in 1..13 {
			for d in 1..10 {
				for s in SEPARATORS {
					assert!(!MMDY.is_match(&format_compact!("{m:0>2}{s}{d}{s}10000")));
				}
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_mddy() {
		assert!(MDDY.is_match("9.31.2020"));
		assert!(MDDY.is_match("9.01.2020"));
		assert!(!MDDY.is_match("9.3.2020"));
		assert!(!MDDY.is_match("9.32.2020"));
		assert!(!MDDY.is_match("903102020"));
		for y in 0..1000 {
			for m in 1..10 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(!MDDY.is_match(&format_compact!("{m}{s}{d:0>2}{s}{y}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(MDDY.is_match(&format_compact!("{m}{s}{d:0>2}{s}{y}")));
					}
				}
			}
		}
		for m in 1..10 {
			for d in 1..32 {
				for s in SEPARATORS {
					assert!(!MDDY.is_match(&format_compact!("{m}{s}{d:0>2}{s}10000")));
				}
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_mmddy() {
		assert!(MMDDY.is_match("12.31.2020"));
		assert!(MMDDY.is_match("01.01.2020"));
		assert!(!MMDDY.is_match("00.00.2020"));
		assert!(!MMDDY.is_match("12.32.2020"));
		assert!(!MMDDY.is_match("13.31.2020"));
		assert!(!MMDDY.is_match("1203102020"));
		for y in 0..1000 {
			for m in 1..13 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(!MMDDY.is_match(&format_compact!("{m:0>2}{s}{d:0>2}{s}{y}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..13 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(MMDDY.is_match(&format_compact!("{m:0>2}{s}{d:0>2}{s}{y}")));
					}
				}
			}
		}
		for m in 1..13 {
			for d in 1..32 {
				for s in SEPARATORS {
					assert!(!MMDDY.is_match(&format_compact!("{m:0>2}{s}{d:0>2}{s}10000")));
				}
			}
		}
	}

	//-------------------------------------------------------------------------------- `DAY MONTH YEAR`
	#[test]
	#[ignore]
	fn regex_dmy() {
		assert!(DMY.is_match("9.9.2020"));
		assert!(DMY.is_match("1.1.2020"));
		assert!(!DMY.is_match("0.0.2020"));
		assert!(!DMY.is_match("10.10.2020"));
		assert!(!DMY.is_match("32.13.2020"));
		assert!(!DMY.is_match("3101202020"));
		for y in 0..1000 {
			for m in 1..10 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(!DMY.is_match(&format_compact!("{d}{s}{m}{s}{y}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				for d in 1..10 {
					for s in SEPARATORS {
						assert!(DMY.is_match(&format_compact!("{d}{s}{m}{s}{y}")));
					}
				}
			}
		}
		for m in 1..10 {
			for d in 1..10 {
				for s in SEPARATORS {
					assert!(!DMY.is_match(&format_compact!("{d}{s}{m}{s}10000")));
				}
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ddmy() {
		assert!(DDMY.is_match("31.9.2020"));
		assert!(!DDMY.is_match("10.10.2020"));
		assert!(!DDMY.is_match("32.9.2020"));
		assert!(!DDMY.is_match("310902020"));
		for y in 0..1000 {
			for m in 1..10 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(!DDMY.is_match(&format_compact!("{d:0>2}{s}{m}{s}{y}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..10 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(DDMY.is_match(&format_compact!("{d:0>2}{s}{m}{s}{y}")));
					}
				}
			}
		}
		for m in 1..10 {
			for d in 1..32 {
				for s in SEPARATORS {
					assert!(!DDMY.is_match(&format_compact!("{d:0>2}{s}{m}{s}10000")));
				}
			}
		}
	}

	#[test]
	#[ignore]
	fn regex_ddmmy() {
		assert!(DDMMY.is_match("31.12.2020"));
		assert!(DDMMY.is_match("01.01.2020"));
		assert!(!DDMMY.is_match("10.13.2020"));
		assert!(!DDMMY.is_match("32.12.2020"));
		assert!(!DDMMY.is_match("00.00.2020"));
		assert!(!DDMMY.is_match("0000002020"));
		for y in 0..1000 {
			for m in 1..13 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(!DDMMY.is_match(&format_compact!("{d:0>2}{s}{m:0>2}{s}{y}")));
					}
				}
			}
		}
		for y in 1000..10_000 {
			for m in 1..13 {
				for d in 1..32 {
					for s in SEPARATORS {
						assert!(DDMMY.is_match(&format_compact!("{d:0>2}{s}{m:0>2}{s}{y}")));
					}
				}
			}
		}
		for m in 1..10 {
			for d in 1..32 {
				for s in SEPARATORS {
					assert!(!DDMMY.is_match(&format_compact!("{d:0>2}{s}{m:0>2}{s}10000")));
				}
			}
		}
	}
}
