#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (
	u64,
	u128,
	i64,
	i128,
	f32,
	f64,
)| {
	let _ = readable::Itoa::from(data.0);
	let _ = readable::Itoa::from(data.1);
	let _ = readable::Itoa::from(data.2);
	let _ = readable::Itoa::from(data.3);
	let _ = readable::Dtoa::from(data.4);
	let _ = readable::Dtoa::from(data.5);
});
