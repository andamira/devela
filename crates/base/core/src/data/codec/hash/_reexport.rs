// devela_base_core::data::codec::hash::_reexport

use crate::{_TAG_HASH, _reexport};

_reexport! { rust: core::hash, location: "data/codec/hash", tag: _TAG_HASH!(),
    doc: "A trait for creating instances of [`Hasher`].",
    @BuildHasher as HasherBuild
}
_reexport! { rust: core::hash, location: "data/codec/hash", tag: _TAG_HASH!(),
    doc: "Create a default [`HasherBuild`] instance for `T:`[`Hasher`]` + `[`Default`] types.",
    @BuildHasherDefault as HasherBuildDefault
}
_reexport! { rust: core::hash, location: "data/codec/hash", tag: _TAG_HASH!(),
    doc: "A trait for hashing an arbitrary stream of bytes.",
    Hasher
}
// NOTE: the trait and the derive macro have the same name:
_reexport! { rust: core::hash, location: "data/codec/hash", tag: _TAG_HASH!(),
    doc: "A trait for creating instances of [`Hasher`]. (Derivable)",
    Hash
}
