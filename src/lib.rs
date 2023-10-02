#![doc = include_str!("../README.md")]

//---------------------------------------------------------------------------------------------------- Docs
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//---------------------------------------------------------------------------------------------------- Lints
#![forbid(
	future_incompatible,
	let_underscore,
	break_with_label_and_loop,
	coherence_leak_check,
	deprecated,
	duplicate_macro_attributes,
	exported_private_dependencies,
	for_loops_over_fallibles,
	large_assignments,
	overlapping_range_endpoints,
	private_in_public,
	semicolon_in_expressions_from_macros,
	redundant_semicolons,
	unconditional_recursion,
	unused_allocation,
	unused_braces,
	unused_doc_comments,
	unused_labels,
	unused_unsafe,
	while_true,
	keyword_idents,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	unreachable_pub,
	single_use_lifetimes,
	variant_size_differences,
	unused_mut,
)]
#![deny(
	unused_comparisons,
	nonstandard_style,
)]

//---------------------------------------------------------------------------------------------------- Hidden imports
pub(crate) mod macros;

#[cfg(feature = "str")]
/// General string utilities
pub mod str;
pub use str::{
	Str,HeadTail,
};


#[cfg(feature = "num")]
/// Human-readable number formatting
pub mod num;
pub use num::{
	Unsigned,Int,Float,Percent,
};
pub(crate) use toa::Itoa64;

#[cfg(feature = "time")]
/// Human-readable time & date formatting
///
/// ## Runtime
/// This module includes various [`Runtime`] types meant for audio/video style formatting (`HH:MM:SS`).
///
/// The basic type is [`Runtime`] which formats strings to what you would expect from most audio/video players, e.g:
/// ```rust
/// # use readable::*;
/// assert_eq!(Runtime::from(0),    "0:00");
/// assert_eq!(Runtime::from(60),   "1:00");
/// assert_eq!(Runtime::from(119),  "1:59");
/// assert_eq!(Runtime::from(3599), "59:59");
/// assert_eq!(Runtime::from(3600), "1:00:00");
/// assert_eq!(Runtime::max(),      "99:59:59");
/// ```
///
/// All [`Runtime`] times can losslessly be converted into each-other using [`From`].
///
/// Here's a diagram of:
/// - What the type's formatting look like
/// - What their sub/super-set relationship is
///
/// <img src="https://github.com/hinto-janai/readable/assets/101352116/424b91fd-7df1-493c-bf85-fcb264470c75" width="50%"/>
pub mod time;
pub use time::{
	Date,Runtime,Time,RuntimePad,RuntimeMilli,RuntimeUnion,
};

#[cfg(feature = "toa")]
/// Fast integer/float to string conversion
///
/// Uses [`itoa`](https://github.com/dtolnay/itoa) & [`dtoa`](https://github.com/dtolnay/dtoa) by `dtolnay` internally.
///
/// These types are for quick formatting, and do not do any `readable`-style formatting (adding commas),
/// it simply converts an numbers into strings (but much faster than [`format!()`]).
///
/// The strings are stack allocated.
///
/// ```rust
/// use readable::{Itoa, Dtoa, Unsigned, Float};
///
/// // No formatting, is extremely fast to create.
/// let itoa = Itoa::new(1000_u32);
/// let dtoa = Dtoa::new(1000.0_f32);
/// assert_eq!(itoa, "1000");   // No comma!
/// assert_eq!(dtoa, "1000.0"); // No comma!
///
/// // The `readable` counterparts, probably
/// // slower (but still very fast).
/// let u = Unsigned::from(1000_u32);
/// let f = Float::from(1000.0_f32);
/// assert_eq!(u, "1,000");     // Comma!
/// assert_eq!(f, "1,000.000"); // Comma!
/// ```
pub mod toa;
pub use toa::{
	Itoa,ItoaTmp,Dtoa,DtoaTmp,
};
