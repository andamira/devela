// devela::thread
//
//! Native threads, extends [`std::thread`].
//

/// Briefer `std::thread::`[`sleep`][std::thread::sleep].
///
/// Sleeps for `$ns` seconds + `$ms` milliseconds + `$us` microseconds + `$ns` nanoseconds
#[macro_export]
macro_rules! sleep4 {
    ($s:expr) => {
        std::thread::sleep(std::time::Duration::from_secs($s));
    };
    ($s:expr, $ms:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($s * 1000 + $ms));
    };
    ($s:expr, $ms:expr, $us:expr) => {
        std::thread::sleep(std::time::Duration::from_micros(
            $s * 1_000_000 + $ms * 1_000 + $us,
        ));
    };
    ($s:expr, $ms:expr, $us:expr, $ns:expr) => {
        std::thread::sleep(std::time::Duration::from_nanos(
            $s * 1_000_000_000 + $ms * 1_000_000 + $us * 1_000 + $ns,
        ));
    };
}
pub use sleep4;
