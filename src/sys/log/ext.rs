// devela::sys::log::ext

use super::{Log, LogLevelFilter, LoggerConfig};
#[cfg(feature = "alloc")]
use crate::mem::Box;

/// Extension trait providing additional methods for [`Log`]gers.
pub trait ExtLog: Log {
    /// Returns the level for this logger.
    #[must_use]
    fn level(&self) -> LogLevelFilter;

    /// Returns a reference of the configuration.
    #[must_use]
    fn config(&self) -> Option<&LoggerConfig>;

    /// Returns the logger as a `Log` trait object.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    fn as_log(self: Box<Self>) -> Box<dyn Log>;
}
