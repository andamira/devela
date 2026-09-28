//
//! Spectrum 48K.
//

crate::mods_in! {
    mod namespace;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            namespace::ComputerSpectrum48,
        };
    }
}
