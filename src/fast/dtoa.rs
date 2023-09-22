// This `dtoa` implementation is taken from `https://github.com/dtolnay/dtoa`.

//---------------------------------------------------------------------------------------------------- Use
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::doc_markdown,
    clippy::expl_impl_clone_on_copy,
    clippy::if_not_else,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::range_plus_one,
    clippy::semicolon_if_nothing_returned, // https://github.com/rust-lang/rust-clippy/issues/7768
    clippy::shadow_unrelated,
    clippy::suspicious_else_formatting,
    clippy::transmute_float_to_int,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix
)]

use core::mem::{self, MaybeUninit};
use core::slice;
use core::str;
#[cfg(feature = "no-panic")]
use no_panic::no_panic;

//----------------------------------------------------------------------------------------------------
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// ---
//
// The C++ implementation preserved here in comments is licensed as follows:
//
// Tencent is pleased to support the open source community by making RapidJSON
// available.
//
// Copyright (C) 2015 THL A29 Limited, a Tencent company, and Milo Yip. All
// rights reserved.
//
// Licensed under the MIT License (the "License"); you may not use this file
// except in compliance with the License. You may obtain a copy of the License
// at
//
// http://opensource.org/licenses/MIT
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

use core::ptr;
#[cfg(feature = "no-panic")]
use no_panic::no_panic;

/*
inline unsigned CountDecimalDigit32(uint32_t n) {
    // Simple pure C++ implementation was faster than __builtin_clz version in this situation.
    if (n < 10) return 1;
    if (n < 100) return 2;
    if (n < 1000) return 3;
    if (n < 10000) return 4;
    if (n < 100000) return 5;
    if (n < 1000000) return 6;
    if (n < 10000000) return 7;
    if (n < 100000000) return 8;
    // Will not reach 10 digits in DigitGen()
    //if (n < 1000000000) return 9;
    //return 10;
    return 9;
}
*/

#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
fn count_decimal_digit32(n: u32) -> usize {
    if n < 10 {
        1
    } else if n < 100 {
        2
    } else if n < 1000 {
        3
    } else if n < 10000 {
        4
    } else if n < 100000 {
        5
    } else if n < 1000000 {
        6
    } else if n < 10000000 {
        7
    } else if n < 100000000 {
        8
    }
    // Will not reach 10 digits in digit_gen()
    else {
        9
    }
}

/*
inline char* WriteExponent(int K, char* buffer) {
    if (K < 0) {
        *buffer++ = '-';
        K = -K;
    }

    if (K >= 100) {
        *buffer++ = static_cast<char>('0' + static_cast<char>(K / 100));
        K %= 100;
        const char* d = GetDigitsLut() + K * 2;
        *buffer++ = d[0];
        *buffer++ = d[1];
    }
    else if (K >= 10) {
        const char* d = GetDigitsLut() + K * 2;
        *buffer++ = d[0];
        *buffer++ = d[1];
    }
    else
        *buffer++ = static_cast<char>('0' + static_cast<char>(K));

    return buffer;
}
*/

#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
unsafe fn write_exponent(mut k: isize, mut buffer: *mut u8) -> *mut u8 {
    if k < 0 {
        *buffer = b'-';
        buffer = buffer.offset(1);
        k = -k;
    }

    if k >= 100 {
        *buffer = b'0' + (k / 100) as u8;
        k %= 100;
        let d = DEC_DIGITS_LUT.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, buffer.offset(1), 2);
        buffer.offset(3)
    } else if k >= 10 {
        let d = DEC_DIGITS_LUT.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, buffer, 2);
        buffer.offset(2)
    } else {
        *buffer = b'0' + k as u8;
        buffer.offset(1)
    }
}

/*
inline char* Prettify(char* buffer, int length, int k, int maxDecimalPlaces) {
    const int kk = length + k;  // 10^(kk-1) <= v < 10^kk
*/

