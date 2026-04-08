// devela::sys::fs::path::_reexport_std

use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs iterator),
    doc: "An iterator over [`Path`] and its ancestors.",
    @Ancestors as IterPathAncestors
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs iterator),
    doc: "An iterator over the Components of a Path.",
    @Components as IterPathComponents
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs fmt),
    doc: "Helper struct for safely printing paths with [`format!`] and `{}`.",
    @Display as PathDisplay
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs iterator),
    doc: "An iterator over the [`IterPathComponents`]
of a [`Path`], as [`OsStr`][crate::OsStr] slices.",
    @Iter as IterPath
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs lifetime),
    doc: "A slice of a path (akin to [`str`]).",
    Path
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs),
    doc: "An owned, mutable path (akin to [`String`]).",
    PathBuf
}
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs),
    doc: "Wraps a Windows path prefix as well as its unparsed string representation.",
    @PrefixComponent as PathPrefixComponent
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
_reexport! { rust: std::path, location: "sys/fs", tag: _tags!(fs),
    doc: r"Windows path prefixes, e.g., `C:` or `\\server\share.`",
    @Prefix as PathPrefix
}
