#![no_main]

use libfuzzer_sys::fuzz_target;
use readable::num::*;

fuzz_target!(|data: u64| {
	let _ = Unsigned::from(data);
});
