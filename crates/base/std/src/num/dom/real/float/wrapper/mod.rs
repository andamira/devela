// devela_base_std::num::dom::real::float::wrapper
//
//! Floating-point wrapper struct (std version).
//

// NOTE: reimplements core's Float with additional methods
mod definition; // FloatStd

mod basic; // basic operations SYMLINK to /crates/base/core/src/num/dom/real/float/wrapper/basic.rs
mod consts; // constants       SYMLINK to /crates/base/core/src/num/dom/real/float/wrapper/consts.rs
mod minimax; // Horner-rel fns SYMLINK to /crates/base/core/src/num/dom/real/float/wrapper/minimax.rs
mod series; // Taylor-rel fns  SYMLINK to /crates/base/core/src/num/dom/real/float/wrapper/series.rs

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
