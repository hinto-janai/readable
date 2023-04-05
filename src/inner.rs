//---------------------------------------------------------------------------------------------------- Use
#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};
use crate::constants::*;
use std::borrow::Borrow;
use std::cmp::Ordering;

//---------------------------------------------------------------------------------------------------- Inner type for `Int/Unsigned`
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Inner {
	Buf(num_format::Buffer),
	Zero,
	Unknown,
	Nan,
	Inf,
}

impl std::fmt::Display for Inner {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", &self.as_str())
	}
}


impl PartialOrd for Inner {
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.as_str().cmp(other.as_str()))
	}
}

impl Ord for Inner {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_str().cmp(&other.as_str())
	}
}

impl Eq for Inner {}

impl PartialEq for Inner {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.as_str() == other.as_str()
	}
}

impl PartialEq<str> for Inner {
	#[inline(always)]
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}

impl PartialEq<Inner> for str {
	#[inline(always)]
	fn eq(&self, other: &Inner) -> bool {
		self == other.as_str()
	}
}

impl AsRef<str> for Inner {
	#[inline(always)]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl Borrow<str> for Inner {
	#[inline(always)]
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

impl Inner {
	#[inline(always)]
	pub(crate) fn as_str(&self) -> &str {
		match self {
			crate::inner::Inner::Buf(b)  => b.as_str(),
			crate::inner::Inner::Zero    => crate::constants::ZERO,
			crate::inner::Inner::Unknown => crate::constants::UNKNOWN,
			crate::inner::Inner::Nan     => crate::constants::NAN,
			crate::inner::Inner::Inf     => crate::constants::INFINITY,
		}
	}

	#[inline(always)]
	pub(crate) fn len(&self) -> usize {
		self.as_str().len()
	}

	#[inline(always)]
	pub(crate) fn is_zero(&self) -> bool {
		self == &Self::Zero
	}

	#[inline(always)]
	pub(crate) fn is_unknown(&self) -> bool {
		self == &Self::Unknown
	}

	#[inline(always)]
	pub(crate) fn is_nan(&self) -> bool {
		self == &Self::Nan
	}

	#[inline(always)]
	pub(crate) fn is_inf(&self) -> bool {
		self == &Self::Inf
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
