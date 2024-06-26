//---------------------------------------------------------------------------------------------------- Macros for `crate::num::*`

//---------------------------------------------------------------------------------------------------- Common functions.
macro_rules! impl_common {
    ($num:ty) => {
        #[inline]
        #[must_use]
        /// Returns the inner number.
        pub const fn inner(&self) -> $num {
            self.0
        }
    };
}
pub(super) use impl_common;

//---------------------------------------------------------------------------------------------------- Common constant functions.
macro_rules! impl_const {
    () => {
        #[inline]
        #[must_use]
        /// Return a borrowed [`str`] without consuming [`Self`].
        pub const fn as_str(&self) -> &str {
            self.1.as_str()
        }

        #[inline]
        #[must_use]
        /// Returns the _valid_ byte slice of the inner [`String`]
        ///
        /// These bytes can _always_ safely be used for [`std::str::from_utf8_unchecked`].
        pub const fn as_bytes(&self) -> &[u8] {
            self.1.as_bytes()
        }

        #[inline]
        #[must_use]
        #[allow(clippy::len_without_is_empty)]
        /// The length of the inner [`String`]
        pub const fn len(&self) -> usize {
            self.1.len()
        }

        #[inline]
        #[must_use]
        /// The length of the inner [`String`] as a [`u8`]
        pub const fn len_u8(&self) -> u8 {
            self.1.len_u8()
        }
    };
}
pub(crate) use impl_const;

//---------------------------------------------------------------------------------------------------- Implement above for non-const
macro_rules! impl_not_const {
    () => {
        #[inline]
        #[must_use]
        /// Return a borrowed [`str`] without consuming [`Self`].
        pub fn as_str(&self) -> &str {
            self.1.as_str()
        }

        #[inline]
        #[must_use]
        /// Returns the _valid_ byte slice of the inner [`String`]
        ///
        /// These bytes can _always_ safely be used for [`std::str::from_utf8_unchecked`].
        pub fn as_bytes(&self) -> &[u8] {
            self.1.as_bytes()
        }

        #[inline]
        #[must_use]
        /// The length of the inner [`String`]
        pub fn len(&self) -> usize {
            self.1.len()
        }

        #[inline]
        #[must_use]
        /// If the inner [`String`] is empty or not
        pub fn is_empty(&self) -> bool {
            self.1.is_empty()
        }
    };
}
pub(crate) use impl_not_const;

//---------------------------------------------------------------------------------------------------- `usize` functions
macro_rules! impl_usize {
    () => {
        #[inline]
        #[cfg(target_pointer_width = "64")]
        #[must_use]
        /// Returns the inner number as a [`usize`].
        ///
        /// # Notes
        /// This function is only available on 64-bit platforms.
        pub const fn usize(&self) -> usize {
            self.0 as usize
        }
    };
}
pub(crate) use impl_usize;

//---------------------------------------------------------------------------------------------------- `isize` functions
macro_rules! impl_isize {
    () => {
        #[inline]
        #[cfg(target_pointer_width = "64")]
        #[must_use]
        /// Returns the inner number as an [`isize`].
        ///
        /// # Notes
        /// This function is only available on 64-bit platforms.
        pub const fn isize(&self) -> isize {
            self.0 as isize
        }
    };
}
pub(crate) use impl_isize;

