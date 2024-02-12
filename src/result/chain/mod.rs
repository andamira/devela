// devela::result::chain
//
//! Types and macros related to chaining results.
//

/* modules */

// always compiled, non-public
mod own;
mod traits;
mod unwrap;

/* re-exports */

// always compiled, non-public
pub use {own::*, traits::*, unwrap::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{own::*, traits::*, unwrap::*};
}