#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
unsafe fn prettify(buffer: *mut u8, length: isize, k: isize) -> *mut u8 {
    let kk = length + k; // 10^(kk-1) <= v < 10^kk

    /*
    if (0 <= k && kk <= 21) {
        // 1234e7 -> 12340000000
        for (int i = length; i < kk; i++)
            buffer[i] = '0';
        buffer[kk] = '.';
        buffer[kk + 1] = '0';
        return &buffer[kk + 2];
    }
    */
    if 0 <= k && kk <= 21 {
        // 1234e7 -> 12340000000
        for i in length..kk {
            *buffer.offset(i) = b'0';
        }
        *buffer.offset(kk) = b'.';
        *buffer.offset(kk + 1) = b'0';
        buffer.offset(kk + 2)
    }
    /*
    else if (0 < kk && kk <= 21) {
        // 1234e-2 -> 12.34
        std::memmove(&buffer[kk + 1], &buffer[kk], static_cast<size_t>(length - kk));
        buffer[kk] = '.';
        if (0 > k + maxDecimalPlaces) {
            // When maxDecimalPlaces = 2, 1.2345 -> 1.23, 1.102 -> 1.1
            // Remove extra trailing zeros (at least one) after truncation.
            for (int i = kk + maxDecimalPlaces; i > kk + 1; i--)
                if (buffer[i] != '0')
                    return &buffer[i + 1];
            return &buffer[kk + 2]; // Reserve one zero
        }
        else
            return &buffer[length + 1];
    }
    */
    else if 0 < kk && kk <= 21 {
        // 1234e-2 -> 12.34
        ptr::copy(
            buffer.offset(kk),
            buffer.offset(kk + 1),
            (length - kk) as usize,
        );
        *buffer.offset(kk) = b'.';
        if 0 > k + MAX_DECIMAL_PLACES {
            // When MAX_DECIMAL_PLACES = 2, 1.2345 -> 1.23, 1.102 -> 1.1
            // Remove extra trailing zeros (at least one) after truncation.
            for i in (kk + 2..kk + MAX_DECIMAL_PLACES + 1).rev() {
                if *buffer.offset(i) != b'0' {
                    return buffer.offset(i + 1);
                }
            }
            buffer.offset(kk + 2) // Reserve one zero
        } else {
            buffer.offset(length + 1)
        }
    }
    /*
    else if (-6 < kk && kk <= 0) {
        // 1234e-6 -> 0.001234
        const int offset = 2 - kk;
        std::memmove(&buffer[offset], &buffer[0], static_cast<size_t>(length));
        buffer[0] = '0';
        buffer[1] = '.';
        for (int i = 2; i < offset; i++)
            buffer[i] = '0';
        if (length - kk > maxDecimalPlaces) {
            // When maxDecimalPlaces = 2, 0.123 -> 0.12, 0.102 -> 0.1
            // Remove extra trailing zeros (at least one) after truncation.
            for (int i = maxDecimalPlaces + 1; i > 2; i--)
                if (buffer[i] != '0')
                    return &buffer[i + 1];
            return &buffer[3]; // Reserve one zero
        }
        else
            return &buffer[length + offset];
    }
    */
    else if -6 < kk && kk <= 0 {
        // 1234e-6 -> 0.001234
        let offset = 2 - kk;
        ptr::copy(buffer, buffer.offset(offset), length as usize);
        *buffer = b'0';
        *buffer.offset(1) = b'.';
        for i in 2..offset {
            *buffer.offset(i) = b'0';
        }
        if length - kk > MAX_DECIMAL_PLACES {
            // When MAX_DECIMAL_PLACES = 2, 0.123 -> 0.12, 0.102 -> 0.1
            // Remove extra trailing zeros (at least one) after truncation.
            for i in (3..MAX_DECIMAL_PLACES + 2).rev() {
                if *buffer.offset(i) != b'0' {
                    return buffer.offset(i + 1);
                }
            }
            buffer.offset(3) // Reserve one zero
        } else {
            buffer.offset(length + offset)
        }
    }
    /*
    else if (kk < -maxDecimalPlaces) {
        // Truncate to zero
        buffer[0] = '0';
        buffer[1] = '.';
        buffer[2] = '0';
        return &buffer[3];
    }
    */
    else if kk < -MAX_DECIMAL_PLACES {
        *buffer = b'0';
        *buffer.offset(1) = b'.';
        *buffer.offset(2) = b'0';
        buffer.offset(3)
    }
    /*
    else if (length == 1) {
        // 1e30
        buffer[1] = 'e';
        return WriteExponent(kk - 1, &buffer[2]);
    }
    */
    else if length == 1 {
        // 1e30
        *buffer.offset(1) = b'e';
        write_exponent(kk - 1, buffer.offset(2))
    }
    /*
    else {
        // 1234e30 -> 1.234e33
        std::memmove(&buffer[2], &buffer[1], static_cast<size_t>(length - 1));
        buffer[1] = '.';
        buffer[length + 1] = 'e';
        return WriteExponent(kk - 1, &buffer[0 + length + 2]);
    }
    */
    else {
        // 1234e30 -> 1.234e33
        ptr::copy(buffer.offset(1), buffer.offset(2), (length - 1) as usize);
        *buffer.offset(1) = b'.';
        *buffer.offset(length + 1) = b'e';
        write_exponent(kk - 1, buffer.offset(length + 2))
    }
}

