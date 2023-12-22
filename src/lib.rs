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
	clippy::all,
	clippy::correctness,
	clippy::suspicious,
	clippy::style,
	clippy::complexity,
	clippy::perf,
	clippy::pedantic,
	clippy::restriction,
	clippy::nursery,
	clippy::cargo,
	unused_comparisons,
	nonstandard_style,
)]
#![allow(
	clippy::single_char_lifetime_names,
	clippy::implicit_return,
	clippy::std_instead_of_alloc,
	clippy::std_instead_of_core,
	clippy::unwrap_used,
	clippy::min_ident_chars,
	clippy::absolute_paths,
	clippy::missing_inline_in_public_items,
	clippy::arithmetic_side_effects,
	clippy::unwrap_in_result,
	clippy::pattern_type_mismatch,
	clippy::shadow_reuse,
	clippy::shadow_unrelated,
	clippy::missing_trait_methods,
	clippy::pub_use,
	clippy::pub_with_shorthand,
	clippy::blanket_clippy_restriction_lints,
	clippy::exhaustive_structs,
	clippy::exhaustive_enums,
	clippy::unsafe_derive_deserialize,
	clippy::multiple_inherent_impl,
	clippy::unreadable_literal,
	clippy::indexing_slicing,
	clippy::float_arithmetic,
	clippy::cast_possible_truncation,
	clippy::as_conversions,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::missing_asserts_for_indexing,
	clippy::default_numeric_fallback,
	clippy::module_inception,
	clippy::mod_module_files,
	clippy::multiple_unsafe_ops_per_block,
	clippy::too_many_lines,
	clippy::missing_assert_message,
	clippy::len_zero,
	clippy::separated_literal_suffix,
	clippy::single_call_fn,
	clippy::unreachable,
	clippy::many_single_char_names,
	clippy::redundant_pub_crate,
	clippy::decimal_literal_representation,
	clippy::option_if_let_else,
	clippy::lossy_float_literal,
	clippy::modulo_arithmetic,
	clippy::print_stdout,
	clippy::module_name_repetitions,
	clippy::no_effect,
	clippy::semicolon_outside_block,
	clippy::panic,
	clippy::question_mark_used,
	clippy::expect_used,
	clippy::integer_division,
	clippy::wildcard_imports,
	clippy::similar_names,
	clippy::multiple_crate_versions, // SOMEDAY: fix deps
	clippy::missing_docs_in_private_items, // SOMEDAY: fix me, document priv stuff
)]

//---------------------------------------------------------------------------------------------------- Hidden imports
pub(crate) mod macros;

pub mod str;
pub use str::{
	Str,HeadTail,
};

pub mod num;
pub use num::{
	Unsigned,Int,Float,Percent,
};
pub(crate) use toa::Itoa64;

pub mod run;
pub use run::{
	Runtime,RuntimePad,RuntimeMilli,RuntimeUnion,
};

pub mod up;
pub use up::{
	Uptime,UptimeFull,Htop,SysUptime,
};

pub mod time;
pub use time::{
	SysTime,Time,TimeUnit,Military,
};

pub mod date;
pub use date::{
	Date,Nichi,NichiFull,SysDate,
};

pub mod toa;
pub use toa::{
	Itoa,ItoaTmp,Dtoa,DtoaTmp,
};

pub mod byte;
pub use byte::Byte;