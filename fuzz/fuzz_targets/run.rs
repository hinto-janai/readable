#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: f64| {
	let r1 = readable::Runtime::from(data);
	let r2 = readable::RuntimePad::from(data);
	let r3 = readable::RuntimeMilli::from(data);
	let r4 = readable::RuntimeUnion::from(data);
	assert_eq!(r4.as_str(),       r1.as_str());
	assert_eq!(r4.as_str_pad(),   r2.as_str());
	assert_eq!(r4.as_str_milli(), r3.as_str());
});
