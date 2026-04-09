// devela::sys::mem::view::slice
//
#![doc = crate::_DOC_SYS_MEM_VIEW_SLICE!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; slice)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: slice)]

#[cfg(test)]
mod tests;

mod ext; // SliceExt
mod iter; // SliceIter. SliceIterMut
mod join; // const_join!
mod namespace; // Slice, slice!

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            ext::*,
            iter::*,
            join::*,
            namespace::*,
        };
    }
}
