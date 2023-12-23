// Manual Borsh serde functions.

use borsh::BorshSerialize;
use borsh::BorshDeserialize;
use borsh::io::{Read,Write};
use compact_str::CompactString;

// CompactString
#[inline]
pub(crate) fn ser_compact_string<W: Write>(s: &CompactString, writer: &mut W) -> Result<(), borsh::io::Error> {
	let s = s.as_str();
	s.serialize(writer)?;
	Ok(())
}

#[inline]
pub(crate) fn de_compact_string<R: Read>(reader: &mut R) -> Result<CompactString, borsh::io::Error> {
	let s: String = BorshDeserialize::deserialize_reader(reader)?;
	Ok(CompactString::from(s))
}