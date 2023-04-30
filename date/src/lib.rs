//! Inlined data for [`readable`](https://docs.rs/readable).
//!
//! Do not use this crate directly.

mod year;
mod month;
mod day;

//---------------------------------------------------------------------------------------------------- Inlined `1900-2100`
/// # INVARIANT
/// Input must be [u8; 10].
///
pub const fn inlined(bytes: &[u8]) -> Option<(u16, u8, u8, [u8; 10])> {
	let y = match crate::year::year(&bytes) {
		Some(y) => y,
		_ => return None,
	};

	let m = match crate::month::month(&bytes) {
		Some(m) => m,
		_ => return None,
	};

	let d = match crate::day::day(&bytes) {
		Some(m) => m,
		_ => return None,
	};

	// `-` is `45` in UTF-8 encoding.
	Some((y.0, m.0, d.0,
		[
			y.1[0], y.1[1], y.1[2], y.1[3],
			45,
			m.1[0], m.1[1],
			45,
			d.1[0], d.1[1],
		]
	))
}
