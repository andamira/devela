// devela::ops
//
//! Operators, extends [`std::ops`].
//

/* always compiled for internal use */

mod chain;
#[allow(unused)]
#[cfg(all(not(feature = "ops"), not(test)))]
pub(crate) use chain::*;
#[cfg(test)]
pub use chain::*;

/* only compiled with the `ops` feature */

/* re-exports */

#[cfg(feature = "ops")]
pub use all::*;
#[cfg(feature = "ops")]
pub(crate) mod all {
    pub use super::chain::*;
}
