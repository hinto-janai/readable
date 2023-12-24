#![no_main]

use libfuzzer_sys::fuzz_target;
use readable::num::*;

fuzz_target!(|data: i64| {
	let _ = Int::from(data);
});
