// devela_base_std::num::float::wrapper
//
//! Floating-point wrapper struct (std version).
//

mod definition; // FloatStd

mod basic; // [SYMLINK:base_core] basic operations
mod consts; // [SYMLINK:base_core] constants
mod series; // [SYMLINK:base_core] series-based functions

mod std; // std methods

#[cfg(test)]
mod tests_f32;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::Float as FloatStd,
        };
    }
}