macro_rules! dtoa {
    (
        floating_type: $fty:ty,
        significand_type: $sigty:ty,
        exponent_type: $expty:ty,
        $($diyfp_param:ident: $diyfp_value:tt,)*
    ) => {
        diyfp! {
            floating_type: $fty,
            significand_type: $sigty,
            exponent_type: $expty,
            $($diyfp_param: $diyfp_value,)*
        };

        /*
        inline void GrisuRound(char* buffer, int len, uint64_t delta, uint64_t rest, uint64_t ten_kappa, uint64_t wp_w) {
            while (rest < wp_w && delta - rest >= ten_kappa &&
                (rest + ten_kappa < wp_w ||  /// closer
                    wp_w - rest > rest + ten_kappa - wp_w)) {
                buffer[len - 1]--;
                rest += ten_kappa;
            }
        }
        */

        #[inline]
        #[cfg_attr(feature = "no-panic", no_panic)]
        unsafe fn grisu_round(buffer: *mut u8, len: isize, delta: $sigty, mut rest: $sigty, ten_kappa: $sigty, wp_w: $sigty) {
            while rest < wp_w && delta - rest >= ten_kappa &&
                (rest + ten_kappa < wp_w || // closer
                    wp_w - rest > rest + ten_kappa - wp_w) {
                *buffer.offset(len - 1) -= 1;
                rest += ten_kappa;
            }
        }

        /*
        inline void DigitGen(const DiyFp& W, const DiyFp& Mp, uint64_t delta, char* buffer, int* len, int* K) {
            static const uint32_t kPow10[] = { 1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000 };
            const DiyFp one(uint64_t(1) << -Mp.e, Mp.e);
            const DiyFp wp_w = Mp - W;
            uint32_t p1 = static_cast<uint32_t>(Mp.f >> -one.e);
            uint64_t p2 = Mp.f & (one.f - 1);
            unsigned kappa = CountDecimalDigit32(p1); // kappa in [0, 9]
            *len = 0;
        */

        // Returns length and k.
        #[inline]
        #[cfg_attr(feature = "no-panic", no_panic)]
        unsafe fn digit_gen(w: DiyFp, mp: DiyFp, mut delta: $sigty, buffer: *mut u8, mut k: isize) -> (isize, isize) {
            static POW10: [$sigty; 10] = [ 1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000 ];
            let one = DiyFp::new(1 << -mp.e, mp.e);
            let wp_w = mp - w;
            let mut p1 = (mp.f >> -one.e) as u32;
            let mut p2 = mp.f & (one.f - 1);
            let mut kappa = count_decimal_digit32(p1); // kappa in [0, 9]
            let mut len = 0;

            /*
            while (kappa > 0) {
                uint32_t d = 0;
                switch (kappa) {
                    case  9: d = p1 /  100000000; p1 %=  100000000; break;
                    case  8: d = p1 /   10000000; p1 %=   10000000; break;
                    case  7: d = p1 /    1000000; p1 %=    1000000; break;
                    case  6: d = p1 /     100000; p1 %=     100000; break;
                    case  5: d = p1 /      10000; p1 %=      10000; break;
                    case  4: d = p1 /       1000; p1 %=       1000; break;
                    case  3: d = p1 /        100; p1 %=        100; break;
                    case  2: d = p1 /         10; p1 %=         10; break;
                    case  1: d = p1;              p1 =           0; break;
                    default:;
                }
                if (d || *len)
                    buffer[(*len)++] = static_cast<char>('0' + static_cast<char>(d));
                kappa--;
                uint64_t tmp = (static_cast<uint64_t>(p1) << -one.e) + p2;
                if (tmp <= delta) {
                    *K += kappa;
                    GrisuRound(buffer, *len, delta, tmp, static_cast<uint64_t>(kPow10[kappa]) << -one.e, wp_w.f);
                    return;
                }
            }
            */
            while kappa > 0 {
                let mut d = 0u32;
                match kappa {
                    9 => { d = p1 /  100000000; p1 %=  100000000; }
                    8 => { d = p1 /   10000000; p1 %=   10000000; }
                    7 => { d = p1 /    1000000; p1 %=    1000000; }
                    6 => { d = p1 /     100000; p1 %=     100000; }
                    5 => { d = p1 /      10000; p1 %=      10000; }
                    4 => { d = p1 /       1000; p1 %=       1000; }
                    3 => { d = p1 /        100; p1 %=        100; }
                    2 => { d = p1 /         10; p1 %=         10; }
                    1 => { d = p1;              p1 =           0; }
                    _ => {}
                }
                if d != 0 || len != 0 {
                    *buffer.offset(len) = b'0' + d as u8;
                    len += 1;
                }
                kappa -= 1;
                let tmp = ((p1 as $sigty) << -one.e) + p2;
                if tmp <= delta {
                    k += kappa as isize;
                    grisu_round(buffer, len, delta, tmp, *POW10.get_unchecked(kappa) << -one.e, wp_w.f);
                    return (len, k);
                }
            }

            // kappa = 0
            /*
            for (;;) {
                p2 *= 10;
                delta *= 10;
                char d = static_cast<char>(p2 >> -one.e);
                if (d || *len)
                    buffer[(*len)++] = static_cast<char>('0' + d);
                p2 &= one.f - 1;
                kappa--;
                if (p2 < delta) {
                    *K += kappa;
                    int index = -static_cast<int>(kappa);
                    GrisuRound(buffer, *len, delta, p2, one.f, wp_w.f * (index < 9 ? kPow10[-static_cast<int>(kappa)] : 0));
                    return;
                }
            }
            */
            loop {
                p2 *= 10;
                delta *= 10;
                let d = (p2 >> -one.e) as u8;
                if d != 0 || len != 0 {
                    *buffer.offset(len) = b'0' + d;
                    len += 1;
                }
                p2 &= one.f - 1;
                kappa = kappa.wrapping_sub(1);
                if p2 < delta {
                    k += kappa as isize;
                    let index = -(kappa as isize);
                    grisu_round(
                        buffer,
                        len,
                        delta,
                        p2,
                        one.f,
                        wp_w.f * if index < 9 {
                            *POW10.get_unchecked(-(kappa as isize) as usize)
                        } else {
                            0
                        },
                    );
                    return (len, k);
                }
            }
        }

        /*
        inline void Grisu2(double value, char* buffer, int* length, int* K) {
            const DiyFp v(value);
            DiyFp w_m, w_p;
            v.NormalizedBoundaries(&w_m, &w_p);

            const DiyFp c_mk = GetCachedPower(w_p.e, K);
            const DiyFp W = v.Normalize() * c_mk;
            DiyFp Wp = w_p * c_mk;
            DiyFp Wm = w_m * c_mk;
            Wm.f++;
            Wp.f--;
            DigitGen(W, Wp, Wp.f - Wm.f, buffer, length, K);
        }
        */

        // Returns length and k.
        #[inline]
        #[cfg_attr(feature = "no-panic", no_panic)]
        unsafe fn grisu2(value: $fty, buffer: *mut u8) -> (isize, isize) {
            let v = DiyFp::from(value);
            let (w_m, w_p) = v.normalized_boundaries();

            let (c_mk, k) = get_cached_power(w_p.e);
            let w = v.normalize() * c_mk;
            let mut wp = w_p * c_mk;
            let mut wm = w_m * c_mk;
            wm.f += 1;
            wp.f -= 1;
            digit_gen(w, wp, wp.f - wm.f, buffer, k)
        }

        /*
        inline char* dtoa(double value, char* buffer, int maxDecimalPlaces = 324) {
            RAPIDJSON_ASSERT(maxDecimalPlaces >= 1);
            Double d(value);
            if (d.IsZero()) {
                if (d.Sign())
                    *buffer++ = '-';     // -0.0, Issue #289
                buffer[0] = '0';
                buffer[1] = '.';
                buffer[2] = '0';
                return &buffer[3];
            }
            else {
                if (value < 0) {
                    *buffer++ = '-';
                    value = -value;
                }
                int length, K;
                Grisu2(value, buffer, &length, &K);
                return Prettify(buffer, length, K, maxDecimalPlaces);
            }
        }
        */

        #[inline]
        #[cfg_attr(feature = "no-panic", no_panic)]
        unsafe fn dtoa(buf: &mut [MaybeUninit<u8>; 25], mut value: $fty) -> &str {
            if value == 0.0 {
                if value.is_sign_negative() {
                    "-0.0"
                } else {
                    "0.0"
                }
            } else {
                let start = buf.as_mut_ptr() as *mut u8;
                let mut buf_ptr = start;
                if value < 0.0 {
                    *buf_ptr = b'-';
                    buf_ptr = buf_ptr.offset(1);
                    value = -value;
                }
                let (length, k) = grisu2(value, buf_ptr);
                let end = prettify(buf_ptr, length, k);
                let len = end as usize - start as usize;
                str::from_utf8_unchecked(slice::from_raw_parts(start, len))
            }
        }
    };
}

