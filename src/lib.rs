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
#![allow(
	unused_braces,
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
pub use byte::{
	Byte,
};