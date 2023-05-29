//---------------------------------------------------------------------------------------------------- __NAME__
/// Use [`itoa`](https://docs.rs/itoa) to format an integer (without commas).
///
/// This takes an [`itoa::Integer`] as input and outputs a [`&str`].
#[macro_export]
macro_rules! itoa {
	($integer:expr) => {{
		itoa::Buffer::new().format($integer)
	}}
}
pub use itoa;

/// Use [`ryu`](https://docs.ryu) to format a float (without commas).
///
/// This takes a [`ryu::Float`] as input and outputs a [`&str`].
///
/// If the feature flag `ignore_nan_inf` is enabled, this uses
/// [`ryu::Buffer::format_finite`] which does _not_ check for `NaN` or infinity.
///
/// Else, it uses [`ryu::Buffer::format`], which does.
#[macro_export]
macro_rules! ryu {
	($float:expr) => {{
		ryu::Buffer::new().format($float)
	}}
}
pub use ryu;

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn itoa() {
		for i in 0..u16::MAX {
			assert!(itoa!(i) == format!("{i}"));
		}
	}

	#[test]
	fn ryu() {
		let i = 1111.1;
		assert!(ryu!(i) == format!("{i}"));
	}
}
