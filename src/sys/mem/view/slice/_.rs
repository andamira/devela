// devela/src/sys/mem/view/slice/_.rs
//
#![doc = crate::_DOC_SYS_MEM_VIEW_SLICE!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; slice)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: slice)]

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod ext; // SliceExt
    mod iter; // SliceIter. SliceIterMut
    mod join; // const_join!
    mod_ namespace; // Slice, slice!
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            ext::{SliceExt, SliceExtMut},
            iter::{SliceIter, SliceIterMut},
            join::const_join,
            namespace::{Slice, slice},
        };
    }
}
