// devela/src/num/fin/bit/_.rs
//
#![doc = crate::_DOC_NUM_FIN_BIT!()] // private
#![doc = crate::_doc!(modules: crate::num; bit)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    // mod _benches;
    mod _docs;

    mod ops; // BitOps
    mod span; // BitSpan
    mod_ wise; // Bitwise
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            ops::*,
            span::*,
            wise::_all::*,
        };
    }
}
