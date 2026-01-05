// devela::sys::log::_reexport_dep

#[allow(unused_imports, reason = "dep_log feature-gate")]
use crate::{_TAG_ERROR, _TAG_LOG, _reexport};

// HIDE
_reexport! { "dep_log", "log", log, location: "sys/log", tag: _TAG_LOG!(),
    doc: "Logs a message at the indicated level.",
    debug, error, info, log, trace, warn
}
_reexport! { "dep_log", "log", log, location: "sys/log", tag: _TAG_LOG!(),
    doc: "A trait encapsulating the operations required of a logger.",
    @Log as Logger
}
_reexport! { "dep_log", "log", log, location: "sys/log", tag: _TAG_LOG!() _TAG_ERROR!(),
    doc: "Returned by [`set_logger`][crate::Log::set_logger] if it has already been called.",
    @SetLoggerError as LoggerSetError
}
_reexport! { "dep_log", "log", log, location: "sys/log", tag: _TAG_LOG!(),
    doc: "An enum representing the available verbosity levels of the logger.",
    @Level as LogLevel
}
_reexport! { "dep_log", "log", log, location: "sys/log", tag: _TAG_LOG!(),
    doc: "An enum representing the available verbosity level filters of the logger.",
    @LevelFilter as LogLevelFilter
}
_reexport! { "dep_log", "log", log, location: "sys/log", tag: _TAG_LOG!(),
    doc: "The “payload” of a log message.",
    @Record as LogRecord
}
_reexport! { "dep_log", "log", log, location: "sys/log", tag: _TAG_LOG!(),
    doc: "Metadata about a log message.",
    @Metadata as LogMetadata
}
