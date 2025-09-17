// devela::sys::log::namespace

use super::{LogLevelFilter, Logger, LoggerSetError};
#[cfg(feature = "std")]
use ::log::set_boxed_logger;
#[cfg(all(not(feature = "safe_sys"), feature = "unsafe_thread"))]
use ::log::set_logger_racy;
use ::log::{STATIC_MAX_LEVEL, logger, max_level, set_logger, set_max_level};

#[doc = crate::_TAG_NAMESPACE!()]
/// Log-related operations.
///
/// It is a namespace for the `log` crate standalone functions.
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_log")))]
#[derive(Debug)]
pub struct Log;

impl Log {
    /* log crate */

    /// The statically resolved maximum log level.
    ///
    /// Returns [`STATIC_MAX_LEVEL`].
    pub const MAX_LEVEL: LogLevelFilter = STATIC_MAX_LEVEL;

    /// Returns the current maximum log level.
    ///
    /// See `::log::`[`max_level`].
    #[rustfmt::skip]
    pub fn max_level() -> LogLevelFilter { max_level() }

    /// Sets the global maximum log level.
    ///
    /// See `::log::`[`set_max_level`].
    #[rustfmt::skip]
    pub fn set_max_level(level: LogLevelFilter) { set_max_level(level) }

    /// Returns a reference to the current global logger.
    ///
    /// See `::log::`[`logger`].
    pub fn logger() -> &'static dyn Logger {
        logger()
    }

    /// Sets the global logger to a `&'static Log`.
    ///
    /// See `::log::`[`set_logger`].
    #[cfg(target_has_atomic = "ptr")]
    #[cfg_attr(nightly_doc, doc(cfg(target_has_atomic = "ptr")))]
    pub fn set_logger(logger: &'static dyn Logger) -> Result<(), LoggerSetError> {
        set_logger(logger)
    }

    /// Sets the global logger to a `Box<Log>`.
    ///
    /// See `::log::`[`set_boxed_logger`].
    #[cfg(all(feature = "std", target_has_atomic = "ptr"))]
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", target_has_atomic = "ptr"))))]
    pub fn set_boxed_logger(logger: Box<dyn Logger>) -> Result<(), LoggerSetError> {
        set_boxed_logger(logger)
    }

    /// A thread-unsafe version of set_logger.
    ///
    /// See `::log::`[`set_logger_racy`].
    ///
    /// # Safety
    /// See the related documentation in `set_logger_racy`.
    #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_thread"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_thread")))]
    pub unsafe fn set_logger_racy(logger: &'static dyn Logger) -> Result<(), LoggerSetError> {
        // SAFETY: caller must ensure safety
        unsafe { set_logger_racy(logger) }
    }

    /* */
}
