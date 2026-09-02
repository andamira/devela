// devela/src/text/str/ext/_.rs
//
//! Defines [`StrExt`], [`StringExt`].
//

crate::mods_in! {
    #[cfg(feature = "alloc")]
    mod alloc; // StringExt
    mod slice; // StrExt
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            slice::StrExt,
        };
        #[cfg(feature = "alloc")]
        pub use super::alloc::StringExt;
    }
}
