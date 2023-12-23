//---------------------------------------------------------------------------------------------------- Use
// use bincode::{Encode,Decode};
// use serde::{Serialize,serde::Deserialize};
// use anyhow::anyhow;
// use log::{error,info,warn,debug,trace};
// use disk::{Bincode2,Json};
use std::sync::Arc;
use std::rc::Rc;
use std::borrow::Cow;

//---------------------------------------------------------------------------------------------------- Str
/// A fixed sized stack string
///
/// [`Str`] is a generic stack-based string with a maximum byte length of [`u8::MAX`].
///
/// The generic `N` is a [`usize`] and represents the maximum length of the string,
/// however all constructor functions for [`Str`] will panic at _compile time_ if `N > 255`.
///
/// ## Size
/// The internal length is stored as a [`u8`], and as such will
/// take minimal space, allowing for longer strings to be stored.
///
/// Due to `#[repr(C)]`, `N + 1` is how many bytes your [`Str`] will take up.
///
/// Using [`Str`] in powers of 2 is recommended.
/// ```rust
/// # use readable::str::*;
/// // 64 bytes in total, 63 bytes available for the string.
/// // This will fit in a typical CPU cache-line.
/// assert_eq!(std::mem::size_of::<Str::<63>>(), 64);
///
/// // Maximum string length of 255 fits into 256 bytes.
/// assert_eq!(std::mem::size_of::<Str::<255>>(), 256);
///
/// // Beware, due to `#[repr(C)]`, `Str` is not
/// // automatically re-arranged and padded by Rust.
/// assert_eq!(std::mem::size_of::<Str::<6>>(), 7);
/// ```
///
/// ## Compile-time panic
/// Any usage of [`Str`] will panic at compile time if `N > 255`:
/// ```rust,ignore
/// # use readable::str::*;
/// /// These will all panic at _compile time_
/// Str::<256>::new();
/// Str::<256>::try_from("");
/// Str::<256>::from_static_str("");
/// Str::<256>::from_static_bytes(b"");
/// ```
///
/// ## Usage
/// ```rust
/// # use readable::str::*;
/// // Create a `Str` with a maximum capacity of `24` bytes.
/// const N: usize  = 24;
/// let mut string = Str::<N>::new();
/// assert!(string.is_empty());
///
/// // Copy the bytes from an actual `str`
/// let other_str = "this str is 24 bytes :-)";
/// assert_eq!(other_str.len(), N);
/// string.copy_str(other_str).unwrap();
///
/// // They're the same.
/// assert_eq!(string, other_str);
///
/// // Clear the string.
/// string.clear();
/// assert!(string.is_empty());
/// assert_eq!(string.len(), 0);
///
/// // `push_str()` should be the exact same.
/// string.push_str(other_str).unwrap();
/// assert_eq!(string, other_str);
///
/// // This string is full.
/// assert!(string.is_full());
/// assert_eq!(string.len(), N);
///
/// // Pushing new strings will error.
/// let err = string.push_str(other_str);
/// assert_eq!(err, Err(24));
/// // Still the same.
/// assert_eq!(string, other_str);
///
/// // Although, we can still overwrite it.
/// string.copy_str("hello-------------------");
/// assert_eq!(string, "hello-------------------");
/// assert_eq!(string.len(), 24);
/// ```
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct Str<const N: usize> {
	buf: [u8; N],
	len: u8,
}

//---------------------------------------------------------------------------------------------------- Impl
impl<const N: usize> Str<N> {
	/// The maximum length of this string as a [`u8`].
	///
	/// This should `==` to `N` in valid cases.
	///
	/// ## Compile-time panic
	/// This associated constant will cause [`Str`] constructor
	/// functions to panic at compile time is `N > 255`.
	pub const CAPACITY: u8 = {
		if N > u8::MAX as usize {
			panic!("N must not be greater than 255");
		} else {
			N as u8
		}
	};

	#[inline]
	#[must_use]
	/// Returns an empty [`Str`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// let string = Str::<4>::new();
	/// assert!(string.is_empty());
	/// assert_eq!(string.len(), 0);
	/// assert!(string.as_str().is_empty());
	/// assert_eq!(string.as_str().len(), 0);
	/// ```
	pub const fn new() -> Self {
		// Will cause panics at compile time.
		Self::CAPACITY;

		Self {
			buf: [0; N],
			len: 0,
		}
	}

	#[must_use]
	#[allow(clippy::missing_panics_doc)] // compile-time
	/// Create a [`Self`] from static bytes.
	///
	/// The length of the input doesn't need to be the
	/// same as `N`, it just needs to be equal or less.
	///
	/// Exact length:
	/// ```rust
	/// # use readable::str::*;
	/// const BYTES: [u8; 3] = *b"abc";
	/// const STR: Str<3> = Str::from_static_bytes(&BYTES);
	///
	/// assert_eq!(STR, "abc");
	/// ```
	/// Slightly less length is okay too:
	/// ```rust
	/// # use readable::str::*;
	/// const BYTES: [u8; 2] = *b"ab";
	/// const STR: Str<3> = Str::from_static_bytes(&BYTES);
	///
	/// assert_eq!(STR.len(), 2);
	/// assert_eq!(STR, "ab");
	/// ```
	///
	/// # Compile-time panic
	/// This function will panic at compile time if either:
	/// - The `byte` length is longer than `N`
	/// - The byte's are not valid UTF-8 bytes
	///
	/// ```rust,ignore
	/// # use readable::str::*;
	/// // This doesn't fit, will panic at compile time.
	/// const STR: Str<3> = Str::from_static_bytes("abcd");
	/// ```
	pub const fn from_static_bytes(bytes: &'static [u8]) -> Self {
		// Will cause panics at compile time.
		Self::CAPACITY;

		let len = bytes.len();

		assert!(len <= N, "byte length is longer than N");
		assert!(std::str::from_utf8(bytes).is_ok(), "bytes are not valid UTF-8");

		let mut buf = [0_u8; N];

		let mut i = 0;
		while i < len {
			buf[i] = bytes[i];
			i += 1;
		}

		Self {
			buf,
			len: len as u8,
		}
	}

