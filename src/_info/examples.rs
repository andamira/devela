// devela::_info::examples
//
//!
//! These examples generate example structures for documentation purposes.
//
// NOTE: rustdoc doesn't detect changes in the examples, but only in the library.
// So it's necessary to modify the library in order to rebuild the examples docs.

#![allow(unused)]

/* in-crate integrated examples */

#[path = "../../examples/code/enumset.rs"]
pub mod enumset;
#[cfg(feature = "devela_macros")]
#[path = "../../examples/code/enumint.rs"]
pub mod enumint;

#[path = "../../examples/data/bitfield.rs"]
pub mod bitfield;
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
