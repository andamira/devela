// devela/src/num/quant/scale/_.rs
//
#![doc = crate::_DOC_NUM_QUANT_SCALE!()] // private
#![doc = crate::_doc!(modules: crate::num::quant; scale)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod namespace; // Scale
    // mod composition; // Vernier…
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            namespace::Scale,
        };
    }
}
