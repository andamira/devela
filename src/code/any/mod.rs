// devela::code::any
//
#![doc = crate::_DOC_CODE_ANY!()]
// #![doc = crate::_doc!(extends: any)]
// #![doc = crate::_doc!(modules: crate::code; any)]
// #![doc = crate::_doc!(newline)]
//

crate::mod_path!(_c "../../../libs/base_core/src/code/any/reexports.rs");

mod ext;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{ext::*, _c::*};
    }
    _always {
        pub use super::{ext::*, _c::*};
    }
}
