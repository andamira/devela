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

crate::structural_mods! { //_mods, _pub_mods
    _mods {
        pub use super::{cast::_all::*, float::_all::*, int::_all::*, ord::_all::*};
    }
    _pub_mods {
        pub use super::{error::*, niche::_all::*, quant::_all::*};
    }
}
