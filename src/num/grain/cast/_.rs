// devela/src/num/grain/cast/_.rs
//
//! Helpers for converting between primitives.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod macros; // cast!
    mod_ namespace; // Cast
    mod_ traits; // PrimCast, PrimJoin, PrimSplit
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            macros::cast,
            namespace::_all::Cast,
            traits::_all::*,
        };
    }
}
