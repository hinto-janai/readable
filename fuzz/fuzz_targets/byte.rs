#![no_main]

use libfuzzer_sys::fuzz_target;
use readable::byte::*;

fuzz_target!(|data: u64| {
	let _ = Byte::from(data);
});
