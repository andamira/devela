// devela::sys::mem::cell::hedge
//
//! Defines [`MemHedgeState`].
//

#[doc = crate::_tags!(mem concurrency atomic)]
/// Lifecycle state of a hedged read request.
#[doc = crate::_doc_location!("sys/mem/cell")]
///
/// The control object begins in [`Idle`](Self::Idle).
/// A caller then [`arm`s][crate::MemHedgeCtrl::arm] it with a logical index,
/// making it [`Armed`](Self::Armed).
/// The first worker that successfully [`try_claim`s][crate::MemHedgeCtrl::try_claim]
/// the request transitions it to [`Claimed`](Self::Claimed).
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MemHedgeState {
    /// No request is currently active.
    #[default]
    Idle = 0,
    /// A request is active and waiting for a winning replica.
    Armed = 1,
    /// A replica has already claimed the current request.
    Claimed = 2,
}
impl MemHedgeState {
    /// Returns the compact integer representation of this state.
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Decodes a compact integer representation into a hedge state.
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Idle),
            1 => Some(Self::Armed),
            2 => Some(Self::Claimed),
            _ => None,
        }
    }
}
