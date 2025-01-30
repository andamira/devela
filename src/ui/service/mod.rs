// revela::ui::service
//
//! Ui services.
//

mod cap;
mod definition;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        #[cfg(ui··)]
        pub use super::{cap::*, definition::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
