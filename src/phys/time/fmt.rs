// devela::phys::time::fmt
//
//! Defines [`Timecode`].
//
// IMPROVE:
// - secs_f64: make a version with fixed format, and another custom reducible.
// - nanos_u64: to not show leading zeros for seconds.
// - nanos_u64: not just clamp the seconds but all to 999?

use crate::{Cmp, Digits, Float, NoTime, StringU8, TimeSplit, TimeSplitHourNano, format_buf};
#[cfg(feature = "alloc")]
use crate::{String, format};

#[doc = crate::_TAG_TIME!()]
/// Timecode splitting and formatting.
#[doc = crate::_doc_location!("phys/time")]
///
/// # Examples
/// ```
/// # use devela::Timecode;
/// assert_eq!(Timecode::secs_f64(3661.5), "01:01:01.500");
/// assert_eq!(Timecode::nanos_u64(1_002_003_004), "001s 002ms 003µs 004ns");
/// ```
#[derive(Debug)]
#[doc = crate::_doc_location!("phys/time")]
pub struct Timecode;

impl Timecode {
    /* time splitting */

    /// Decomposes a number of `seconds` in `{ h, m, s, ms }`.
    ///
    /// The maximum decomposition for [`u64::MAX`] is
    /// `{ h: 5_124_095_576_030_431, .. }` (more than 584_942_417 millenia).
    // -> 64 bits
    #[must_use]
    pub const fn split_secs_f64(
        seconds: f64,
    ) -> TimeSplitHourNano<u32, u8, u8, u16, NoTime, NoTime> {
        let ms = (Float(seconds).fract().0 * 1000.) as u16;
        let mut ts = Float(seconds).trunc().0 as u64;
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
    // TODO: make const, replace format_buf.
    pub fn secs_f64(seconds: f64) -> StringU8<12> {
        let TimeSplitHourNano { h, m, s, ms, .. } = Self::split_secs_f64(seconds);
        let m = Digits(m as u32).digits10_str(2);
        let s = Digits(s as u32).digits10_str(2);
        let ms = Digits(ms as u32).digits10_str(3);

        let mut buf = [0; 12];
        let mut buf_len = 12;
        if h > 0 {
            let h = Digits(Cmp(h).min(99)).digits10_str(2);
            let _str = format_buf![&mut buf, "{h}:{m}:{s}.{ms}"];
        } else {
            buf_len = 9;
            let _str = format_buf![&mut buf, "{m}:{s}.{ms}"];
        }

        #[cfg(any(feature = "safe_time", not(feature = "unsafe_str")))]
        return crate::unwrap![ok StringU8::<12>::from_array_nleft(buf, buf_len)];

        #[cfg(all(not(feature = "safe_time"), feature = "unsafe_str"))]
        // SAFETY: the buffer contains only ASCII characters.
        unsafe {
            StringU8::<12>::from_array_nleft_unchecked(buf, buf_len)
        }
    }

    /// Returns the time code, up to seconds, as `001s 012ms 012µs 012ns`.
    ///
    /// The seconds are clamped to 999 (more than 16 minutes).
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
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
    // TODO: make const, replace format_buf.
    pub fn nanos_u64(nanos: u64) -> StringU8<23> {
        let TimeSplitHourNano { s, ms, us, ns, .. } = Self::split_nanos_u64(nanos);
        let s_str = Digits(s.min(999)).digits10_str(3);
        let ms_str = Digits(ms as u32).digits10_str(3);
        let us_str = Digits(us as u32).digits10_str(3);
        let ns_str = Digits(ns as u32).digits10_str(3);

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
        return crate::unwrap![ok StringU8::<23>::from_array_nleft(buf, buf_len)];

        #[cfg(all(not(feature = "safe_time"), feature = "unsafe_str"))]
        // SAFETY: the buffer contains only ASCII characters.
        unsafe {
            StringU8::<23>::from_array_nleft_unchecked(buf, buf_len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    fn timecode_nanos_u64() {
        let formatted = Timecode::nanos_u64(1_002_003_004);
        assert_eq!(formatted, "001s 002ms 003µs 004ns");
    }
}
