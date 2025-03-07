// devela::data::list:array::d1::uninit
//
//!
//

use crate::{Bare, MaybeUninit, Storage};

mod methods;

#[doc = crate::TAG_DATA_STRUCTURE!()]
/// A static array allowing uninitialized elements.
#[derive(Default)]
pub struct ArrayUninit<T, const CAP: usize, S: Storage = Bare> {
    data: S::Stored<[MaybeUninit<T>; CAP]>,

    // The number of already initialized elements, and
    // the index of the first uninitialized element (if CAP > 0).
    init_len: usize,
}
