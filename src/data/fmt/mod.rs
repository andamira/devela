// devela::data::serde
//
//! Data serialization and deserialization.
//

mod types;
#[allow(unused_imports)]
pub use types::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::types::*;
}
