// devela::data::collections:array::d1::uninit
//
//!
//

use crate::{
    data::Array,
    mem::{Bare, MaybeUninit, Storage},
};

mod methods;

/// A static array allowing uninitialized elements.
#[derive(Default)]
pub struct UninitArray<T, const CAP: usize, S: Storage = Bare> {
    data: S::Stored<[MaybeUninit<T>; CAP]>,

    // The number of already initialized elements, and
    // the index of the first uninitialized element (if CAP > 0).
    init_len: usize,
}
