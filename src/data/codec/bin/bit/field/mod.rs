// devela::data::codec::bit::bin::field

#[cfg(any(test, doctest))]
mod tests;

#[cfg(any(test, feature = "_docs_examples"))]
mod examples; // BitfieldExample

mod definition; // bitfield!

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            definition::bitfield,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::examples::*;
    }
}
