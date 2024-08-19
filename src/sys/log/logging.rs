// devela::sys::log::logging

use super::{Log, LogLevelFilter};
#[cfg(feature = "std")]
use ::log::set_boxed_logger;
#[cfg(all(not(feature = "safe_sys"), feature = "unsafe_thread"))]
use ::log::set_logger_racy;
use ::log::{logger, max_level, set_logger, set_max_level, SetLoggerError, STATIC_MAX_LEVEL};

/// Provides logging methods.
///
/// It is a namespace for the `log` crate standalone functions.
pub struct Logging;

impl Logging {
    /* log crate */

    /// The statically resolved maximum log level.
    ///
    /// Returns [`STATIC_MAX_LEVEL`].
    pub const MAX_LEVEL: LogLevelFilter = STATIC_MAX_LEVEL;

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
    #[cfg(target_has_atomic = "ptr")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(target_has_atomic = "ptr")))]
    pub fn set_logger(logger: &'static dyn Log) -> Result<(), SetLoggerError> {
        set_logger(logger)
    }

    /// Sets the global logger to a `Box<Log>`.
    ///
    /// Calls [`set_boxed_logger`].
    #[inline]
    #[cfg(all(feature = "std", target_has_atomic = "ptr"))]
    #[cfg_attr(
        feature = "nightly_doc",
        doc(cfg(all(feature = "std", target_has_atomic = "ptr")))
    )]
    pub fn set_boxed_logger(logger: Box<dyn Log>) -> Result<(), SetLoggerError> {
        set_boxed_logger(logger)
    }

    /// A thread-unsafe version of set_logger.
    ///
    /// Calls [`set_logger_racy`].
    ///
    /// # Safety
    /// See the related documentation in `set_logger_racy`.
    #[inline]
    #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_thread"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_thread")))]
    pub unsafe fn set_logger_racy(logger: &'static dyn Log) -> Result<(), SetLoggerError> {
        // SAFETY: caller must ensure safety
        unsafe { set_logger_racy(logger) }
    }

    /* */
}
