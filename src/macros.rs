//---------------------------------------------------------------------------------------------------- Use
// Creates a num_format::Buffer, turns it into a compact_str::CompactString, and returns it.
macro_rules! buf {
	($number:expr) => {
		{
			let mut buf = ::num_format::Buffer::new();
			buf.write_formatted(&$number, &num_format::Locale::en);

			// SAFETY: numbers should always be valid UTF-8.
			unsafe { compact_str::CompactString::from_utf8_unchecked(buf.as_bytes()) }
		}
	}
}
pub(crate) use buf;

// "Handle NaN/Infinite" Macro.
macro_rules! handle_nan {
	($float:ident) => {
		#[cfg(not(feature = "ignore_nan_inf"))]
		{
			let fpcat = $float.classify();
			use std::num::FpCategory;
			match fpcat {
				FpCategory::Normal   => (),
				FpCategory::Nan      => return Self($float, ::compact_str::CompactString::new(crate::constants::NAN)),
				FpCategory::Infinite => return Self($float, ::compact_str::CompactString::new(crate::constants::INFINITY)),
				_ => (),
			}
		}
	}
}
pub(crate) use handle_nan;

// Implement `PartialEq`, `AsRef<str>`
macro_rules! impl_traits {
	($s:ident, $num:ident) => {
		impl std::fmt::Display for $s {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(f, "{}", &self.1)
			}
		}

		impl PartialEq<str> for $s {
			fn eq(&self, other: &str) -> bool {
				self.1 == other
			}
		}

		impl PartialEq<$s> for str {
			fn eq(&self, other: &$s) -> bool {
				self == other.1
			}
		}

		impl PartialEq<&str> for $s {
			fn eq(&self, other: &&str) -> bool {
				self.1 == other
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
