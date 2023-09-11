//---------------------------------------------------------------------------------------------------- Use
use crate::num::constants::{
	MAX_BUF_LEN,COMMA,
};

//---------------------------------------------------------------------------------------------------- Buffer
// Shared `Buffer` for quickly formatting (float, percent, etc)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) struct Buffer {
	// Bytes representing a valid UTF-8 string.
	pub(crate) buf: [u8; MAX_BUF_LEN],
	// How many bytes we're taking up.
	pub(crate) len: usize,
}

impl Buffer {
	#[inline]
	// Returns only the valid bytes.
	pub(crate) fn as_bytes(&self) -> &[u8] {
		&self.buf[..self.len]
	}

	#[inline]
	pub(crate) fn as_str(&self) -> &str {
		// SAFETY:
		// The buffer at this point should be
		// valid UTF-8 bytes representing integers.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}
}

// Shared functions for `Buffer` between `Unsigned` and `Int`.

//---------------------------------------------------------------------------------------------------- Frontend function for `u*` -> buf
#[inline]
#[allow(clippy::match_overlapping_arm)]
pub(crate) fn from_u(u: u64) -> ([u8; MAX_BUF_LEN], usize) {
	let mut buffer = itoa::Buffer::new();
	let string = &buffer.format(u).as_bytes();
	let mut buf = [0_u8; MAX_BUF_LEN];

	let len = match u {
		0..=9                         => { from_1(&mut buf, string); 1 },
		0..=99                        => { from_2(&mut buf, string); 2 },
		0..=999                       => { from_3(&mut buf, string); 3 },
		0..=9_999                     => { from_4(&mut buf, string); 5 },
		0..=99_999                    => { from_5(&mut buf, string); 6 },
		0..=999_999                   => { from_6(&mut buf, string); 7 },
		0..=9_999_999                 => { from_7(&mut buf, string); 9 },
		0..=99_999_999                => { from_8(&mut buf, string); 10 },
		0..=999_999_999               => { from_9(&mut buf, string); 11 },
		0..=9_999_999_999             => { from_10(&mut buf, string); 13 },
		0..=99_999_999_999            => { from_11(&mut buf, string); 14 },
		0..=999_999_999_999           => { from_12(&mut buf, string); 15 },
		0..=9_999_999_999_999         => { from_13(&mut buf, string); 17 },
		0..=99_999_999_999_999        => { from_14(&mut buf, string); 18 },
		0..=999_999_999_999_999       => { from_15(&mut buf, string); 19 },
		0..=9_999_999_999_999_999     => { from_16(&mut buf, string); 21 },
		0..=99_999_999_999_999_999    => { from_17(&mut buf, string); 22 },
		0..=999_999_999_999_999_999   => { from_18(&mut buf, string); 23 },
		0..=9_999_999_999_999_999_999 => { from_19(&mut buf, string); 25 },
		_                             => { from_20(&mut buf, string); 26 },
	};

	(buf, len)
}

//---------------------------------------------------------------------------------------------------- Frontend function for `i*` -> buf
#[inline]
#[allow(clippy::match_overlapping_arm)]
pub(crate) fn from_i(i: i64) -> ([u8; MAX_BUF_LEN], usize) {
	let mut buffer = itoa::Buffer::new();
	let string = &buffer.format(i).as_bytes();
	let mut buf = [0_u8; MAX_BUF_LEN];

	if i.is_negative() {
		let len = match string.len() {
			// Must be at least two bytes: `-1`
			2 => { from_neg_2(&mut buf, string); 2 },
			3 => { from_neg_3(&mut buf, string); 3 },
			4 => { from_neg_4(&mut buf, string); 4 },
			5 => { from_neg_5(&mut buf, string); 6 },
			6 => { from_neg_6(&mut buf, string); 7 },
			7 => { from_neg_7(&mut buf, string); 8 },
			8 => { from_neg_8(&mut buf, string); 10 },
			9 => { from_neg_9(&mut buf, string); 11 },
			10 => { from_neg_10(&mut buf, string); 12 },
			11 => { from_neg_11(&mut buf, string); 14 },
			12 => { from_neg_12(&mut buf, string); 15 },
			13 => { from_neg_13(&mut buf, string); 16 },
			14 => { from_neg_14(&mut buf, string); 18 },
			15 => { from_neg_15(&mut buf, string); 19 },
			16 => { from_neg_16(&mut buf, string); 20 },
			17 => { from_neg_17(&mut buf, string); 22 },
			18 => { from_neg_18(&mut buf, string); 23 },
			19 => { from_neg_19(&mut buf, string); 24 },
			20 => { from_neg_20(&mut buf, string); 26 },

			// We've covered all possible negative `i64` lengths.
			_ => unreachable!(),
		};
		(buf, len)
	} else {
		let len = match i {
			0..=9                         => { from_1(&mut buf, string); 1 },
			0..=99                        => { from_2(&mut buf, string); 2 },
			0..=999                       => { from_3(&mut buf, string); 3 },
			0..=9_999                     => { from_4(&mut buf, string); 5 },
			0..=99_999                    => { from_5(&mut buf, string); 6 },
			0..=999_999                   => { from_6(&mut buf, string); 7 },
			0..=9_999_999                 => { from_7(&mut buf, string); 9 },
			0..=99_999_999                => { from_8(&mut buf, string); 10 },
			0..=999_999_999               => { from_9(&mut buf, string); 11 },
			0..=9_999_999_999             => { from_10(&mut buf, string); 13 },
			0..=99_999_999_999            => { from_11(&mut buf, string); 14 },
			0..=999_999_999_999           => { from_12(&mut buf, string); 15 },
			0..=9_999_999_999_999         => { from_13(&mut buf, string); 17 },
			0..=99_999_999_999_999        => { from_14(&mut buf, string); 18 },
			0..=999_999_999_999_999       => { from_15(&mut buf, string); 19 },
			0..=9_999_999_999_999_999     => { from_16(&mut buf, string); 21 },
			0..=99_999_999_999_999_999    => { from_17(&mut buf, string); 22 },
			0..=999_999_999_999_999_999   => { from_18(&mut buf, string); 23 },
			0..=9_223_372_036_854_775_807 => { from_19(&mut buf, string); 25 },

			// We've covered all possible positive `i64` lengths.
			_ => unreachable!(),
		};
		(buf, len)
	}
}

//---------------------------------------------------------------------------------------------------- Unsigned (assumes no `-`)
#[inline]
// 9
pub(crate) fn from_1(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
}

#[inline]
// 99
pub(crate) fn from_2(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
}

#[inline]
// 999
pub(crate) fn from_3(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
}

#[inline]
// 9,999
pub(crate) fn from_4(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
	buf[1] = COMMA;
	buf[2..5].copy_from_slice(&string[1..4]);
}

#[inline]
// 99,999
pub(crate) fn from_5(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
}

#[inline]
// 999,999
pub(crate) fn from_6(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
}

#[inline]
// 9,999,999
pub(crate) fn from_7(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
	buf[1] = COMMA;
	buf[2..5].copy_from_slice(&string[1..4]);
	buf[5] = COMMA;
	buf[6..9].copy_from_slice(&string[4..7]);
}

#[inline]
// 99,999,999
pub(crate) fn from_8(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
}

#[inline]
// 999,999,999
pub(crate) fn from_9(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
}

#[inline]
// 9,999,999,999
pub(crate) fn from_10(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
	buf[1] = COMMA;
	buf[2..5].copy_from_slice(&string[1..4]);
	buf[5] = COMMA;
	buf[6..9].copy_from_slice(&string[4..7]);
	buf[9] = COMMA;
	buf[10..13].copy_from_slice(&string[7..10]);
}

#[inline]
// 99,999,999,999
pub(crate) fn from_11(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
	buf[10] = COMMA;
	buf[11..14].copy_from_slice(&string[8..11]);
}

#[inline]
// 999,999,999,999
pub(crate) fn from_12(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
	buf[11] = COMMA;
	buf[12..15].copy_from_slice(&string[9..12]);
}

#[inline]
// 9,999,999,999,999
pub(crate) fn from_13(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
	buf[1] = COMMA;
	buf[2..5].copy_from_slice(&string[1..4]);
	buf[5] = COMMA;
	buf[6..9].copy_from_slice(&string[4..7]);
	buf[9] = COMMA;
	buf[10..13].copy_from_slice(&string[7..10]);
	buf[13] = COMMA;
	buf[14..17].copy_from_slice(&string[10..13]);
}

#[inline]
// 99,999,999,999,999
pub(crate) fn from_14(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
	buf[10] = COMMA;
	buf[11..14].copy_from_slice(&string[8..11]);
	buf[14] = COMMA;
	buf[15..18].copy_from_slice(&string[11..14]);
}

#[inline]
// 999,999,999,999,999
pub(crate) fn from_15(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
	buf[11] = COMMA;
	buf[12..15].copy_from_slice(&string[9..12]);
	buf[15] = COMMA;
	buf[16..19].copy_from_slice(&string[12..15]);
}

#[inline]
// 9,999,999,999,999,999
pub(crate) fn from_16(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
	buf[1] = COMMA;
	buf[2..5].copy_from_slice(&string[1..4]);
	buf[5] = COMMA;
	buf[6..9].copy_from_slice(&string[4..7]);
	buf[9] = COMMA;
	buf[10..13].copy_from_slice(&string[7..10]);
	buf[13] = COMMA;
	buf[14..17].copy_from_slice(&string[10..13]);
	buf[17] = COMMA;
	buf[18..21].copy_from_slice(&string[13..16]);
}

#[inline]
// 99,999,999,999,999,999
pub(crate) fn from_17(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
	buf[10] = COMMA;
	buf[11..14].copy_from_slice(&string[8..11]);
	buf[14] = COMMA;
	buf[15..18].copy_from_slice(&string[11..14]);
	buf[18] = COMMA;
	buf[19..22].copy_from_slice(&string[14..17]);
}

#[inline]
// 999,999,999,999,999,999
pub(crate) fn from_18(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
	buf[11] = COMMA;
	buf[12..15].copy_from_slice(&string[9..12]);
	buf[15] = COMMA;
	buf[16..19].copy_from_slice(&string[12..15]);
	buf[19] = COMMA;
	buf[20..23].copy_from_slice(&string[15..18]);
}

#[inline]
// 9,999,999,999,999,999,999
pub(crate) fn from_19(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
	buf[1] = COMMA;
	buf[2..5].copy_from_slice(&string[1..4]);
	buf[5] = COMMA;
	buf[6..9].copy_from_slice(&string[4..7]);
	buf[9] = COMMA;
	buf[10..13].copy_from_slice(&string[7..10]);
	buf[13] = COMMA;
	buf[14..17].copy_from_slice(&string[10..13]);
	buf[17] = COMMA;
	buf[18..21].copy_from_slice(&string[13..16]);
	buf[21] = COMMA;
	buf[22..25].copy_from_slice(&string[16..19]);
}

#[inline]
// 99,999,999,999,999,999,999
pub(crate) fn from_20(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
	buf[10] = COMMA;
	buf[11..14].copy_from_slice(&string[8..11]);
	buf[14] = COMMA;
	buf[15..18].copy_from_slice(&string[11..14]);
	buf[18] = COMMA;
	buf[19..22].copy_from_slice(&string[14..17]);
	buf[22] = COMMA;
	buf[23..26].copy_from_slice(&string[17..20]);
}

//---------------------------------------------------------------------------------------------------- Signed (assumes `-`)
#[inline]
// -9
pub(crate) fn from_neg_2(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2])
}

