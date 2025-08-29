// devela_base::code::util::asserts
//
//! Asserts.
//

mod dynamic; // assert_eq_all, assert_approx_eq_all
// mod r#static;

crate::items! { // structural access: _mods, _all
    #[allow(unused_imports)]
    pub use _mods::*;

    mod _mods {
        pub use super::{
            dynamic::*,
            // r#static::_all::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
