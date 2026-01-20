// devela::num::frac
//
//! Fractional functionality.
//

#[cfg(feature = "devela_base_num")]
mod wrapper;

// WIPZONE
// mod r#trait;

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "devela_base_num")]
        pub use super::wrapper::*;

        // WIPZONE
        // pub use super::r#trait::*;
    }
}
