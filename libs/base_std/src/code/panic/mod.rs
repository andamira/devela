// devela_base_std::code::panic
//
#![doc = crate::_DOC_CODE_PANIC!()]
//

mod _reexport;

mod namespace; // Panic

crate::structural_mods! { // _mods, _reexports
    _mods { pub use super::namespace::*; }
    _reexports { pub use super::_reexport::*; }
}
