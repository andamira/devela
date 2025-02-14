// devela::code::util::asserts::static
//
//! Static assertions.
//

mod r#const;
mod r#impl;

crate::items! { // structural access: _mods, _all
    #[allow(unused_imports)]
    pub use _mods::*;

    mod _mods { #[allow(unused)]
        pub use super::{r#const::*, r#impl::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
