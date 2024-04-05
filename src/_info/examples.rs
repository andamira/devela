// devela::_info::examples
//
//!
//
// NOTE: rustdoc doesn't detect changes in the examples, only in the library,
// so it's necessary to modify the library to rebuild the examples docs

#![allow(unused)]
#![path = "../../examples"]

// /// Shows how to document an in-crate integrated example.
// #[path = "examples/hello_world.rs"]
// pub mod example;
// // Example file contents:
// // ```
// // //! An example.
// // // Non-separate crate examples must call devela like this:
// // #[rustfmt::skip]     #[cfg(any(test,doc))]  use crate::*;
// // #[rustfmt::skip] #[cfg(not(any(test,doc)))] use devela::*;
// //
// // /// A struct.
// // pub struct World;
// // impl World {
// //     /// A constant.
// //     pub const HELLO: &'static str = "hello world";
// // }
// // fn main() {}
// // ```

#[cfg(all(feature = "data_bit", feature = "_bit_u8"))]
#[cfg_attr(any(doc, test), path = "examples/bitfield.rs")]
#[cfg_attr(not(any(doc, test)), path = "../../examples/bitfield.rs")]
pub mod bitfield;

#[cfg(all(feature = "data_bit", feature = "_bit_u8"))]
#[cfg_attr(any(doc, test), path = "examples/enumset.rs")]
#[cfg_attr(not(any(doc, test)), path = "../../examples/enumset.rs")]
pub mod enumset;

// /// Shows how to document an out-crate standalone example.
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
