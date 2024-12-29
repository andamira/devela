// devela::sys::path::reexports
//
//! Reexported items from `core`.
//

use crate::{reexport, TAG_ERROR, TAG_ITERATOR};

/* structs */

reexport! { rust: std::path,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over [`Path`] and its ancestors.",
    @Ancestors as IterPathAncestors
}
reexport! { rust: std::path,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over the Components of a Path.",
    @Components as IterPathComponents
}
reexport! { rust: std::path,
    doc: "Helper struct for safely printing paths with [`format!`] and `{}`.",
    @Display as PathDisplay
}
reexport! { rust: std::path,
    doc: "An iterator over the [`IterPathComponents`]
of a [`Path`], as [`OsStr`][crate::OsStr] slices.",
    @Iter as IterPath
}
reexport! { rust: std::path,
    doc: "A slice of a path (akin to [`str`]).",
    Path
}
reexport! { rust: std::path,
    doc: "An owned, mutable path (akin to [`String`]).",
    PathBuf
}
reexport! { rust: std::path,
    doc: "A structure wrapping a Windows path prefix
as well as its unparsed string representation.",
    @PrefixComponent as PathPrefixComponent
}
reexport! { rust: std::path,
    tag: TAG_ERROR!(),
    doc: "An error returned from [`Path::strip_prefix`] if the prefix was not found.",
    @StripPrefixError as PathStripPrefixError
}

/* enums */

reexport! { rust: std::path,
    doc: "A single component of a path.",
    @Component as PathComponent
}
reexport! { rust: std::path,
    doc: r"Windows path prefixes, e.g., `C:` or `\\server\share.`",
    @Prefix as PathPrefix
}
