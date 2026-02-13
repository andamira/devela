// devela_base_core::code::ops::call
//
//! Function dispatch.
//

mod semantics; // Call[Semantics|BindTime|Context|Dispatch|Openness|Storage]

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            semantics::*,
        };
    }
}
