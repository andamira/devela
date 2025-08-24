// devela_base::data::codec::hash::reexports

use crate::_reexport;

_reexport! { rust: core::hash,
    doc: "A trait for creating instances of [`Hasher`].",
    @BuildHasher as HasherBuild
}
_reexport! { rust: core::hash,
    doc: "Create a default [`HasherBuild`] instance for `T:`[`Hasher`]` + `[`Default`] types.",
    @BuildHasherDefault as HasherBuildDefault
}
_reexport! { rust: core::hash,
    doc: "A trait for hashing an arbitrary stream of bytes.",
    Hasher
}

// NOTE: the trait and the derive macro have the same name:
// _reexport! { rust: core::hash, doc: "A hashable type.", Hash }
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use ::core::hash::Hash;
