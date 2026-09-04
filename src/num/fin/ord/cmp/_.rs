// devela/src/num/fin/ord/cmp/_.rs
//
//! Items to help comparing.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // Cmp
    mod macros; // cmp!
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            define::Cmp,
            macros::cmp,
        };
    }
}