#[inline]
// -99
pub(crate) fn from_neg_3(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
}

#[inline]
// -999
pub(crate) fn from_neg_4(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..4].copy_from_slice(&string[0..4]);
}

#[inline]
// -9,999
pub(crate) fn from_neg_5(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
}

#[inline]
// -99,999
pub(crate) fn from_neg_6(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
}

#[inline]
// -999,999
pub(crate) fn from_neg_7(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..4].copy_from_slice(&string[0..4]);
	buf[4] = COMMA;
	buf[5..8].copy_from_slice(&string[4..7]);
}

#[inline]
// -9,999,999
pub(crate) fn from_neg_8(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
}

#[inline]
// -99,999,999
pub(crate) fn from_neg_9(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
}

#[inline]
// -999,999,999
pub(crate) fn from_neg_10(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..4].copy_from_slice(&string[0..4]);
	buf[4] = COMMA;
	buf[5..8].copy_from_slice(&string[4..7]);
	buf[8] = COMMA;
	buf[9..12].copy_from_slice(&string[7..10]);
}

#[inline]
// -9,999,999,999
pub(crate) fn from_neg_11(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
	buf[10] = COMMA;
	buf[11..14].copy_from_slice(&string[8..11]);
}

#[inline]
// -99,999,999,999
pub(crate) fn from_neg_12(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
	buf[11] = COMMA;
	buf[12..15].copy_from_slice(&string[9..12]);
}

