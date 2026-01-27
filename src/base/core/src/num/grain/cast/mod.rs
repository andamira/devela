// devela_base_core::num::grain::cast
//
#![doc = crate::_DOC_NUM_GRAIN_CAST!()]
//

mod namespace; // Cast
mod cast;
mod join;
mod split;

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
