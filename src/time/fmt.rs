// devela::time::fmt
//
//!
//
// IMPROVE:
// - secs_f64: make a version with fixed format, and another custom reducible.
// - nanos_u64: to not show leading zeros for seconds.
// - nanos_u64: not just clamp the seconds but all to 999?

#[cfg(feature = "alloc")]
use crate::_liballoc::{format, string::String};
#[cfg(feature = "num_float")]
#[allow(unused_imports)]
use crate::num::ExtFloat;
#[allow(unused_imports)]
use crate::time::HourMilliSplit;
use crate::{
    lex::{format_buf, Ascii, StringU8},
    num::Compare,
    time::SecNanoSplit,
};

/// Timecode splitting and formatting.
///
/// # Examples
/// ```
/// # use devela::time::Timecode;
/// #[cfg(any(feature = "std", all(feature = "num_float", feature = "_f64")))]
/// assert_eq!(Timecode::secs_f64(3661.5), "01:01:01.500");
///
/// assert_eq!(Timecode::nanos_u64(1_002_003_004), "001s 002ms 003µs 00004ns");
/// ```
pub struct Timecode;

impl Timecode {
    /* time splitting */

    /// Decomposes a number of `seconds` in `{ h, m, s, ms }`.
    ///
    /// The maximum decomposition for [`u64::MAX`] is
    /// `{ h: 5_124_095_576_030_431, .. }` (more than 584_942_417 millenia).
    // -> 64 bits
    #[inline]
    #[must_use]
    #[cfg(any(feature = "std", all(feature = "num_float", feature = "_f64")))]
    #[cfg_attr(
        feature = "nightly_doc",
        doc(cfg(any(feature = "std", all(feature = "num_float", feature = "_f64"))))
    )]
    pub fn split_secs_f64(seconds: f64) -> HourMilliSplit<u32, u8, u8, u16> {
        let ms = (seconds.fract() * 1000.) as u16;
        let mut ts = seconds.trunc() as u64;
        let h = (ts / 3600) as u32;
        ts %= 3600;
        let m = (ts / 60) as u8;
        let s = (ts % 60) as u8;
        HourMilliSplit { h, m, s, ms }
    }

    /// Splits a number of `nanoseconds` in `{ s, ms, µs, ns }`.
    ///
    /// The maximum decomposition for [`u64::MAX`] is
    /// `{ s: 1_266_874_889, .. }` (more than 40 years).
    // -> 80 bits
    #[inline]
    #[must_use]
    pub fn split_nanos_u64(nanos: u64) -> SecNanoSplit<u32, u16, u16, u16> {
        let (us_tmp, ns) = (nanos / 1000, (nanos % 1000) as u16);
        let (ms_tmp, us) = (us_tmp / 1000, (us_tmp % 1000) as u16);
        let (s, ms) = ((ms_tmp / 1000) as u32, (ms_tmp % 1000) as u16);
        SecNanoSplit { s, ms, us, ns }
    }

    /// Splits a number of `nanoseconds` in `{ s, ms, µs, ns }`.
    ///
    /// The maximum decomposition for [`u32::MAX`] is
    /// `{ ns: 295, µs: 967, ms: 294, s: 4 }` (more than 4 seconds).
    // -> 56 bits
    #[inline]
    #[must_use]
    pub fn split_nanos_u32(nanos: u32) -> SecNanoSplit<u8, u16, u16, u16> {
        let (us_tmp, ns) = (nanos / 1000, (nanos % 1000) as u16);
        let (ms_tmp, us) = (us_tmp / 1000, (us_tmp % 1000) as u16);
        let (s, ms) = ((ms_tmp / 1000) as u8, (ms_tmp % 1000) as u16);
        SecNanoSplit { ns, us, ms, s }
    }

    /// Returns the time code as `HH:MM:SS:MIL` or `MM:SS:MIL`.
    ///
    /// The hours are clamped to 99 (more than 4 days).
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    //
    // -> 96 bits
    #[cfg(any(feature = "std", all(feature = "num_float", feature = "_f64")))]
    #[cfg_attr(
        feature = "nightly_doc",
        doc(cfg(any(feature = "std", all(feature = "num_float", feature = "_f64"))))
    )]
    pub fn secs_f64(seconds: f64) -> StringU8<12> {
        let HourMilliSplit { h, m, s, ms } = Self::split_secs_f64(seconds);
        let m = Ascii(m).digits_str(2);
        let s = Ascii(s).digits_str(2);
        let ms = Ascii(ms).digits_str(3);

        let mut buf = [0; 12];
        let mut buf_len = 12;

        if h > 0 {
            let h = Ascii(Compare(h).min(99)).digits_str(2);
            let _str = format_buf![&mut buf, "{h}:{m}:{s}.{ms}"];
        } else {
            buf_len = 9;
            let _str = format_buf![&mut buf, "{m}:{s}.{ms}"];
        }

        #[cfg(any(feature = "safe_time", not(feature = "unsafe_str")))]
        return StringU8::<12>::from_bytes_nleft(buf, buf_len).unwrap();

        #[cfg(all(not(feature = "safe_time"), feature = "unsafe_str"))]
        // SAFETY: the buffer contains only ASCII characters.
        unsafe {
            StringU8::<12>::from_bytes_nleft_unchecked(buf, buf_len)
        }
    }

    /// Returns the time code, up to seconds, as `001s 012ms 012µs 012345ns`.
    ///
    /// The seconds are clamped to 999 (more than 16 minutes).
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    // -> 192 bits in "64", 92 bits in "32"
    pub fn nanos_u64_alloc(ns: u64) -> String {
        let (us, ns_rem) = (ns / 1_000, ns % 1_000);
        let (ms, us_rem) = (us / 1_000, us % 1_000);
        let (s, ms_rem) = (ms / 1_000, ms % 1_000);
        let s = Compare(s).min(999);

        if s > 0 {
            format!["{s}s {ms_rem:03}ms {us_rem:03}µs {ns_rem:06}ns"]
        } else if ms > 0 {
            format!["{ms_rem}ms {us_rem:03}µs {ns_rem:06}ns"]
        } else if us > 0 {
            format!["{us_rem}µs {ns_rem:06}ns"]
        } else {
            format!["{ns_rem:06}ns"]
        }
    }

    /// Returns the time code, up to seconds, as `001s 012ms 012µs 012345ns`.
    ///
    /// The seconds are clamped to 999 (more than 16 minutes).
    // -> 208 bits
    pub fn nanos_u64(nanos: u64) -> StringU8<25> {
        let SecNanoSplit { s, ms, us, ns } = Self::split_nanos_u64(nanos);
        let s_str = Ascii(Compare(s).min(999)).digits_str(3);
        let ms_str = Ascii(ms).digits_str(3);
        let us_str = Ascii(us).digits_str(3);
        let ns_str = Ascii(ns).digits_str(6);

        let mut buf = [0; 25]; // max
        let mut buf_len = 25;

        if s > 0 {
            let _ = format_buf![&mut buf, "{s_str}s {ms_str}ms {us_str}µs {ns_str}ns"];
        } else if ms > 0 {
            buf_len = 20;
            let _ = format_buf![&mut buf, "{ms_str}ms {us_str}µs {ns_str}ns"];
        } else if us > 0 {
            buf_len = 14;
            let _ = format_buf![&mut buf, "{us_str}µs {ns_str}ns"];
        } else {
            buf_len = 8;
            let _ = format_buf![&mut buf, "{ns_str}ns"];
        }

        #[cfg(any(feature = "safe_time", not(feature = "unsafe_str")))]
        return StringU8::<25>::from_bytes_nleft(buf, buf_len).unwrap();

        #[cfg(all(not(feature = "safe_time"), feature = "unsafe_str"))]
        // SAFETY: the buffer contains only ASCII characters.
        unsafe {
            StringU8::<25>::from_bytes_nleft_unchecked(buf, buf_len)
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(any(feature = "std", all(feature = "num_float", feature = "_f64")))]
    fn timecode_split_secs_f64() {
        let result = Timecode::split_secs_f64(3661.500);
        assert_eq!(result, HourMilliSplit { h: 1, m: 1, s: 1, ms: 500 });
    }

    #[test]
    fn timecode_split_nanos_u64() {
        let result = Timecode::split_nanos_u64(1_002_003_004);
        assert_eq!(result, SecNanoSplit { s: 1, ms: 2, us: 3, ns: 4 });
    }

    #[test]
    fn timecode_split_nanos_u32() {
        let result = Timecode::split_nanos_u32(1_002_003);
        assert_eq!(result, SecNanoSplit { s: 0, ms: 1, us: 2, ns: 3 });
    }

    #[test]
    #[cfg(any(feature = "std", all(feature = "num_float", feature = "_f64")))]
    fn timecode_secs_f64() {
        let formatted = Timecode::secs_f64(3661.5);
        assert_eq!(formatted, "01:01:01.500");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn timecode_nanos_u64_alloc() {
        let formatted = Timecode::nanos_u64_alloc(1_002_003_004);
        assert_eq!(formatted, "1s 002ms 003µs 000004ns");
    }

    #[test]
    fn timecode_nanos_u64() {
        let formatted = Timecode::nanos_u64(1_002_003_004);
        assert_eq!(formatted, "001s 002ms 003µs 00004ns");
    }
}
