// devela/src/sys/mem/size/bit/mod.rs
//
//! Functionality related to memory bit size.
//

#[cfg(test)]
mod _test;

mod define; // BitSized
mod impls;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            define::*,
        };
    }
}
