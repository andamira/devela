// devela::sys::mem::cell::hedge::error
//
//! Defines [`MemHedgeError`].
//

#[doc = crate::_tags!(mem concurrency error)]
/// Errors produced by hedged control-state operations.
#[doc = crate::_doc_location!("sys/mem/cell")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MemHedgeError {
    /// The control object is not idle and cannot be armed again yet.
    #[default]
    Busy,
    /// No armed request is currently available to claim.
    NotArmed,
    /// The requested logical index lies outside the readable logical length.
    OutOfBounds,
    /// The requested replica index is not in `0..N`.
    InvalidReplica,
}
