// devela_base_core::data::access::iter::strided
//
//!
//

#[cfg(test)]
mod tests;

mod canonical; // StridedIter, StridedIterMut
mod define; // strided_iter!

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            canonical::*,
            define::*,
        };
    }
}
