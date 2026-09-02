// devela/src/lang/prog/script/form/shell/_.rs
//
#![doc = crate::_DOC_LANG_PROG_SCRIPT_FORM_SHELL!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog::script::form; shell)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod cmd; // command phrase representation
    // mod expand; // variables, globbing, command substitution
    // mod posix; //
    mod_ word; // Shell word parsing and quoting.
}
crate::mods_out! { // _mods
    _mods {
        pub use super:: {
            // cmd::*,
            // expand::*,
            // posix::*,
            word::_all::*,
        };
    }
}
