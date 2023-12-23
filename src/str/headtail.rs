use std::fmt::{self,Display};
use std::borrow::Cow;

//---------------------------------------------------------------------------------------------------- Impl
impl<T: AsRef<str>> HeadTail for T {}

/// The separator string inserted when using [`HeadTail`]'s `dot` functions.
pub const DOT: &str = "...";

//---------------------------------------------------------------------------------------------------- Head
/// Head/Tail characters of a [`str`]
///
/// This trait provides some functionality for
/// cutting off a string either by the head, tail,
/// or both, with optional `...` after/before/in-between.
///
/// Anything that implements [`AsRef<str>`] can use this trait.
///
/// ## Examples
/// ```rust
/// use readable::str::HeadTail;
///
/// let string = "hello world";
/// assert_eq!(string.len(), 11);
///
/// assert_eq!(            string.head(5), "hello");
/// assert_eq!(        string.head_dot(5), "hello...");
/// assert_eq!(            string.tail(5), "world");
/// assert_eq!(        string.tail_dot(5), "...world");
/// assert_eq!(    string.head_tail(5, 5), "helloworld");
/// assert_eq!(string.head_tail_dot(5, 5), "hello...world");
/// ```
///
/// The characters are split as `UTF-8` characters, so strings like this will work:
/// ```rust
/// use readable::str::HeadTail;
///
/// let emojis = "🦀🦀🦀🐸🐸🐸";
/// assert_eq!(emojis.len(), 24);
///
/// assert_eq!(            emojis.head(2), "🦀🦀");
/// assert_eq!(        emojis.head_dot(2), "🦀🦀...");
/// assert_eq!(            emojis.tail(2), "🐸🐸");
/// assert_eq!(        emojis.tail_dot(2), "...🐸🐸");
/// assert_eq!(    emojis.head_tail(2, 2), "🦀🦀🐸🐸");
/// assert_eq!(emojis.head_tail_dot(2, 2), "🦀🦀...🐸🐸");
/// ```
///
/// ## Returned [`HeadTail`] Types
/// All types returned by this trait can compare with strings
/// without any allocation, e.g:
/// ```rust
/// # use readable::str::HeadTail;
/// let emojis = "🦀🦀🦀🐸🐸🐸";
/// // This comparison isn't allocating anything.
/// assert_eq!(emojis.head_tail_dot(2, 2), "🦀🦀...🐸🐸");
/// ```
///
/// The `head + tail` types can selectively show each side:
/// ```rust
/// use readable::str::{HeadTail, HeadTailStr};
///
/// let emojis: &str = "🦀🦀🦀🐸🐸🐸";
/// let headtail: HeadTailStr = emojis.head_tail(1, 1);
/// assert_eq!(headtail.head(), "🦀");
/// assert_eq!(headtail.tail(), "🐸");
/// ```
///
/// And they all implement [`std::fmt::Display`], so they can also use `.to_string()`:
/// ```rust
/// use readable::str::{
///     // This is the main trait.
///     HeadTail,
///     // This is the returned struct
///     // holding `str` references.
///     HeadTailDot
/// };
///
/// let string: &str = "hello world";
/// let dot: HeadTailDot = string.head_tail_dot(2, 2);
///
/// // No allocation needed here.
/// assert_eq!(dot, "he...ld");
///
/// // Now it's an owned String.
/// let new: String = dot.to_string();
/// assert_eq!(new, "he...ld");
/// ```
pub trait HeadTail: AsRef<str> {
	/// Return the first `head` UTF-8 characters of this [`str`].
	///
	/// This will return the full [`str`] if `head` is
	/// longer than the actual inner [`str`].
	///
	/// ```rust
	/// # use readable::str::HeadTail;
	/// let string = "hello world";
	/// assert_eq!(string.head(5), "hello");
	/// ```
	fn head(&self, head: usize) -> Head<'_> {
		let s = self.as_ref();

