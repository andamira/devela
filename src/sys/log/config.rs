// devela::sys::log::config

use super::{LogLevel, LogLevelFilter};

/// Configuration for a logger.
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_log")))]
#[derive(Clone, Debug)]
pub struct LogConfig {
    target_level: LogLevelFilter,
}

// impl LogConfig {}
