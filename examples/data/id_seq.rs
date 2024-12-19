// devela::examples::id_seq
//
//! Shows a sequential ID generator made with the [`id_seq!`] macro.
//!
//! # Example
//! ```
//! # use devela::id_seq;
//! // Construct an id generator for a maximum of 254 unsigned 8-bit unique Ids.
//! id_seq![IdU8, u8];
//!
//! assert_eq![1, IdU8::new().unwrap().value()];
//! ```
//

use devela::id_seq;

id_seq![ExampleIdSeqUsize, usize];

fn main() {}
