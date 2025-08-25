// devela_base::num
//
#![doc = crate::_DOC_NUM!()]
//

// mod cast; // Cast
mod float;
mod int;
mod sign; // Sign

pub mod error; // error types

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods {
        pub use super::{
            error::*,
        };
    }
    mod _mods { #![allow(unused)]
        pub use super::{float::_all::*, int::_all::*, sign::*};
        // pub use super::cast::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
