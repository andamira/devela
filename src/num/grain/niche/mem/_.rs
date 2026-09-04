// devela/src/num/grain/niche/mem/_.rs
//
//! Numeric types for memory-efficient representations.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod non_value; // NonValue*, NonMax*, NonMin*
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            non_value::*,
        };
    }
}
