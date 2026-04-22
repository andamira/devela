// devela::sys::mem::cell::hedge
//
//! Defines [`MemHedgeCtrl`].
//

use crate::is;
use crate::{AtomicOrdering, AtomicU8, AtomicUsize, CacheAlign};
use crate::{MemHedgeError, MemHedgeState};

#[doc = crate::_tags!(mem concurrency atomic)]
/// Shared control plane for one hedged read request at a time.
#[doc = crate::_doc_location!("sys/mem/cell")]
///
/// `MemHedgeCtrl` does not own data and does not spawn workers.
/// It only coordinates a request lifecycle:
///
/// - a caller arms a logical index,
/// - workers observe the new epoch,
/// - one worker claims victory,
/// - the caller clears the control state for reuse.
///
/// The hot atomic fields are cache-line aligned to reduce false sharing.
#[derive(Debug)]
pub struct MemHedgeCtrl {
    epoch: CacheAlign<AtomicUsize>,
    logical_index: CacheAlign<AtomicUsize>,
    winner: CacheAlign<AtomicUsize>,
    state: CacheAlign<AtomicU8>,
}

impl MemHedgeCtrl {
    /// Sentinel value meaning that no replica has won yet.
    pub const NONE_WINNER: usize = usize::MAX;

    /// Creates a fresh idle hedge-control object.
    pub const fn new() -> Self {
        Self {
            epoch: CacheAlign::new(AtomicUsize::new(0)),
            logical_index: CacheAlign::new(AtomicUsize::new(0)),
            winner: CacheAlign::new(AtomicUsize::new(Self::NONE_WINNER)),
            state: CacheAlign::new(AtomicU8::new(MemHedgeState::Idle as u8)),
        }
    }

    /// Returns the current request epoch.
    ///
    /// Each successful call to [`arm`](Self::arm) increments this value.
    /// Workers can poll this to detect that a new request has been published.
    pub fn epoch(&self) -> usize {
        self.epoch.load(AtomicOrdering::Acquire)
    }

    /// Returns the currently armed logical index.
    ///
    /// This value is meaningful after a successful [`arm`](Self::arm).
    pub fn logical_index(&self) -> usize {
        self.logical_index.load(AtomicOrdering::Acquire)
    }

    /// Returns the current lifecycle state.
    pub fn state(&self) -> MemHedgeState {
        let raw = self.state.load(AtomicOrdering::Acquire);
        if let Some(state) = MemHedgeState::from_u8(raw) {
            state
        } else {
            debug_assert!(false, "invalid MemHedgeState value: {raw}");
            MemHedgeState::Idle
        }
    }

    /// Returns the winning replica index, if any.
    pub fn winner(&self) -> Option<usize> {
        let winner = self.winner.load(AtomicOrdering::Acquire);
        is! { winner == Self::NONE_WINNER, None, Some(winner) }
    }

    /// Arms a new hedged read request for `logical_index`.
    ///
    /// This succeeds only from the idle state.
    ///
    /// On success:
    /// - the logical index is published,
    /// - the winner is reset,
    /// - the state becomes [`Armed`](MemHedgeState::Armed),
    /// - and the epoch is incremented.
    ///
    /// The returned value is the new epoch.
    ///
    /// # Errors
    /// Returns [`MemHedgeError::Busy`] if the control object is not idle.
    pub fn arm(&self, logical_index: usize) -> Result<usize, MemHedgeError> {
        let idle = MemHedgeState::Idle.as_u8();
        let armed = MemHedgeState::Armed.as_u8();

        self.state
            .compare_exchange(idle, armed, AtomicOrdering::AcqRel, AtomicOrdering::Acquire)
            .map_err(|_| MemHedgeError::Busy)?;

        self.logical_index.store(logical_index, AtomicOrdering::Relaxed);
        self.winner.store(Self::NONE_WINNER, AtomicOrdering::Relaxed);

        Ok(self.epoch.fetch_add(1, AtomicOrdering::Release) + 1)
    }

    /// Clears the control object back to the idle state.
    ///
    /// This resets:
    /// - the logical index to `0`,
    /// - the winner to [`NONE_WINNER`](Self::NONE_WINNER),
    /// - and the state to [`Idle`](MemHedgeState::Idle).
    ///
    /// The epoch is left unchanged.
    pub fn clear(&self) {
        self.logical_index.store(0, AtomicOrdering::Relaxed);
        self.winner.store(Self::NONE_WINNER, AtomicOrdering::Relaxed);
        self.state.store(MemHedgeState::Idle.as_u8(), AtomicOrdering::Release);
    }

    /// Attempts to claim the currently armed request for `replica`.
    ///
    /// Returns:
    /// - `Ok(true)` if this replica became the winner,
    /// - `Ok(false)` if another replica had already won,
    /// - `Err(NotArmed)` if no armed request is active.
    ///
    /// On success, the state transitions to [`Claimed`](MemHedgeState::Claimed).
    pub fn try_claim(&self, replica: usize) -> Result<bool, MemHedgeError> {
        is! { self.state() != MemHedgeState::Armed, return Err(MemHedgeError::NotArmed) }
        match self.winner.compare_exchange(
            Self::NONE_WINNER,
            replica,
            AtomicOrdering::AcqRel,
            AtomicOrdering::Acquire,
        ) {
            Ok(_) => {
                self.state.store(MemHedgeState::Claimed.as_u8(), AtomicOrdering::Release);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
}

impl Default for MemHedgeCtrl {
    fn default() -> Self {
        Self::new()
    }
}
