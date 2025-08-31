// devela_base::num
//
#![doc = crate::_DOC_NUM!()]
//

mod cast; // Cast
mod float;
mod int;
mod logic; // ConstBool, False, True, const_bool!
mod ord;

pub mod error; // error types
pub mod niche; // NonZero*
pub mod quant; // Cycle*, Interval, interval!, Ratio

crate::structural_mods! { //_mods, _pub_mods
    _mods {
        pub use super::{
            cast::_all::*, float::_all::*, int::_all::*, logic::_all::*, ord::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::error::*;
        pub use super::{niche::_all::*, quant::_all::*};
    }
}
