#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: u32| {
	let _ = readable::Time::from(data);
	let _ = readable::Military::from(data);
});
