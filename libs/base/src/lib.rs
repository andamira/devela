// devela_base::lib
//
//!
//

// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]

/* imports */

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate self as devela_base;

/* root modules */

mod code;
// mod num;

#[doc(hidden)]
pub use all::*;
pub mod all {
    // public items, feature-gated, visible at their origin and here in `all`
    //
    //! All the crate's items flat re-exported.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module's contents from their parent.
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        code::_all::*,
        // num::_all::*,
    };
}
