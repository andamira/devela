// devela_base_core::num::cast
//
//! Casting between primitives.
//

mod namespace; // Cast
mod cast;
mod join;
mod split;

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods, _all
    _mods { pub use super::namespace::*; }
}
