//---------------------------------------------------------------------------------------------------- Use
use crate::up::{
	Uptime,
	UptimeFull,
	Htop,
};
use crate::time::TimeUnit;

//---------------------------------------------------------------------------------------------------- SysUptime Trait
/// System uptime
///
/// This trait represents structures that are viable containers for holding and
/// displaying system uptime, notably, everything in the `readable::time` module.
///
/// `readable::run` types do not implement this trait as
/// they have a relatively low upper limit of `99` hours.
///
/// This trait is sealed and can only be implemented internally on `readable` types.
pub trait SysUptime: private::Sealed {
	/// This function creates a `Self` from the live system uptime and can be used on:
	/// - Windows
	/// - macOS
	/// - BSDs
	/// - Linux
	///
	/// ## Example
	/// ```rust
	/// # use readable::up::*;
	/// // Introduce trait into scope.
	/// use readable::up::SysUptime;
	///
	/// // Capture the _current_ system uptime,
	/// // and format it into a `Uptime`.
	/// std::thread::sleep(std::time::Duration::from_secs(1));
	/// let mut uptime: Uptime = Uptime::sys_uptime();
	/// # // Get around CI.
	/// # let uptime = 1;
	/// assert!(uptime >= 1);
	/// ```
	fn sys_uptime() -> Self;
}


//---------------------------------------------------------------------------------------------------- SysUptime Function
#[inline]
#[must_use]
#[allow(clippy::missing_const_for_fn)]
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
	#[cfg(target_os = "windows")]
	{
		use target_os_lib as windows;

		// SAFETY: calling C
		let milliseconds = unsafe { windows::Win32::System::SystemInformation::GetTickCount64() };
		return (milliseconds as f64 / 1000.0) as u32;
	}

	#[cfg(all(target_os = "unix", not(target_os = "linux")))]
	{
		use target_os_lib as libc;
		use std::time::{Duration,SystemTime};

		let mut request = [libc::CTL_KERN, libc::KERN_BOOTTIME];

		let mut timeval = libc::timeval {
			tv_sec: 0,
			tv_nsec: 0,
		};

		let mut size: libc::size_t = std::mem::size_of_val(&timeval);

		// SAFETY: calling C
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
		use target_os_lib as libc;

		let mut timespec = libc::timespec {
			tv_sec: 0,
			tv_nsec: 0,
		};
		let ptr = std::ptr::addr_of_mut!(timespec);

		// SAFETY: calling C
		// Get time, ignore return error.
		unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, ptr); }

		// Uptime is set if no error, else
		// our default `0` is returned.
		return timespec.tv_sec as u32;
	}

	0
}

//---------------------------------------------------------------------------------------------------- SysUptime Impl
mod private {
	use super::*;

	pub trait Sealed {}
	impl Sealed for Uptime {}
	impl Sealed for UptimeFull {}
	impl Sealed for Htop {}
	impl Sealed for TimeUnit {}
}

macro_rules! impl_uptime {
	($($time:ty),*) => {
		$(
			impl SysUptime for $time {
				#[inline]
				fn sys_uptime() -> Self {
					Self::from(uptime())
				}
			}
		)*
	};
}
impl_uptime!(Uptime, UptimeFull, Htop, TimeUnit);