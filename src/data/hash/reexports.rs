// devela::data::hash::reexports
//
//! Reexported items from `core`.
//

use crate::reexport;

reexport! { rust: core::hash,
    doc: "A trait for creating instances of [`Hasher`].",
    @BuildHasher as HasherBuild
}
reexport! { rust: core::hash,
    doc: "Create a default [`HasherBuild`] instance for `T:`[`Hasher`]` + `[`Default`] types.",
    @BuildHasherDefault as HasherBuildDefault
}
reexport! { rust: core::hash,
    doc: "A trait for hashing an arbitrary stream of bytes.",
    Hasher
}
reexport! { rust: std::hash,
    doc: "The default state for [`HashMapStd`][crate::HashMapStd].",
    RandomState
}

// NOTE: the trait and the derive macro have the same name:
// reexport! { rust: core::hash, doc: "A hashable type.", Hash }
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use crate::_core::hash::Hash;
