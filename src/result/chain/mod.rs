// devela::result::chain
//
//! Types and macros related to chaining results.
//

/* modules */

// always compiled, non-public
mod macros;
mod own;
mod traits;

/* re-exports */

// always compiled, non-public
pub use {macros::*, own::*, traits::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{macros::*, own::*, traits::*};
}
