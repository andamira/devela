// devela_base_core::num::float::wrapper
//
//! Floating-point wrapper struct.
//

mod definition; // Float

mod basic; // basic operations
mod consts; // constants
mod series; // series-based functions

mod no_std; // no_std methods

#[cfg(test)]
mod tests_f32;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::Float,
        };
    }
}