//---------------------------------------------------------------------------------------------------- Const
const NAN:          &str = "NaN";
const INFINITY:     &str = "inf";
const NEG_INFINITY: &str = "-inf";

//---------------------------------------------------------------------------------------------------- Dtoa
/// Fast float to string conversion
///
/// This struct represents a stack-based string converted from an [`Float`]
/// ([`f32`] & [`f64`]).
///
/// It internally uses [`dtoa`](https://docs.rs/dtoa) by `dtolnay`,
/// however [`Dtoa`] stores the string computation and can be turned into a [`&str`]
/// again and again after construction.
///
/// This does not do any `readable`-style formatting (adding commas), it simply
/// converts an integer into a string (but is much faster than [`format!()`]).
///
/// ## Example
/// ```rust
/// # use readable::Dtoa;
/// let dtoa = Dtoa::new(1000.0);
/// assert_eq!(dtoa, "1000.0");
///
/// let copy = dtoa;
/// assert_eq!(dtoa.as_str(), copy.as_str());
/// ```
///
/// ## Size
/// [`Dtoa`] is `26` bytes.
///
/// ```rust
/// assert_eq!(std::mem::size_of::<readable::Dtoa>(), 26);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[derive(Copy, Clone, Debug)]
pub struct Dtoa {
	len: u8,
    bytes: [MaybeUninit<u8>; 25],
}

