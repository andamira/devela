// devela::num::frac
//
//! Fractional functionality.
//

#[cfg(feature = "int")]
mod wrapper;

// WIPZONE
// mod r#trait;

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "int")]
        pub use super::wrapper::*;

        // WIPZONE
        // pub use super::r#trait::*;
    }
}
