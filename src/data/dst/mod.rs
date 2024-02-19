// devela::data::dst
//
//! Dynamically-sized types stored in the stack.
//!
#![doc = include_str!("./Mod.md")]
//!
//! # Derived Work
#![doc = include_str!("./MODIFICATIONS.md")]
//
#![allow(clippy::result_unit_err)] // IMPROVE

mod helpers;
use helpers::{
    check_fat_pointer, decompose_pointer, list_push_gen, make_fat_ptr, round_to_words,
    store_metadata,
};

#[cfg(test)]
mod tests;

mod buffer;
mod queue;
mod stack;
mod value;

pub use buffer::*;
pub use queue::{DstQueue, DstQueueIter, DstQueueIterMut, DstQueuePopHandle, DstQueueUsize};
pub use stack::{DstStack, DstStackIter, DstStackIterMut, DstStackUsize};
pub use value::{DstValue, DstValueUsize};