impl Default for Dtoa {
    #[inline]
    fn default() -> Dtoa {
        Dtoa::new_finite(0.0)
    }
}

impl Dtoa {
	/// Create a new [`Dtoa`].
	///
	/// Takes any [`Float`] ([`f32`], [`f64`]).
	///
	/// This function will properly format non-finite floats.
	///
	/// See [`Dtoa::new_finite()`] if you know your float is finite
	/// (not [`f32::NAN`], [`f32::INFINITY`], [`f32::NEG_INFINITY`]).
	///
	/// ```rust
	/// # use readable::Dtoa;
	/// let dtoa = Dtoa::new(1.0);
	/// assert_eq!(dtoa, "1.0");
	///
	/// let dtoa = Dtoa::new(f64::NAN);
	/// assert_eq!(dtoa, "NaN");
	/// let dtoa = Dtoa::new(f64::INFINITY);
	/// assert_eq!(dtoa, "inf");
	/// let dtoa = Dtoa::new(f64::NEG_INFINITY);
	/// assert_eq!(dtoa, "-inf");
	/// ```
	#[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new<F: Float>(float: F) -> Self {
        if float.is_nonfinite() {
			let mut bytes = [MaybeUninit::<u8>::uninit(); 25];
            let string = float.format_nonfinite();
			for (i, byte) in string.as_bytes().into_iter().enumerate() {
				bytes[i].write(*byte);
			}
			Self {
				len: string.len() as u8,
				bytes,
			}
        } else {
            Self::new_finite(float)
        }
    }

	/// Create a new [`Dtoa`].
	///
	/// Takes any [`Float`] ([`f32`], [`f64`]).
	///
	/// This function **will not** properly format non-finite floats.
	///
	/// See [`Dtoa::new()`] for non-finite float formatting
	/// (not [`f32::NAN`], [`f32::INFINITY`], [`f32::NEG_INFINITY`]).
	///
	/// ```rust
	/// # use readable::Dtoa;
	/// let dtoa = Dtoa::new(18.425);
	/// assert_eq!(dtoa, "18.425");
	/// let dtoa = Dtoa::new(19.0918);
	/// assert_eq!(dtoa, "19.0918");
	///
	/// // This is safe, but the output strings will be incorrect.
	/// let dtoa = Dtoa::new_finite(f64::NAN);
	/// assert_eq!(dtoa, "2.696539702293474e308");
	/// let dtoa = Dtoa::new_finite(f64::INFINITY);
	/// assert_eq!(dtoa, "1.797693134862316e308");
	/// let dtoa = Dtoa::new_finite(f64::NEG_INFINITY);
	/// assert_eq!(dtoa, "-1.797693134862316e308");
	/// ```
	#[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new_finite<F: Float>(float: F) -> Self {
		let mut bytes = [MaybeUninit::<u8>::uninit(); 25];
		let len = float.write(&mut bytes).len() as u8;
		Self {
			len,
			bytes,
		}
    }

	#[inline]
	/// Turns [`Dtoa`] into a `&str`.
	///
	/// ```rust
	/// # use readable::Dtoa;
	/// let dtoa:   Dtoa = Dtoa::new(123.456);
	/// let string: &str = dtoa.as_str();
	/// assert_eq!(string, "123.456");
	/// ```
	pub const fn as_str(&self) -> &str {
		// Safety: Constructors must set state correctly.
		unsafe {
			let slice = slice::from_raw_parts(
				self.bytes.as_ptr() as *const u8,
				self.len as usize
			);
			std::str::from_utf8_unchecked(slice)
		}
	}
}

