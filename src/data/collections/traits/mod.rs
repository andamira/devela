// devela::data::collections::traits
//
//! Abstract data types
//

mod array;
mod collection;
mod queues;
mod stacks;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{array::*, collection::*, queues::*, stacks::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
