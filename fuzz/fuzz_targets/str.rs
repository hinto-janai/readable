#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (&str, char)| {
	let s = data.0;
	let c = data.1;

	if s.len() < 255 {
		let _ = readable::Str::<255>::try_from(s).unwrap();
		let mut new = readable::Str::<255>::new();
		new.push_str_panic(s);
	}

	let mut new = readable::Str::<255>::new();
	new.push_char_panic(c);
});
