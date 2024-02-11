// devela::data::hash::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

reexport! { rust: core::hash, local_module: "data",
    doc: "A trait for creating instances of [`Hasher`].",
    BuildHasher
}
reexport! { rust: core::hash, local_module: "data",
    doc: "Used to create a default [`BuildHasher`] instance for types that implement
        [`Hasher`] and [`Default`]",
    BuildHasherDefault
}
reexport! { rust: core::hash, local_module: "data",
    doc: "A trait for hashing an arbitrary stream of bytes..",
    Hasher
}

// NOTE: the trait and the derive macro have the same name
//
// reexport! { rust: core::hash, local_module: "data",
//     doc: "A hashable type.",
//     Hash
// }
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
pub use core::hash::Hash;
