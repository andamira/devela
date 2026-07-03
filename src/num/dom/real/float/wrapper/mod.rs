// devela/src/num/dom/real/float/wrapper/mod.rs
//
//! Floating-point wrapper struct.
//

#[cfg(test)]
mod _test_f32;

mod define; // Float

// impls
mod basic; // basic operations
mod consts; // constants
mod minimax; // Horner-rel fns
mod series; // Taylor-rel fns

cfg_select! {
    feature = "std" => { mod std; }
                  _ => { mod no_std; } // no_std methods
}

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::Float,
        };
    }
}
