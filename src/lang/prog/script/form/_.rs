// devela/src/lang/prog/script/form/mod.rs
//
#![doc = crate::_DOC_LANG_PROG_SCRIPT_FORM!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog::script; form: shell)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // pub mod_ apl;
    // pub mod_ awk;
    // pub mod_ cmd;
    // pub mod_ false;
    // pub mod_ forth;
    // pub mod_ j;
    // pub mod_ lisp;
    // pub mod_ php;
    #[cfg(feature = "shell")]
    pub mod_ shell; // Shell command words and quoting
}
crate::mods_out! { // _pub_mods
    _pub_mods {
        // pub use super::{
        //     // apl::_all::*,
        //     // awk::_all::*,
        //     // cmd::_all::*,
        //     // false::_all::*,
        //     // forth::_all::*,
        //     // j::_all::*,
        //     // lisp::_all::*,
        //     // php::_all::*,
        // };
        #[cfg(feature = "shell")]
        pub use super::shell::_all::*;
    }
}