		#[allow(clippy::string_slice)]
		if let Some((index, _)) = s.char_indices().nth(head) {
			Head { string: &s[..index], cut: true }
		} else {
			Head { string: s, cut: false }
		}
	}

	/// Same as [`HeadTail::head()`] but this will allocate
	/// a new [`String`] ending with `...`.
	///
	/// This will not allocate and will return the input without
	/// `...` if `head` is longer than the actual inner [`str`].
	///
	/// ```rust
	/// # use readable::str::HeadTail;
	/// let string = "hello world";
	/// assert_eq!(string.head_dot(5), "hello...");
	///
	/// // No dot appended.
	/// let string = "hello world";
	/// assert_eq!(string.head_dot(11), string);
	/// ```
	fn head_dot(&self, head: usize) -> HeadDot<'_> {
		let s = self.as_ref();

		#[allow(clippy::string_slice)]
		if let Some((index, _)) = s.char_indices().nth(head) {
			let mut string = String::with_capacity(s.len() + 3);
			string += &s[..index];
			string += DOT;
			HeadDot { cow: Cow::Owned(string) }
		} else {
			HeadDot { cow: Cow::Borrowed(s) }
		}
	}

	/// Return the last `tail` UTF-8 characters of this [`str`].
	///
	/// This will return the full [`str`] if `tail` is
	/// longer than the actual inner [`str`].
	///
	/// ```rust
	/// # use readable::str::HeadTail;
	/// let string = "hello world";
	/// assert_eq!(string.tail(5), "world");
	/// ```
	fn tail(&self, tail: usize) -> Tail<'_> {
		let s = self.as_ref();

		let end = s.chars().count();

		if tail >= end {
			return Tail { string: s, cut: false };
		}

		#[allow(clippy::string_slice)]
		if let Some((index, _)) = s.char_indices().nth(end - tail) {
			Tail { string: &s[index..], cut: true }
		} else {
			Tail { string: s, cut: false }
		}
	}

	/// Same as [`HeadTail::tail()`] but this allocated
	/// a new [`String`] ending with `...`.
	///
	/// This will return the full string without `...` if
	/// `tail` is longer than the actual inner [`str`].
	///
	/// ```rust
	/// # use readable::str::HeadTail;
	/// let string = "hello world";
	/// assert_eq!(string.tail_dot(5), "...world");
	///
	/// // No dot appended.
	/// let string = "hello world";
	/// assert_eq!(string.tail_dot(11), string);
	/// ```
	fn tail_dot(&self, tail: usize) -> TailDot<'_> {
		let s = self.as_ref();

		let end = s.chars().count();

		if tail >= end {
			return TailDot { cow: Cow::Borrowed(s) }
		}

		#[allow(clippy::string_slice)]
		if let Some((index, _)) = s.char_indices().nth(end - tail) {
			let mut string = String::with_capacity(end + 3);
			string += DOT;
			string += &s[index..];
			TailDot { cow: Cow::Owned(string) }
		} else {
			TailDot { cow: Cow::Borrowed(s) }
		}
	}

	/// Return the first `head` UTF-8 characters and last `tail`
	/// UTF-8 characters of this [`str`].
	///
	/// ```rust
	/// # use readable::str::HeadTail;
	/// let string = "hello world";
	/// assert_eq!(string.head_tail(5, 5), "helloworld");
	///
	/// // No string allocated for this.
	/// let string = "hello world";
	/// assert_eq!(string.head_tail(6, 5), string);
	///
	/// // Non-ASCII characters.
	/// let sixteen_chars = "🦀🦀🐸🐸";
	/// let four_chars    = "ですけど";
	///
	/// assert_eq!(sixteen_chars.len(), 16);
	/// assert_eq!(four_chars.len(),    12);
	///
	/// assert_eq!(sixteen_chars.head_tail(1, 1), "🦀🐸");
	/// assert_eq!(four_chars.head_tail(1, 1),    "でど");
	///
	/// assert_eq!(sixteen_chars.head_tail(2, 2), sixteen_chars);
	/// assert_eq!(four_chars.head_tail(2, 2),    four_chars);
	/// ```
	fn head_tail(&self, head: usize, tail: usize) -> HeadTailStr<'_> {
		let s = self.as_ref();

		let end = s.chars().count();

		if head + tail >= end {
			return HeadTailStr { head: s, tail: None }
		}

		// Iterator is consumed, must create twice.
		let head = s.char_indices().nth(head);
		let tail = s.char_indices().nth(end - tail);

		#[allow(clippy::string_slice)]
		if let (Some((head, _)), Some((tail, _))) = (head, tail) {
			HeadTailStr { head: &s[..head], tail: Some(&s[tail..]) }
		} else {
			HeadTailStr { head: s, tail: None }
		}
	}

	/// Return the first `head` UTF-8 characters and last `tail`
	/// UTF-8 characters of this [`str`] separated with `...`.
	///
	/// ```rust
	/// # use readable::str::HeadTail;
	/// let string = "hello world";
	/// assert_eq!(string.head_tail_dot(5, 5), "hello...world");
	///
	/// // No dot appended.
	/// let string = "hello world";
	/// assert_eq!(string.head_tail_dot(6, 5), string);
	///
	/// // Non-ASCII characters.
	/// let sixteen_chars = "🦀🦀🐸🐸";
	/// let four_chars    = "ですけど";
	///
	/// assert_eq!(sixteen_chars.len(), 16);
	/// assert_eq!(four_chars.len(),    12);
	///
	/// assert_eq!(sixteen_chars.head_tail_dot(1, 1), "🦀...🐸");
	/// assert_eq!(four_chars.head_tail_dot(1, 1),    "で...ど");
	///
	/// assert_eq!(sixteen_chars.head_tail_dot(3, 3), sixteen_chars);
	/// assert_eq!(four_chars.head_tail_dot(2, 2),    four_chars);
	/// ```
	fn head_tail_dot(&self, head: usize, tail: usize) -> HeadTailDot<'_> {
		let s = self.as_ref();

		let end = s.chars().count();

		if head + tail >= end {
			return HeadTailDot { head: s, tail: None }
		}

		// Iterator is consumed, must create twice.
		let head = s.char_indices().nth(head);
		let tail = s.char_indices().nth(end - tail);

		#[allow(clippy::string_slice)]
		if let (Some((head, _)), Some((tail, _))) = (head, tail) {
			HeadTailDot { head: &s[..head], tail: Some(&s[tail..]) }
		} else {
			HeadTailDot { head: s, tail: None }
		}
	}
}

