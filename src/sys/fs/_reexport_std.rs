// devela/src/sys/fs/_reexport_std.rs

#[allow(unused_imports)]
use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: std::fs,
    location: "sys/fs" => struct File, tag: _tags!(fs),
    doc: "An object providing access to an open file on the filesystem.",
    File
}
_reexport! { rust: std::fs,
    location: "sys/fs" => struct FileTimes, tag: _tags!(fs),
    doc: "Representation of the various timestamps on a file.",
    FileTimes
}
_reexport! { rust: std::fs,
    location: "sys/fs" => struct FileType, tag: _tags!(fs),
    doc: "Represents a type of file with accessors for each file type.",
    FileType
}
_reexport! { rust: std::fs,
    location: "sys/fs" => struct FileMetadata, tag: _tags!(fs),
    doc: "Metadata information about a file.",
    @Metadata as FileMetadata
}
_reexport! { rust: std::fs,
    location: "sys/fs" => struct FileOpenOptions, tag: _tags!(fs),
    doc: "Options and flags which can be used to configure how a file is opened.",
    @OpenOptions as FileOpenOptions
}
_reexport! { rust: std::fs,
    location: "sys/fs" => struct FilePermissions, tag: _tags!(fs),
    doc: "Representation of the various permissions on a file.",
    @Permissions as FilePermissions
}

_reexport! { rust: std::fs,
    location: "sys/fs" => struct DirBuilder, tag: _tags!(fs),
    doc: "A builder used to create directories in various manners.",
    DirBuilder
}
_reexport! { rust: std::fs,
    location: "sys/fs" => struct DirEntry, tag: _tags!(fs),
    doc: "Entries returned by the ReadDir iterator.",
    DirEntry
}
_reexport! { rust: std::fs,
    location: "sys/fs" => struct IterDirRead, tag: _tags!(fs iterator),
    doc: "Iterator over the entries in a directory.",
    @ReadDir as IterDirRead
}
