#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: f64| {
	let _ = readable::Float::from(data);
	let _ = readable::Percent::from(data);
});
