// devela::sys::log::ext

use super::{LogConfig, LogLevelFilter, Logger};
#[cfg(feature = "alloc")]
use crate::Box;

#[doc = crate::_TAG_LOG!()]
/// Extension trait providing additional methods for [`Logger`]s.
#[doc = crate::_doc_location!("sys/log")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_log")))]
pub trait LoggerExt: Logger {
    /// Returns the logger as a `Logger` trait object.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    fn as_logger(self: Box<Self>) -> Box<dyn Logger>;

    /// Returns a reference of the configuration.
    #[must_use]
    fn config(&self) -> Option<&LogConfig>;

    /// Returns the level filter for this logger.
    #[must_use]
    fn level_filter(&self) -> LogLevelFilter;
}
