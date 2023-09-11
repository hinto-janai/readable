//---------------------------------------------------------------------------------------------------- Use

//---------------------------------------------------------------------------------------------------- __NAME__
/// Quickly format integers to strings (without commas).
///
/// This takes a:
/// - [`u8`]
/// - [`u16`]
/// - [`u32`]
/// - [`u64`]
/// - [`u128`]
/// - [`usize`]
/// - [`i8`]
/// - [`i16`]
/// - [`i32`]
/// - [`i64`]
/// - [`i128`]
/// - [`isize`]
///
/// as input and outputs a [`&str`].
///
/// This macro is cheaper than [`std::format!`], as it uses [`itoa`](https://docs.rs/itoa) under the hood.
///
/// If this macro were a function, the signature would look something like:
/// ```rust,ignore
/// fn itoa<N>(number: N) -> &str where N: IsANumber { /* ... */ }
/// ```
///
/// ## Example
/// ```rust
/// assert_eq!(readable::itoa!(100_u8), "100");
/// assert_eq!(readable::itoa!(-100_i8), "-100");
/// assert_eq!(readable::itoa!(u128::MAX), "340282366920938463463374607431768211455");
/// assert_eq!(readable::itoa!(i128::MIN), "-170141183460469231731687303715884105728");
/// ```
#[macro_export]
macro_rules! itoa {
	($integer:expr) => {{
		$crate::__readable_itoa::Buffer::new().format($integer)
	}}
}

/// Quickly format floats to strings (without commas) using [`ryu`](https://docs.rs/ryu)
///
/// This takes a [`f32`] or [`f64`] as input and outputs a [`&str`].
///
/// This macro is cheaper than [`std::format!`], as it uses [`ryu`](https://docs.rs/ryu) under the hood.
///
/// If this macro were a function, the signature would look something like:
/// ```rust,ignore
/// fn ryu<F>(float: F) -> &str where F: IsAFloat { /* ... */ }
/// ```
///
/// ## `NaN` & Infinity
/// If the feature flag `ignore_nan_inf` is enabled, this macro will use [`ryu::Buffer::format_finite`] which does _not_ check for `NaN` or infinity.
///
/// Else, it uses [`ryu::Buffer::format`], which does.
///
/// ## Example
#[macro_export]
macro_rules! ryu {
	($float:expr) => {{
		#[cfg(feature = "ignore_nan_inf")]
		{
			$crate::__readable_ryu::Buffer::new().format_finite($float)
		}

		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			$crate::__readable_ryu::Buffer::new().format($float)
		}
	}}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn itoa() {
		for i in 0..u16::MAX {
			let s = format!("{i}");
			assert_eq!(itoa!(i), s);
			assert_eq!(crate::itoa!(i), s);
		}
	}

	#[test]
	fn ryu() {
		let i = 1111.1;
		let s = format!("{i}");
		assert_eq!(ryu!(i), s);
		assert_eq!(crate::ryu!(i), s);
	}
}
