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
	unused_comparisons,
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
	nonstandard_style,
)]

//---------------------------------------------------------------------------------------------------- Hidden imports
#[doc(hidden)]
#[cfg(feature = "itoa")]
pub use itoa as __readable_itoa;
#[doc(hidden)]
#[cfg(feature = "ryu")]
pub use ryu as __readable_ryu;

pub(crate) mod macros;

//mod constants;
//pub use constants::*;

/// General string utilities
pub mod str;
pub use str::{
	Str,HeadTail,
};

mod free;

/// Number formatting
pub mod num;
pub use num::{
	Unsigned,Int,Float,Percent,
};

/// Time & date formatting
pub mod time;
pub use time::{
	Date,Runtime,Time,
};