	#[must_use]
	/// Create a [`Self`] from a static [`str`].
	///
	/// The length of the input doesn't need to be the
	/// same as `N`, it just needs to be equal or less.
	///
	/// Exact length:
	/// ```rust
	/// # use readable::str::*;
	/// const S: &str = "abc";
	/// const STR: Str<3> = Str::from_static_str(&S);
	///
	/// assert_eq!(STR, "abc");
	/// ```
	/// Slightly less length is okay too:
	/// ```rust
	/// # use readable::str::*;
	/// const S: &str = "ab";
	/// const STR: Str<3> = Str::from_static_str(&S);
	///
	/// assert_eq!(STR.len(), 2);
	/// assert_eq!(STR, "ab");
	/// ```
	///
	/// ## Compile-time panic
	/// This function will panic at compile time
	/// if the [`str`] length is longer than `N`.
	///
	/// ```rust,ignore
	/// # use readable::str::*;
	/// // This doesn't fit, will panic at compile time.
	/// const STR: Str<3> = Str::from_static_str("abcd");
	/// ```
	pub const fn from_static_str(s: &'static str) -> Self {
		Self::from_static_bytes(s.as_bytes())
	}

	#[inline]
	#[must_use]
	/// Return all the bytes of this [`Str`], whether valid UTF-8 or not.
	///
	/// ``` rust
	/// # use readable::str::*;
	/// let mut string = Str::<10>::new();
	/// string.push_str("hello").unwrap();
	///
	/// // The string length is 5, but the slice
	/// // returned is the full capacity, 10.
	/// assert_eq!(string.as_bytes_all().len(), 10);
	/// ```
	pub const fn as_bytes_all(&self) -> &[u8] {
		self.buf.as_slice()
	}

	#[inline]
	#[must_use]
	/// Return all the bytes of this [`Str`] (mutably), whether valid UTF-8 or not
	///
	/// ## Safety
	/// The caller must ensure that the content of the slice is valid
	/// UTF-8 before the borrow ends and the underlying [`Str`] is used.
	///
	/// The caller must also ensure the `len` is correctly set
	/// with [`Str::set_len`] or [`Str::set_len_u8`].
	///
	/// ``` rust
	/// # use readable::str::*;
	/// let mut string = Str::<5>::new();
	/// string.push_str("hi").unwrap();
	/// assert_eq!(string, "hi");
	/// assert_eq!(string.len(), 2);
	///
	/// // Safety: We must ensure we leave
	/// // leave the bytes as valid UTF-8 bytes
	/// // and that we set the length correctly.
	/// unsafe {
	///     // Mutate to valid UTF-8 bytes.
	///     let mut_ref = string.as_bytes_all_mut();
	///     mut_ref.copy_from_slice(&b"world"[..]);
	///     // Set the new length.
	///     string.set_len(5);
	/// }
	///
	/// assert_eq!(string, "world");
	/// assert_eq!(string.len(), 5);
	/// ```
	pub unsafe fn as_bytes_all_mut(&mut self) -> &mut [u8] {
		self.buf.as_mut_slice()
	}

	#[inline]
	#[must_use]
	/// Return the length of the _valid_ UTF-8 bytes of this [`Str`]
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<5>::new();
	/// s.push_str("h").unwrap();
	/// assert_eq!(s.len(), 1_usize);
	///
	/// s.push_str("ello").unwrap();
	/// assert_eq!(s.len(), 5_usize);
	/// ```
	pub const fn len(&self) -> usize {
		self.len as usize
	}

	#[inline]
	#[must_use]
	/// Return the length of the _valid_ UTF-8 bytes of this [`Str`] as a [`u8`]
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<5>::new();
	/// s.push_str("h").unwrap();
	/// assert_eq!(s.len_u8(), 1_u8);
	///
	/// s.push_str("ello").unwrap();
	/// assert_eq!(s.len_u8(), 5_u8);
	/// ```
	pub const fn len_u8(&self) -> u8 {
		self.len
	}

	#[inline]
	/// Set the length of the _valid_ UTF-8 bytes of this [`Str`]
	///
	/// This will usually be used when manually mutating [`Str`] with [`Str::as_bytes_all_mut()`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<3>::new();
	/// assert_eq!(s.len(), 0);
	///
	/// unsafe { s.set_len(3); } // <- Using the `Str`
	/// assert_eq!(s.len(), 3);  //    beyond this point
	///                          //    is a bad idea.
	///
	/// // This wouldn't be undefined behavior,
	/// // but the inner buffer is all zeros.
	/// assert_eq!(s.as_str(), "\0\0\0");
	///
	/// // Overwrite the bytes.
	/// unsafe {
	///     let mut_ref = s.as_bytes_all_mut();
	///     mut_ref[0] = b'a';
	///     mut_ref[1] = b'b';
	///     mut_ref[2] = b'c';
	/// }
	/// // Should be safe from this point.
	/// assert_eq!(s.as_str(), "abc");
	/// assert_eq!(s.len(),    3);
	/// ```
	///
	/// ## Safety
	/// Other functions will rely on the internal length
	/// to be correct, so the caller must ensure this length
	/// is actually correct.
	pub unsafe fn set_len(&mut self, len: usize) {
		self.len = len as u8;
	}

	#[inline]
	/// Set the length of the _valid_ UTF-8 bytes of this [`Str`]
	///
	/// This will usually be used when manually mutating [`Str`] with [`Str::as_bytes_all_mut()`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<3>::new();
	/// assert_eq!(s.len(), 0);
	///
	/// unsafe { s.set_len_u8(3); } // <- Using the `Str`
	/// assert_eq!(s.len(), 3);     //    beyond this point
	///                             //    is a bad idea.
	///
	/// // This wouldn't be undefined behavior,
	/// // but the inner buffer is all zeros.
	/// assert_eq!(s.as_str(), "\0\0\0");
	///
	/// // Overwrite the bytes.
	/// unsafe {
	///     let mut_ref = s.as_bytes_all_mut();
	///     mut_ref[0] = b'a';
	///     mut_ref[1] = b'b';
	///     mut_ref[2] = b'c';
	/// }
	/// // Should be safe from this point.
	/// assert_eq!(s.as_str(), "abc");
	/// assert_eq!(s.len(),    3);
	/// ```
	///
	/// ## Safety
	/// Other functions will rely on the internal length
	/// to be correct, so the caller must ensure this length
	/// is actually correct.
	pub unsafe fn set_len_u8(&mut self, len: u8) {
		self.len = len;
	}

	#[inline]
	#[must_use]
	/// How many available bytes are left in this [`Str`]
	/// before the [`Self::CAPACITY`] is completely filled.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<5>::new();
	/// s.push_str("hi");
	/// assert_eq!(s.remaining(), 3);
	/// ```
	pub const fn remaining(&self) -> usize {
		(Self::CAPACITY - self.len) as usize
	}

