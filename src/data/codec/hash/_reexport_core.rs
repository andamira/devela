// devela/src/data/codec/hash/_reexport_core.rs

use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: core::hash,
    location: "data/codec/hash" => struct HasherBuildDefault, tag: _tags!(hash),
    doc: "Create a default [`HasherBuild`] instance for `T:`[`Hasher`]` + `[`Default`] types.",
    @BuildHasherDefault as HasherBuildDefault
}

/* traits */

// NOTE: the following trait and the corresponding derive macro have the same name:
_reexport! { rust: core::hash,
    location: "data/codec/hash" => trait Hash, tag: _tags!(hash),
    doc: "A trait for creating instances of [`Hasher`]. (Derivable)",
    Hash
}
_reexport! { rust: core::hash,
    location: "data/codec/hash" => trait Hasher, tag: _tags!(hash),
    doc: "A trait for hashing an arbitrary stream of bytes.",
    Hasher
}
_reexport! { rust: core::hash,
    location: "data/codec/hash" => trait HasherBuild, tag: _tags!(hash),
    doc: "A trait for creating instances of [`Hasher`].",
    @BuildHasher as HasherBuild
}
