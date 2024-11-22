// devela::sys::time::fmt
//
//!
//
// IMPROVE:
// - secs_f64: make a version with fixed format, and another custom reducible.
// - nanos_u64: to not show leading zeros for seconds.
// - nanos_u64: not just clamp the seconds but all to 999?

#[allow(unused_imports)]
#[cfg(feature = "_float_f64")]
use crate::ExtFloat;
#[cfg(feature = "alloc")]
use crate::{format, String};
#[cfg(feature = "_string_u8")]
use crate::{format_buf, Ascii, StringU8};
use crate::{NoTime, TimeSplit, TimeSplitHourNano};

/// Timecode splitting and formatting.
///
/// # Examples
/// ```
/// # use devela::sys::time::Timecode;
/// #[cfg(feature = "_string_u8")]
/// #[cfg(any(feature = "std", feature = "_float_f64"))]
/// assert_eq!(Timecode::secs_f64(3661.5), "01:01:01.500");
///
/// #[cfg(feature = "_string_u8")]
/// assert_eq!(Timecode::nanos_u64(1_002_003_004), "001s 002ms 003µs 004ns");
/// ```
pub struct Timecode;

impl Timecode {
    /* time splitting */

    /// Decomposes a number of `seconds` in `{ h, m, s, ms }`.
    ///
    /// The maximum decomposition for [`u64::MAX`] is
    /// `{ h: 5_124_095_576_030_431, .. }` (more than 584_942_417 millenia).
    // -> 64 bits
    #[must_use]
    #[cfg(any(feature = "std", feature = "_float_f64"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = "_float_f64"))))]
    pub fn split_secs_f64(seconds: f64) -> TimeSplitHourNano<u32, u8, u8, u16, NoTime, NoTime> {
        let ms = (seconds.fract() * 1000.) as u16;
        let mut ts = seconds.trunc() as u64;
        let h = (ts / 3600) as u32;
        ts %= 3600;
        let m = (ts / 60) as u8;
        let s = (ts % 60) as u8;
        TimeSplit::new_hour_nano(h, m, s, ms, (), ())
    }

    /// Splits a number of `nanoseconds` in `{ s, ms, µs, ns }`.
    ///
    /// The maximum decomposition for [`u64::MAX`] is
    /// `{ s: 1_266_874_889, .. }` (more than 40 years).
    // -> 80 bits
    #[must_use]
    pub const fn split_nanos_u64(
        nanos: u64,
    ) -> TimeSplitHourNano<NoTime, NoTime, u32, u16, u16, u16> {
        let (us_tmp, ns) = (nanos / 1000, (nanos % 1000) as u16);
        let (ms_tmp, us) = (us_tmp / 1000, (us_tmp % 1000) as u16);
        let (s, ms) = ((ms_tmp / 1000) as u32, (ms_tmp % 1000) as u16);
        TimeSplit::new_hour_nano((), (), s, ms, us, ns)
    }

    /// Splits a number of `nanoseconds` in `{ s, ms, µs, ns }`.
    ///
    /// The maximum decomposition for [`u32::MAX`] is
    /// `{ ns: 295, µs: 967, ms: 294, s: 4 }` (more than 4 seconds).
    // -> 56 bits
    #[must_use]
    pub const fn split_nanos_u32(
        nanos: u32,
    ) -> TimeSplitHourNano<NoTime, NoTime, u8, u16, u16, u16> {
        let (us_tmp, ns) = (nanos / 1000, (nanos % 1000) as u16);
        let (ms_tmp, us) = (us_tmp / 1000, (us_tmp % 1000) as u16);
        let (s, ms) = ((ms_tmp / 1000) as u8, (ms_tmp % 1000) as u16);
        TimeSplit::new_hour_nano((), (), s, ms, us, ns)
    }