//---------------------------------------------------------------------------------------------------- Dtoa Traits
impl std::ops::Deref for Dtoa {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl AsRef<str> for Dtoa {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl std::borrow::Borrow<str> for Dtoa {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

impl PartialEq<str> for Dtoa {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}

impl PartialEq<&str> for Dtoa {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}

impl PartialEq<String> for Dtoa {
	fn eq(&self, other: &String) -> bool {
		self.as_str() == other
	}
}

impl<T: Float> std::convert::From<T> for Dtoa {
	fn from(float: T) -> Self {
		Self::new(float)
	}
}

impl std::fmt::Display for Dtoa {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

//---------------------------------------------------------------------------------------------------- Float
/// An integer that can be written into a [`Dtoa`].
pub trait Float: private::Sealed {}

impl Float for f32 {}
impl Float for f64 {}

// Seal to prevent downstream implementations of Float trait.
mod private {
    pub trait Sealed: Copy {
        fn is_nonfinite(self) -> bool;
        fn format_nonfinite(self) -> &'static str;
        fn write(self, buf: &mut [core::mem::MaybeUninit<u8>; 25]) -> &str;
    }
}

impl private::Sealed for f32 {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u32 = 0x7f800000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u32 = 0x007fffff;
        const SIGN_MASK: u32 = 0x80000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }

