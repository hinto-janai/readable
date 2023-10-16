//---------------------------------------------------------------------------------------------------- Use
use crate::time::{
	Time,
	TimeFull,
	Htop,
};

//---------------------------------------------------------------------------------------------------- Uptime Trait
/// System uptime
///
/// This trait represents structures that are viable containers for holding and
/// displaying system uptime, notably, everything in the `readable::time` module.
///
/// `readable::run` types do not implement this trait as
/// they have a relatively low upper limit of `99` hours.
///
/// This trait is sealed and can only be implemented internally on `readable` types.
pub trait Uptime: private::Sealed {
	/// This function creates a `Self` from the live system uptime and can be used on:
	/// - Windows
	/// - macOS
	/// - BSDs
	/// - Linux
	///
	/// If the underlying call fails (unlikely) this function will return an `unknown` variant.
	///
	/// The max uptime possible from this function is [`u32::MAX`] or `136 years`.
	///
	/// ## Example
	/// ```rust
	/// # use readable::time::*;
	/// // Introduce trait into scope.
	/// use readable::Uptime;
	///
	/// // Capture the _current_ system uptime,
	/// // and format it into a `Time`.
	/// let mut time: Time = Time::uptime();
	///
	/// // No matter the test environment, this
	/// // machine has probably been online for
	/// // more than 1 second.
	/// assert!(time > 1);
	///
	/// // Note that the `time` variable isn't an
	/// // incrementing time like `Duration`, it just
	/// // just a formatted `Time` which used the system's
	/// // uptime as input.
	/// //
	/// // Although, we can use `uptime_mut()` to mutate
	/// // our `time` to reflect the current uptime.
	/// std::thread::sleep(std::time::Duration::from_seconds(1));
	/// assert!(time.uptime_mut() > 2);
	/// ```
	fn uptime() -> Self;

	/// This takes an existing instance of `Self` and mutates
	/// it if the current system uptime is different than the `self` value.
	///
	/// E.g:
	/// 1. `Self::uptime()` is called
	/// 2. A few seconds passes
	/// 3. `Self::uptime_mut()` is called
	/// 4. The above will mutate `self` to reflect the new uptime
	///
	/// This returns the input `&mut self` for method chaining.
	fn uptime_mut(&mut self) -> &mut Self;
}

//---------------------------------------------------------------------------------------------------- Uptime Impl
mod private {
	use super::*;

	pub trait Sealed {}
	impl Sealed for Time {}
	impl Sealed for TimeFull {}
	impl Sealed for Htop {}
}

macro_rules! impl_uptime {
	($($time:ty),*) => {
		$(
			impl Uptime for $time {
				#[inline]
				fn uptime() -> Self {
					Self::from(uptime_inner())
				}
				fn uptime_mut(&mut self) -> &mut Self {
					let inner = uptime_inner();
					if inner != self.inner() {
						*self = Self::from(inner);
					}
					self
				}
			}
		)*
	};
}
impl_uptime!(Time, TimeFull, Htop);

//---------------------------------------------------------------------------------------------------- Uptime Function
#[inline]
fn uptime_inner() -> u32 {
	// SAFETY: we're calling C.
	unsafe {
		#[cfg(target_os = "windows")]
		{
			let milliseconds = windows::Win32::System::SystemInformation::GetTickCount64();
			(milliseconds as f64 / 1000.0) as u32
		}

		#[cfg(all(target_os = "unix", not(target_os = "linux")))]
		{
			use std::time::{Duration,SystemTime};

			let mut request = [libc::CTL_KERN, libc::KERN_BOOTTIME];

			let mut timeval = libc::timeval {
				tv_sec: 0,
				tv_nsec: 0,
			};

			let mut size: libc::size_t = std::mem::size_of_val(&timeval);

			let err = libc::sysctl(
				&mut request[0],
				2,
				&mut timeval as _,
				&mut size,
				std::ptr::null_mut(),
				0,
			);

			if err == 0 {
				if let Ok(mut sys) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
					return sys - Duration::from_secs(timeval.tv_sec as u64);
				}
			}

			return 0;
		}

		#[cfg(target_os = "linux")]
		{
			let mut timespec = libc::timespec {
				tv_sec: 0,
				tv_nsec: 0,
			};
			let ptr = std::ptr::addr_of_mut!(timespec);

			// Get time, ignore return error.
			libc::clock_gettime(libc::CLOCK_MONOTONIC, ptr);

			// Time is set if no error, else
			// our default `0` is returned.
			return timespec.tv_sec as u32;
		}
	}
}