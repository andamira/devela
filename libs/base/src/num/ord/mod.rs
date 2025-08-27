// devela_base::num::ord
//
#![doc = crate::_DOC_NUM_ORD!()]
//

mod compare; // `Compare`
mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{compare::*, reexports::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{compare::*, _mods::*};
    }
}
