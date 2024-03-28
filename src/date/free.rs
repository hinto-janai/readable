//---------------------------------------------------------------------------------------------------- Ok
#[inline]
/// If `year` is in-between `1000..=9999`
pub(crate) const fn ok_year(year: u16) -> bool {
    year >= 1000 && year <= 9999
}

#[inline]
/// If `month` is in-between `1..=12`
pub(crate) const fn ok_month(month: u8) -> bool {
    month >= 1 && month <= 12
}

#[inline]
/// If `day` is in-between `1..=31`
pub(crate) const fn ok_day(day: u8) -> bool {
    day >= 1 && day <= 31
}

#[inline]
/// If `ok_year`, `ok_month`, and `ok_day` are all okay
pub(crate) const fn ok(year: u16, month: u8, day: u8) -> bool {
    ok_year(year) && ok_month(month) && ok_day(day)
}

//---------------------------------------------------------------------------------------------------- Date
#[inline]
#[must_use]
/// Get the current system date in the system's timezone.
///
/// The returned value is `(year, month, day)`.
pub fn date() -> (i16, u8, u8) {
    use chrono::Datelike;
    let now = chrono::offset::Local::now().date_naive();
    (now.year() as i16, now.month() as u8, now.day() as u8)
}

#[inline]
#[must_use]
/// Get the current system date in UTC locale
///
/// The returned value is `(year, month, day)`.
pub fn date_utc() -> (i16, u8, u8) {
    let unix = i128::from(chrono::offset::Local::now().timestamp());
    nichi::Date::from_unix(unix).inner()
}
