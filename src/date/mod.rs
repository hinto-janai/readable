//! Human-readable date formatting
//!
//! This module includes various [`Date`] types meant for calendar day formatting:
//! ```rust
//! # use readable::*;
//! let date  = Date::from_ymd(2020, 12, 25).unwrap();
//! let nichi = Nichi::new(2020, 12, 25).unwrap();
//!
//! assert_eq!(date,  "2020-12-25");
//! assert_eq!(nichi, "Fri, Dec 25, 2020");
//! assert_eq!(date.inner(), nichi.inner());
//! ```
//!
//! The inner "integer" type is a tuple of: `(u16, u8, u8)` representing the `(Year, Month, Day)`
//!
//! - The year must be `1000-9999`
//! - The month must be `1-12`
//! - The day must be `1-31`
//!
//! ## Weekday
//! These types all have `.weekday()` functions that calculate
//! the weekday given the `year`, `month`, and `day`.
//!
//! This uses [Tomohiko Sakamoto's](https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Sakamoto's_methods) algorithm.
//!
//! It is accurate for any [`Nichi`] or [`NichiFull`]
//! but only accurate for [`Date`] when it has the `month` and `day`.
//!
//! ```rust
//! # use readable::*;
//! // US Independence day was on a Thursday.
//! assert_eq!(
//! 	Nichi::new(1776, 7, 4).unwrap().weekday().as_str(),
//! 	"Thursday"
//! );
//!
//! // Nintendo Switch was released on a Friday.
//! assert_eq!(
//! 	Nichi::new(2017, 3, 3).unwrap().weekday().as_str(),
//! 	"Friday"
//! );
//!
//! // Christmas in 1999 was on a Saturday.
//! assert_eq!(
//! 	Nichi::new(1999, 12, 25).unwrap().weekday().as_str(),
//! 	"Saturday"
//! );
//!
//! // A good album was released on a Wednesday.
//! assert_eq!(
//! 	Nichi::new(2018, 4, 25).unwrap().weekday().as_str(),
//! 	"Wednesday"
//! );
//! ```
//! ## From other types
//! All types support conversion with each other using [`From`],
//! although [`Date`] itself is only lossless if the full `year-month-day` is available.
//!
//! ```rust
//! # use readable::*;
//! // Lossless.
//! let date  = Date::from_ymd(2020, 12, 25).unwrap();
//! let nichi = Nichi::from(date);
//! assert_eq!(date, (2020, 12, 25));
//! assert_eq!(nichi, (2020, 12, 25));
//!
//! // Missing date, unknown is returned.
//! let date  = Date::from_ym(2020, 12).unwrap();
//! let nichi = Nichi::from(date);
//! assert_eq!(date, (2020, 12, 0));
//! assert_eq!(nichi, "???");
//! assert!(nichi.is_unknown());
//! ```
//!
//! ## Copy
//! [`Copy`] is available.
//!
//! ```rust
//! # use readable::Date;
//! let a = Date::from_str("2014-04-22").unwrap();
//!
//! // Copy 'a', use 'b'.
//! let b = a;
//! assert_eq!(b, "2014-04-22");
//!
//! // We can still use 'a'
//! assert_eq!(a, "2014-04-22");
//! ```

mod date;
pub use date::*;

mod nichi;
pub use nichi::*;

mod nichi_full;
pub use nichi_full::*;

pub(super) mod free;
pub use free::*;

mod sysdate;
pub use sysdate::*;