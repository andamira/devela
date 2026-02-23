// devela_base_core::data::access::iter::strided
//
//!
//

// #[cfg(test)]
// mod tests;

mod core; // (StridedIterCore)

mod r#mut; // StridedIterMut
mod r#ref; // StridedIterMut

// mod define; // strided_iter! WIP

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            r#mut::*,
            r#ref::*,
            // define::*,
        };
    }
    _crate_internals {
        pub(crate) use super::core::*;
    }
}
