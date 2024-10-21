// devela::_info::examples
//
//!
//
// NOTE1: rustdoc doesn't detect changes in the examples, only in the library,
// so it's necessary to modify the library to rebuild the examples docs
//
// NOTE2: The examples can't import macros from the root, but from submodules.
// (Triggers the lint MACRO_EXPANDED_MACRO_EXPORTS_ACCESSED_BY_ABSOLUTE_PATHS)
// - https://github.com/rust-lang/rust/issues/53495
// - https://github.com/rust-lang/rust/pull/52234

#![allow(unused)]

/* in-crate integrated examples */

#[cfg(all(feature = "mem_bit", feature = "_bit_u8"))]
#[path = "../../examples/bitfield.rs"]
pub mod bitfield;
#[cfg(all(feature = "mem_bit", feature = "_bit_u8"))]
#[path = "../../examples/enumset.rs"]
pub mod enumset;

#[path = "../../examples/enumint.rs"]
pub mod enumint;

#[path = "../../examples/id_seq.rs"]
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
