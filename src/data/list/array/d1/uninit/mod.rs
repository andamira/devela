// devela::data::list:array::d1::uninit
//
//!
//

use crate::{Bare, MaybeUninit, Storage};

// mod impl_traits; // TODO: FIXME
mod methods;

#[doc = crate::_tags!(data_structure)]
/// A static array allowing uninitialized elements.
#[doc = crate::_doc_location!("data/list/array")]
#[derive(Debug, Default)]
pub struct ArrayUninit<T, const CAP: usize, S: Storage = Bare> {
    data: S::Stored<[MaybeUninit<T>; CAP]>,

    // The number of already initialized elements, and
    // the index of the first uninitialized element (if CAP > 0).
    init_len: usize,
}
