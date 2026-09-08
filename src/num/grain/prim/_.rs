// devela/src/num/grain/prim/_.rs
//
#![doc = crate::_DOC_NUM_GRAIN_PRIM!()]
#![doc = crate::_doc!(modules: crate::num::grain; prim)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod_ cast; // Casting between primitives.
    // mod family; // TODO Finite tagged descriptions/values of primitive families
    // mod macros; // FUTURE primatch!
    mod repr; // Primitive-backed integer representation trait
    mod scalar; // Primitive scalar classification traits
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            cast::_all::{Cast, PrimCast, PrimJoin, PrimSplit, cast},
            // family::{PrimSintKind, PrimSintValue, PrimUintKind, PrimUintValue},
            repr::{ReprIndex, ReprInt, ReprSint, ReprUint},
            scalar::{PrimScalar, PrimInt, PrimSint, PrimUint, PrimFloat, PrimIndex},
        };
    }
}
