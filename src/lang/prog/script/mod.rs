// devela/src/lang/prog/script/mod.rs
//
#![doc = crate::_DOC_LANG_PROG_SCRIPT!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog; script: shell)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

// pub mod apl;
// pub mod awk;
// pub mod cmd;
// pub mod false;
// pub mod forth;
// pub mod j;
// pub mod lisp;
// pub mod php;
#[cfg(feature = "shell")]
pub mod shell; // Shell command words and quoting

crate::structural_mods! { // _pub_mods
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
