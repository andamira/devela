// devela::work::process::thread::sleep::macros

/// A more compact [`Thread::sleep`][crate::Thread#method.sleep].
///
/// Sleeps for `$ns` seconds + `$ms` milliseconds + `$us` microseconds + `$ns` nanoseconds
///
/// # Examples
/// ```
/// # use devela::sleep4;
/// sleep4![1, 0, 500]; // sleeps for 1 second + 500 microseconds
/// ```
#[macro_export]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! sleep4 {
    ($s:expr) => {
        std::thread::sleep(core::time::Duration::from_secs($s));
    };
    ($s:expr, $ms:expr) => {
        std::thread::sleep(core::time::Duration::from_millis($s * 1000 + $ms));
    };
    ($s:expr, $ms:expr, $us:expr) => {
        std::thread::sleep(core::time::Duration::from_micros($s * 1_000_000 + $ms * 1_000 + $us));
    };
    ($s:expr, $ms:expr, $us:expr, $ns:expr) => {
        std::thread::sleep(core::time::Duration::from_nanos(
            $s * 1_000_000_000 + $ms * 1_000_000 + $us * 1_000 + $ns,
        ));
    };
}
#[cfg(feature = "std")]
#[doc(inline)]
pub use sleep4;
