// devela/src/sys/fs/path/_reexport_std.rs

use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs lifetime),
    doc: "A slice of a path (akin to [`str`]).",
    Path
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs),
    doc: "An owned, mutable path (akin to [`String`]).",
    PathBuf
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs windows),
    doc: "Wraps a Windows path prefix as well as its unparsed string representation.",
    @PrefixComponent as PathWindowsPrefixComponent
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs error),
    doc: "An error returned from [`Path::strip_prefix`] if the prefix was not found.",
    @StripPrefixError as PathStripPrefixError
}

/* enums */

_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs),
    doc: "A single component of a path.",
    @Component as PathComponent
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs windows),
    doc: r"Windows path prefixes, e.g., `C:` or `\\server\share.`",
    @Prefix as PathWindowsPrefix
}
