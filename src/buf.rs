use crate::constants::*;

// Shared functions for `Buffer` between `Unsigned` and `Int`.

//---------------------------------------------------------------------------------------------------- Unsigned (assumes no `-`)
#[inline(always)]
// 9
pub(crate) fn from_1(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
}

#[inline(always)]
// 99
pub(crate) fn from_2(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
}

#[inline(always)]
// 999
pub(crate) fn from_3(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
}

#[inline(always)]
// 9,999
pub(crate) fn from_4(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
	buf[1] = COMMA;
	buf[2..5].copy_from_slice(&string[1..4]);
}

#[inline(always)]
// 99,999
pub(crate) fn from_5(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
}

#[inline(always)]
// 999,999
pub(crate) fn from_6(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
}

#[inline(always)]
// 9,999,999
pub(crate) fn from_7(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0] = string[0];
	buf[1] = COMMA;
	buf[2..5].copy_from_slice(&string[1..4]);
	buf[5] = COMMA;
	buf[6..9].copy_from_slice(&string[4..7]);
}

#[inline(always)]
// 99,999,999
pub(crate) fn from_8(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
}

#[inline(always)]
// 999,999,999
pub(crate) fn from_9(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
}

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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
#[inline(always)]
// -9
pub(crate) fn from_neg_2(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2])
}

#[inline(always)]
// -99
pub(crate) fn from_neg_3(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
}

#[inline(always)]
// -999
pub(crate) fn from_neg_4(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..4].copy_from_slice(&string[0..4]);
}

#[inline(always)]
// -9,999
pub(crate) fn from_neg_5(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
}

#[inline(always)]
// -99,999
pub(crate) fn from_neg_6(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
}

#[inline(always)]
// -999,999
pub(crate) fn from_neg_7(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..4].copy_from_slice(&string[0..4]);
	buf[4] = COMMA;
	buf[5..8].copy_from_slice(&string[4..7]);
}

#[inline(always)]
// -9,999,999
pub(crate) fn from_neg_8(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..2].copy_from_slice(&string[0..2]);
	buf[2] = COMMA;
	buf[3..6].copy_from_slice(&string[2..5]);
	buf[6] = COMMA;
	buf[7..10].copy_from_slice(&string[5..8]);
}

#[inline(always)]
// -99,999,999
pub(crate) fn from_neg_9(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..3].copy_from_slice(&string[0..3]);
	buf[3] = COMMA;
	buf[4..7].copy_from_slice(&string[3..6]);
	buf[7] = COMMA;
	buf[8..11].copy_from_slice(&string[6..9]);
}

#[inline(always)]
// -999,999,999
pub(crate) fn from_neg_10(buf: &mut [u8; MAX_BUF_LEN], string: &[u8]) {
	buf[0..4].copy_from_slice(&string[0..4]);
	buf[4] = COMMA;
	buf[5..8].copy_from_slice(&string[4..7]);
	buf[8] = COMMA;
	buf[9..12].copy_from_slice(&string[7..10]);
}

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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
