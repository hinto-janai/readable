//! Inlined data for [`readable`](https://docs.rs/readable).
//!
//! Do not use this crate directly.

mod _1900;
mod _1910;
mod _1920;
mod _1930;
mod _1940;
mod _1950;
mod _1960;
mod _1970;
mod _1980;
mod _1990;
mod _2000;
mod _2010;
mod _2020;
mod _2030;
mod _2040;
mod _2050;
mod _2060;
mod _2070;
mod _2080;
mod _2090;

//---------------------------------------------------------------------------------------------------- Inlined `1900-2100`
// These functions are split up because LLVM
// will spend actual hours churning on a single
// function 10k+ lines long.

/// # INVARIANT
/// Input must be [u8; 10].
///
pub const fn inlined(bytes: &[u8]) -> Option<(u16, u8, u8, [u8; 10])> {
	// `-` is `45` in UTF-8 encoding.
	match bytes {
		[49, _, 48, _, _, _, _, _, _, _] => crate::_1900::inlined_1900(bytes),
		[49, _, 49, _, _, _, _, _, _, _] => crate::_1910::inlined_1910(bytes),
		[49, _, 50, _, _, _, _, _, _, _] => crate::_1920::inlined_1920(bytes),
		[49, _, 51, _, _, _, _, _, _, _] => crate::_1930::inlined_1930(bytes),
		[49, _, 52, _, _, _, _, _, _, _] => crate::_1940::inlined_1940(bytes),
		[49, _, 53, _, _, _, _, _, _, _] => crate::_1950::inlined_1950(bytes),
		[49, _, 54, _, _, _, _, _, _, _] => crate::_1960::inlined_1960(bytes),
		[49, _, 55, _, _, _, _, _, _, _] => crate::_1970::inlined_1970(bytes),
		[49, _, 56, _, _, _, _, _, _, _] => crate::_1980::inlined_1980(bytes),
		[49, _, 57, _, _, _, _, _, _, _] => crate::_1990::inlined_1990(bytes),
		[50, _, 48, _, _, _, _, _, _, _] => crate::_2000::inlined_2000(bytes),
		[50, _, 49, _, _, _, _, _, _, _] => crate::_2010::inlined_2010(bytes),
		[50, _, 50, _, _, _, _, _, _, _] => crate::_2020::inlined_2020(bytes),
		[50, _, 51, _, _, _, _, _, _, _] => crate::_2030::inlined_2030(bytes),
		[50, _, 52, _, _, _, _, _, _, _] => crate::_2040::inlined_2040(bytes),
		[50, _, 53, _, _, _, _, _, _, _] => crate::_2050::inlined_2050(bytes),
		[50, _, 54, _, _, _, _, _, _, _] => crate::_2060::inlined_2060(bytes),
		[50, _, 55, _, _, _, _, _, _, _] => crate::_2070::inlined_2070(bytes),
		[50, _, 56, _, _, _, _, _, _, _] => crate::_2080::inlined_2080(bytes),
		[50, _, 57, _, _, _, _, _, _, _] => crate::_2090::inlined_2090(bytes),
		_ => None,
	}
}
