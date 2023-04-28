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
	($max_len:expr, $unknown_buffer:expr, $unknown_len:expr) => {
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
		struct Buffer {
			// Bytes representing a valid UTF-8 string.
			buf: [u8; $max_len],
			// How many bytes we're taking up.
			len: usize,
		}

		impl Buffer {
			#[inline(always)]
			const fn unknown() -> Self {
				Self {
					buf: $unknown_buffer,
					len: $unknown_len,
				}
			}

			#[inline(always)]
			const fn to_buffer(&self) -> [u8; $max_len] {
				self.buf
			}

			#[inline(always)]
			const fn into_buffer(self) -> [u8; $max_len] {
				self.buf
			}

			#[inline(always)]
			const fn as_buffer(&self) -> &[u8; $max_len] {
				&self.buf
			}

			#[inline(always)]
			// Returns only the valid bytes.
			fn as_bytes(&self) -> &[u8] {
				&self.buf[..self.len]
			}

			#[inline(always)]
			const fn is_empty(&self) -> bool {
				self.len == 0
			}

			#[inline(always)]
			fn as_str(&self) -> &str {
				// SAFETY:
				// The buffer at this point should be
				// valid UTF-8 bytes representing integers.
				unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
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
			const fn len(&self) -> usize {
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
				std::num::FpCategory::Normal   => (),
				std::num::FpCategory::Nan      => return Self(f64::NAN, crate::inner::Inner::Nan(crate::constants::NAN)),
				std::num::FpCategory::Infinite => return Self(f64::INFINITY, crate::inner::Inner::Inf(crate::constants::INFINITY)),
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
				std::num::FpCategory::Normal   => (),
				std::num::FpCategory::Nan      => return Self(f64::NAN, ::compact_str::CompactString::new(crate::constants::NAN)),
				std::num::FpCategory::Infinite => return Self(f64::INFINITY, ::compact_str::CompactString::new(crate::constants::INFINITY)),
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
				std::num::FpCategory::Normal   => (),
				std::num::FpCategory::Nan      => return Self::unknown(),
				std::num::FpCategory::Infinite => return Self::unknown(),
				_ => (),
			}
		}
	}
}
pub(crate) use handle_nan_runtime;

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
		/// let buffer      = u.to_buffer();
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
		pub const fn to_buffer(&self) -> [u8; $max_len] {
			self.1.to_buffer()
		}

		/// Same as [`Self::to_buffer`] but consumes self.
		pub const fn into_buffer(self) -> [u8; $max_len] {
			self.1.into_buffer()
		}

		#[inline(always)]
		/// Same as [`Self::to_buffer`] but returns a borrowed array.
		pub const fn as_buffer(&self) -> &[u8; $max_len] {
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
						std::num::FpCategory::Normal   => (),
						std::num::FpCategory::Nan      => return Self(number as $to, crate::inner::Inner::Nan),
						std::num::FpCategory::Infinite => return Self(number as $to, crate::inner::Inner::Inf),
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
						std::num::FpCategory::Normal   => (),
						std::num::FpCategory::Nan      => return Self(number as $to, crate::inner::Inner::Nan),
						std::num::FpCategory::Infinite => return Self(number as $to, crate::inner::Inner::Inf),
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
						std::num::FpCategory::Normal   => (),
						std::num::FpCategory::Nan      => return Self(*number as $to, crate::inner::Inner::Nan),
						std::num::FpCategory::Infinite => return Self(*number as $to, crate::inner::Inner::Inf),
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
						std::num::FpCategory::Normal   => (),
						std::num::FpCategory::Nan      => return Self(*number as $to, crate::inner::Inner::Nan),
						std::num::FpCategory::Infinite => return Self(*number as $to, crate::inner::Inner::Inf),
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

		/// Return the first `len` bytes of this [`str`].
		///
		/// This will return the full [`str`] if the `len` is
		/// longer than the actual inner [`str`].
		///
		/// Since all `readable` types happen to only contain
		/// ASCII characters, all [`char`]'s are equal to `1` byte.
		/// ```rust
		/// # use readable::*;
		/// let date = Date::from_str("2021-12-11").unwrap();
		///
		/// assert!(date.head(5) == "2021-");
		/// ```
		pub fn head(&self, len: usize) -> &str {
			let s = self.as_str();

			if len >= s.len() {
				s
			} else {
				&s[..len]
			}
		}

		/// Same as [`Self::head`] but returns a [`String`] ending with `...`
		///
		/// This will return the full string without `...` if
		/// the `len` is longer than the actual inner [`str`].
		/// ```rust
		/// # use readable::*;
		/// let date = Date::from_str("2021-12-11").unwrap();
		///
		/// assert!(date.head_dot(4) == "2021...");
		/// ```
		pub fn head_dot(&self, len: usize) -> String {
			let s = self.as_str();

			if len >= s.len() {
				s.to_string()
			} else {
				format!("{}...", &s[..len])
			}
		}

		/// Return the last `len` bytes of this [`str`].
		///
		/// This will return the full [`str`] if the `len` is
		/// longer than the actual inner [`str`].
		///
		/// Since all `readable` types happen to only contain
		/// ASCII characters, all [`char`]'s are equal to `1` byte.
		/// ```rust
		/// # use readable::*;
		/// let date = Date::from_str("2021-12-11").unwrap();
		///
		/// assert!(date.tail(5) == "12-11");
		/// ```
		pub fn tail(&self, len: usize) -> &str {
			let s = self.as_str();
			let s_len = s.len();

			if len >= s_len {
				s
			} else {
				&s[s_len - len..]
			}
		}

		/// Same as [`Self::tail`] but returns a [`String`] starting with `...`
		///
		/// This will return the full string without `...` if
		/// the `len` is longer than the actual inner [`str`].
		/// ```rust
		/// # use readable::*;
		/// let date = Date::from_str("2021-12-11").unwrap();
		///
		/// assert!(date.tail_dot(5) == "...12-11");
		/// ```
		pub fn tail_dot(&self, len: usize) -> String {
			let s = self.as_str();
			let s_len = s.len();

			if len >= s_len {
				s.to_string()
			} else {
				format!("...{}", &s[s_len - len..])
			}
		}

		/// Return the first `head` bytes and last `tail`
		/// bytes of this string separated with `...`.
		///
		/// Since all `readable` types happen to only contain
		/// ASCII characters, all [`char`]'s are equal to `1` byte.
		/// ```rust
		/// # use readable::*;
		/// let date = Date::from_str("2021-12-11").unwrap();
		///
		/// assert!(date.head_tail(3, 2) == "202...11");
		/// assert!(date.head_tail(3, 3) == "202...-11");
		/// assert!(date.head_tail(3, 5) == "202...12-11");
		/// ```
		pub fn head_tail(&self, head: usize, tail: usize) -> String {
			let s = self.as_str();
			let len = s.len();

			if head > len || tail > len {
				return s.to_string();
			}

			format!("{}...{}", &s[..head], &s[len - tail..])
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

// Implement common `Inner` functions.
macro_rules! impl_inner {
	($num:ident) => {
		#[inline]
		/// Returns a [`Self`] with the value `0`.
		pub const fn zero() -> Self {
			Self(0, crate::inner::Inner::Zero)
		}

		#[inline]
		/// Returns a [`Self`] set to `0`, but the [`String`] set to `???`.
		pub const fn unknown() -> Self {
			Self(0, crate::inner::Inner::Unknown)
		}

		#[inline(always)]
		/// Returns true if [`Self`] is [`Self::zero`].
		pub fn is_zero(&self) -> bool {
			self.1.is_zero()
		}

		#[inline(always)]
		/// Returns true if [`Self`] is [`Self::unknown`].
		pub fn is_unknown(&self) -> bool {
			self.1.is_unknown()
		}

		#[inline(always)]
		/// Returns true if [`Self`] is `NAN`.
		pub fn is_nan(&self) -> bool {
			self.1.is_nan()
		}

		#[inline(always)]
		/// Returns true if [`Self`] is `INF`.
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