#[inline]
// -999,999,999,999
pub(crate) fn from_neg_13(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..4].copy_from_slice(&string[0..4]);
	buf[4] = COMMA;
	buf[5..8].copy_from_slice(&string[4..7]);
	buf[8] = COMMA;
	buf[9..12].copy_from_slice(&string[7..10]);
	buf[12] = COMMA;
	buf[13..16].copy_from_slice(&string[10..13]);
}

#[inline]
// -9,999,999,999,999
pub(crate) fn from_neg_14(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
	buf[10] = COMMA;
	buf[11..14].copy_from_slice(&string[8..11]);
	buf[14] = COMMA;
	buf[15..18].copy_from_slice(&string[11..14]);
}

#[inline]
// -99,999,999,999,999
pub(crate) fn from_neg_15(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
	buf[11] = COMMA;
	buf[12..15].copy_from_slice(&string[9..12]);
	buf[15] = COMMA;
	buf[16..19].copy_from_slice(&string[12..15]);
}

#[inline]
// -999,999,999,999,999
pub(crate) fn from_neg_16(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..4].copy_from_slice(&string[0..4]);
	buf[4] = COMMA;
	buf[5..8].copy_from_slice(&string[4..7]);
	buf[8] = COMMA;
	buf[9..12].copy_from_slice(&string[7..10]);
	buf[12] = COMMA;
	buf[13..16].copy_from_slice(&string[10..13]);
	buf[16] = COMMA;
	buf[17..20].copy_from_slice(&string[13..16]);
}

