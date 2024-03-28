//! General string utilities
//!
//! This module contain [`Str`], the stack-based that
//! backs almost all string types in `readable`.
//!
//! It also contains some general string utilities.

mod str;
pub use self::str::Str;

mod headtail;
pub use headtail::{Head, HeadDot, HeadTail, HeadTailDot, HeadTailStr, Tail, TailDot, DOT};
