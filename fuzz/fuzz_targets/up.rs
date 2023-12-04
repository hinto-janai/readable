#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: u32| {
	let _ = readable::Uptime::from(data);
	let _ = readable::UptimeFull::from(data);
	let _ = readable::Htop::from(data);
});
