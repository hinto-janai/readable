//---------------------------------------------------------------------------------------------------- 1-off Buffer
// This quickly creates a `crate::num::buf::Buffer` for 1-off, quick formatting.

// Converts anything `i64` and below to a formatted `str`.
macro_rules! str_i64 {
	($number:expr) => {
		{
			let (buf, len) = crate::num::buf::from_i($number);
			crate::num::buf::Buffer { buf, len }.as_str()
		}
	}
}
pub(crate) use str_i64;

// Converts anything `u64` and below to a formatted `str`.
macro_rules! str_u64 {
	($number:expr) => {
		{
			let (buf, len) = crate::num::buf::from_u($number);
			crate::num::buf::Buffer { buf, len }.as_str()
		}
	}
}
pub(crate) use str_u64;

macro_rules! itoa {
	($number:expr) => {
		{
			crate::Itoa::new($number).as_bytes()
		}
	}
}
pub(crate) use itoa;

//---------------------------------------------------------------------------------------------------- Internal Buffer.
// Implement a private module `Buffer` type
// with a variable amount of array space.
macro_rules! buffer {
	($max_len:expr, $unknown_buffer:expr, $unknown_len:expr) => {
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
		#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
		struct Buffer {
			// Bytes representing a valid UTF-8 string.
			buf: [u8; $max_len],
			// How many bytes we're taking up.
			len: usize,
		}

		impl Buffer {
			#[inline]
			const fn unknown() -> Self {
				Self {
					buf: $unknown_buffer,
					len: $unknown_len,
				}
			}

			#[inline]
			#[allow(clippy::wrong_self_convention)]
			const fn to_buf(&self) -> [u8; $max_len] {
				self.buf
			}

			#[inline]
			const fn into_buf(self) -> [u8; $max_len] {
				self.buf
			}

			#[inline]
			const fn as_buf(&self) -> &[u8; $max_len] {
				&self.buf
			}

			#[inline]
			// Returns only the valid bytes.
			fn as_bytes(&self) -> &[u8] {
				&self.buf[..self.len]
			}

			#[inline]
			const fn is_empty(&self) -> bool {
				self.len == 0
			}

			#[inline]
			fn as_str(&self) -> &str {
				// SAFETY:
				// The buffer at this point should be
				// valid UTF-8 bytes representing integers.
				unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
			}

			#[inline]
			fn into_string(self) -> String {
				self.as_str().to_string()
			}

		    #[inline]
			const fn len(&self) -> usize {
				self.len
		    }
		}
	}
}
pub(crate) use buffer;

//---------------------------------------------------------------------------------------------------- NaN.
// "Handle NaN/Infinite" Macro for `compact_str`.
macro_rules! handle_nan_string {
	($float:ident) => {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			match $float.classify() {
				std::num::FpCategory::Normal   => (),
				std::num::FpCategory::Nan      => return Self(f64::NAN, ::compact_str::CompactString::new(crate::num::NAN)),
				std::num::FpCategory::Infinite => return Self(f64::INFINITY, ::compact_str::CompactString::new(crate::num::INFINITY)),
				_ => (),
			}
		}
	}
}
pub(crate) use handle_nan_string;

// "Handle NaN/Infinite"
//
// `fn handle_nan(unknown: Fn() -> Self, float: f32) -> Self`
macro_rules! handle_float {
	($unknown:expr, $float:ident) => {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			match $float.classify() {
				std::num::FpCategory::Normal   => (),
				std::num::FpCategory::Nan      => return $unknown(),
				std::num::FpCategory::Infinite => return $unknown(),
				_ => (),
			}
		}
	}
}
pub(crate) use handle_float;

