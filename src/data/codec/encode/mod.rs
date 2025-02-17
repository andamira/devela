// devela::data::codec::encode
//
//!
//

mod impls;

mod combinators; // Encode*
mod traits; // Encodable, EncodableLen

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
