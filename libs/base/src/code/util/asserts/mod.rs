// devela_base::code::util::asserts
//
//! Asserts.
//

mod dynamic; // assert_eq_all, assert_approx_eq_all
mod r#static;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            dynamic::*,
            r#static::_all::*,
        };
    }
}