//---------------------------------------------------------------------------------------------------- Implement common traits
macro_rules! impl_traits {
    ($s:ty, $num:ty) => {
        impl std::ops::Deref for $s {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }

        impl AsRef<str> for $s {
            #[inline]
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl AsRef<[u8]> for $s {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                self.as_bytes()
            }
        }

        impl std::borrow::Borrow<str> for $s {
            #[inline]
            fn borrow(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $s {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", &self.1.as_str())
            }
        }

        impl std::default::Default for $s {
            #[inline]
            /// Returns [`Self::ZERO`]
            fn default() -> Self {
                Self::ZERO
            }
        }

        impl PartialEq<&$s> for $s {
            #[inline]
            fn eq(&self, other: &&$s) -> bool {
                self == other
            }
        }

        impl PartialEq<$s> for &$s {
            #[inline]
            fn eq(&self, other: &$s) -> bool {
                self == other
            }
        }

        impl PartialEq<str> for $s {
            #[inline]
            fn eq(&self, other: &str) -> bool {
                self.1.as_str() == other
            }
        }

        impl PartialEq<$s> for str {
            #[inline]
            fn eq(&self, other: &$s) -> bool {
                self == other.1.as_str()
            }
        }

        impl PartialEq<&str> for $s {
            #[inline]
            fn eq(&self, other: &&str) -> bool {
                &self.1.as_str() == other
            }
        }

        impl PartialEq<&$s> for str {
            #[inline]
            fn eq(&self, other: &&$s) -> bool {
                self == other.1.as_str()
            }
        }

        impl PartialEq<$num> for $s {
            #[inline]
            fn eq(&self, other: &$num) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<$s> for $num {
            #[inline]
            fn eq(&self, other: &$s) -> bool {
                *self == other.0
            }
        }

        impl PartialEq<$num> for &$s {
            #[inline]
            fn eq(&self, other: &$num) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<&$s> for $num {
            #[inline]
            fn eq(&self, other: &&$s) -> bool {
                *self == other.0
            }
        }

        // Ord
        impl PartialOrd<str> for $s {
            #[inline]
            fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
                Some(self.1.as_str().cmp(other))
            }
        }

        impl PartialOrd<$s> for str {
            #[inline]
            fn partial_cmp(&self, other: &$s) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other.1.as_str()))
            }
        }

        impl PartialOrd<&str> for $s {
            #[inline]
            fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
                Some(self.1.as_str().cmp(other))
            }
        }

        impl PartialOrd<&$s> for str {
            #[inline]
            fn partial_cmp(&self, other: &&$s) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other.1.as_str()))
            }
        }

        impl PartialOrd<$num> for $s {
            #[inline]
            fn partial_cmp(&self, other: &$num) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl PartialOrd<$s> for $num {
            #[inline]
            fn partial_cmp(&self, other: &$s) -> Option<std::cmp::Ordering> {
                self.partial_cmp(&other.0)
            }
        }

        impl PartialOrd<$num> for &$s {
            #[inline]
            fn partial_cmp(&self, other: &$num) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl PartialOrd<&$s> for $num {
            #[inline]
            fn partial_cmp(&self, other: &&$s) -> Option<std::cmp::Ordering> {
                self.partial_cmp(&other.0)
            }
        }
    };
}
pub(crate) use impl_traits;

