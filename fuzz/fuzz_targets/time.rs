#![no_main]

use libfuzzer_sys::fuzz_target;
use readable::time::*;

fuzz_target!(|data: u32| {
	let _ = Time::from(data);
	let _ = Military::from(data);
});
