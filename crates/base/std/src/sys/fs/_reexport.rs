// devela_base_std::sys::fs::_reexport

#[allow(unused_imports)]
use crate::{_TAG_FS, _TAG_ITERATOR, _reexport};

/* structs: files */

_reexport! { rust: std::fs, location: "sys/fs", tag: _TAG_FS!(),
    doc: "An object providing access to an open file on the filesystem.",
    File
}
_reexport! { rust: std::fs, location: "sys/fs", tag: _TAG_FS!(),
    doc: "Representation of the various timestamps on a file.",
    FileTimes
}
_reexport! { rust: std::fs, location: "sys/fs", tag: _TAG_FS!(),
    doc: "Represents a type of file with accessors for each file type.",
    FileType
}
_reexport! { rust: std::fs, location: "sys/fs", tag: _TAG_FS!(),
    doc: "Metadata information about a file.",
    @Metadata as FileMetadata
}
_reexport! { rust: std::fs, location: "sys/fs", tag: _TAG_FS!(),
    doc: "Options and flags which can be used to configure how a file is opened.",
    @OpenOptions as FileOpenOptions
}
_reexport! { rust: std::fs, location: "sys/fs", tag: _TAG_FS!(),
    doc: "Representation of the various permissions on a file.",
    @Permissions as FilePermissions
}

/* structs: files */

_reexport! { rust: std::fs, location: "sys/fs", tag: _TAG_FS!(),
    doc: "A builder used to create directories in various manners.",
    DirBuilder
}
_reexport! { rust: std::fs, location: "sys/fs", tag: _TAG_FS!(),
    doc: "Entries returned by the ReadDir iterator.",
    DirEntry
}
_reexport! { rust: std::fs, location: "sys/fs", tag: _TAG_FS!() _TAG_ITERATOR!(),
    doc: "Iterator over the entries in a directory.",
    @ReadDir as IterDirRead
}
