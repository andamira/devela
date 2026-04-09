// devela::num::dom::real::float::wrapper
//
//! Floating-point wrapper struct.
//

mod definition; // Float

mod basic; // basic operations
mod consts; // constants
mod minimax; // Horner-rel fns
mod series; // Taylor-rel fns

crate::cfg_if! { if #[cfg(feature = "std")] {
    mod std;
} else {
    mod no_std; // no_std methods
}}

#[cfg(test)]
mod tests_f32;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::Float,
        };
    }
}
