#![no_main]

use libfuzzer_sys::fuzz_target;
use readable::toa::*;

fuzz_target!(|data: (
	u64,
	u128,
	i64,
	i128,
	f32,
	f64,
)| {
	let _ = Itoa::from(data.0);
	let _ = Itoa::from(data.1);
	let _ = Itoa::from(data.2);
	let _ = Itoa::from(data.3);
	let _ = Dtoa::from(data.4);
	let _ = Dtoa::from(data.5);
});
