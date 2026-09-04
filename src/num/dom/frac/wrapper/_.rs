// devela/src/num/dom/frac/wrapper/_.rs
//
//! Fraction-related wrapper struct.
//

crate::mods_in! {
    mod define; // Frac
    mod impl_frac;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::Frac;
    }
}
