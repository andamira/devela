// devela::phys::time::source::impls
//
// TOC
// - impl for SystemTime
// - impl for SystemInstant
// - impl for JsInstant

#[allow(unused_imports, reason = "various feature-gates")]
use crate::{TimeScale, TimeSource};

#[rustfmt::skip]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod impl_std {
    use crate::{TimeSource, TimeScale, UNIX_EPOCH, OnceLock, SystemInstant, SystemTime};

    impl TimeSource for SystemTime {
        fn time_is_monotonic() -> bool { false }
        fn time_is_absolute() -> bool { true }
        fn time_scale() -> TimeScale { TimeScale::Nanos }
        fn time_now_millis() -> u64 {
            SystemTime::now().duration_since(UNIX_EPOCH).expect("backwards time").as_millis() as u64
        }
        //
        fn time_now_micros() -> u64 {
            SystemTime::now().duration_since(UNIX_EPOCH).expect("backwards time").as_micros() as u64
        }
        fn time_now_nanos() -> u64 {
            SystemTime::now().duration_since(UNIX_EPOCH).expect("backwards time").as_nanos() as u64
        }
    }

    /// Process-local base instant used to derive a numeric timeline for `SystemInstant`.
    ///
    /// `SystemInstant` does not expose a global or absolute epoch;
    /// it only supports measuring durations between two instants.
    /// To fit the `TimeSource` numeric timeline model, a synthetic,
    /// process-local origin is established on first use.
    ///
    /// All timestamps produced by the `SystemInstant` time source
    /// are measured as the elapsed duration since this base instant.
    ///
    /// This base is initialized lazily and exactly once.
    static SYSTEM_INSTANT_BASE: OnceLock<SystemInstant> = OnceLock::new();

    impl TimeSource for SystemInstant {
        fn time_is_monotonic() -> bool { true }
        fn time_is_absolute() -> bool { false }
        fn time_scale() -> TimeScale { TimeScale::Nanos }
        fn time_now_millis() -> u64 {
            let base = SYSTEM_INSTANT_BASE.get_or_init(SystemInstant::now);
            base.elapsed().as_millis() as u64
        }
        //
        fn time_now_micros() -> u64 {
            let base = SYSTEM_INSTANT_BASE.get_or_init(SystemInstant::now);
            base.elapsed().as_micros() as u64
        }
        fn time_now_nanos() -> u64 {
            let base = SYSTEM_INSTANT_BASE.get_or_init(SystemInstant::now);
            base.elapsed().as_nanos() as u64
        }
    }
}

#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "js", feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "js", feature = "unsafe_ffi"))))]
impl TimeSource for crate::JsInstant {
    fn time_is_monotonic() -> bool { true }
    fn time_is_absolute() -> bool { false }
    fn time_scale() -> TimeScale { TimeScale::Millis }
    fn time_now_millis() -> u64 { crate::JsInstant::now().as_millis_f64() as u64 }
    //
    fn time_now_millis_f64() -> f64 { crate::JsInstant::now().as_millis_f64() }
}
