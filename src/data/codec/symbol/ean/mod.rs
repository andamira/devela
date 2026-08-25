// devela/src/data/codec/symbol/ean/mod.rs
//
//! European Article Number barcodes.
//

mod eight; // Ean8
// mod thirteen; // Ean13

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            eight::Ean8,
            // thirteen::Ean13,
        };
    }
}
