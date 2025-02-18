// devela::data::codec::encode
//
//!
//

mod impls;
#[cfg(test)]
mod tests;

mod combinators; // Codec*
mod traits; // Decodable, Encodable, EncodableLen

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{combinators::*, traits::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
