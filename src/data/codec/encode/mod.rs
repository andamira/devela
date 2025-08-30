// devela::data::codec::encode
//
//!
//

mod impls;
#[cfg(test)]
mod tests;

mod combinators; // Codec*
mod traits; // Decodable, Encodable, EncodableLen

crate::structural_mods! { // _mods
    _mods {
        pub use super::{combinators::*, traits::*};
    }
}
