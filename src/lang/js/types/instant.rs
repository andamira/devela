// devela::lang:js::types::instant

#[allow(unused_imports)]
use crate::{Js, TimeDelta};

/// A high-resolution timestamp based on JavaScript's `performance.now()`.
///
/// Provides millisecond-precision timing suitable for benchmarking and animations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JsInstant {
    /// Milliseconds since `performance.timeOrigin`.
    pub(crate) ms_timestamp: f64,
}
#[rustfmt::skip]
impl JsInstant {
    /// Returns a new `JsInstant` from a timestamp in milliseconds.
    pub(crate) const fn new(ms_timestamp: f64) -> Self { Self { ms_timestamp } }
}

#[rustfmt::skip]
#[cfg(all(not(windows), feature = "unsafe_ffi"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(target_arch = "wasm32")))]
impl JsInstant {
    /// Returns the current instant using `performance.now()`.
    pub fn now() -> Self { Js::performance_now() }

    /// Resets this instant to the current time.
    pub fn reset(&mut self) { *self = Js::performance_now(); }

    /// Returns the duration between this and an earlier `JsInstant`.
    pub const fn since(self, earlier: Self) -> Self {
        Self::new(self.ms_timestamp - earlier.ms_timestamp)
    }

    /// Returns the time in `f64` milliseconds (the internal representation)
    pub const fn as_millis_f64(self) -> f64 { self.ms_timestamp }

    /// Returns the time in `f64` seconds.
    pub const fn as_secs_f64(self) -> f64 { self.ms_timestamp / 1_000.0 }
}
