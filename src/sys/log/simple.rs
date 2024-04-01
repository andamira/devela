// devela::sys::log::simple
//
//! A simple logger.
//

use super::{Log, LogLevel, LogLevelFilter, LogMetadata, LogRecord};
use crate::_deps::log::{max_level, set_logger, set_max_level};

/// A simple logger.
pub struct LoggerSimple;

impl LoggerSimple {
    /// Sets itself as the global logger.
    ///
    /// This function should be called very early in the program.
    ///
    /// # Errors
    /// Returns an error if the logger cannot be set.
    ///
    /// # Features
    /// If the `std` feature is enabled it will use the log level defined by
    /// the `RUST_LOG` env variable.
    ///
    /// For now it only prints out to stderr if the `std` feature is enabled.
    #[allow(unused_mut)]
    pub fn init(mut level: LogLevelFilter) -> Result<(), ::log::SetLoggerError> {
        static LOGGER: LoggerSimple = LoggerSimple {};

        #[cfg(feature = "std")]
        if let Ok(rust_log_var) = std::env::var("RUST_LOG") {
            if let Ok(rust_log_var) = rust_log_var.parse::<LogLevelFilter>() {
                level = rust_log_var;
            }
        }
        set_logger(&LOGGER)?;
        set_max_level(level);
        Ok(())
    }
}

impl Log for LoggerSimple {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= max_level()
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            #[allow(unused_variables)]
            let lvl = match record.level() {
                LogLevel::Error => "Error",
                LogLevel::Warn => "Warn ",
                LogLevel::Info => "Info ",
                LogLevel::Debug => "Debug",
                LogLevel::Trace => "Trace",
            };
            #[cfg(feature = "std")]
            eprintln!("{} {}", lvl, record.args());
        }
    }

    fn flush(&self) {}
}
