//---------------------------------------------------------------------------------------------------- num_format::Buffer.
// Creates a num_format::Buffer, turns it into a compact_str::CompactString, and returns it.
macro_rules! num {
	($number:expr) => {
		{
			let mut num = ::num_format::Buffer::new();
			num.write_formatted(&$number, &::num_format::Locale::en);
			num
		}
	}
}
pub(crate) use num;

//---------------------------------------------------------------------------------------------------- Internal Buffer.
// Implement a private module `Buffer` type
// with a variable amount of array space.
macro_rules! buffer {
	($max_length:expr, $unknown_buffer:expr, $unknown_len:expr) => {
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
		struct Buffer {
			// Bytes representing a valid UTF-8 string.
			buf: [u8; $max_length],
			// How many bytes we're taking up.
			len: usize,
		}

		impl Buffer {
			#[inline(always)]
			fn unknown() -> Self {
				Self {
					buf: $unknown_buffer,
					len: $unknown_len,
				}
			}

			#[inline(always)]
			fn to_buffer(&self) -> [u8; $max_length] {
				self.buf
			}

			#[inline(always)]
			fn as_buffer(&self) -> &[u8; $max_length] {
				&self.buf
			}

			#[inline(always)]
			fn as_bytes(&self) -> &[u8] {
				&self.buf[..self.len]
			}

			#[inline(always)]
			fn is_empty(&self) -> bool {
				self.len == 0
			}

			#[inline(always)]
			fn as_str(&self) -> &str {
				// SAFETY:
				// This is intended to format numbers, which are valid UTF-8.
				unsafe { ::std::str::from_utf8_unchecked(self.as_bytes()) }
			}

			#[inline(always)]
			fn to_string(&self) -> String {
				self.as_str().to_string()
			}

			#[inline(always)]
			fn into_string(self) -> String {
				self.as_str().to_string()
			}

		    #[inline(always)]
		    fn len(&self) -> usize {
				self.len
		    }
		}
	}
}
pub(crate) use buffer;

//---------------------------------------------------------------------------------------------------- NaN.
// "Handle NaN/Infinite" Macro.
macro_rules! handle_nan {
	($float:ident) => {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			match $float.classify() {
				::std::num::FpCategory::Normal   => (),
				::std::num::FpCategory::Nan      => return Self(f64::NAN, crate::inner::Inner::Nan(crate::constants::NAN)),
				::std::num::FpCategory::Infinite => return Self(f64::INFINITY, crate::inner::Inner::Inf(crate::constants::INFINITY)),
				_ => (),
			}
		}
	}
}
pub(crate) use handle_nan;

// "Handle NaN/Infinite" Macro for `compact_str`.
macro_rules! handle_nan_string {
	($float:ident) => {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			match $float.classify() {
				::std::num::FpCategory::Normal   => (),
				::std::num::FpCategory::Nan      => return Self(f64::NAN, ::compact_str::CompactString::new(crate::constants::NAN)),
				::std::num::FpCategory::Infinite => return Self(f64::INFINITY, ::compact_str::CompactString::new(crate::constants::INFINITY)),
				_ => (),
			}
		}
	}
}
pub(crate) use handle_nan_string;

// "Handle NaN/Infinite" Macro for `Runtime`.
macro_rules! handle_nan_runtime {
	($float:ident) => {
//		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			match $float.classify() {
				::std::num::FpCategory::Normal   => (),
				::std::num::FpCategory::Nan      => return Self::unknown(),
				::std::num::FpCategory::Infinite => return Self::unknown(),
				_ => (),
			}
		}
	}
}
pub(crate) use handle_nan_runtime;

//---------------------------------------------------------------------------------------------------- Impl.
// `Buffer`.
macro_rules! impl_buffer {
	($max_length:expr, $unknown_buffer:expr, $unknown_len:expr) => {
		#[inline(always)]
		/// Return the inner buffer that represents the [`String`].
		///
		/// These are guaranteed to be valid UTF-8 bytes.
		///
		/// Not all bytes are necessarily used, however.
		/// To find the valid portion of the string, use [`Self::len`].
		/// That will determine where to stop on the buffer, e.g:
		/// ```rust,ignore
		/// let buffer      = self.to_buffer();
		/// let valid_bytes = buffer[0..self.len()];
		///
		/// # SAFETY: These bytes are always be valid UTF-8.
		/// unsafe {
		///     let string = std::str::from_utf8_unchecked(&valid_bytes);
		/// }
		/// ```
		fn to_buffer(&self) -> [u8; $max_length] {
			self.1.to_buffer()
		}

		#[inline(always)]
		/// Same as [`Self::to_buffer`] but returns a borrowed array.
		fn as_buffer(&self) -> &[u8; $max_length] {
			&self.1.as_buffer()
		}
	}
}
pub(crate) use impl_buffer;

