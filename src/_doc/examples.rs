// devela::_doc::examples
//
#![doc = crate::_tags!(example)]
//! Documented examples.
//!
//! These examples generate example structures for documentation purposes.
//
// NOTE: rustdoc doesn't detect changes in the examples, but only in the library.
// So it's necessary to modify the library in order to rebuild the examples docs.

#![allow(unused)]

/* in-crate integrated examples */

#[path = "../../examples/code/enumint.rs"]
pub mod enumint;
#[path = "../../examples/code/enumset.rs"]
pub mod enumset;

#[path = "../../examples/data/id_seq.rs"]
pub mod id_seq;

/* out-crate standalone examples */

// #[path = "examples/separate_crate"]
// pub mod example_separate_crate {
//     // The library.
//     #[path = "src/lib.rs"]
//     pub mod lib;
//
//     // A selected example.
//     // #[cfg(feature = "std")] // any needed features
//     #[path = "examples/hello_world.rs"]
//     pub mod example_hello_world;
// }

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[doc(inline)]
        #[doc = crate::_tags!(example num niche)]
        pub use super::enumint::*;

        #[doc(inline)]
        #[doc = crate::_tags!(example code member)]
        pub use super::enumset::EnumExample;
        #[doc(inline)]
        #[doc = crate::_tags!(example code set)]
        pub use super::enumset::EnumSetExample;

        #[doc(inline)]
        #[doc = crate::_tags!(example uid construction)]
        pub use super::id_seq::*;
    }
}
