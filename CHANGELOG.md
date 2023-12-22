All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Types of changes:
- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities


---


# readable Unreleased
## Changed
- `Str::invalid()` is now `const`
- `Str::as_str()` now panics in debug mode when `Str::invalid()` returns `true`
- `Str::from_bytes_exact()` takes `impl AsRef<[u8]>` instead of `&[u8]`

## Removed
- `Str::capacity()`

---


# readable v0.15.0 - 2023-12-21
## Changed
- `#[must_use]` on applicable functions
- `Str::as_bytes_mut()` is now `unsafe`
- `Str::from_bytes_exact()` is now `unsafe`
- `Str::from_str_exact()` takes `impl AsRef<str>` instead of `&str`

## Removed
- Redundant function constructors for `const` types (e.g `Uptime::day()` -> `Uptime::DAY`)


---


# readable v0.14.0 - 2023-12-03
## Added
- `readable::date::*`:
	* impl `TryFrom<(u16, u8, u8)>`
	* impl `TryFrom<(u16, u8)>`
	* impl `TryFrom<u16>`
- `readable::str::Str`:
	* impl `std::fmt::Write`
	* impl `Index`
	* impl `Extend<char>`, `Extend<&str>`
	* impl `Deref` and `AsRef`
	* impl `Add` and `AddAssign`
	* `push_{str,char}_unchecked()` -> `push_{str,char}_panic()`
	* add `push_char_*()` variants
	* add `push_str_saturating()`
	* add `remove()`
	* add `pop()`
	* add `truncate()`
	* add `make_ascii_uppercase()`, `make_ascii_lowercase()`
	* add `as_bytes_mut()`, `as_mut_ptr()`, `as_str_mut()`


---


# readable v0.13.0 - 2023-10-23
## Added
* `Time` type for clock time - `11:59:59 PM`
* `Military` type for military-style clock time - `23:59:59`
* `readable::time` free functions (`unix()`, `unix_clock()`, `secs_to_hms()`, etc)
* `readable::date` free functions (`from_unix()`, `as_unix()`, etc)
* `SysUptime`, `SysTime`, `SysDate` traits for getting live-system info

## Changed
* Uptime modules renamed for clarity `readable::time` -> `readable::up`
* Time-like types from `readable::date` into `time` module


---


# readable v0.12.0 - 2023-10-19
## Added
* `Nichi` type for calendar date formatting (`Fri, Dec 25, 2020`)
* `NichiFull` type for calendar date formatting (`Friday, December 25th, 2020`)
* `TimeUnit` type for remainder time measuring (`61` == `1 minute, 1 second`)
* `From` impl for `std::time::Duration` from all `readable::time` and `readable::run` types
* `Default` impl for `Str`

## Removed
* Feature-flags (except for `serde` & `bincode`)


---


# readable v0.11.0 - 2023-10-15
## Added
* `Htop` type in `readable::time` for [`htop`](https://github.com/htop-dev/htop)-style uptime formatting
* `Uptime` trait for getting system uptime and formatting directly into a `Time`, `TimeFull` or `Htop`
* `TryFrom` implementations from smart pointer strings (`Arc<str>`, `Box<str>`, etc) for `Str`

## Changed
* `Str::from_str()` -> `Str::from_str_exact()`
* `Str::from_bytes()` -> `Str::from_bytes_exact()`
* `Time::UNKNOWN` changed from `???` to `(unknown)`
* `TimeFull::UNKNOWN` changed from `???` to `(unknown)`


---


# readable v0.10.0 - 2023-10-04
First "public" release.
