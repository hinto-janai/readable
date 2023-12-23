//! General string utilities
//!
//! This module contain [`Str`], the stack-based that
//! backs almost all string types in `readable`.
//!
//! It also contains some general string utilities.

mod str;
pub use str::Str;

mod headtail;
pub use headtail::{
	DOT,
	HeadTail,Head,Tail,HeadDot,
	TailDot,HeadTailStr,HeadTailDot,
};
