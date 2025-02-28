// devela::data::key::static_map
//
//! Defines the [`define_static_map!`] macro and the [`StaticEntry`] enum.
//

#[cfg(test)]
mod tests;

mod define; // define_static_map!
mod entry; // StaticEntry

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{define::*, entry::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
