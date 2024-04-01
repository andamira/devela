// devela::sys::log::logging

use super::{Log, LogLevelFilter};
#[cfg(feature = "std")]
use ::log::set_boxed_logger;
use ::log::{logger, max_level, set_logger, set_max_level, SetLoggerError, STATIC_MAX_LEVEL};

/// Provides logging methods.
///
/// It is a namespace for the `log` crate standalone functions.
pub struct Logging;

impl Logging {
    /// The statically resolved maximum log level.
    ///
    /// Returns [`STATIC_MAX_LEVEL`].
    pub const MAX_LEVEL: LogLevelFilter = STATIC_MAX_LEVEL;

    /// Returns a reference to the current global logger.
    ///
    /// Calls [`logger`].
    pub fn logger() -> &'static dyn Log {
        logger()
    }

    /// Sets the global logger to a `&'static Log`.
    ///
    /// Calls [`set_logger`].
    #[inline]
    pub fn set_logger(logger: &'static dyn Log) -> Result<(), SetLoggerError> {
        set_logger(logger)
    }

    /// Sets the global logger to a `Box<Log>`.
    ///
    /// Calls [`set_boxed_logger`].
    #[inline]
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn set_boxed_logger(logger: Box<dyn Log>) -> Result<(), SetLoggerError> {
        set_boxed_logger(logger)
    }

    // /// A thread-unsafe version of set_logger.
    // ///
    // /// Calls [`set_logger_racy`].
    // #[inline]
    // // IMPROVE: needs a specific unsafe feature, ex: unsafe_[sync|embedded]
    // #[cfg(not(feature = "safe_sys"))]
    // #[cfg_attr(feature = "nightly", doc(cfg(not(feature = "safe_sys"))))]
    // pub unsafe fn set_logger(logger: &'static dyn Log) -> Result<(), SetLoggerError> {
    //     set_logger_racy(logger)
    // }

    /// Returns the current maximum log level.
    ///
    /// Calls [`max_level`].
    #[inline] #[rustfmt::skip]
    pub fn max_level() -> LogLevelFilter { max_level() }

    /// Sets the global maximum log level.
    ///
    /// Calls [`set_max_level`].
    #[inline] #[rustfmt::skip]
    pub fn set_max_level(level: LogLevelFilter) { set_max_level(level) }
}