	#[inline]
	#[must_use]
	/// Returns only the valid `UTF-8` bytes of this [`Str`] as a byte slice.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let s = Str::<10>::from_static_str("hello");
	/// assert_eq!(s.as_bytes().len(), 5);
	/// ```
	pub const fn as_bytes(&self) -> &[u8] {
		// SAFETY: we trust `.len()`.
		unsafe {
			std::slice::from_raw_parts(
				self.as_ptr(),
				self.len(),
			)
		}
	}

	#[inline]
	#[must_use]
	/// [`Self::as_bytes()`], but returns mutable bytes
	///
	/// ## Safety
	/// The length must be set correctly if mutated.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<10>::from_static_str("hello");
	/// assert_eq!(s.as_bytes().len(), 5);
	///
	/// unsafe {
	///
	///     // Length not set yet.
	///     s.as_bytes_mut().copy_from_slice(&[0; 5]);
	///     assert_eq!(s.as_bytes_mut().len(), 5);
	///
	///     // Set.
	///     s.set_len(0);
	/// }
	///
	/// assert_eq!(s.as_str(),         "");
	/// assert_eq!(s.as_bytes().len(), 0);
	/// ```
	pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
		// SAFETY: we trust `.len()`.
		unsafe {
			std::slice::from_raw_parts_mut(
				self.as_mut_ptr(),
				self.len(),
			)
		}
	}

	#[inline]
	#[must_use]
	/// Returns a pointer to the first byte in the string array.
	/// ```rust
	/// # use readable::str::*;
	/// let s = Str::<5>::from_static_str("hello");
	///
	/// let ptr = s.as_ptr();
	/// unsafe {
	///     // The first byte is the char `h`.
	///     assert_eq!(*ptr, b'h');
	/// }
	/// ```
	pub const fn as_ptr(&self) -> *const u8 {
		self.buf.as_ptr()
	}

	#[inline]
	/// Returns a mutable pointer to the first byte in the string array.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<5>::from_static_str("hello");
	///
	/// let ptr = s.as_mut_ptr();
	/// unsafe {
	///     // The first byte is the char `h`.
	///     assert_eq!(*ptr, b'h');
	///     // Let's change it.
	///     *ptr = b'e';
	/// }
	///
	/// assert_eq!(s, "eello");
	/// ```
	pub fn as_mut_ptr(&mut self) -> *mut u8 {
		self.buf.as_mut_ptr()
	}

	#[inline]
	#[must_use]
	/// Returns only the valid `UTF-8` bytes of this [`Str`] as a `Vec<u8>`
	///
	/// ```rust
	/// # use readable::str::*;
	/// let s = Str::<10>::from_static_str("hello");
	/// let v = s.into_vec();
	/// assert_eq!(v.len(), 5);
	///
	/// let s = unsafe { String::from_utf8_unchecked(v) };
	/// assert_eq!(s, "hello");
	/// ```
	pub fn into_vec(self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}

	#[must_use]
	/// Check this [`Str`] for correctness.
	///
	/// When constructing/receiving a [`Str`] outside of
	/// its constructors, it may not be guaranteed that
	/// the invariants are upheld.
	///
	/// This function will return `true` if:
	/// - Internal length is greater than the internal byte array
	/// - `.as_str()` would return invalid UTF-8
	///
	/// ```rust
	/// # use readable::str::*;
	/// // Create `Str` with maximum 5 length.
	/// let mut string = Str::<5>::new();
	/// assert_eq!(string.invalid(), false);
	///
	/// // Unsafely set the length to 10.
	/// unsafe { string.set_len(10); }
	/// // This string is now invalid.
	/// assert_eq!(string.invalid(), true);
	/// ```
	pub const fn invalid(&self) -> bool {
		let len     = self.len as usize;
		let buf_len = self.buf.len();

		len > buf_len || std::str::from_utf8(self.as_bytes()).is_err()
	}

	#[inline]
	/// Clears all bytes of this [`Str`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// // Create a string.
	/// let mut s = Str::<5>::from_static_str("hello");
	/// assert_eq!(s, "hello");
	///
	/// // Clear the string.
	/// s.clear();
	/// assert_eq!(s, "");
	/// assert!(s.is_empty());
	/// ```
	///
	/// ## Note
	/// This does not actually mutate any bytes,
	/// it simply sets the internal length to `0`.
	///
	/// Do not rely on this to clear the actual bytes.
	pub fn clear(&mut self) {
		// SAFETY: We are manually setting the length.
		unsafe { self.set_len(0); }
	}

	/// Zeros all bytes of this [`Str`] and sets the length to `0`
	///
	/// Unlike [`Str::clear()`], this actually sets all
	/// the bytes in the internal array to `0`.
	///
	/// ```rust
	/// # use readable::str::*;
	/// // Create a string.
	/// let mut s = Str::<5>::from_static_str("hello");
	/// assert_eq!(s, "hello");
	///
	/// // Zero the string.
	/// s.zero();
	/// assert_eq!(s, "");
	/// assert!(s.is_empty());
	/// ```
	pub fn zero(&mut self) {
		// should be a fast 0 memset.
		// https://github.com/rust-lang/rfcs/issues/2067
		self.buf.fill(0);

		// SAFETY: We are manually setting the length.
		unsafe { self.set_len(0); }
	}

	#[inline]
	#[must_use]
	/// If this [`Str`] is empty.
	///
	/// ``` rust
	/// # use readable::str::*;
	/// let mut s = Str::<10>::new();
	/// assert_eq!(s, "");
	/// assert!(s.is_empty());
	///
	/// s.push_str("a").unwrap();
	/// assert!(!s.is_empty());
	/// ```
	pub const fn is_empty(&self) -> bool {
		self.len == 0
	}

	#[inline]
	#[must_use]
	/// If this [`Str`] is full (no more capacity left).
	///
	/// ``` rust
	/// # use readable::str::*;
	/// let mut s = Str::<3>::new();
	/// assert_eq!(s.len(), 0);
	/// assert!(!s.is_full());
	///
	/// s.push_str("123").unwrap();
	/// assert_eq!(s.len(), 3);
	/// assert!(s.is_full());
	/// ```
	pub const fn is_full(&self) -> bool {
		self.len == Self::CAPACITY
	}

	#[inline]
	#[must_use]
	/// This [`Str`], as a valid UTF-8 [`str`].
	///
	/// ``` rust
	/// # use readable::str::*;
	/// let s = Str::<5>::from_static_str("hello");
	/// assert_eq!(s.as_str(), "hello");
	/// ```
	///
	/// # Panics
	/// This will panic in debug mode if [`Self::invalid`] returns true.
	pub const fn as_str(&self) -> &str {
		debug_assert!(
			!self.invalid(),
			"Str::invalid() returned true, inner str is corrupt"
		);

		// SAFETY: `.as_valid_slice()` must be correctly implemented.
		// The internal state must be correct.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}

	#[inline]
	/// This [`Str`], as a valid, mutable, UTF-8 [`str`].
	///
	/// ## Safety
	/// The length must be set correctly if mutated.
	///
	/// The `str` must be valid UTF-8.
	///
	/// ``` rust
	/// # use readable::str::*;
	/// let mut s = Str::<5>::from_static_str("hello");
	/// assert_eq!(s.as_str(), "hello");
	///
	/// unsafe {
	///     s.as_str_mut().make_ascii_uppercase();
	/// }
	///
	/// assert_eq!(s.as_str(), "HELLO");
	/// ```
	pub unsafe fn as_str_mut(&mut self) -> &mut str {
		// SAFETY: `.as_valid_slice()` must be correctly implemented.
		// The internal state must be correct.
		unsafe { std::str::from_utf8_unchecked_mut(self.as_bytes_mut()) }
	}

	#[inline]
	#[must_use]
	/// Consumes `self` into a [`String`]
	///
	/// ``` rust
	/// # use readable::str::*;
	/// let s = Str::<5>::from_static_str("hello");
	///
	/// let s: String = s.into_string();
	/// assert_eq!(s, "hello");
	/// ```
	pub fn into_string(self) -> String {
		// SAFETY: The internal state must be correct.
		unsafe { String::from_utf8_unchecked(self.into_vec()) }
	}

	#[inline]
	/// Overwrites `self` with the [`str`] `s`.
	///
	/// The input `s` must be the exact same length
	/// as `N` or this function will error.
	///
	/// # Errors
	/// If the copy was successful, [`Result::Ok`] is returned with the new length of the string.
	///
	/// If the copy failed because `s.len() > N`, [`Result::Err`] is returned with how many extra bytes couldn't fit.
	///
	/// If the copy failed because `s.len() != N`, [`Result::Err`] is returned as `Err(0)`.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut string = Str::<3>::new();
	///
	/// // Input string is 4 in length, we can't copy it.
	/// // There is 1 extra byte that can't fit.
	/// assert_eq!(string.copy_str("abcd"), Err(1));
	///
	/// // Input string is 2 in length, not exactly 3.
	/// // `Err(0)` will be returned to indicate this.
	/// assert_eq!(string.copy_str("ab"), Err(0));
	///
	/// // This fits.
	/// assert_eq!(string.copy_str("abc"), Ok(3));
	/// ```
	pub fn copy_str(&mut self, s: impl AsRef<str>) -> Result<usize, usize> {
		let s       = s.as_ref();
		let s_bytes = s.as_bytes();
		let s_len   = s.len();

		if s_len > N {
			return Err(s_len - N);
		}

		if s_len != N {
			return Err(0);
		}

		// SAFETY: We are directly mutating the bytes and length.
		// We know the correct values.
		unsafe {
			self.as_bytes_all_mut().copy_from_slice(s_bytes);
			self.set_len(s_len);
		}

		Ok(s_len)
	}

	#[inline]
	/// Performs the same operation as [`Self::copy_str()`] except
	/// this function does not check if the input [`str`] `s` is too long.
	///
	/// If the copy was successful, the new length of the string is returned.
	///
	/// If the copy failed, this function will panic.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut string = Str::<3>::new();
	///
	/// // Input string is 3 in length, we can copy it.
	/// assert_eq!(string.copy_str_unchecked("abc"), 3);
	/// ```
	///
	/// # Panics
	/// Instead of erroring, this function will panic if the input `s.len() != N`.
	///
	/// Input too long:
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// let mut string = Str::<3>::new();
	///
	/// // Input string is 5 in length, this will panic.
	/// string.copy_str_unchecked("abcd");
	/// ```
	/// Input not long enough:
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// let mut string = Str::<3>::new();
	///
	/// // Input string is 2 in length, this will panic.
	/// string.copy_str_unchecked("ab");
	/// ```
	/// Input is just right:
	/// ```rust
	/// # use readable::str::*;
	/// let mut string = Str::<3>::new();
	/// string.copy_str_unchecked("abc");
	/// assert_eq!(string, "abc")
	/// ```
	pub fn copy_str_unchecked(&mut self, s: impl AsRef<str>) -> usize {
		let s       = s.as_ref();
		let s_bytes = s.as_bytes();
		let s_len   = s.len();

		// SAFETY: We are directly mutating the bytes and length.
		// We know the correct values.
		unsafe {
			self.as_bytes_all_mut().copy_from_slice(s_bytes);
			self.set_len(s_len);
		}

		s_len
	}

	#[inline]
	/// Appends `self` with the [`str`] `s`.
	///
	/// # Errors
	/// If the push was successful (or `s` was empty),
	/// [`Result::Ok`] is returned with the new length of the string.
	///
	/// If the push failed, [`Result::Err`] is returned
	/// with how many extra bytes couldn't fit.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut string = Str::<3>::new();
	///
	/// // Input string is 4 in length.
	/// // We can't push it.
	/// let err = string.push_str("abcd");
	/// assert_eq!(err, Err(1));
	///
	/// // The string is still empty.
	/// assert!(string.is_empty());
	///
	/// // This 2 length string will fit.
	/// string.push_str("ab").unwrap();
	/// assert_eq!(string, "ab");
	/// // This 1 length string will fit.
	/// string.push_str("c").unwrap();
	/// assert_eq!(string, "abc");
	///
	/// // But not anymore.
	/// let err = string.push_str("d");
	/// assert_eq!(err, Err(1));
	/// assert_eq!(string, "abc");
	/// ```
	pub fn push_str(&mut self, s: impl AsRef<str>) -> Result<usize, usize> {
		let s       = s.as_ref();
		let s_bytes = s.as_bytes();
		let s_len   = s.len();

		if s_len == 0 {
			return Ok(self.len());
		}

		let remaining = self.remaining();

		if s_len > remaining {
			return Err(s_len - remaining);
		}

		let self_len = self.len();
		let new_len  = s_len + self.len();

		// SAFETY: We are directly mutating the bytes and length.
		// We know the correct values.
		unsafe {
			self.as_bytes_all_mut()[self_len..new_len].copy_from_slice(s_bytes);
			self.set_len(new_len);
		}

		Ok(new_len)
	}

	/// Appends `self` with the [`str`] `s`.
	///
	/// If the push was successful (or `s` was empty),
	/// a `usize` is returned, representing the new length of the string.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<5>::new();
	/// assert_eq!(s.push_str_panic("wow"), 3);
	/// ```
	///
	/// ## Panics
	/// If the push failed, this function panics.
	///
	/// Input string is `>` than capacity:
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// let mut s = Str::<3>::new();
	/// s.push_str_panic("abcd");
	/// ```
	///
	/// [`Str`] has no more remaining capacity:
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// let mut s = Str::<4>::from_static_str("wow");
	/// assert_eq!(s.len(),       3);
	/// assert_eq!(s.remaining(), 1);
	///
	/// // This won't fit, will panic.
	/// s.push_str_panic("wow");
	/// ```
	pub fn push_str_panic(&mut self, s: impl AsRef<str>) -> usize {
		let s       = s.as_ref();
		let s_bytes = s.as_bytes();
		let s_len   = s.len();

		if s_len == 0 {
			return self.len as usize;
		}

		let remaining = self.remaining();

		assert!(
			s_len <= remaining,
			"no more space - remaining: {remaining}, input length: {s_len}, capacity: {N}"
		);

		let self_len = self.len();
		let new_len  = s_len + self.len();

		// SAFETY: We are directly mutating the bytes and length.
		// We know the correct values.
		unsafe {
			self.as_bytes_all_mut()[self_len..new_len].copy_from_slice(s_bytes);
			self.set_len(new_len);
		}
		new_len
	}

	#[inline]
	/// Appends `self` with the [`str`] `s`, saturating if there is no [`Self::CAPACITY`] left
	///
	/// This function returns a `usize`, representing how many _bytes_ were written.
	///
	/// If there is no _byte_ capacity left, this function will return `0`.
	///
	/// UTF-8 strings are accounted for, and are split on `char` basis, for example:
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<7>::new();
	///
	/// // Crab is 4 bytes.
	/// assert_eq!(4, "🦀".len());
	///
	/// // Our capacity is only 7, so we can only fit 1.
	/// assert_eq!(4, s.push_str_saturating("🦀"));
	/// assert_eq!(s, "🦀");
	/// assert_eq!(4, s.len());
	/// assert_eq!(3, s.remaining());
	/// ```
	///
	/// ## Examples
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<3>::new();
	///
	/// // Only 1 char, 3 bytes can fit.
	/// assert_eq!(3, s.push_str_saturating("です"));
	/// assert_eq!(s, "で");
	/// s.clear();
	///
	/// // Only 3 ASCII characters can fit.
	/// assert_eq!(3, s.push_str_saturating("hello"));
	/// assert_eq!(s, "hel");
	/// s.clear();
	///
	/// // Here, we push 3 characters with 1 capacity left.
	/// s.push_str("wo").unwrap();
	/// assert_eq!(1, s.push_str_saturating("rld"));
	/// // And only 1 character was pushed.
	/// assert_eq!(s, "wor");
	///
	/// // No matter how many times we push now, nothing will be added.
	/// assert_eq!(0, s.push_str_saturating("!"));
	/// assert_eq!(s, "wor");
	/// assert_eq!(0, s.push_str_saturating("へええ"));
	/// assert_eq!(s, "wor");
	/// assert_eq!(0, s.push_str_saturating("枕"));
	/// assert_eq!(s, "wor");
	/// assert_eq!(0, s.push_str_saturating("🦀"));
	/// assert_eq!(s, "wor");
	/// ```
	pub fn push_str_saturating(&mut self, s: impl AsRef<str>) -> usize {
		let s       = s.as_ref();
		let s_len   = s.len();

		let remaining = self.remaining();

		// If byte length is the same or less, we can just copy.
		if s_len <= remaining {
			self.push_str_panic(s);
			return s_len;
		}

		// Figure out what `char` index we can stop at.
		let index = if s.is_ascii() {
			remaining
		} else {
			// Handle UTF-8 correctly.
			// We use `.rev()` because we assume the string
			// is only slightly longer, so starting linear
			// search from the end is faster.
			let mut index = 0;
			for (i, _) in s.char_indices().rev() {
				index = i;

				if i <= remaining {
					break;
				}
			}

			// We didn't find a good index, push nothing.
			if index == 0 {
				return 0;
			}

			index
		};

		#[allow(clippy::string_slice)]
		self.push_str_panic(&s[..index]);
		index
	}

	#[inline]
	#[allow(clippy::missing_errors_doc)]
	/// [`Str::push_str`], but with a `char`
	///
	/// This acts in the same way as [`Str::push_str`], but the input is a single [`char`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut string = Str::<3>::new();
	///
	/// // Input char is 4 in length.
	/// // We can't push it.
	/// let err = string.push_char('🦀');
	/// assert_eq!(err, Err(1));
	///
	/// // The string is still empty.
	/// assert!(string.is_empty());
	///
	/// // This 3 length char will fit.
	/// assert_eq!(string.push_char('で'), Ok(3));
	/// assert_eq!(string, "で");
	/// ```
	pub fn push_char(&mut self, c: char) -> Result<usize, usize> {
		if self.remaining() == 0 {
			return Err(0);
		}

		match c.len_utf8() {
			1 => self.push_str(c.encode_utf8(&mut [0; 1])),
			2 => self.push_str(c.encode_utf8(&mut [0; 2])),
			3 => self.push_str(c.encode_utf8(&mut [0; 3])),
			_ => self.push_str(c.encode_utf8(&mut [0; 4])),
		}
	}

	#[inline]
	/// [`Str::push_str_panic`], but with a `char`
	///
	/// This acts in the same way as [`Str::push_str_panic`], but the input is a single [`char`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<5>::new();
	/// assert_eq!(s.push_char_panic('す'), 3);
	/// ```
	///
	/// ## Panics
	/// If the push failed, this function panics.
	///
	/// Input `char` is `>` than capacity:
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// let mut s = Str::<3>::new();
	/// s.push_char_panic('🦀');
	/// ```
	///
	/// [`Str`] has no more remaining capacity:
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// let mut s = Str::<4>::from_static_str("wow");
	/// assert_eq!(s.len(),       3);
	/// assert_eq!(s.remaining(), 1);
	///
	/// // This won't fit, will panic.
	/// s.push_char_panic('🦀');
	/// ```
	pub fn push_char_panic(&mut self, c: char) -> usize {
		match c.len_utf8() {
			1 => self.push_str_panic(c.encode_utf8(&mut [0; 1])),
			2 => self.push_str_panic(c.encode_utf8(&mut [0; 2])),
			3 => self.push_str_panic(c.encode_utf8(&mut [0; 3])),
			_ => self.push_str_panic(c.encode_utf8(&mut [0; 4])),
		}
	}

	/// [`Str::push_str_saturating`], but with a `char`
	///
	/// This acts in the same way as [`Str::push_str_saturating`], but the input is a single [`char`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<7>::new();
	///
	/// // Crab is 4 bytes.
	/// assert_eq!(4, "🦀".len());
	///
	/// // Our capacity is only 7, so we can only fit 1.
	/// assert_eq!(4, s.push_char_saturating('🦀'));
	/// assert_eq!(0, s.push_char_saturating('🦀'));
	/// assert_eq!(s, "🦀");
	/// assert_eq!(4, s.len());
	/// assert_eq!(3, s.remaining());
	/// ```
	///
	/// ## Examples
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<3>::new();
	///
	/// assert_eq!(1, s.push_char_saturating('w'));
	/// assert_eq!(1, s.push_char_saturating('o'));
	/// assert_eq!(1, s.push_char_saturating('w'));
	/// assert_eq!(s, "wow");
	///
	/// // No matter how many times we push now, nothing will be added.
	/// assert_eq!(0, s.push_char_saturating('!'));
	/// assert_eq!(s, "wow");
	/// assert_eq!(0, s.push_char_saturating('へ'));
	/// assert_eq!(s, "wow");
	/// assert_eq!(0, s.push_char_saturating('枕'));
	/// assert_eq!(s, "wow");
	/// assert_eq!(0, s.push_char_saturating('🦀'));
	/// assert_eq!(s, "wow");
	/// ```
	pub fn push_char_saturating(&mut self, c: char) -> usize {
		if self.remaining() == 0 {
			return 0;
		}

		match c.len_utf8() {
			1 => self.push_str_saturating(c.encode_utf8(&mut [0; 1])),
			2 => self.push_str_saturating(c.encode_utf8(&mut [0; 2])),
			3 => self.push_str_saturating(c.encode_utf8(&mut [0; 3])),
			_ => self.push_str_saturating(c.encode_utf8(&mut [0; 4])),
		}
	}

	#[inline]
	#[must_use]
	/// Decomposes a [`Str`] into its raw components
	///
	/// Returns the byte array buffer and the valid UTF-8 length of the [`Str`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// let s = Str::<5>::from_static_str("hi");
	/// let (buf, len) = s.into_raw();
	///
	/// assert_eq!(buf, [b'h', b'i', 0, 0, 0]);
	/// assert_eq!(len, 2);
	/// ```
	pub const fn into_raw(self) -> ([u8; N], u8) {
		(self.buf, self.len)
	}

	#[inline]
	#[must_use]
	/// Creates a new [`Str`] from a byte array buffer and a length
	///
	/// ```rust
	/// # use readable::str::*;
	/// let buf = [b'h', b'i', 0, 0, 0];
	/// let len = 2;
	///
	/// // SAFETY: The length covers valid
	/// // UTF-8 bytes in the provided buffer.
	/// let s = unsafe { Str::<5>::from_raw(buf, len) };
	/// assert_eq!(s, "hi");
	/// ```
	///
	/// ## Safety
	/// The caller needs to make sure the bytes covered
	/// by the `len` are actual valid UTF-8 bytes.
	pub const unsafe fn from_raw(buf: [u8; N], len: u8) -> Self {
		Self { buf, len }
	}

	#[inline]
	#[must_use]
	/// Create a [`Str`] directly from a [`str`]
	///
	/// ```rust
	/// # use readable::str::*;
	/// let s = Str::<5>::from_str_exact("12345");
	/// assert_eq!(s, "12345");
	/// ```
	///
	/// ## Panics
	/// The input input [`str`] `string`'s length must
	/// be exactly equal to `Self::CAPACITY` or this
	/// function will panic.
	///
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// // 1 too many characters, will panic.
	/// let s = Str::<4>::from_str_exact("12345");
	/// ```
	pub fn from_str_exact(string: impl AsRef<str>) -> Self {
		// SAFETY: `str` is valid UTF-8
		unsafe { Self::from_bytes_exact(string.as_ref().as_bytes()) }
	}

	#[inline]
	#[must_use]
	/// Create a [`Str`] directly from bytes
	///
	/// ```rust
	/// # use readable::str::*;
	/// let s = unsafe { Str::<5>::from_bytes_exact(b"12345") };
	/// assert_eq!(s, "12345");
	/// ```
	///
	/// ## Safety
	/// The bytes must be valid UTF-8.
	///
	/// ## Panics
	/// The input bytes `bytes`'s length must
	/// be exactly equal to `Self::CAPACITY` or this
	/// function will panic.
	///
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// // 1 too many characters, will panic.
	/// let s = unsafe { Str::<4>::from_bytes_exact(b"12345") };
	/// ```
	pub unsafe fn from_bytes_exact(bytes: impl AsRef<[u8]>) -> Self {
		let mut buf = [0; N];
		buf.copy_from_slice(bytes.as_ref());
		Self {
			len: N as u8,
			buf,
		}
	}

	#[inline]
	/// Calls [`str::make_ascii_uppercase`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<5>::from_static_str("hello");
	///
	/// s.make_ascii_uppercase();
	/// assert_eq!(s, "HELLO");
	/// ```
	pub fn make_ascii_uppercase(&mut self) {
		// SAFETY: we aren't changing the length, safe to call.
		unsafe { self.as_str_mut().make_ascii_uppercase(); }
	}

	#[inline]
	/// Calls [`str::make_ascii_lowercase`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<5>::from_static_str("HELLO");
	///
	/// s.make_ascii_lowercase();
	/// assert_eq!(s, "hello");
	/// ```
	pub fn make_ascii_lowercase(&mut self) {
		// SAFETY: we aren't changing the length, safe to call.
		unsafe { self.as_str_mut().make_ascii_lowercase(); }
	}

	#[inline]
	/// Shortens this [`Str`] to the specified length.
	///
	/// If `new_len` is greater than the string’s current length, this has no effect.
	///
	/// Note that this method has no effect on the allocated capacity of the string
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<4>::from_static_str("asdf");
	///
	/// s.truncate(1);
	/// assert_eq!(s, "a");
	/// ```
	///
	/// ## Panics
	/// Panics if `new_len` does not lie on a [`char`] boundary.
	///
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// let mut s = Str::<6>::from_static_str("です");
	///
	/// // This does not lie on a full char, it will panic.
	/// s.truncate(4);
	/// ```
	pub fn truncate(&mut self, new_len: usize) {
		if new_len <= self.len() {
			assert!(self.as_str().is_char_boundary(new_len));
			// SAFETY: bytes are valid.
			unsafe { self.set_len(new_len); }
		}
	}

	#[inline]
	/// Removes a [`char`] from this [`Str`] at a byte position and returns it.
	///
	/// This is an _O(n)_ operation, as it requires copying every element in the buffer.
	///
	/// ```
	/// # use readable::str::*;
	/// let mut s = Str::<3>::from_static_str("foo");
	///
	/// assert_eq!(s.remove(0), 'f');
	/// assert_eq!(s.remove(1), 'o');
	/// assert_eq!(s.remove(0), 'o');
	/// ```
	///
	/// ## Panics
	/// Panics if `idx` is larger than or equal to the [`Str`]’s length,
	/// or if it does not lie on a [`char`] boundary.
	pub fn remove(&mut self, idx: usize) -> char {

		#[allow(clippy::string_slice)]
		let ch = self.as_str()[idx..].chars().next().expect("cannot remove a char from the end of a string");

		let next = idx + ch.len_utf8();
		let len = self.len();

		// SAFETY: https://doc.rust-lang.org/1.74.0/src/alloc/string.rs.html#1298
		unsafe {
			std::ptr::copy(self.as_ptr().add(next), self.as_mut_ptr().add(idx), len - next);
			self.set_len(len - (next - idx));
		}
		ch
	}

	#[inline]
	/// Removes the last character from the [`Str`] and returns it.
	///
	/// Returns `None` if this [`Str`] is empty.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<3>::from_static_str("foo");
	///
	/// assert_eq!(s.len(), 3);
	/// assert_eq!(s.pop(), Some('o'));
	/// assert_eq!(s.len(), 2);
	/// assert_eq!(s.pop(), Some('o'));
	/// assert_eq!(s.len(), 1);
	/// assert_eq!(s.pop(), Some('f'));
	/// assert_eq!(s.len(), 0);
	/// assert_eq!(s.pop(), None);
	/// ```
	pub fn pop(&mut self) -> Option<char> {
		// https://doc.rust-lang.org/1.74.0/src/alloc/string.rs.html#1268

		let ch = self.as_str().chars().next_back()?;
		let newlen = self.len() - ch.len_utf8();

		// SAFETY: setting length.
		unsafe { self.set_len(newlen); }
		Some(ch)
	}
}

