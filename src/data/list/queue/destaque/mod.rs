// devela::data::list::queue::destaque
//
//! A type that can be used as a double-ended queue and a double-ended stack.
//

mod impl_traits;
mod methods;
#[cfg(all(test, feature = "_destaque_u8"))]
mod tests;

mod definitions; // Destaque, DestaqueIter, …

crate::structural_mods! { // _mods
    _mods {
        pub use super::definitions::*;
    }
}
