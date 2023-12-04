#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: u64| {
// fuzz_target!(|data: (u16, u8, u8)| {
//fuzz_target!(|data: &str| {
	// TODO/FIXME:
	// these functions are terrible, fuzzing even
	// for a bit reveals an insane amount of panics.
//	if data.is_ascii() {
//		let _ = readable::Date::from_str(data);
//		let _ = readable::Nichi::from_str(data);
//		let _ = readable::NichiFull::from_str(data);
//	}
	// let _ = readable::Date::from(data);
	// let _ = readable::Nichi::from(data);
	// let _ = readable::NichiFull::from(data);
	// let _ = readable::Date::from_unix(data);
	// let _ = readable::Nichi::from_unix(data);
	// let _ = readable::NichiFull::from_unix(data);
});
