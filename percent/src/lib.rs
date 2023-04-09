//! Inlined data for [`readable`](https://docs.rs/readable).
//!
//! Do not use this crate directly.

mod _0;
mod _25;
mod _50;
mod _75;

/// INVARIANT:
/// Input must be `0.0..100.0`.
pub fn inlined(f: f64) -> &'static str {
	match f {
		0.0..=25.0 => crate::_0::inlined_0(f),
		25.0..=50.0 => crate::_25::inlined_25(f),
		50.0..=75.0 => crate::_50::inlined_50(f),
		_ => crate::_75::inlined_75(f),
	}
}
