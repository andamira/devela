// devela_base::text
//
//! Text types and operations, text processing.
// #![doc = crate::doc_!(modules: crate; text: fmt, parse, str)]
// #![doc = crate::doc_!(newline)]
// //!
// #![doc = crate::doc_!(extends: ascii, char, fmt, str, string)]
//
// safety
// #![cfg_attr(feature = "safe_text", forbid(unsafe_code))]

pub mod fmt;
pub mod str;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            fmt::_all::*, str::_all::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