//---------------------------------------------------------------------------------------------------- From
/// This is a macro for now since `TryFrom<AsRef<str>>` has some conflicts.
macro_rules! impl_from_str {
	($($string:ty),*) => {
		$(
			impl<const N: usize> TryFrom<$string> for Str<N> {
				type Error = usize;

				#[inline]
				/// This takes in a [`&str`] of any length (equal to or less than N)
				/// and will return a `Str` with that same string.
				///
				/// If this function fails, [`Result::Err`] is returned with how many extra bytes couldn't fit.
				///
				/// ```rust
				/// # use readable::str::*;
				/// // Input string is 4 in length, we can't copy it.
				/// // There is 1 extra byte that can't fit.
				/// assert_eq!(Str::<3>::try_from("abcd"), Err(1));
				/// ```
				///
				/// ## Compile-time panic
				/// This function will panic at compile time if `N > 255`.
				/// ```rust,ignore
				/// # use readable::str::*;
				/// // Compile error!
				/// Str::<256>::try_from("");
				/// ```
				fn try_from(string: $string) -> Result<Self, Self::Error> {
					let len = string.len();

					if len == 0 {
						Ok(Self::new())
					} else if len < N {
						let mut this = Self::new();
						this.push_str_panic(&string);
						Ok(this)
					} else if len == N {
						let this = Self::from_str_exact(&string);
						Ok(this)
					} else {
						Err(len - N)
					}
				}
			}
		)*
	};
}
impl_from_str! {
	&str,
	Arc<str>, &Arc<str>,
	Box<str>, &Box<str>,
	Rc<str>, &Rc<str>,
	Cow<'_, str>, &Cow<'_, str>,
	String, &String
}

