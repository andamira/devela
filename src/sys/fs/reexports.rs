// devela::sys::fs::reexports
//
//! Reexported items from `std::fs`.
//

#[allow(unused_imports)]
use crate::_reexport;

/* structs: files */

_reexport! { rust: std::fs,
    doc: "An object providing access to an open file on the filesystem.",
    File
}
_reexport! { rust: std::fs,
    doc: "Representation of the various timestamps on a file.",
    FileTimes
}
_reexport! { rust: std::fs,
    doc: "Represents a type of file with accessors for each file type.",
    FileType
}
_reexport! { rust: std::fs,
    doc: "Metadata information about a file.",
    @Metadata as FileMetadata
}
_reexport! { rust: std::fs,
    doc: "Options and flags which can be used to configure how a file is opened.",
    @OpenOptions as FileOpenOptions
}
_reexport! { rust: std::fs,
    doc: "Representation of the various permissions on a file.",
    @Permissions as FilePermissions
}

/* structs: files */

_reexport! { rust: std::fs,
    doc: "A builder used to create directories in various manners.",
    DirBuilder
}
_reexport! { rust: std::fs,
    doc: "Entries returned by the ReadDir iterator.",
    DirEntry
}
_reexport! { rust: std::fs,
    tag: crate::TAG_ITERATOR!(),
    doc: "Iterator over the entries in a directory.",
    @ReadDir as IterDirRead
}