//---------------------------------------------------------------------------------------------------- Impl.
// `Buffer`.
macro_rules! impl_buffer {
	($max_len:expr, $unknown_buffer:expr, $unknown_len:expr) => {
		#[inline(always)]
		/// Return the _full_ inner buffer that represents the [`String`].
		///
		/// These are guaranteed to be valid UTF-8 bytes.
		///
		/// Not all bytes are necessarily used, however.
		/// To find the valid portion of the string, use [`Self::len`].
		/// ```rust
		/// # use readable::Unsigned;
		/// let u           = Unsigned::from(123_u8);
		/// let buffer      = u.to_buf();
		/// let valid_bytes = &buffer[0..u.len()];
		///
		/// // SAFETY: These bytes are always be valid UTF-8.
		/// unsafe {
		///     let specified = std::str::from_utf8_unchecked(&valid_bytes);
		///     let all_bytes = std::str::from_utf8_unchecked(&buffer);
		///
		///     // Bunch of trailing `\0\0\0`'s at the end.
		///     assert!(all_bytes != "123");
		///     assert!(specified == "123");
		/// }
		/// ```
		pub const fn to_buf(&self) -> [u8; $max_len] {
			self.1.to_buf()
		}

		/// Same as [`Self::to_buf`] but consumes self.
		pub const fn into_buf(self) -> [u8; $max_len] {
			self.1.into_buf()
		}

		#[inline(always)]
		/// Same as [`Self::to_buf`] but returns a borrowed array.
		pub const fn as_buf(&self) -> &[u8; $max_len] {
			&self.1.as_buf()
		}

		#[inline(always)]
		/// Same as [`Self::to_buf`] but returns the length as well.
		pub const fn to_buf_parts(&self) -> ([u8; $max_len], usize) {
			(self.1.to_buf(), self.1.len())
		}

		#[inline(always)]
		/// Same as [`Self::into_buf`] but returns the length as well.
		pub const fn into_buf_parts(self) -> ([u8; $max_len], usize) {
			(self.1.into_buf(), self.1.len())
		}
	}
}
pub(crate) use impl_buffer;


// Implement common const functions.
macro_rules! impl_const {
	() => {
		#[inline]
		/// The length of the inner [`String`]
		pub const fn len(&self) -> usize {
			self.1.len()
		}

		#[inline]
		/// If the inner [`String`] is empty or not
		pub const fn is_empty(&self) -> bool {
			self.1.is_empty()
		}
	}
}
pub(crate) use impl_const;

// Implement common non-const functions.
macro_rules! impl_not_const {
	() => {
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
	}
}
pub(crate) use impl_not_const;