//---------------------------------------------------------------------------------------------------- HeadTail structs
/// Struct returned from [`HeadTail::head()`]
///
/// This struct:
/// - Implements [`PartialEq`] with strings
/// - Implements [`Display`] and thus `.to_string()`
/// - Can indicate whether the input string was actually cut or not
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Head<'a> {
	string: &'a str,
	cut: bool,
}

/// Struct returned from [`HeadTail::tail()`]
///
/// This struct:
/// - Implements [`PartialEq`] with strings
/// - Implements [`Display`] and thus `.to_string()`
/// - Can indicate whether the input string was actually cut or not
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Tail<'a> {
	string: &'a str,
	cut: bool,
}

macro_rules! impl_string {
	($($name:ident),*) => {
		$(
			impl<'a> $name<'a> {
				#[inline]
				#[must_use]
				/// Returns the inner `string`, whether cut off or not
				pub const fn as_str(&self) -> &str {
					self.string
				}

				#[inline]
				#[must_use]
				/// Returns the inner parts that make this type up.
				///
				/// The returned `str` is the head/tail portion.
				///
				/// The returned `bool` is whether the input string was cut or not.
				///
				/// ```rust
				/// # use readable::str::HeadTail;
				/// let string = "hello world";
				///
				/// // Input (11) can capture the whole string, so no cutting.
				/// let (head, cut) = string.head(11).into_parts();
				/// assert_eq!(head, string);
				/// assert!(!cut);
				///
				/// // If it can't capture it all (5 != 11),
				/// // then a `cut` will be `true`.
				/// let (head, cut) = string.head(5).into_parts();
				/// assert_eq!(head, "hello");
				/// assert!(cut);
				/// ```
				pub const fn into_parts(self) -> (&'a str, bool) {
					(self.string, self.cut)
				}

				#[inline]
				#[must_use]
				/// Returns `true` is the string was cut in any way.
				///
				/// Returns `false` if running `.to_string()` on this
				/// would result in the same string as the input string.
				pub const fn cut(&self) -> bool {
					self.cut
				}
			}
			impl PartialEq<str> for $name<'_> {
				fn eq(&self, other: &str) -> bool {
					self.string == other
				}
			}
			impl PartialEq<&str> for $name<'_> {
				fn eq(&self, other: &&str) -> bool {
					self.string == *other
				}
			}
			impl AsRef<str> for $name<'_> {
				fn as_ref(&self) -> &str {
					self.string
				}
			}
			impl std::ops::Deref for $name<'_> {
				type Target = str;

				fn deref(&self) -> &Self::Target {
					self.string
				}
			}
			impl fmt::Display for $name<'_> {
				fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
					write!(f, "{}", self.string)
				}
			}
		)*
	};
}
impl_string!(Head, Tail);

//---------------------------------------------------------------------------------------------------- HeadTail Cow
/// Struct returned from [`HeadTail::head_dot()`]
///
/// This struct:
/// - Implements [`PartialEq`] with strings
/// - Implements [`Display`] and thus `.to_string()`
/// - Can indicate whether the input string was actually cut or not
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct HeadDot<'a> {
	cow: Cow<'a, str>,
}