// `From`.
macro_rules! impl_from_single {
	($from:ident, $to:ident, $s:ident) => {
		impl From<$from> for $s {
			#[inline]
			fn from(number: $from) -> Self {
				let n = number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}

		impl From<&$from> for $s {
			#[inline]
			fn from(number: &$from) -> Self {
				let n = *number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}
	}
}
pub(crate) use impl_from_single;

// `From`.
macro_rules! impl_from {
	($from_8:ident, $from_16:ident, $from_32:ident, $to:ident, $from_size:ident, $s:ident) => {
		// Same.
		impl From<$to> for $s {
			#[inline]
			fn from(number: $to) -> Self {
				Self(number, crate::inner::Inner::Buf(crate::macros::num!(number)))
			}
		}
		impl From<&$to> for $s {
			#[inline]
			fn from(number: &$to) -> Self {
				Self(*number, crate::inner::Inner::Buf(crate::macros::num!(*number)))
			}
		}

		// Other types.
		impl From<$from_8> for $s {
			#[inline]
			fn from(number: $from_8) -> Self {
				let n = number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}
		impl From<$from_16> for $s {
			#[inline]
			fn from(number: $from_16) -> Self {
				let n = number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}
		impl From<$from_32> for $s {
			#[inline]
			fn from(number: $from_32) -> Self {
				let n = number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}
		impl From<$from_size> for $s {
			#[inline]
			fn from(number: $from_size) -> Self {
				let n = number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}
		// Borrowed
		impl From<&$from_8> for $s {
			#[inline]
			fn from(number: &$from_8) -> Self {
				let n = *number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}
		impl From<&$from_16> for $s {
			#[inline]
			fn from(number: &$from_16) -> Self {
				let n = *number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}
		impl From<&$from_32> for $s {
			#[inline]
			fn from(number: &$from_32) -> Self {
				let n = *number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}
		impl From<&$from_size> for $s {
			#[inline]
			fn from(number: &$from_size) -> Self {
				let n = *number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}

		// Floats.
		impl From<f32> for $s {
			#[inline]
			fn from(number: f32) -> Self {
				#[cfg(not(feature = "ignore_nan_inf"))]
				{
					match number.classify() {
						::std::num::FpCategory::Normal   => (),
						::std::num::FpCategory::Nan      => return Self(number as $to, crate::inner::Inner::Nan),
						::std::num::FpCategory::Infinite => return Self(number as $to, crate::inner::Inner::Inf),
						_ => (),
					}
				}

				let n = number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}

		impl From<f64> for $s {
			#[inline]
			fn from(number: f64) -> Self {
				#[cfg(not(feature = "ignore_nan_inf"))]
				{
					match number.classify() {
						::std::num::FpCategory::Normal   => (),
						::std::num::FpCategory::Nan      => return Self(number as $to, crate::inner::Inner::Nan),
						::std::num::FpCategory::Infinite => return Self(number as $to, crate::inner::Inner::Inf),
						_ => (),
					}
				}

				let n = number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}

		// Borrowed.
		impl From<&f32> for $s {
			#[inline]
			fn from(number: &f32) -> Self {
				#[cfg(not(feature = "ignore_nan_inf"))]
				{
					match number.classify() {
						::std::num::FpCategory::Normal   => (),
						::std::num::FpCategory::Nan      => return Self(*number as $to, crate::inner::Inner::Nan),
						::std::num::FpCategory::Infinite => return Self(*number as $to, crate::inner::Inner::Inf),
						_ => (),
					}
				}

				let n = *number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}

		impl From<&f64> for $s {
			#[inline]
			fn from(number: &f64) -> Self {
				#[cfg(not(feature = "ignore_nan_inf"))]
				{
					match number.classify() {
						::std::num::FpCategory::Normal   => (),
						::std::num::FpCategory::Nan      => return Self(*number as $to, crate::inner::Inner::Nan),
						::std::num::FpCategory::Infinite => return Self(*number as $to, crate::inner::Inner::Inf),
						_ => (),
					}
				}

				let n = *number as $to;
				Self(n, crate::inner::Inner::Buf(crate::macros::num!(n)))
			}
		}
	}
}
pub(crate) use impl_from;

// Implement common functions.
macro_rules! impl_common {
	($num:ty) => {
		#[inline]
		/// The length of the inner [`String`]
		pub fn len(&self) -> usize {
			self.1.len()
		}

		#[inline]
		/// If the inner [`String`] is empty or not
		pub fn is_empty(&self) -> bool {
			self.1.is_empty()
		}

		#[inline]
		/// Return a borrowed [`str`] without consuming [`Self`].
		pub fn as_str(&self) -> &str {
			self.1.as_str()
		}

		#[inline]
		/// Return the bytes of the inner [`String`]
		pub fn as_bytes(&self) -> &[u8] {
			self.1.as_bytes()
		}

		#[inline]
		/// Return the bytes of the inner [`String`] as a [`Vec`]
		pub fn to_vec(&self) -> Vec<u8> {
			Vec::from(self.1.as_bytes())
		}

		#[inline]
		/// Return the bytes of the inner [`String`] as a [`Vec`], consuming [`Self`]
		pub fn into_vec(self) -> Vec<u8> {
			Vec::from(self.1.as_bytes())
		}

		#[inline]
		/// Returns the inner number.
		pub fn inner(&self) -> $num {
			self.0
		}

		#[inline]
		/// Consumes [`Self`], returning the inner [`String`].
		pub fn into_string(self) -> String {
			self.1.into_string()
		}

		#[inline]
		/// Consumes [`Self`], returning the inner parts.
		pub fn into_raw(self) -> ($num, String) {
			(self.0, self.1.into_string())
		}
	}
}
pub(crate) use impl_common;

// Implement inner `usize` function.
macro_rules! impl_usize {
	() => {
		#[inline]
		#[cfg(target_pointer_width = "64")]
		/// Returns the inner [`u64`] as a [`usize`].
		///
		/// # Notes
		/// This function is only available on 64-bit platforms.
		pub fn usize(&self) -> usize {
			self.0 as usize
		}
	}
}
pub(crate) use impl_usize;

// Implement inner `usize` function.
macro_rules! impl_isize {
	() => {
		#[inline]
		#[cfg(target_pointer_width = "64")]
		/// Returns the inner [`i64`] as an [`isize`].
		///
		/// # Notes
		/// This function is only available on 64-bit platforms.
		pub fn isize(&self) -> isize {
			self.0 as isize
		}
	}
}
pub(crate) use impl_isize;

// Implement common `Inner` functions.
macro_rules! impl_inner {
	($num:ident) => {
		#[inline]
		/// Returns a [`Self`] with the value `0`.
		pub fn zero() -> Self {
			Self(0, crate::inner::Inner::Zero)
		}

		#[inline]
		/// Returns a [`Self`] set to `0`, but the [`String`] set to `???`.
		pub fn unknown() -> Self {
			Self(0, crate::inner::Inner::Unknown)
		}

		#[inline(always)]
		pub fn is_zero(&self) -> bool {
			self.1.is_zero()
		}

		#[inline(always)]
		pub fn is_unknown(&self) -> bool {
			self.1.is_unknown()
		}

		#[inline(always)]
		pub fn is_nan(&self) -> bool {
			self.1.is_nan()
		}

		#[inline(always)]
		pub fn is_inf(&self) -> bool {
			self.1.is_inf()
		}
	}
}
pub(crate) use impl_inner;

// Implement traits.
macro_rules! impl_traits {
	($s:ty, $num:ty) => {
		impl std::fmt::Display for $s {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(f, "{}", &self.1.as_str())
			}
		}

		impl std::default::Default for $s {
			/// Calls [`Self::zero`]
			fn default() -> Self {
				Self::zero()
			}
		}

		impl AsRef<str> for $s {
			#[inline(always)]
			fn as_ref(&self) -> &str {
				self.1.as_str()
			}
		}

		impl std::borrow::Borrow<str> for $s {
			#[inline(always)]
			fn borrow(&self) -> &str {
				self.1.as_str()
			}
		}

		impl std::ops::Deref for $s {
			type Target = str;

			fn deref(&self) -> &Self::Target {
				&self.1.as_str()
			}
		}

		impl std::ops::Index<std::ops::Range<usize>> for $s {
			type Output = [u8];

			fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
				&self.as_bytes()[range]
			}
		}

		impl std::ops::Index<usize> for $s {
			type Output = u8;

			fn index(&self, byte: usize) -> &Self::Output {
				&self.as_bytes()[byte]
			}
		}

		impl PartialEq<str> for $s {
			fn eq(&self, other: &str) -> bool {
				self.1.as_str() == other
			}
		}

		impl PartialEq<$s> for str {
			fn eq(&self, other: &$s) -> bool {
				self == other.1.as_str()
			}
		}

		impl PartialEq<&str> for $s {
			fn eq(&self, other: &&str) -> bool {
				&self.1.as_str() == other
			}
		}

		impl PartialEq<&$s> for str {
			fn eq(&self, other: &&$s) -> bool {
				self == other.1.as_str()
			}
		}

		impl PartialEq<$num> for $s {
			fn eq(&self, other: &$num) -> bool {
				self.0 == *other
			}
		}

		impl PartialEq<$s> for $num {
			fn eq(&self, other: &$s) -> bool {
				*self == other.0
			}
		}

		impl PartialEq<$num> for &$s {
			fn eq(&self, other: &$num) -> bool {
				self.0 == *other
			}
		}

		impl PartialEq<&$s> for $num {
			fn eq(&self, other: &&$s) -> bool {
				*self == other.0
			}
		}
	}
}
pub(crate) use impl_traits;

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
