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
## Added
* `Htop` type in `readable::time` for [`htop`](https://github.com/htop-dev/htop)-style uptime formatting
* `Uptime` trait for getting system uptime and formatting directly into a `Time`, `TimeFull` or `Htop`
* `TryFrom` implementations for smart pointer strings (`Arc<str>`, `Box<str>`, etc)

## Changed
* `Str::from_str()` -> `Str::from_str_exact()`
* `Str::from_bytes()` -> `Str::from_bytes_exact()`
* `Time::UNKNOWN` changed from `???` to `(unknown)`
* `TimeFull::UNKNOWN` changed from `???` to `(unknown)`

---

# readable v0.10.0 - 2023-10-04
First "public" release.

---