//---------------------------------------------------------------------------------------------------- Math Traits
// Macro for a math macro impl.
macro_rules! impl_impl_math {
    ($trait_word:ident, $operator:tt, $s:ty, $num:ty) => {
        paste::paste! {
            // Standard ops.
            impl std::ops::[<$trait_word>]<$s> for $s {
                type Output = Self;
                #[inline]
                fn [<$trait_word:lower>](self, other: $s) -> Self {
                    let r = self.inner() $operator other.inner();
                    Self::from(r)
                }
            }
            impl std::ops::[<$trait_word>]<$num> for $s {
                type Output = Self;
                #[inline]
                fn [<$trait_word:lower>](self, other: $num) -> Self {
                    Self::from(self.inner() $operator other)
                }
            }
            impl std::ops::[<$trait_word>]<$s> for $num {
                type Output = Self;
                #[inline]
                fn [<$trait_word:lower>](self, other: $s) -> Self {
                    Self::from(self $operator other.inner())
                }
            }
            impl std::ops::[<$trait_word>]<&$s> for $s {
                type Output = Self;
                #[inline]
                fn [<$trait_word:lower>](self, other: &$s) -> Self {
                    Self::from(self.inner() $operator other.inner())
                }
            }
            impl std::ops::[<$trait_word>]<&$num> for $s {
                type Output = Self;
                #[inline]
                fn [<$trait_word:lower>](self, other: &$num) -> Self {
                    Self::from(self.inner() $operator other)
                }
            }
            impl std::ops::[<$trait_word>]<&$s> for $num {
                type Output = Self;
                #[inline]
                fn [<$trait_word:lower>](self, other: &$s) -> Self {
                    Self::from(self $operator other.inner())
                }
            }
        }
    };
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

//---------------------------------------------------------------------------------------------------- Handle bad floats
macro_rules! return_bad_float {
    ($float:ident, $nan:expr, $infinite:expr) => {
        match $float.classify() {
            std::num::FpCategory::Normal => (),
            std::num::FpCategory::Nan => return $nan,
            std::num::FpCategory::Infinite => return $infinite,
            _ => (),
        }
    };
}
pub(crate) use return_bad_float;

//---------------------------------------------------------------------------------------------------- `u64/i64` -> `str`
macro_rules! str_u64 {
    ($number:expr) => {{
        $crate::num::Unsigned::from_priv_inner($number).as_str()
    }};
}
pub(crate) use str_u64;

//---------------------------------------------------------------------------------------------------- `u64/i64` -> `str`
macro_rules! str_i64 {
    ($number:expr) => {{
        $crate::num::Int::from_priv_inner($number).as_str()
    }};
}
pub(crate) use str_i64;

//---------------------------------------------------------------------------------------------------- `u64/i64` -> `str`
macro_rules! handle_over_u32 {
    ($value:expr, $type:ty) => {
        if $value > (u32::MAX as $type) {
            return Self::UNKNOWN;
        }
    };
}
pub(crate) use handle_over_u32;

// //---------------------------------------------------------------------------------------------------- serde impl
// // Macro to implement all the serde functions.
// macro_rules! impl_serde {
// 	(
// 		serde =>             // Serde test/docs
// 		$(#[$serde:meta])*   //
// 		bincode =>           // Bincode test/docs
// 		$(#[$bincode:meta])* //
// 		borsh =>             // Borsh test/docs
// 		$(#[$borsh:meta])*   //
// 		$inner:ty,           // Inner number representation of the string type
// 		$name:ty,            // Name of the actual type being implemented on
// 		$new:ident           // Constructor function
// 		$(,)?
// 	) => {
// 		#[cfg(feature = "serde")]
// 		impl serde::Serialize for $name {
// 			#[inline]
// 			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// 				where S: serde::Serializer
// 			{
// 				self.serialize(serializer)
// 			}
// 		}

// 		#[cfg(feature = "serde")]
// 		impl<'de> serde::Deserialize<'de> for $name {
// 			#[inline]
// 			$(#[$serde])*
// 			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// 				where D: serde::Deserializer<'de>
// 			{
// 				let this: Self = serde::Deserialize::deserialize(deserializer)?;

// 				// Protect against bad input.
// 				if cfg!(feature = "check_deserialization") {
// 					let de = Self::$new(this.inner());
// 					if de == this {
// 						Ok(this)
// 					} else {
// 						use serde::de::Error;
// 						Err(D::Error::custom(
// 							format!("deserialized version does not match new: de: {de:?}, new: {this:?}")
// 						))
// 					}
// 				} else {
// 					Ok(this)
// 				}
// 			}
// 		}

// 		#[cfg(feature = "bincode")]
// 		impl bincode::Encode for $name {
// 			#[inline]
// 			fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
// 				self.encode(encoder)
// 			}
// 		}

// 		#[cfg(feature = "bincode")]
// 		impl bincode::Decode for $name {
// 			#[inline]
// 			$(#[$bincode])*
// 			fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
// 				let this: Self = bincode::Decode::decode(decoder)?;

// 				// Protect against bad input.
// 				if cfg!(feature = "check_deserialization") {
// 					let de = Self::$new(this.inner());
// 					if de == this {
// 						Ok(this)
// 					} else {
// 						Err(bincode::error::DecodeError::OtherString(
// 							format!("deserialized version does not match new: de: {de:?}, new: {this:?}")
// 						))
// 					}
// 				} else {
// 					Ok(this)
// 				}
// 			}
// 		}
// 		#[cfg(feature = "bincode")]
// 		impl<'de> bincode::BorrowDecode<'de> for $name {
// 			#[inline]
// 			fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
// 				bincode::Decode::decode(decoder)
// 			}
// 		}

// 		#[cfg(feature = "borsh")]
// 		impl borsh::BorshSerialize for $name {
// 			#[inline]
// 			fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
// 				self.serialize(writer)
// 			}
// 		}

// 		#[cfg(feature = "borsh")]
// 		impl borsh::BorshDeserialize for $name {
// 			#[inline]
// 			$(#[$borsh])*
// 			fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
// 				let this: Self = borsh::BorshDeserialize::deserialize_reader(reader)?;

// 				// Protect against bad input.
// 				if cfg!(feature = "check_deserialization") {
// 					let de = Self::$new(this.inner());
// 					if de == this {
// 						Ok(this)
// 					} else {
// 						Err(borsh::io::Error::new(
// 							borsh::io::ErrorKind::InvalidData,
// 							format!("deserialized version does not match new: de: {de:?}, new: {this:?}")
// 						))
// 					}
// 				} else {
// 					Ok(this)
// 				}
// 			}
// 		}
// 	};
// }
// pub(crate) use impl_serde;