// Implement common functions.
macro_rules! impl_common {
	($num:ty) => {
		#[inline]
		/// Return a borrowed [`str`] without consuming [`Self`].
		pub fn as_str(&self) -> &str {
			self.1.as_str()
		}

		#[inline]
		/// Returns the _valid_ byte slice of the inner [`String`]
		///
		/// These bytes can _always_ safely be used for [`std::str::from_utf8_unchecked`].
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
		pub const fn inner(&self) -> $num {
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
		/// Returns the inner number as a [`usize`].
		///
		/// # Notes
		/// This function is only available on 64-bit platforms.
		pub const fn usize(&self) -> usize {
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
		/// Returns the inner number as an [`isize`].
		///
		/// # Notes
		/// This function is only available on 64-bit platforms.
		pub const fn isize(&self) -> isize {
			self.0 as isize
		}
	}
}
pub(crate) use impl_isize;

// Implement traits.
macro_rules! impl_traits {
	($s:ty, $num:ty) => {
		impl std::ops::Deref for $s {
			type Target = str;

			fn deref(&self) -> &Self::Target {
				self.as_str()
			}
		}

		impl AsRef<str> for $s {
			fn as_ref(&self) -> &str {
				self.as_str()
			}
		}

		impl AsRef<[u8]> for $s {
			fn as_ref(&self) -> &[u8] {
				self.as_bytes()
			}
		}

		impl std::borrow::Borrow<str> for $s {
			fn borrow(&self) -> &str {
				self.as_str()
			}
		}

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

		impl PartialEq<&$s> for $s {
			fn eq(&self, other: &&$s) -> bool {
				self == other
			}
		}

		impl PartialEq<$s> for &$s {
			fn eq(&self, other: &$s) -> bool {
				self == other
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

		// Ord
		impl PartialOrd<str> for $s {
			fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
				Some(self.1.as_str().cmp(other))
			}
		}

		impl PartialOrd<$s> for str {
			fn partial_cmp(&self, other: &$s) -> Option<std::cmp::Ordering> {
				Some(self.cmp(other.1.as_str()))
			}
		}

		impl PartialOrd<&str> for $s {
			fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
				Some(self.1.as_str().cmp(other))
			}
		}

		impl PartialOrd<&$s> for str {
			fn partial_cmp(&self, other: &&$s) -> Option<std::cmp::Ordering> {
				Some(self.cmp(other.1.as_str()))
			}
		}

		impl PartialOrd<$num> for $s {
			fn partial_cmp(&self, other: &$num) -> Option<std::cmp::Ordering> {
				self.0.partial_cmp(other)
			}
		}

		impl PartialOrd<$s> for $num {
			fn partial_cmp(&self, other: &$s) -> Option<std::cmp::Ordering> {
				self.partial_cmp(&other.0)
			}
		}

		impl PartialOrd<$num> for &$s {
			fn partial_cmp(&self, other: &$num) -> Option<std::cmp::Ordering> {
				self.0.partial_cmp(other)
			}
		}

		impl PartialOrd<&$s> for $num {
			fn partial_cmp(&self, other: &&$s) -> Option<std::cmp::Ordering> {
				self.partial_cmp(&other.0)
			}
		}
	}
}
pub(crate) use impl_traits;

// Macro for a math macro impl.
macro_rules! impl_impl_math {
	($trait_word:ident, $operator:tt, $s:ty, $num:ty) => {
		paste::item! {
			// Standard ops.
			impl std::ops::[<$trait_word>]<$s> for $s {
				type Output = Self;
				fn [<$trait_word:lower>](self, other: $s) -> Self {
					let r = self.inner() $operator other.inner();
					Self::from(r)
				}
			}
			impl std::ops::[<$trait_word>]<$num> for $s {
				type Output = Self;
				fn [<$trait_word:lower>](self, other: $num) -> Self {
					Self::from(self.inner() $operator other)
				}
			}
			impl std::ops::[<$trait_word>]<$s> for $num {
				type Output = Self;
				fn [<$trait_word:lower>](self, other: $s) -> Self {
					Self::from(self $operator other.inner())
				}
			}
			impl std::ops::[<$trait_word>]<&$s> for $s {
				type Output = Self;
				fn [<$trait_word:lower>](self, other: &$s) -> Self {
					Self::from(self.inner() $operator other.inner())
				}
			}
			impl std::ops::[<$trait_word>]<&$num> for $s {
				type Output = Self;
				fn [<$trait_word:lower>](self, other: &$num) -> Self {
					Self::from(self.inner() $operator other)
				}
			}
			impl std::ops::[<$trait_word>]<&$s> for $num {
				type Output = Self;
				fn [<$trait_word:lower>](self, other: &$s) -> Self {
					Self::from(self $operator other.inner())
				}
			}

			// Assign ops.
			// TODO:
			// These types are meant to be immutable so
			// creating a new value with normal operators
			// instead of assigned seems more correct.
//			impl std::ops::[<$trait_word Assign>]<$s> for $s {
//				fn [<$trait_word:lower _assign>](&mut self, other: $s) {
//					*self = Self::from(self.inner() $operator other.inner())
//				}
//			}
//			impl std::ops::[<$trait_word Assign>]<&$s> for $s {
//				fn [<$trait_word:lower _assign>](&mut self, other: &$s) {
//					*self = Self::from(self.inner() $operator other.inner())
//				}
//			}
//			impl std::ops::[<$trait_word Assign>]<$num> for $s {
//				fn [<$trait_word:lower _assign>](&mut self, other: $num) {
//					*self = Self::from(self.inner() $operator other)
//				}
//			}
//			impl std::ops::[<$trait_word Assign>]<&$num> for $s {
//				fn [<$trait_word:lower _assign>](&mut self, other: &$num) {
//					*self = Self::from(self.inner() $operator other)
//				}
//			}
		}
	}
}
pub(crate) use impl_impl_math;

// Implement math operators.
macro_rules! impl_math {
	($s:ty, $num:ty) => {
		impl_impl_math!(Add, +, $s, $num);
		impl_impl_math!(Sub, -, $s, $num);
		impl_impl_math!(Div, /, $s, $num);
		impl_impl_math!(Mul, *, $s, $num);
		impl_impl_math!(Rem, %, $s, $num);
	}
}
pub(crate) use impl_math;

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
