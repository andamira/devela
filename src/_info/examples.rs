// devela::_info::examples
//
//!
//! These examples generate example structures for documentation purposes.
//
// NOTE: rustdoc doesn't detect changes in the examples, only in the library,
// so it's necessary to modify the library to rebuild the examples docs

#![allow(unused)]

/* in-crate integrated examples */

#[cfg(feature = "_bit_u8")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_bit_u8")))]
#[path = "../../examples/data/bitfield.rs"]
pub mod bitfield;
#[cfg(all(feature = "bit", feature = "_bit_u8"))]
#[path = "../../examples/code/enumset.rs"]
pub mod enumset;

#[path = "../../examples/code/enumint.rs"]
pub mod enumint;

#[path = "../../examples/data/id_seq.rs"]
pub mod id_seq;

#[path = "../../examples/num/niche.rs"]
pub mod niche;

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