/// Struct returned from [`HeadTail::tail_dot()`]
///
/// This struct:
/// - Implements [`PartialEq`] with strings
/// - Implements [`Display`] and thus `.to_string()`
/// - Can indicate whether the input string was actually cut or not
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct TailDot<'a> {
	cow: Cow<'a, str>,
}

macro_rules! impl_cow {
	($($name:ident),*) => {
		$(
			impl PartialEq<str> for $name<'_> {
				fn eq(&self, other: &str) -> bool {
					self.cow == other
				}
			}
			impl PartialEq<&str> for $name<'_> {
				fn eq(&self, other: &&str) -> bool {
					self.cow == *other
				}
			}
			impl fmt::Display for $name<'_> {
				fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
					write!(f, "{}", self.cow)
				}
			}
			impl AsRef<str> for $name<'_> {
				fn as_ref(&self) -> &str {
					self.as_str()
				}
			}
			impl std::ops::Deref for $name<'_> {
				type Target = str;

				fn deref(&self) -> &Self::Target {
					self.as_ref()
				}
			}
			impl<'a> $name<'a> {
				#[inline]
				#[must_use]
				/// Returns the inner `string`, whether cut off or not
				pub fn as_str(&self) -> &str {
					match self.cow {
						Cow::Borrowed(s) => s,
						Cow::Owned(ref s) => s,
					}
				}

				#[inline]
				#[must_use]
				/// Returns the inner `Cow<'a, str>`.
				///
				/// The returned `str` is the head/tail portion.
				///
				/// If the [`Cow`] is [`Cow::Owned`] then it means the string was cut.
				///
				/// If the [`Cow`] is [`Cow::Borrowed`] then it means the string was not cut.
				///
				/// ```rust
				/// # use readable::str::HeadTail;
				/// # use std::borrow::Cow;
				/// let string = "hello world";
				///
				/// // Input (11) can capture the whole string,
				/// // so there is no allocation or dot.
				/// let head = string.head_dot(11).into_cow();
				/// assert_eq!(head, string);
				/// match head {
				///     Cow::Owned(_)    => unreachable!(),
				///     Cow::Borrowed(_) => (),
				/// }
				///
				/// // If it can't capture it all (5 != 11),
				/// // then a string is allocated suffixed with `...`
				/// let head = string.head_dot(5).into_cow();
				/// assert_eq!(head, "hello...");
				/// match head {
				///     Cow::Owned(_)    => (),
				///     Cow::Borrowed(_) => unreachable!(),
				/// }
				/// ```
				pub fn into_cow(self) -> Cow<'a, str> {
					self.cow
				}

				#[inline]
				#[must_use]
				/// Returns `true` is the string was cut in any way.
				///
				/// Returns `false` if running `.to_string()` on this
				/// would result in the same string as the input string.
				pub const fn cut(&self) -> bool {
					match self.cow {
						Cow::Borrowed(_) => false,
						Cow::Owned(_) => true,
					}
				}
			}
		)*
	};
}
impl_cow!(HeadDot, TailDot);

//---------------------------------------------------------------------------------------------------- HeadTail cut
/// Struct returned from [`HeadTail::head_tail()`]
///
/// This struct:
/// - Implements no-allocation [`PartialEq`] with [`str`]
/// - Implements [`Display`] and thus `.to_string()`
/// - Can selectively show head/tail portions
/// - Can indicate whether the input string was actually cut or not
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct HeadTailStr<'a> {
	// This holds the whole string if `head + tail > input_len`
	head: &'a str,
	tail: Option<&'a str>,
}
impl fmt::Display for HeadTailStr<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.tail {
			Some(t) => write!(f, "{}{t}", self.head),
			None    => write!(f, "{}", self.head),
		}
	}
}
impl HeadTailStr<'_> {
	#[inline]
	fn str_cmp(&self, other: &str) -> bool {
		match self.tail {
			Some(t) => {
				let head_bytes = self.head.as_bytes();
				let tail_bytes = t.as_bytes();
				let str_bytes  = other.as_bytes();

				let head_len  = head_bytes.len();
				let tail_len  = tail_bytes.len();
				let total_len = head_len + tail_len;

				total_len == str_bytes.len() &&
				head_bytes == &str_bytes[..head_len] &&
				tail_bytes == &str_bytes[tail_len..]
			},

			None => self.head == other,
		}
	}
}

