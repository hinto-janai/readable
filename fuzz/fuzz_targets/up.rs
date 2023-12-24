#![no_main]

use libfuzzer_sys::fuzz_target;
use readable::up::*;

fuzz_target!(|data: u32| {
	let _ = Uptime::from(data);
	let _ = UptimeFull::from(data);
	let _ = Htop::from(data);
});
