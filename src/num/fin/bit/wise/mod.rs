// devela::num::fin::bit::wise
//
//! Defines the [`Bitwise`] namespace.
//

#[cfg(test)]
mod tests;

mod definition; // Bitwise
mod impls;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::*,
        };
    }
}
