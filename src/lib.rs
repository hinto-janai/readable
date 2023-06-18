//! Human **readable** data formatting.
//!
//! This crate turns various data into human-readable strings.
//!
//! Most of the internal strings are implemented as fixed length, stack allocated arrays that are [`Copy`](https://doc.rust-lang.org/stable/std/marker/trait.Copy.html)-able.
//!
//! # Feature flags
//! | Flag             | Purpose |
//! |------------------|---------|
//! | `serde`          | Enables [`serde`](https://docs.rs/serde) on all types
//! | `bincode`        | Enables [`bincode 2.0.0`](https://docs.rs/bincode/2.0.0-rc.3/bincode/index.html)'s `Encode/Decode` on all types
//! | `ignore_nan_inf` | Disables checking `f64`'s for `f64::NAN`, `f64::INFINITY`, and `f64::NEG_INFINITY`
//! | `inline_date`    | Inlines any `Date` that is in `YYYY-MM-HH` format and is between year `1900-2100`
//! | `inline_time`    | Inlines any `Time` that is under `1 hour, 1 minute` (`0..=3660`)
//! | `inline_runtime` | Inlines ALL of `Runtime` (`0:00..99:59:59`/`0..=359999`)
//! | `full`           | Enables everything above
//!
//! **Warning:** The `inline_*` features are disabled by default. While they increase speed,
//! they also _heavily_ increase build time and binary size.
//!
//! #### Unsigned integers:
//! ```
//! let a = readable::Unsigned::from(1000_u64);
//!
//! assert!(a == 1000_u64);
//! assert!(a == "1,000");
//! ```
//!
//! #### Signed integers:
//! ```
//! let a = readable::Int::from(-1000);
//!
//! assert!(a == -1000);
//! assert!(a == "-1,000");
//! ```
//!
//! #### Floats:
//! ```
//! let a = readable::Float::from(1000.123);
//!
//! assert!(a == 1000.123);
//! assert!(a == "1,000.123");
//! ```
//!
//! #### Percents:
//! ```
//! let a = readable::Percent::from(1000.123);
//!
//! assert!(a == 1000.123);
//! assert!(a == "1,000.12%");
//! ```
//!
//! #### Runtime:
//! ```
//! let a = readable::Runtime::from(11111_u16);
//!
//! assert!(a == 11111);
//! assert!(a == "3:05:11");
//! ```
//!
//! #### Time:
//! ```
//! let a = readable::Time::from(86399_u64);
//!
//! assert!(a == 86399_u64);
//! assert!(a == "23 hours, 59 minutes, 59 seconds");
//! ```
//!
//! #### Date:
//! ```rust
//! let a = readable::Date::from_str("2014-12-31").unwrap();
//!
//! assert!(a == (2014, 12, 31));
//! assert!(a == "2014-12-31");
//! ```
//!
//! # Comparison
//! All types implement `Display`, `PartialEq`, `PartialEq<&str>` and `PartialEq` for their inner number primitive.
//!
//! Example 1:
//! ```rust
//! let a = std::time::Duration::from_secs(86399);
//! let b = readable::Time::from(a);
//!
//! assert!(b == "23 hours, 59 minutes, 59 seconds");
//! ```
//! This is comparing `b`'s inner `String`.
//!
//! Example 2:
//! ```rust
//! let a = readable::Int::from(-1000);
//!
//! assert!(a == -1000);
//! ```
//! This is comparing `a`'s inner `i64`.
//!
//! Example 3:
//! ```rust
//! let a = readable::Unsigned::from(1000_u64);
//! let b = readable::Unsigned::from(1000_u64);
//!
//! assert!(a == b);
//! ```
//! This compares both the `u64` AND `String` inside `a` and `b`.
//!
//! # Math
//! Most types implement `+, -, /, *, %`, outputting a new `Self`.
//!
//! Example - `Add +`:
//! ```rust
//! let f1 = readable::Float::from(1.0);
//! let f2 = readable::Float::from(2.0);
//! let f3 = readable::Float::from(3.0);
//!
//! assert!(f1 + f2 == f3);
//! ```
//! Example - `Sub -`:
//! ```rust
//! let p50 = readable::Percent::from(50.0);
//! let p25 = readable::Percent::from(25.0);
//!
//! assert!(p50 - p25 == "25.00%");
//! ```
//! Example - `Div /`:
//! ```rust
//! let u100 = readable::Unsigned::from(100_u64);
//! let u10  = readable::Unsigned::from(10_u64);
//!
//! assert!(u100 / u10 == 10);
//! ```
//! Example - `Mul *`:
//! ```rust
//! let u10 = readable::Unsigned::from(10_u64);
//!
//! assert!(u10 * u10 == readable::Unsigned::from(100_u64));
//! ```
//! Example - `Rem %`:
//! ```rust
//! let u10 = readable::Unsigned::from(10_u64);
//!
//! assert!(u10 % u10 == 0);
//! ```

//------ Docs
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//------ Lints
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

pub(crate) mod buf;
pub(crate) mod macros;

mod constants;
pub use constants::*;

mod date;
pub use date::*;

mod unsigned;
pub use unsigned::*;

mod int;
pub use int::*;

mod float;
pub use float::*;

mod percent;
pub use percent::*;

mod time;
pub use time::*;

mod runtime;
pub use runtime::*;

mod headtail;
pub use headtail::*;

mod free;

#[doc(hidden)]
pub use itoa as __readable_itoa;
#[doc(hidden)]
pub use ryu as __readable_ryu;
