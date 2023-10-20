//---------------------------------------------------------------------------------------------------- Use

//---------------------------------------------------------------------------------------------------- Uptime Trait
/// Current system time
///
/// This trait represents structures that are viable containers for holding and
/// displaying the current system date, notably, everything in the `readable::date` module.
///
/// This trait is sealed and can only be implemented internally on `readable` types.
pub trait SysDate {
	/// This function creates a `Self` from the live system date
	///
	/// If the underlying call fails (unlikely) this function will return an `unknown` variant.
	///
	/// ## Example
	/// ```rust
	/// # use readable::date::*;
	/// // Introduce trait into scope.
	/// use readable::SysDate;
	///
	/// // Capture the _current_ system date,
	/// // and format it into a `Date`.
	/// let date: Date = Date::sysdate();
	/// ```
	fn sysdate() -> Self;

	/// This takes an existing instance of `Self` and mutates
	/// it if the current system uptime is different than the `self` value.
	///
	/// This returns the input `&mut self` for method chaining.
	fn sysdate_mut(&mut self) -> &mut Self;
}

//---------------------------------------------------------------------------------------------------- SysDate
/// Get the current system date in UTC locale
///
/// The returned value is `(year, month, day)`.
///
/// This will return `(0, 0, 0)` if the underlying system call fails.
pub fn sysdate() -> (i16, u8, u8) {
	let unix = unix();
	if unix == 0 {
		(0,0,0)
	} else {
		nichi::Date::from_unix(unix as i128).inner()
	}
}

/// Get the current system UNIX timestamp
///
/// The returned value is how many seconds has passed since `UNIX_EPOCH`
///
/// This will return `0` if the underlying system call fails.
pub fn unix() -> u64 {
	use std::time::{SystemTime, UNIX_EPOCH};

	match SystemTime::now().duration_since(UNIX_EPOCH) {
		Ok(unix) => unix.as_secs(),
		_ => 0,
	}
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
						let (y,m,d) = super::sysdate();
						if (y,m,d) == (0,0,0) {
							Self::unknown()
						} else {
							Self::$fn(y as u16, m, d)
						}
					}
					fn sysdate_mut(&mut self) -> &mut Self {
						*self = Self::sysdate();
						self
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