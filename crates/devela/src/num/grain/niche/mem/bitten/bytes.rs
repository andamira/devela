//
//!
//

use crate::Hash;

/// Shared representation selection.
pub(crate) trait _BittenSpec {
    type Repr: Clone + Copy + Eq + Hash;
}

/// A compact integer representation consisting of
/// complete low bytes and one upper boundary value.
///
/// The boundary type determines whether the representation
/// has unused bit patterns that can provide a niche.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct _BittenBytes<const N: usize, Tail> {
    pub(crate) low: [u8; N],
    pub(crate) high: Tail,
}

#[cfg(feature = "bitten")]
impl<const N: usize, Tail> _BittenBytes<N, Tail> {
    pub(crate) const fn new(low: [u8; N], high: Tail) -> Self {
        Self { low, high }
    }
}
