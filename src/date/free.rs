//---------------------------------------------------------------------------------------------------- Functions.
#[inline(always)]
pub(super) const fn ok_year(y: u16) -> bool {
	y >= 1000 && y <= 9999
}

#[inline(always)]
pub(super) const fn ok_month(m: u8) -> bool {
	m >= 1 && m <= 12
}

#[inline(always)]
pub(super) const fn ok_day(d: u8) -> bool {
	d >= 1 && d <= 31
}

#[inline(always)]
pub(super) const fn ok(y:u16, m: u8, d: u8) -> bool {
	ok_year(y) && ok_month(m) && ok_day(d)
}