//---------------------------------------------------------------------------------------------------- Traits
impl<const N: usize> PartialEq<str> for Str<N> {
	#[inline]
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}
impl<const N: usize> PartialEq<&str> for Str<N> {
	#[inline]
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}

impl<const N: usize> std::fmt::Display for Str<N> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl<const N: usize> std::ops::Deref for Str<N> {
	type Target = str;

	#[inline]
	/// Equivalent to [`Str::as_str()`].
	///
	/// ```rust
	/// # use readable::str::*;
	/// use std::ops::Deref;
	/// let mut s = Str::<3>::from_static_str("foo");
	///
	/// assert_eq!(s.deref(), "foo");
	/// ```
	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl<const N: usize> std::convert::AsRef<str> for Str<N> {
	#[inline]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<const N: usize> std::borrow::Borrow<str> for Str<N> {
	#[inline]
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

impl<const N: usize> std::default::Default for Str<N> {
	#[inline]
	/// Calls [`Self::new`]
	fn default() -> Self {
		Self::new()
	}
}

impl<const N: usize, T: AsRef<str>> std::ops::Add<T> for Str<N> {
	type Output = Self;

	#[inline]
	/// Implements the `+` operator.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<6>::from_static_str("foo");
	///
	/// assert_eq!(s + "bar", "foobar");
	/// ```
	///
	/// ## Panics
	/// This calls [`Str::push_str_panic`] and will panic in the same ways.
	///
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// let mut s = Str::<3>::from_static_str("foo");
	///
	/// // This will panic, not enough capacity!
	/// let _ = s + "bar";
	/// ```
	fn add(self, s: T) -> Self::Output {
		let mut new = self;
		new.push_str_panic(s.as_ref());
		new
	}
}

impl<const N: usize, T: AsRef<str>> std::ops::AddAssign<T> for Str<N> {
	#[inline]
	/// Implements the `+=` operator.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<6>::from_static_str("foo");
	/// s += "bar";
	///
	/// assert_eq!(s, "foobar");
	/// ```
	///
	/// ## Panics
	/// This calls [`Str::push_str_panic`] and will panic in the same ways.
	///
	/// ```rust,should_panic
	/// # use readable::str::*;
	/// let mut s = Str::<3>::from_static_str("foo");
	///
	/// // This will panic, not enough capacity!
	/// s += "bar";
	/// ```
	fn add_assign(&mut self, s: T) {
		self.push_str_panic(s.as_ref());
	}
}

impl<const N: usize> AsRef<[u8]> for Str<N> {
	#[inline]
	/// Calls [`Str::as_bytes()`], only including valid `UTF-8` bytes.
	///
	/// ```rust
	/// # use readable::str::*;
	/// // 6 in capacity, but only 3 in length.
	/// let mut s = Str::<6>::from_static_str("foo");
	///
	/// assert_eq!(AsRef::<[u8]>::as_ref(&s), "foo".as_bytes());
	/// ```
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<const N: usize> AsRef<std::path::Path> for Str<N> {
	#[inline]
	fn as_ref(&self) -> &std::path::Path {
		std::path::Path::new(self.as_str())
	}
}

impl<const N: usize> AsRef<std::ffi::OsStr> for Str<N> {
	#[inline]
	fn as_ref(&self) -> &std::ffi::OsStr {
		std::ffi::OsStr::new(self.as_str())
	}
}

impl<const N: usize> Extend<char> for Str<N> {
	#[inline]
	/// Calls [`Str::push_char_panic`] for each `char`.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<3>::new();
	///
	/// s.extend(['a', 'b', 'c']);
	/// assert_eq!(s, "abc");
	/// ```
	fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
		iter
			.into_iter()
			.for_each(|c| { self.push_char_panic(c); });
	}
}

impl<'a, const N: usize> Extend<&'a str> for Str<N> {
	#[inline]
	/// Calls [`Str::push_str_panic`] for each `str`.
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<12>::new();
	///
	/// s.extend(["hello", " ", "world", "!"]);
	/// assert_eq!(s, "hello world!");
	/// ```
	fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
		iter
			.into_iter()
			.for_each(|c|{ self.push_str_panic(c); });
	}
}

macro_rules! impl_index {
	($($range:ident),* $(,)?) => {
		$(
			impl<const N: usize> std::ops::Index<std::ops::$range<usize>> for Str<N> {
				type Output = str;
				#[inline]
				fn index(&self, index: std::ops::$range<usize>) -> &Self::Output {
					self.as_str().index(index)
				}
			}
		)*
	};
}

impl<const N: usize> std::ops::Index<std::ops::RangeFull> for Str<N> {
	type Output = str;
	#[inline]
	fn index(&self, index: std::ops::RangeFull) -> &Self::Output {
		self.as_str().index(index)
	}
}

impl_index! {
	Range,
	RangeFrom,
	RangeInclusive,
	RangeTo,
	RangeToInclusive,
}

impl<const N: usize> std::fmt::Write for Str<N> {
	#[inline]
	/// Calls [`Str::push_str()`]
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<12>::new();
	///
	/// std::fmt::Write::write_str(&mut s, "hello world!").unwrap();
	/// assert_eq!(s, "hello world!");
	/// ```
	fn write_str(&mut self, s: &str) -> std::fmt::Result {
		match self.push_str(s) {
			Ok(_) => Ok(()),
			Err(_) => Err(std::fmt::Error),
		}
	}
	#[inline]
	/// Calls [`Str::push_char()`]
	///
	/// ```rust
	/// # use readable::str::*;
	/// let mut s = Str::<3>::new();
	///
	/// std::fmt::Write::write_char(&mut s, 'で').unwrap();
	/// assert_eq!(s, "で");
	/// ```
	fn write_char(&mut self, c: char) -> std::fmt::Result {
		match self.push_char(c) {
			Ok(_) => Ok(()),
			Err(_) => Err(std::fmt::Error),
		}
	}
}

//---------------------------------------------------------------------------------------------------- Serde
#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for Str<N>
{
	#[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for Str<N>
{
	#[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        use serde::de::{self, Visitor};
        use std::marker::PhantomData;

        struct StrVisitor<const N: usize>(PhantomData<[u8; N]>);

        impl<const N: usize> Visitor<'_> for StrVisitor<N> {
            type Value = Str<N>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a string no more than {N} bytes long")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: de::Error,
            {
				let v_len = v.len();
				if v_len > N {
					return Err(E::invalid_length(v_len, &self));
				}
				let mut s = Str::new();
				s.push_str_panic(v);
				Ok(s)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: de::Error,
            {
				let Ok(v) = std::str::from_utf8(v) else {
					return Err(E::invalid_value(de::Unexpected::Bytes(v), &self));
				};

				let v_len = v.len();
				if v_len > N {
					return Err(E::invalid_length(v_len, &self));
				}

				let mut s = Str::new();
				s.push_str_panic(v);
				Ok(s)
            }
        }

        deserializer.deserialize_str(StrVisitor(PhantomData))
    }
}

#[cfg(feature = "bincode")]
impl<const N: usize> bincode::Encode for Str<N> {
	#[inline]
	fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
		bincode::Encode::encode(self.as_str(), encoder)
	}
}

#[cfg(feature = "bincode")]
impl<const N: usize> bincode::Decode for Str<N> {
	#[inline]
	fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
		let s: String = bincode::Decode::decode(decoder)?;
		#[allow(clippy::map_err_ignore)]
		Self::try_from(s).map_err(|_| bincode::error::DecodeError::Other("Str::invalid() failed"))
	}
}
#[cfg(feature = "bincode")]
impl<'de, const N: usize> bincode::BorrowDecode<'de> for Str<N> {
	fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
		bincode::Decode::decode(decoder)
	}
}

#[cfg(feature = "borsh")]
impl<const N: usize> borsh::BorshSerialize for Str<N> {
	#[inline]
	fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
		borsh::BorshSerialize::serialize(self.as_str(), writer)
	}
}

#[cfg(feature = "borsh")]
impl<const N: usize> borsh::BorshDeserialize for Str<N> {
	#[inline]
	fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
		let s: String = borsh::BorshDeserialize::deserialize_reader(reader)?;
		#[allow(clippy::map_err_ignore)]
		Self::try_from(s).map_err(|_| borsh::io::Error::other("Str::try_from() failed"))
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
