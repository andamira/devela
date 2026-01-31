// revela::ui::front
//
#![doc = crate::_DOC_UI_FRONT!()] // public
#![doc = crate::_doc!(modules: crate::ui; front: term)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

pub mod term;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::term::_all::*;
    }
}