#[inline]
// -9,999,999,999,999,999
pub(crate) fn from_neg_17(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
	buf[10] = COMMA;
	buf[11..14].copy_from_slice(&string[8..11]);
	buf[14] = COMMA;
	buf[15..18].copy_from_slice(&string[11..14]);
	buf[18] = COMMA;
	buf[19..22].copy_from_slice(&string[14..17]);
}

#[inline]
// -99,999,999,999,999,999
pub(crate) fn from_neg_18(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
	buf[11] = COMMA;
	buf[12..15].copy_from_slice(&string[9..12]);
	buf[15] = COMMA;
	buf[16..19].copy_from_slice(&string[12..15]);
	buf[19] = COMMA;
	buf[20..23].copy_from_slice(&string[15..18]);
}

#[inline]
// -999,999,999,999,999,999
pub(crate) fn from_neg_19(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..4].copy_from_slice(&string[0..4]);
	buf[4] = COMMA;
	buf[5..8].copy_from_slice(&string[4..7]);
	buf[8] = COMMA;
	buf[9..12].copy_from_slice(&string[7..10]);
	buf[12] = COMMA;
	buf[13..16].copy_from_slice(&string[10..13]);
	buf[16] = COMMA;
	buf[17..20].copy_from_slice(&string[13..16]);
	buf[20] = COMMA;
	buf[21..24].copy_from_slice(&string[16..19]);
}

#[inline]
// -9,999,999,999,999,999,999
pub(crate) fn from_neg_20(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
	buf[10] = COMMA;
	buf[11..14].copy_from_slice(&string[8..11]);
	buf[14] = COMMA;
	buf[15..18].copy_from_slice(&string[11..14]);
	buf[18] = COMMA;
	buf[19..22].copy_from_slice(&string[14..17]);
	buf[22] = COMMA;
	buf[23..26].copy_from_slice(&string[17..20]);
}
