// devela_base::num::cast
//
//! Casting between primitives.
//

mod namespace; // Cast
mod cast;
mod join;
mod split;

#[cfg(test)]
mod tests;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::namespace::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
