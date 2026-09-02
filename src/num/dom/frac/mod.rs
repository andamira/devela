// devela/src/num/dom/frac/mod.rs
//
//! Fractional functionality.
//

#[cfg(feature = "int")]
mod wrapper;

// mod r#trait;

crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "int")]
        pub use super::wrapper::*;

        // pub use super::r#trait::*;
    }
}
