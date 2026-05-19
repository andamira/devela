// devela::lang::prog::script::shell
//
#![doc = crate::_DOC_LANG_PROG_SCRIPT_SHELL!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog::script; shell)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

// mod cmd; // command phrase representation
// mod expand; // variables, globbing, command substitution
// mod posix; //
mod word; // Shell word parsing and quoting.

crate::structural_mods! { // _mods
    _mods {
        pub use super:: {
            // cmd::*,
            // expand::*,
            // posix::*,
            word::_all::*,
        };
    }
}
