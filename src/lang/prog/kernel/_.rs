// devela/src/lang/prog/kernel/_.rs
//
#![doc = crate::_DOC_LANG_PROG_KERNEL!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog; kernel)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ array; // Array and tacit programming (APL/J/K/BQN/Uiua)
    // mod_ concat; // Concatenative program composition (Forth/False/Joy/Factor)
    // mod_ rule; // Rule-based pattern-action programming (AWK/Make/Prolog)
    // mod_ stack; // Stack-machine semantics and composition kernels
    // mod_ term; // Symbolic term programming kernels (Lisp/Scheme/Racket)
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        //     array::_all::*,
        //     concat::_all::*,
        //     rule::_all::*,
        //     stack::_all::*,
        //     term::_all::*,
        // };
    }
}
