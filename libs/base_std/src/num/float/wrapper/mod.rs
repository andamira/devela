// devela_base_std::num::float::wrapper
//
//! Floating-point wrapper struct (std version).
//

mod definition; // FloatStd

mod basic; // basic operations
mod consts; // constants
mod series; // series-based functions

mod std; // std methods

// #[cfg(test)]
// mod tests_f32;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::Float as FloatStd,
        };
    }
}
