// devela::sys::log::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

reexport! { "log" | log, features: "log",
    doc: "Logs a message at the indicated level.",
    debug, error, info, log, trace, warn
}
reexport! { "log" | log, features: "log",
    doc: "A trait encapsulating the operations required of a logger.",
    Log
}
reexport! { "log" | log, features: "log",
    doc: "An enum representing the available verbosity levels of the logger.",
    @Level as LogLevel
}
reexport! { "log" | log, features: "log",
    doc: "An enum representing the available verbosity level filters of the logger.",
    @LevelFilter as LogLevelFilter
}
reexport! { "log" | log, features: "log",
    doc: "The “payload” of a log message.",
    @Record as LogRecord
}
reexport! { "log" | log, features: "log",
    doc: "Metadata about a log message.",
    @Metadata as LogMetadata
}
