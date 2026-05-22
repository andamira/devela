// devela::data::codec::bit::set

#[cfg(any(test, doctest))]
mod tests;

mod definition; // set!

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            definition::set,
        };
    }
}
