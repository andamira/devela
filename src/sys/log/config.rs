// devela::sys::log::config

#[expect(unused_imports, reason = "wip")]
use super::{LogLevel, LogLevelFilter};

///
#[derive(Clone, Debug)]
pub struct LoggerConfig {
    target_level: LogLevelFilter,
}

impl LoggerConfig {}
