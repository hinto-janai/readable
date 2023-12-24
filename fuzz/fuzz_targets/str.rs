#![no_main]

use libfuzzer_sys::fuzz_target;
use readable::str::*;

fuzz_target!(|data: ([u8; 255], &str, char)| {
	let b = data.0;
	let s = data.1;
	let c = data.2;

	if let Ok(string) = std::str::from_utf8(&b) {
		if string.len() <= 255 {
			let _ = Str::<255>::try_from(string).unwrap();
		}
	}

	if s.len() < 255 {
		let _ = Str::<255>::try_from(s).unwrap();
		let mut new = Str::<255>::new();
		new.push_str_panic(s);
	}

	let mut new = Str::<255>::new();
	new.push_char_panic(c);
});