/// Struct returned from [`HeadTail::head_tail_dot()`]
///
/// This struct:
/// - Implements no-allocation [`PartialEq`] with [`str`]
/// - Implements [`Display`] and thus `.to_string()`
/// - Can selectively show head/tail portions
/// - Can indicate whether the input string was actually cut or not
///
/// ## Note
/// If [`Self::cut()`] is `true`, the proper string to
/// compare against would also contain `...`.
///
/// If [`Self::cut()`] is `false`, no dot will appear
/// after using `.to_string()`, so a string without `...`
/// would be correct to compare against.
///
/// ```rust
/// # use readable::str::HeadTail;
/// let string = "head tail";
/// let dot    = string.head_tail_dot(4, 4);
/// assert_eq!(dot, "head...tail");
/// assert_eq!(dot.to_string(), "head...tail");
///
/// let no_dot = string.head_tail_dot(5, 4);
/// assert_eq!(no_dot, "head tail");
/// assert_eq!(no_dot.to_string(), "head tail");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct HeadTailDot<'a> {
	// This holds the whole string if `head + tail > input_len`
	head: &'a str,
	tail: Option<&'a str>,
}
impl fmt::Display for HeadTailDot<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.tail {
			Some(t) => write!(f, "{}{DOT}{t}", self.head),
			None    => write!(f, "{}", self.head),
		}
	}
}
impl HeadTailDot<'_> {
	#[inline]
	fn str_cmp(&self, other: &str) -> bool {
		match self.tail {

			Some(t) => {
				let head_bytes = self.head.as_bytes();
				let tail_bytes = t.as_bytes();
				let str_bytes  = other.as_bytes();

				let head_len  = head_bytes.len();
				let tail_len  = tail_bytes.len();
				let total_len = head_len + tail_len;

				// Tail exists, that means the string was
				// cut so we should be expecting `...`
				(total_len + 3) == str_bytes.len() &&
				head_bytes == &str_bytes[..head_len] &&
				tail_bytes == &str_bytes[tail_len + 3..]
			},

			None => self.head == other,
		}
	}
}

macro_rules! impl_head_tail {
	($($name:ident),*) => {
		$(
			impl PartialEq<str> for $name<'_> {
				fn eq(&self, other: &str) -> bool {
					self.str_cmp(other)
				}
			}
			impl PartialEq<&str> for $name<'_> {
				fn eq(&self, other: &&str) -> bool {
					self.str_cmp(other)
				}
			}
			impl<'a> $name<'a> {
				#[inline]
				#[must_use]
				/// Returns `true` is the string was cut in any way.
				///
				/// Returns `false` if running `.to_string()` on this
				/// would result in the same string as the input string.
				pub const fn cut(&self) -> bool {
					self.tail.is_some()
				}

				#[inline]
				#[must_use]
				/// Return the only `head` portion of the string
				pub const fn head(&self) -> &str {
					self.head
				}

				#[inline]
				#[must_use]
				/// Return the only `tail` portion of the string
				pub const fn tail(&self) -> &str {
					match self.tail {
						Some(t) => t,
						None => self.head,
					}
				}

				#[inline]
				#[must_use]
				/// Returns the inner `head/tail` `str`'s that make this type up.
				///
				/// The returned `&'a str` is the `head` portion.
				/// If the entire input string could fit, then this will
				/// contain the entire input string and `tail` will be [`None`].
				///
				/// The returned `Option<&'a str>` is the `tail` portion.
				/// If this is [`Some`] that means the input string was cut.
				/// If it is [`None`] that means the input string was not cut
				/// and the entire input resides inside the `head` portion.
				///
				/// ```rust
				/// # use readable::str::HeadTail;
				/// let string = "hello world";
				///
				/// // Input (6+5 == 11) can capture the whole string.
				/// let headtail = string.head_tail(6, 5);
				///
				/// // So there is no tail, everything is in head.
				/// let (head, tail) = headtail.into_parts();
				/// assert_eq!(head, string);
				/// assert_eq!(tail, None);
				///
				/// // If it can't capture it all (5+5 != 11), then there is a tail.
				/// let headtail = string.head_tail(5, 5);
				/// let (head, tail) = headtail.into_parts();
				/// assert_eq!(head, "hello");
				/// assert_eq!(tail, Some("world"));
				/// ```
				pub const fn into_parts(self) -> (&'a str, Option<&'a str>) {
					(self.head, self.tail)
				}
			}
		)*

	};
}
impl_head_tail!(HeadTailStr, HeadTailDot);

//---------------------------------------------------------------------------------------------------- TESTS
// #[cfg(test)]
// mod tests {
// 	use super::*;
// }
