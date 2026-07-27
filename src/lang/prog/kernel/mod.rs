// devela/src/lang/prog/kernel/mod.rs
//
#![doc = crate::_DOC_LANG_PROG_KERNEL!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog; kernel)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

// mod array; // Array and tacit programming (APL/J/K/BQN/Uiua)
// mod concat; // Concatenative program composition (Forth/False/Joy/Factor)
// mod rule; // Rule-based pattern-action programming (AWK/Make/Prolog)
// mod stack; // Stack-machine semantics and composition kernels
// mod term; // Symbolic term programming kernels (Lisp/Scheme/Racket)

crate::structural_mods! { // _mods
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
