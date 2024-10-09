// devela::sys::log::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

reexport! { "dep_log", "log", log,
    doc: "Logs a message at the indicated level.",
    debug, error, info, log, trace, warn
}
reexport! { "dep_log", "log", log,
    doc: "A trait encapsulating the operations required of a logger.",
    Log
}
reexport! { "dep_log", "log", log,
    doc: "An enum representing the available verbosity levels of the logger.",
    @Level as LogLevel
}
reexport! { "dep_log", "log", log,
    doc: "An enum representing the available verbosity level filters of the logger.",
    @LevelFilter as LogLevelFilter
}
reexport! { "dep_log", "log", log,
    doc: "The “payload” of a log message.",
    @Record as LogRecord
}
reexport! { "dep_log", "log", log,
    doc: "Metadata about a log message.",
    @Metadata as LogMetadata
}
