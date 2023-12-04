#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: i64| {
	let _ = readable::Int::from(data);
});
