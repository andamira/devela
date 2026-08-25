// devela/src/data/codec/symbol/ean/mod.rs
//
//! European Article Number barcodes.
//

mod _helper;

mod define; // Ean

mod eight; // impl EAN-8
mod thirteen; // impl EAN-13

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::Ean,
        };
    }
}
