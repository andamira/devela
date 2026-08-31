// devela/src/data/codec/symbol/ean/_.rs
//
//! European Article Number barcodes.
//

crate::mods_in! {
    mod _helper;

    mod define; // Ean

    mod eight; // impl EAN-8
    mod thirteen; // impl EAN-13
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Ean,
        };
    }
}
