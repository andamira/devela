// devela::data::codec::hash::_reexport_core

use crate::{_reexport, _tags};

_reexport! { rust: core::hash, location: "data/codec/hash", tag: _tags!(hash),
    doc: "A trait for creating instances of [`Hasher`].",
    @BuildHasher as HasherBuild
}
_reexport! { rust: core::hash, location: "data/codec/hash", tag: _tags!(hash),
    doc: "Create a default [`HasherBuild`] instance for `T:`[`Hasher`]` + `[`Default`] types.",
    @BuildHasherDefault as HasherBuildDefault
}
_reexport! { rust: core::hash, location: "data/codec/hash", tag: _tags!(hash),
    doc: "A trait for hashing an arbitrary stream of bytes.",
    Hasher
}
// NOTE: the trait and the derive macro have the same name:
_reexport! { rust: core::hash, location: "data/codec/hash", tag: _tags!(hash),
    doc: "A trait for creating instances of [`Hasher`]. (Derivable)",
    Hash
}
