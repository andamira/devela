// devela::sys::log::config

#![allow(dead_code, unused_imports)] // TEMP

use super::{LogLevel, LogLevelFilter};

///
#[derive(Clone, Debug)]
pub struct LoggerConfig {
    target_level: LogLevelFilter,
}

impl LoggerConfig {}
