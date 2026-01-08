// devela::sys::log::config

use super::{LogLevel, LogLevelFilter};

#[doc = crate::_tags!(log)]
/// Configuration for a logger.
#[doc = crate::_doc_location!("sys/mem")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_log")))]
#[derive(Clone, Debug)]
pub struct LogConfig {
    target_level: LogLevelFilter,
}

// impl LogConfig {}
