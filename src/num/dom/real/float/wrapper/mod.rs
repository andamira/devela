// devela::num::dom::real::float::wrapper
//
//! Floating-point wrapper struct.
//

mod definition; // Float

mod basic; // basic operations SYMLINK from /crates/base/std/src/num/dom/real/float/wrapper/basic.rs
mod consts; // constants       SYMLINK from /crates/base/std/src/num/dom/real/float/wrapper/consts.rs
mod minimax; // Horner-rel fns SYMLINK from /crates/base/std/src/num/dom/real/float/wrapper/minimax.rs
mod series; // Taylor-rel fns  SYMLINK from /crates/base/std/src/num/dom/real/float/wrapper/series.rs

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
