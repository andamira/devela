// devela::num::niche
//
//! Numeric types with niche memory layout optimization.
//

mod non_value;
mod reexports;
#[allow(unused_imports)]
pub use {non_value::*, reexports::*};

#[cfg(all(feature = "num_niche_range", feature = "_i8", test))]
mod tests;
//
#[cfg(feature = "num_niche_range")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_niche_range")))]
mod non_range;
#[cfg(feature = "num_niche_range")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_niche_range")))]
mod range;
#[allow(unused_imports)]
#[cfg(feature = "num_niche_range")]
pub use {non_range::*, range::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{non_value::*, reexports::*};

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "num_niche_range")]
    pub use super::{non_range::*, range::*};
}
