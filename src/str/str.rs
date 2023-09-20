//---------------------------------------------------------------------------------------------------- Use
// use bincode::{Encode,Decode};
// use serde::{Serialize,Deserialize};
// use anyhow::anyhow;
// use log::{error,info,warn,debug,trace};
// use disk::{Bincode2,Json};

//---------------------------------------------------------------------------------------------------- Str
/// A (potentially) cache-friendly fixed size stack string
///
/// [`Str`] is a generic stack-based string with a maximum byte length of [`u8::MAX`].
///
/// The generic `N` is a [`usize`] and represents the maximum length of the string,
/// however all constructor functions for [`Str`] will panic at _compile time_ if `N > 255`.
///
/// ## Size
/// The internal length is stored as a [`u8`], and as such will
/// take mimimal space, allowing for longer strings to be stored.
///
/// Due to `#[repr(C)]`, `N + 1` is how many bytes your [`Str`] will take up.
///
/// Using [`Str`] in powers of 2 is recommended.
/// ```rust
/// # use readable::Str;
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
/// # use readable::Str;
/// /// These will all panic at _compile time_
/// Str::<256>::new();
/// Str::<256>::try_from("");
/// Str::<256>::from_static_str("");
/// Str::<256>::from_static_bytes(b"");
/// ```
///
/// ## Usage
/// ```rust
/// # use readable::Str;
/// // Create a `Str` with a maximum capacity of `24` bytes.
/// const N: usize  = 24;
/// let mut string = Str::<N>::new();
/// assert!(string.empty());
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
/// assert!(string.empty());
/// assert_eq!(string.len(), 0);
///
/// // `push_str()` should be the exact same.
/// string.push_str(other_str).unwrap();
/// assert_eq!(string, other_str);
///
/// // This string is full.
/// assert!(string.full());
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
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

	/// Returns an empty [`Str`].
	///
	/// ```rust
	/// # use readable::Str;
	/// let string = Str::<4>::new();
	/// assert!(string.empty());
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

	/// Create a [`Self`] from static bytes.
	///
	/// The length of the input doesn't need to be the
	/// same as `N`, it just needs to be equal or less.
	///
	/// Exact length:
	/// ```rust
	/// # use readable::Str;
	/// const BYTES: [u8; 3] = *b"abc";
	/// const STR: Str<3> = Str::from_static_bytes(&BYTES);
	///
	/// assert_eq!(STR, "abc");
	/// ```
	/// Slightly less length is okay too:
	/// ```rust
	/// # use readable::Str;
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
	/// # use readable::{Str};
	/// // This doesn't fit, will panic at compile time.
	/// const STR: Str<3> = Str::from_static_bytes("abcd");
	/// ```
	pub const fn from_static_bytes(bytes: &'static [u8]) -> Self {
		// Will cause panics at compile time.
		Self::CAPACITY;

		let len = bytes.len();

		if len > N {
			panic!("byte length is longer than N");
		}

		if std::str::from_utf8(&bytes).is_err() {
			panic!("bytes are not valid UTF-8");
		}

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

	/// Create a [`Self`] from a static [`str`].
	///
	/// The length of the input doesn't need to be the
	/// same as `N`, it just needs to be equal or less.
	///
	/// Exact length:
	/// ```rust
	/// # use readable::Str;
	/// const S: &str = "abc";
	/// const STR: Str<3> = Str::from_static_str(&S);
	///
	/// assert_eq!(STR, "abc");
	/// ```
	/// Slightly less length is okay too:
	/// ```rust
	/// # use readable::Str;
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
	/// # use readable::{Str};
	/// // This doesn't fit, will panic at compile time.
	/// const STR: Str<3> = Str::from_static_str("abcd");
	/// ```
	pub const fn from_static_str(s: &'static str) -> Self {
		Self::from_static_bytes(s.as_bytes())
	}

	#[inline]
	/// Return all the bytes of this [`Str`], whether valid UTF-8 or not.
	///
	/// ``` rust
	/// # use readable::Str;
	/// let mut string = Str::<10>::new();
	/// string.push_str("hello").unwrap();
	///
	/// // The string length is 5, but the slice
	/// // returned is the full capacity, 10.
	/// assert_eq!(string.as_slice().len(), 10);
	/// ```
	pub const fn as_slice(&self) -> &[u8] {
		&self.buf.as_slice()
	}

	#[inline]
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
	/// # use readable::Str;
	/// let mut string = Str::<5>::new();
	/// string.push_str("hi").unwrap();
	/// assert_eq!(string, "hi");
	/// assert_eq!(string.len(), 2);
	///
	/// // Safety: We must ensure we leave
	/// // leave the bytes as valid UTF-8 bytes
	/// // and that we set the length correctly.
	/// unsafe {
	/// 	// Mutate to valid UTF-8 bytes.
	/// 	let mut_ref = string.as_mut_slice();
	/// 	mut_ref.copy_from_slice(&b"world"[..]);
	/// 	// Set the new length.
	/// 	string.set_len(5);
	/// }
	///
	/// assert_eq!(string, "world");
	/// assert_eq!(string.len(), 5);
	/// ```
	pub unsafe fn as_mut_slice(&mut self) -> &mut [u8] {
		self.buf.as_mut_slice()
	}

	#[inline]
	/// Return the length of the _valid_ UTF-8 bytes of this [`Str`]
	///
	/// ```rust
	/// # use readable::Str;
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
	/// Return the length of the _valid_ UTF-8 bytes of this [`Str`] as a [`u8`]
	///
	/// ```rust
	/// # use readable::Str;
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
	/// Returns the maximum capacity (`Self::CAPACITY`) of this [`Str`].
	///
	/// Should be exactly equal to `N`.
	///
	/// ```rust
	/// # use readable::Str;
	/// //        This is N (usize)    This is CAPACITY (u8)
	/// //               |           /       |
	/// //               |           |       |
	/// //               v           v       v
	/// assert_eq!(Str::<10>::CAPACITY,  10_u8);
	///
	/// let s = Str::<10>::new();
	/// assert_eq!(s.capacity(), 10_u8);
	/// ```
	pub const fn capacity(&self) -> u8 {
		Self::CAPACITY
	}

	#[inline]
	/// Set the length of the _valid_ UTF-8 bytes of this [`Str`]
	///
	/// This will usually be used when manually mutating [`Str`] with [`Str::as_mut_slice()`].
	///
	/// ```rust
	/// # use readable::Str;
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
	/// 	let mut_ref = s.as_mut_slice();
	/// 	mut_ref[0] = b'a';
	/// 	mut_ref[1] = b'b';
	/// 	mut_ref[2] = b'c';
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
	/// This will usually be used when manually mutating [`Str`] with [`Str::as_mut_slice()`].
	///
	/// ## Safety
	/// Other functions will rely on the internal length
	/// to be correct, so the caller must ensure this length
	/// is actually correct.
	pub unsafe fn set_len_u8(&mut self, len: u8) {
		self.len = len;
	}

	#[inline]
	/// How many available bytes are left in this [`Str`]
	/// before the [`Self::CAPACITY`] is completely filled.
	///
	/// ```rust
	/// # use readable::Str;
	/// let mut s = Str::<5>::new();
	/// s.push_str("hi");
	/// assert_eq!(s.remaining(), 3);
	/// ```
	pub fn remaining(&self) -> usize {
		(Self::CAPACITY - self.len) as usize
	}

	#[inline]
	/// Returns only the valid `UTF-8` bytes of this [`Str`] as a byte slice.
	///
	/// ```rust
	/// # use readable::Str;
	/// let s = Str::<10>::from_static_str("hello");
	/// assert_eq!(s.as_valid_slice().len(), 5);
	/// ```
	pub fn as_valid_slice(&self) -> &[u8] {
		&self.as_slice()[..self.len()]
	}

	#[inline]
	/// Returns only the valid `UTF-8` bytes of this [`Str`] as a `Vec<u8>`
	///
	/// ```rust
	/// # use readable::Str;
	/// let s = Str::<10>::from_static_str("hello");
	/// assert_eq!(s.into_valid_vec().len(), 5);
	/// ```
	pub fn into_valid_vec(self) -> Vec<u8> where Self: Sized {
		self.as_valid_slice().to_vec()
	}

	/// Check this [`Str`] for correctness.
	///
	/// When constructing/receiving a [`Str`] outside of
	/// its constructors, it may not be guaranteed that
	/// the invariants are upheld.
	///
	/// This function will return `true` if:
	/// - Internal length is greater than the internal byte array
	/// - Inner byte array is longer than `255`
	/// - `.as_str()` would return invalid UTF-8
	///
	/// ```rust
	/// # use readable::Str;
	/// // Create `Str` with maximum 5 length.
	/// let mut string = Str::<5>::new();
	/// assert_eq!(string.invalid(), false);
	///
	/// // Unsafely set the length to 10.
	/// unsafe { string.set_len(10); }
	/// // This string is now invalid.
	/// assert_eq!(string.invalid(), true);
	/// ```
	pub fn invalid(&self) -> bool {
		let len     = self.len as usize;
		let buf_len = self.buf.len();

		if {
			len > buf_len ||
			buf_len > 255 ||
			std::str::from_utf8(&self.buf[..len]).is_err()
		} {
			return true;
		}

		false
	}

	#[inline]
	/// Clears all bytes of this [`Str`].
	///
	/// ```rust
	/// # use readable::Str;
	/// // Create a string.
	/// let mut s = Str::<5>::from_static_str("hello");
	/// assert_eq!(s, "hello");
	///
	/// // Clear the string.
	/// s.clear();
	/// assert_eq!(s, "");
	/// assert!(s.empty());
	/// ```
	///
	/// ## Safety
	/// This does not actually mutate any bytes,
	/// it simply sets the internal length to `0`.
	///
	/// Do not rely on this to clear the actual bytes.
	pub fn clear(&mut self) {
		// SAFETY: We are manually setting the length.
		unsafe { self.set_len(0); }
	}

	#[inline]
	/// If this [`Str`] is empty.
	///
	/// ``` rust
	/// # use readable::Str;
	/// let mut s = Str::<10>::new();
	/// assert_eq!(s, "");
	/// assert!(s.empty());
	///
	/// s.push_str("a").unwrap();
	/// assert!(!s.empty());
	/// ```
	pub fn empty(&self) -> bool {
		self.len == 0
	}

	#[inline]
	/// If this [`Str`] is full (no more capacity left).
	///
	/// ``` rust
	/// # use readable::Str;
	/// let mut s = Str::<3>::new();
	/// assert_eq!(s.len(), 0);
	/// assert!(!s.full());
	///
	/// s.push_str("123").unwrap();
	/// assert_eq!(s.len(), 3);
	/// assert!(s.full());
	/// ```
	pub fn full(&self) -> bool {
		self.len == Self::CAPACITY
	}

	#[inline]
	/// This [`Str`], as a valid UTF-8 [`str`].
	///
	/// ``` rust
	/// # use readable::Str;
	/// let s = Str::<5>::from_static_str("hello");
	/// assert_eq!(s.as_str(), "hello");
	/// ```
	pub fn as_str(&self) -> &str {
		// SAFETY: `.as_valid_slice()` must be correctly implemented.
		// The internal state must be correct.
		unsafe { std::str::from_utf8_unchecked(self.as_valid_slice()) }
	}

	#[inline]
	/// Consumes `self` into a [`String`]
	/// ``` rust
	/// # use readable::Str;
	/// let s = Str::<5>::from_static_str("hello");
	///
	/// let s: String = s.into_string();
	/// assert_eq!(s, "hello");
	/// ```
	pub fn into_string(self) -> String where Self: Sized {
		// SAFETY: The internal state must be correct.
		unsafe { String::from_utf8_unchecked(self.into_valid_vec()) }
	}

	/// Overwrites `self` with the [`str`] `s`.
	///
	/// The input `s` must be the exact same length
	/// as `N` or this function will error.
	///
	/// If the copy was successful, [`Result::Ok`] is returned with the new length of the string.
	///
	/// If the copy failed because `s.len() > N`, [`Result::Err`] is returned with how many extra bytes couldn't fit.
	///
	/// If the copy failed because `s.len() != N`, [`Result::Err`] is returned as `Err(0)`.
	///
	/// ```rust
	/// # use readable::Str;
	/// let mut string = Str::<3>::new();
	///
	/// // Input string is 4 in length, we can't copy it.
	/// // There is 1 extra byte that can't fit.
	/// assert_eq!(string.copy_str("abcd"), Err(1));
	///
	/// // Input string is 2 in length, not exactly 3.
	/// // `Err(0)` will be returned to indicate this.
	/// assert_eq!(string.copy_str("ab"), Err(0));
	/// ```
	pub fn copy_str(&mut self, s: impl AsRef<str>) -> Result<usize, usize> {
		let s       = s.as_ref();
		let s_bytes = s.as_bytes();
		let s_len   = s.len();

		if s_len > N {
			Err(s_len - N)
		} else if s_len != N {
			Err(0)
		} else {
			// SAFETY: We are directly mutating the bytes and length.
			// We know the correct values.
			unsafe {
				self.as_mut_slice().copy_from_slice(&s_bytes[..s_len]);
				self.set_len(s_len);
			}
			Ok(s_len)
		}
	}

	/// Performs the same operation as [`Self::copy_str()`] except
	/// this function does not check if the input [`str`] `s` is too long.
	///
	/// If the copy was successful, the new length of the string is returned.
	///
	/// If the copy failed, this function will panic.
	///
	/// ```rust
	/// # use readable::Str;
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
	/// # use readable::Str;
	/// let mut string = Str::<3>::new();
	///
	/// // Input string is 5 in length, this will panic.
	/// string.copy_str_unchecked("abcd");
	/// ```
	/// Input not long enough:
	/// ```rust,should_panic
	/// # use readable::Str;
	/// let mut string = Str::<3>::new();
	///
	/// // Input string is 2 in length, this will panic.
	/// string.copy_str_unchecked("ab");
	/// ```
	/// Input is just right:
	/// ```rust
	/// # use readable::Str;
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
			self.as_mut_slice().copy_from_slice(&s_bytes[..s_len]);
			self.set_len(s_len);
		}
		s_len
	}

	/// Appends `self` with the [`str`] `s`.
	///
	/// If the push was successful (or `s` was empty),
	/// [`Result::Ok`] is returned with the new length of the string.
	///
	/// If the push failed, [`Result::Err`] is returned
	/// with how many extra bytes couldn't fit.
	///
	/// ```rust
	/// # use readable::Str;
	/// let mut string = Str::<3>::new();
	///
	/// // Input string is 4 in length.
	/// // We can't push it.
	/// let err = string.push_str("abcd");
	/// assert_eq!(err, Err(1));
	///
	/// // The string is still empty.
	/// assert!(string.empty());
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
			Err(s_len - remaining)
		} else {
			let self_len = self.len();
			let new_len  = s_len + self.len();


			// SAFETY: We are directly mutating the bytes and length.
			// We know the correct values.
			unsafe {
				self.as_mut_slice()[self_len..new_len].copy_from_slice(s_bytes);
				self.set_len(new_len);
			}
			Ok(new_len)
		}
	}
	
	/// Appends `self` with the [`str`] `s`.
	///
	/// If the push was successful (or `s` was empty),
	/// a `usize` is returned, representing the new length of the string.
	///
	/// ```rust
	/// # use readable::Str;
	/// let mut s = Str::<5>::new();
	/// assert_eq!(s.push_str_unchecked("wow"), 3);
	/// ```
	///
	/// ## Panics
	/// If the push failed, this function panics.
	///
	/// Input string is `>` than capacity:
	/// ```rust,should_panic
	/// # use readable::Str;
	/// let mut s = Str::<3>::new();
	/// s.push_str_unchecked("abcd");
	/// ```
	///
	/// [`Str`] has no more remaining capacity:
	/// ```rust,should_panic
	/// # use readable::Str;
	/// let mut s = Str::<4>::from_static_str("wow");
	/// assert_eq!(s.len(),       3);
	/// assert_eq!(s.remaining(), 1);
	///
	/// // This won't fit, will panic.
	/// s.push_str_unchecked("wow");
	/// ```
	pub fn push_str_unchecked(&mut self, s: impl AsRef<str>) -> usize {
		let s       = s.as_ref();
		let s_bytes = s.as_bytes();
		let s_len   = s.len();

		if s_len == 0 {
			return self.len as usize;
		}

		let remaining = self.remaining();

		if s_len > remaining {
			panic!("no more space - remaining: {remaining}, input length: {s_len}");
		} else {
			let self_len = self.len();
			let new_len  = s_len + self.len();

			// SAFETY: We are directly mutating the bytes and length.
			// We know the correct values.
			unsafe {
				self.as_mut_slice()[self_len..new_len].copy_from_slice(s_bytes);
				self.set_len(new_len);
			}
			new_len
		}
	}
}

//---------------------------------------------------------------------------------------------------- Traits
impl<const N: usize> PartialEq<str> for Str<N> {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}
impl<const N: usize> PartialEq<&str> for Str<N> {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}

impl<const N: usize> std::fmt::Display for Str<N> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl<const N: usize> std::convert::AsRef<str> for Str<N> {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<const N: usize> std::convert::TryFrom<&str> for Str<N> {
	type Error = usize;

	/// If this function fails, [`Result::Err`] is returned with how many extra bytes couldn't fit.
	///
	/// ```rust
	/// # use readable::Str;
	/// // Input string is 4 in length, we can't copy it.
	/// // There is 1 extra byte that can't fit.
	/// assert_eq!(Str::<3>::try_from("abcd"), Err(1));
	/// ```
	///
	/// ## Compile-time panic
	/// This function will panic at compile time if `N > 255`.
	/// ```rust,ignore
	/// # use readable::Str;
	/// // Compile error!
	/// Str::<256>::try_from("");
	/// ```
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let str_len = s.len();

		if str_len > N {
			return Err(str_len - N);
		}

		let mut string = Self::new();

		// Safety: At this point, we know the string
		// can fit so we don't need to check.
		string.copy_str_unchecked(s);

		Ok(string)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
