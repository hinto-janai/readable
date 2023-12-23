//---------------------------------------------------------------------------------------------------- Use

//---------------------------------------------------------------------------------------------------- Uptime Trait
/// Current system date
///
/// This trait represents structures that are viable containers for holding and
/// displaying the current system date, notably, everything in the `readable::date` module.
///
/// This trait is sealed and can only be implemented internally on `readable` types.
pub trait SysDate {
	/// This function creates a `Self` from the live system date
	///
	/// ## Example
	/// ```rust
	/// # use readable::date::*;
	/// // Introduce trait into scope.
	/// use readable::date::SysDate;
	///
	/// // Capture the _current_ system date,
	/// // and format it into a `Date`.
	/// let date: Date = Date::sysdate();
	/// ```
	fn sysdate() -> Self;
}

//---------------------------------------------------------------------------------------------------- Uptime Function
mod private {
	use crate::date::{Date,Nichi,NichiFull};

	trait Sealed {}

	macro_rules! impl_sealed {
		($($n:ty => $fn:ident),* $(,)?) => {
			$(
				impl super::SysDate for $n {
					fn sysdate() -> Self {
						let (y,m,d) = crate::date::free::date();
						Self::$fn(y as u16, m, d)
					}
				}
				impl Sealed for $n {}
			)*
		};
	}
	impl_sealed! {
		Date      => priv_ymd_num,
		Nichi     => priv_from,
		NichiFull => priv_from
	}
}