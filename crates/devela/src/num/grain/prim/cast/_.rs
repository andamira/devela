//
//! Casting between primitives.
//

crate::mods_in! {
    #[cfg(test)]
    #[cfg(feature = "cast")]
    mod _test;

    mod macros; // cast!
    mod_ namespace; // Cast
    #[cfg(feature = "cast")]
    mod_ traits; // PrimCast, PrimJoin, PrimSplit
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            macros::cast,
            namespace::_all::Cast,
        };
        #[cfg(feature = "cast")]
        #[doc(inline)]
        pub use super::traits::_all::*;
    }
}
