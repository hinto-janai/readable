
//---------------------------------------------------------------------------------------------------- Use

//---------------------------------------------------------------------------------------------------- Uptime Trait
/// Current system clock time
///
/// This trait represents structures that are viable containers
/// for holding and displaying the current system clock time.
///
/// This trait is sealed and can only be implemented internally on `readable` types.
pub trait SysTime {
	/// This function creates a `Self` from the live system date
	///
	/// ## Example
	/// ```rust
	/// # use readable::time::*;
	/// // Introduce trait into scope.
	/// use readable::SysTime;
	///
	/// // Capture the _current_ system date,
	/// // and format it into a `Date`.
	/// let time: Time = Time::sys_time();
	/// ```
	fn sys_time() -> Self;
}

//---------------------------------------------------------------------------------------------------- Uptime Function
mod private {
	use crate::time::{
		Time,Military,TimeUnit,
	};
	trait Sealed {}
	macro_rules! impl_sealed {
		($($n:ty => $fn:ident),* $(,)?) => {
			$(
				impl super::SysTime for $n {
					fn sys_time() -> Self {
						Self::$fn(crate::time::free::time())
					}
				}
				impl Sealed for $n {}
			)*
		};
	}
	impl_sealed! {
		Time => priv_from,
		Military => priv_from,
		TimeUnit => new,
	}
}
