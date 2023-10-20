//---------------------------------------------------------------------------------------------------- Use
use crate::time::{
	Time,
	TimeFull,
	Htop,
	TimeUnit,
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
	/// ## Example
	/// ```rust
	/// # use readable::time::*;
	/// // Introduce trait into scope.
	/// use readable::Uptime;
	///
	/// // Capture the _current_ system uptime,
	/// // and format it into a `Time`.
	/// std::thread::sleep(std::time::Duration::from_secs(1));
	/// let mut time: Time = Time::uptime();
	/// # // Get around CI.
	/// # let time = 1;
	/// assert!(time >= 1);
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


//---------------------------------------------------------------------------------------------------- Uptime Function
#[inline]
/// Get the current system uptime in seconds
///
/// This function can be used on:
/// - Windows
/// - macOS
/// - BSDs
/// - Linux
///
/// This will return `0` if the underlying system call fails.
pub fn uptime() -> u32 {
	// SAFETY: we're calling C.

	#[cfg(target_os = "windows")]
	{
		let milliseconds = unsafe { windows::Win32::System::SystemInformation::GetTickCount64() };
		return (milliseconds as f64 / 1000.0) as u32;
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

		let err = unsafe { libc::sysctl(
			&mut request[0],
			2,
			&mut timeval as _,
			&mut size,
			std::ptr::null_mut(),
			0,
		)};

		if err == 0 {
			if let Ok(mut sys) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
				return sys - Duration::from_secs(timeval.tv_sec as u64);
			}
		}
	}

	#[cfg(target_os = "linux")]
	{
		let mut timespec = libc::timespec {
			tv_sec: 0,
			tv_nsec: 0,
		};
		let ptr = std::ptr::addr_of_mut!(timespec);

		// Get time, ignore return error.
		unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, ptr) };

		// Time is set if no error, else
		// our default `0` is returned.
		return timespec.tv_sec as u32;
	}

	0
}

//---------------------------------------------------------------------------------------------------- Uptime Impl
mod private {
	use super::*;

	pub trait Sealed {}
	impl Sealed for Time {}
	impl Sealed for TimeFull {}
	impl Sealed for Htop {}
	impl Sealed for TimeUnit {}
}

macro_rules! impl_uptime {
	($($time:ty),*) => {
		$(
			impl Uptime for $time {
				#[inline]
				fn uptime() -> Self {
					Self::from(uptime())
				}
				fn uptime_mut(&mut self) -> &mut Self {
					let inner = uptime();
					if inner != self.inner() {
						*self = Self::from(inner);
					}
					self
				}
			}
		)*
	};
}
impl_uptime!(Time, TimeFull, Htop, TimeUnit);