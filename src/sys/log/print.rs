// devela::sys::log::simple
//
//! A simple logger.
//

use super::{Log, LogLevelFilter, LogMetadata, LogRecord, Logging};

/// A logger that prints out the logs.
pub struct LoggerPrint;

impl LoggerPrint {
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
    #[allow(unused_mut, reason = "only mutated with std enabled")]
    pub fn init(mut level: LogLevelFilter) -> Result<(), ::log::SetLoggerError> {
        static LOGGER: LoggerPrint = LoggerPrint {};

        #[cfg(feature = "std")]
        if let Ok(rust_log_var) = std::env::var("RUST_LOG") {
            if let Ok(rust_log_var) = rust_log_var.parse::<LogLevelFilter>() {
                level = rust_log_var;
            }
        }
        Logging::set_logger(&LOGGER)?;
        Logging::set_max_level(level);
        Ok(())
    }
}

impl Log for LoggerPrint {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= Logging::max_level()
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            #[cfg(feature = "std")]
            {
                use super::LogLevel;
                let lvl = match record.level() {
                    LogLevel::Error => "Error",
                    LogLevel::Warn => "Warn ",
                    LogLevel::Info => "Info ",
                    LogLevel::Debug => "Debug",
                    LogLevel::Trace => "Trace",
                };
                eprintln!("{} {}", lvl, record.args());
            }
        }
    }

    fn flush(&self) {}
}
