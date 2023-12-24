#![no_main]

use libfuzzer_sys::fuzz_target;
use readable::num::*;

fuzz_target!(|data: f64| {
	let _ = Float::from(data);
	let _ = Percent::from(data);
});
