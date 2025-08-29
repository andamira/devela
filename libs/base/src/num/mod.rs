// devela_base::num
//
#![doc = crate::_DOC_NUM!()]
//

mod cast; // Cast
mod float;
mod int;
mod ord;

pub mod error; // error types
pub mod niche; // NonZero*
pub mod quant; // Cycle*, Interval, interval!, Ratio

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{cast::_all::*, float::_all::*, int::_all::*, ord::_all::*};
    }
    mod _pub_mods {
        #[doc(inline)]
        pub use super::{error::*, niche::_all::*, quant::_all::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
