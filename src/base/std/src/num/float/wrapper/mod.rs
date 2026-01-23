// devela_base_std::num::float::wrapper
//
//! Floating-point wrapper struct (std version).
//

mod definition; // FloatStd

mod basic; // basic operations           SYMLINK to /src/base/core/src/num/float/wrapper/basic.rs
mod consts; // constants                 SYMLINK to /src/base/core/src/num/float/wrapper/consts.rs
mod minimax; // Horner minimax-based fns SYMLINK to /src/base/core/src/num/float/wrapper/minimax.rs
mod series; // Taylor series-based fns   SYMLINK to /src/base/core/src/num/float/wrapper/series.rs

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
