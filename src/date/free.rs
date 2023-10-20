//---------------------------------------------------------------------------------------------------- Ok
#[inline(always)]
/// If `year` is in-between `1000..=9999`
///
/// ```rust
/// # use readable::date::*;
/// assert!(ok_year(1000));
/// assert!(ok_year(9999));
/// assert!(!ok_year(999));
/// assert!(!ok_year(10000));
/// ```
pub const fn ok_year(year: u16) -> bool {
	year >= 1000 && year <= 9999
}

#[inline(always)]
/// If `month` is in-between `1..=12`
///
/// ```rust
/// # use readable::date::*;
/// assert!(ok_month(1));
/// assert!(ok_month(12));
/// assert!(!ok_month(0));
/// assert!(!ok_month(13));
/// ```
pub const fn ok_month(month: u8) -> bool {
	month >= 1 && month <= 12
}

#[inline(always)]
/// If `day` is in-between `1..=31`
///
/// ```rust
/// # use readable::date::*;
/// assert!(ok_day(1));
/// assert!(ok_day(31));
/// assert!(!ok_day(0));
/// assert!(!ok_day(32));
/// ```
pub const fn ok_day(day: u8) -> bool {
	day >= 1 && day <= 31
}

#[inline(always)]
/// If `ok_year`, `ok_month`, and `ok_day` are all okay
///
/// This returns `true` if:
/// - `year` is in-between `1000..=9999`
/// - `month` is in-between `1..=12`
/// - `day` is in-between `1..=31`
///
/// else if returns `false`.
///
/// ```rust
/// # use readable::date::*;
/// assert!(ok(2020, 12, 31));
/// assert!(ok(1000, 1, 1));
/// assert!(!ok(0, 0, 0));
/// assert!(!ok(0, 12, 31));
/// ```
pub const fn ok(year: u16, month: u8, day: u8) -> bool {
	ok_year(year) && ok_month(month) && ok_day(day)
}

