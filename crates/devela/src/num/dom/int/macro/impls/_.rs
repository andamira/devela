//
//! Macro emitters for individual integer operation families.
//

crate::mods_in! {
    mod base; // __int_impl_base!
}
crate::mods_out! { // _hidden
    _hidden {
        #[doc(hidden)]
        pub use super::base::__int_impl_base;
    }
}
