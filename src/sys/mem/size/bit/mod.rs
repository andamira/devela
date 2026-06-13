// devela/src/sys/mem/size/bit/mod.rs
//
//! Functionality related to memory bit size.
//

#[cfg(test)]
mod tests;

mod definition; // BitSized
mod impls;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            definition::*,
        };
    }
}