    /// Returns the time code as `HH:MM:SS:MIL` or `MM:SS:MIL`.
    ///
    /// The hours are clamped to 99 (more than 4 days).
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    //
    // -> 96 bits
    #[cfg(any(feature = "std", feature = "_float_f64"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = "_float_f64"))))]
    #[cfg(feature = "_string_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_string_u8")))]
    pub fn secs_f64(seconds: f64) -> StringU8<12> {
        let TimeSplitHourNano { h, m, s, ms, .. } = Self::split_secs_f64(seconds);
        let m = Ascii(m as u32).digits_str(2);
        let s = Ascii(s as u32).digits_str(2);
        let ms = Ascii(ms as u32).digits_str(3);

        let mut buf = [0; 12];
        let mut buf_len = 12;

        if h > 0 {
            let h = Ascii(h.min(99)).digits_str(2);
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

    /// Returns the time code, up to seconds, as `001s 012ms 012µs 012ns`.
    ///
    /// The seconds are clamped to 999 (more than 16 minutes).
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    // -> 192 bits in "64", 92 bits in "32"
    pub fn nanos_u64_alloc(ns: u64) -> String {
        let (us, ns_rem) = (ns / 1_000, ns % 1_000);
        let (ms, us_rem) = (us / 1_000, us % 1_000);
        let (s, ms_rem) = (ms / 1_000, ms % 1_000);
        let s = s.min(999);

        if s > 0 {
            format!["{s}s {ms_rem:03}ms {us_rem:03}µs {ns_rem:03}ns"]
        } else if ms > 0 {
            format!["{ms_rem}ms {us_rem:03}µs {ns_rem:03}ns"]
        } else if us > 0 {
            format!["{us_rem}µs {ns_rem:03}ns"]
        } else {
            format!["{ns_rem:03}ns"]
        }
    }

    /// Returns the time code, up to seconds, as `001s 012ms 012µs 012345ns`.
    ///
    /// The seconds are clamped to 999 (more than 16 minutes).
    // -> 208 bits
    #[cfg(feature = "_string_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_string_u8")))]
    pub fn nanos_u64(nanos: u64) -> StringU8<23> {
        let TimeSplitHourNano { s, ms, us, ns, .. } = Self::split_nanos_u64(nanos);
        let s_str = Ascii(s.min(999)).digits_str(3);
        let ms_str = Ascii(ms as u32).digits_str(3);
        let us_str = Ascii(us as u32).digits_str(3);
        let ns_str = Ascii(ns as u32).digits_str(3);

        let mut buf = [0; 23];
        let mut buf_len = 23; // = 18 + 3digits + 1name(s) + 1space

        if s > 0 {
            let _ = format_buf![&mut buf, "{s_str}s {ms_str}ms {us_str}µs {ns_str}ns"];
        } else if ms > 0 {
            buf_len = 18; // = 18 + 3digits + 2name(ms) + 1space
            let _ = format_buf![&mut buf, "{ms_str}ms {us_str}µs {ns_str}ns"];
        } else if us > 0 {
            buf_len = 12; // = 5 + 3digits + 3name(µs) + 1space
            let _ = format_buf![&mut buf, "{us_str}µs {ns_str}ns"];
        } else {
            buf_len = 5; // = 0 + 3digits + 2name(ns)
            let _ = format_buf![&mut buf, "{ns_str}ns"];
        }

        #[cfg(any(feature = "safe_time", not(feature = "unsafe_str")))]
        return StringU8::<23>::from_bytes_nleft(buf, buf_len).unwrap();

        #[cfg(all(not(feature = "safe_time"), feature = "unsafe_str"))]
        // SAFETY: the buffer contains only ASCII characters.
        unsafe {
            StringU8::<23>::from_bytes_nleft_unchecked(buf, buf_len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(any(feature = "std", feature = "_float_f64"))]
    fn timecode_split_secs_f64() {
        let result = Timecode::split_secs_f64(3661.500);
        assert_eq!(8, size_of_val(&result)); //
        assert_eq!(result, TimeSplit::new_hour_nano(1, 1, 1, 500, (), ()));
    }

    #[test]
    fn timecode_split_nanos_u64() {
        let result = Timecode::split_nanos_u64(1_002_003_004);
        assert_eq!(12, size_of_val(&result));
        assert_eq!(result, TimeSplit::new_hour_nano((), (), 1, 2, 3, 4));
    }

    #[test]
    fn timecode_split_nanos_u32() {
        let result = Timecode::split_nanos_u32(1_002_003);
        assert_eq!(8, size_of_val(&result));
        assert_eq!(result, TimeSplit::new_hour_nano((), (), 0, 1, 2, 3));
    }

    #[test]
    #[cfg(feature = "_string_u8")]
    #[cfg(any(feature = "std", feature = "_float_f64"))]
    fn timecode_secs_f64() {
        let formatted = Timecode::secs_f64(3661.5);
        assert_eq!(formatted, "01:01:01.500");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn timecode_nanos_u64_alloc() {
        let formatted = Timecode::nanos_u64_alloc(1_002_003_004);
        assert_eq!(formatted, "1s 002ms 003µs 004ns");
    }

    #[test]
    #[cfg(feature = "_string_u8")]
    fn timecode_nanos_u64() {
        let formatted = Timecode::nanos_u64(1_002_003_004);
        assert_eq!(formatted, "001s 002ms 003µs 004ns");
    }
}
