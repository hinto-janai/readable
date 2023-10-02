//! Fast integer/float to string conversion
//!
//! Uses [`itoa`](https://github.com/dtolnay/itoa) & [`dtoa`](https://github.com/dtolnay/dtoa) by `dtolnay` internally.
//!
//! These types are for quick formatting, and do not do any `readable`-style formatting (adding commas),
//! it simply converts an numbers into strings (but much faster than [`format!()`]).
//!
//! The strings are stack allocated.
//!
//! ```rust
//! use readable::{Itoa, Dtoa, Unsigned, Float};
//!
//! // No formatting, is extremely fast to create.
//! let itoa = Itoa::new(1000_u32);
//! let dtoa = Dtoa::new(1000.0_f32);
//! assert_eq!(itoa, "1000");   // No comma!
//! assert_eq!(dtoa, "1000.0"); // No comma!
//!
//! // The `readable` counterparts, probably
//! // slower (but still very fast).
//! let u = Unsigned::from(1000_u32);
//! let f = Float::from(1000.0_f32);
//! assert_eq!(u, "1,000");     // Comma!
//! assert_eq!(f, "1,000.000"); // Comma!
//! ```

//---------------------------------------------------------------------------------------------------- Dtoa
#[macro_use]
mod diyfp;
#[macro_use]
mod dtoa;
pub use dtoa::{
	Dtoa,IntoDtoa,DtoaTmp,
};

//---------------------------------------------------------------------------------------------------- Itoa
mod itoa;
mod udiv128;
pub use itoa::{
	Itoa,ItoaTmp,Integer
};
pub(crate) use itoa::Itoa64;