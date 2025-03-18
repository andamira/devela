// devela::phys::time::source
//
//! Defines the [`TimeSource`] trait.
//

use crate::{Enum, Ratio, TimeGranularity};

#[cfg(all(feature = "js", feature = "unsafe_ffi"))]
use crate::JsInstant;
#[cfg(feature = "std")]
use crate::{SystemInstant, SystemTime, UNIX_EPOCH};

#[rustfmt::skip]
#[doc = crate::TAG_TIME!()]
/// A source of timestamps with a known granularity and monotonicity.
///
/// Provides a consistent API for querying timestamps at various precisions,
/// abstracting over different time sources.
pub trait TimeSource<const MONOTONIC: bool> {
    /// Returns the granularity of this time source.
    fn granularity() -> Enum<2, TimeGranularity, Ratio<u32, u32>>;

    /// Returns the current timestamp in milliseconds.
    fn now_millis() -> u64;

    /// Returns the current timestamp in microseconds.
    ///
    /// Default: Uses `now_millis()`.
    fn now_micros() -> u64 { Self::now_millis() * 1_000 }

    /// Returns the current timestamp in nanoseconds.
    ///
    /// Default: Uses `now_micros()`.
    fn now_nanos() -> u64 { Self::now_micros() * 1_000 }

    /// Returns the current timestamp as an `f64` value in milliseconds.
    ///
    /// Default: Converts `now_millis()` to `f64`.
    fn now_millis_f64() -> f64 { Self::now_millis() as f64 }
}

#[cfg(feature = "std")] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl TimeSource<false> for SystemTime {
    fn granularity() -> Enum<2, TimeGranularity, Ratio<u32, u32>> {
        Enum::A(TimeGranularity::Nanos)
    }
    fn now_millis() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("backwards time").as_millis() as u64
    }
    fn now_micros() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("backwards time").as_micros() as u64
    }
    fn now_nanos() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("backwards time").as_nanos() as u64
    }
}
#[cfg(feature = "std")] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl TimeSource<true> for SystemInstant {
    fn granularity() -> Enum<2, TimeGranularity, Ratio<u32, u32>> {
        Enum::A(TimeGranularity::Nanos)
    }
    fn now_millis() -> u64 { SystemInstant::now().elapsed().as_millis() as u64 }
    fn now_micros() -> u64 { SystemInstant::now().elapsed().as_micros() as u64 }
    fn now_nanos() -> u64 { SystemInstant::now().elapsed().as_nanos() as u64 }
}

#[cfg(all(feature = "js", feature = "unsafe_ffi"))] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "js", feature = "unsafe_ffi"))))]
impl TimeSource<true> for JsInstant {
    fn granularity() -> Enum<2, TimeGranularity, Ratio<u32, u32>> {
        Enum::A(TimeGranularity::Millis)
    }
    fn now_millis() -> u64 { JsInstant::now().as_millis_f64() as u64 }
    fn now_millis_f64() -> f64 { JsInstant::now().as_millis_f64() }
}

// #[cfg(all(target_arch = "arm", feature = "dep_cortex_m"))]
// impl TimeSource<true> for ::cortex_m::peripheral::DWT {
//     fn granularity() -> Enum<TimeGranularity, Ratio<u32, u32>> {
//         Enum::B(Ratio<1, 32_768>)
//     }
//     fn now_millis() -> u64 {
//         unsafe { ::cortex_m::peripheral::DWT::cycle_count() as u64 / (SystemCoreClock / 1_000) }
//     }
//     fn now_micros() -> u64 {
//         unsafe { ::cortex_m::peripheral::DWT::cycle_count() as u64 / (SystemCoreClock / 1_000_000) }
//     }
// }
