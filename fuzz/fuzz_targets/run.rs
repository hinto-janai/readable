#![no_main]

use libfuzzer_sys::fuzz_target;
use readable::run::*;

fuzz_target!(|data: f64| {
	let r1 = Runtime::from(data);
	let r2 = RuntimePad::from(data);
	let r3 = RuntimeMilli::from(data);
	let r4 = RuntimeUnion::from(data);
	assert_eq!(r4.as_str(),       r1.as_str());
	assert_eq!(r4.as_str_pad(),   r2.as_str());
	assert_eq!(r4.as_str_milli(), r3.as_str());
});