    #[inline]
    fn write(self, buf: &mut [MaybeUninit<u8>; 25]) -> &str {
        dtoa! {
            floating_type: f32,
            significand_type: u32,
            exponent_type: i32,

            diy_significand_size: 32,
            significand_size: 23,
            exponent_bias: 0x7F,
            mask_type: u32,
            exponent_mask: 0x7F800000,
            significand_mask: 0x007FFFFF,
            hidden_bit: 0x00800000,
            cached_powers_f: CACHED_POWERS_F_32,
            cached_powers_e: CACHED_POWERS_E_32,
            min_power: (-36),
        };
        unsafe { dtoa(buf, self) }
    }
}

impl private::Sealed for f64 {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u64 = 0x7ff0000000000000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u64 = 0x000fffffffffffff;
        const SIGN_MASK: u64 = 0x8000000000000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }

    #[inline]
    fn write(self, buf: &mut [MaybeUninit<u8>; 25]) -> &str {
        dtoa! {
            floating_type: f64,
            significand_type: u64,
            exponent_type: isize,

            diy_significand_size: 64,
            significand_size: 52,
            exponent_bias: 0x3FF,
            mask_type: u64,
            exponent_mask: 0x7FF0000000000000,
            significand_mask: 0x000FFFFFFFFFFFFF,
            hidden_bit: 0x0010000000000000,
            cached_powers_f: CACHED_POWERS_F_64,
            cached_powers_e: CACHED_POWERS_E_64,
            min_power: (-348),
        };
        unsafe { dtoa(buf, self) }
    }
}

////////////////////////////////////////////////////////////////////////////////

const MAX_DECIMAL_PLACES: isize = 324;

static DEC_DIGITS_LUT: [u8; 200] = *b"\
    0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";

// 10^-36, 10^-28, ..., 10^52
#[rustfmt::skip]
static CACHED_POWERS_F_32: [u32; 12] = [
    0xaa242499, 0xfd87b5f3, 0xbce50865, 0x8cbccc09,
    0xd1b71759, 0x9c400000, 0xe8d4a510, 0xad78ebc6,
    0x813f3979, 0xc097ce7c, 0x8f7e32ce, 0xd5d238a5,
];

#[rustfmt::skip]
static CACHED_POWERS_E_32: [i16; 12] = [
    -151, -125, -98, -71, -45, -18, 8, 35, 62, 88, 115, 141,
];

// 10^-348, 10^-340, ..., 10^340
#[rustfmt::skip]
static CACHED_POWERS_F_64: [u64; 87] = [
    0xfa8fd5a0081c0288, 0xbaaee17fa23ebf76,
    0x8b16fb203055ac76, 0xcf42894a5dce35ea,
    0x9a6bb0aa55653b2d, 0xe61acf033d1a45df,
    0xab70fe17c79ac6ca, 0xff77b1fcbebcdc4f,
    0xbe5691ef416bd60c, 0x8dd01fad907ffc3c,
    0xd3515c2831559a83, 0x9d71ac8fada6c9b5,
    0xea9c227723ee8bcb, 0xaecc49914078536d,
    0x823c12795db6ce57, 0xc21094364dfb5637,
    0x9096ea6f3848984f, 0xd77485cb25823ac7,
    0xa086cfcd97bf97f4, 0xef340a98172aace5,
    0xb23867fb2a35b28e, 0x84c8d4dfd2c63f3b,
    0xc5dd44271ad3cdba, 0x936b9fcebb25c996,
    0xdbac6c247d62a584, 0xa3ab66580d5fdaf6,
    0xf3e2f893dec3f126, 0xb5b5ada8aaff80b8,
    0x87625f056c7c4a8b, 0xc9bcff6034c13053,
    0x964e858c91ba2655, 0xdff9772470297ebd,
    0xa6dfbd9fb8e5b88f, 0xf8a95fcf88747d94,
    0xb94470938fa89bcf, 0x8a08f0f8bf0f156b,
    0xcdb02555653131b6, 0x993fe2c6d07b7fac,
    0xe45c10c42a2b3b06, 0xaa242499697392d3,
    0xfd87b5f28300ca0e, 0xbce5086492111aeb,
    0x8cbccc096f5088cc, 0xd1b71758e219652c,
    0x9c40000000000000, 0xe8d4a51000000000,
    0xad78ebc5ac620000, 0x813f3978f8940984,
    0xc097ce7bc90715b3, 0x8f7e32ce7bea5c70,
    0xd5d238a4abe98068, 0x9f4f2726179a2245,
    0xed63a231d4c4fb27, 0xb0de65388cc8ada8,
    0x83c7088e1aab65db, 0xc45d1df942711d9a,
    0x924d692ca61be758, 0xda01ee641a708dea,
    0xa26da3999aef774a, 0xf209787bb47d6b85,
    0xb454e4a179dd1877, 0x865b86925b9bc5c2,
    0xc83553c5c8965d3d, 0x952ab45cfa97a0b3,
    0xde469fbd99a05fe3, 0xa59bc234db398c25,
    0xf6c69a72a3989f5c, 0xb7dcbf5354e9bece,
    0x88fcf317f22241e2, 0xcc20ce9bd35c78a5,
    0x98165af37b2153df, 0xe2a0b5dc971f303a,
    0xa8d9d1535ce3b396, 0xfb9b7cd9a4a7443c,
    0xbb764c4ca7a44410, 0x8bab8eefb6409c1a,
    0xd01fef10a657842c, 0x9b10a4e5e9913129,
    0xe7109bfba19c0c9d, 0xac2820d9623bf429,
    0x80444b5e7aa7cf85, 0xbf21e44003acdd2d,
    0x8e679c2f5e44ff8f, 0xd433179d9c8cb841,
    0x9e19db92b4e31ba9, 0xeb96bf6ebadf77d9,
    0xaf87023b9bf0ee6b,
];

#[rustfmt::skip]
static CACHED_POWERS_E_64: [i16; 87] = [
    -1220, -1193, -1166, -1140, -1113, -1087, -1060, -1034, -1007,  -980,
    -954,   -927,  -901,  -874,  -847,  -821,  -794,  -768,  -741,  -715,
    -688,   -661,  -635,  -608,  -582,  -555,  -529,  -502,  -475,  -449,
    -422,   -396,  -369,  -343,  -316,  -289,  -263,  -236,  -210,  -183,
    -157,   -130,  -103,   -77,   -50,   -24,     3,    30,    56,    83,
     109,    136,   162,   189,   216,   242,   269,   295,   322,   348,
     375,    402,   428,   455,   481,   508,   534,   561,   588,   614,
     641,    667,   694,   720,   747,   774,   800,   827,   853,   880,
     907,    933,   960,   986,  1013,  1039,  1066,
];