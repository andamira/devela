// devela::sys::mem::cell::hedge::read
//
//! Defines [`MemHedgeRead`].
//

use crate::is;
use crate::{MemHedgeCtrl, MemHedgeError, MemHedgeState, MemReplicaSlice};

#[doc = crate::_tags!(mem concurrency lifetime runtime)]
/// Binds a hedge-control cell to a replicated logical slice view.
#[doc = crate::_doc_location!("sys/mem/cell")]
///
/// `MemHedgeRead` does not spawn workers or perform any platform-specific work.
/// It only provides the protocol-facing surface that connects:
///
/// - [`MemReplicaSlice`] as the replicated data source,
/// - [`MemHedgeCtrl`] as the shared coordination state.
///
/// This is the first layer where the hedged-read pattern becomes explicit.
#[derive(Debug, Clone, Copy)]
pub struct MemHedgeRead<'a, 's, T, const N: usize> {
    ctrl: &'a MemHedgeCtrl,
    replicas: &'a MemReplicaSlice<'s, T, N>,
}

impl<'a, 's, T, const N: usize> MemHedgeRead<'a, 's, T, N> {
    /// Creates a hedge-read façade over `ctrl` and `replicas`.
    pub const fn new(ctrl: &'a MemHedgeCtrl, replicas: &'a MemReplicaSlice<'s, T, N>) -> Self {
        Self { ctrl, replicas }
    }

    /// Returns the underlying hedge-control cell.
    pub const fn ctrl(&self) -> &'a MemHedgeCtrl {
        self.ctrl
    }

    /// Returns the underlying replicated slice view.
    pub const fn replicas(&self) -> &'a MemReplicaSlice<'s, T, N> {
        self.replicas
    }

    /// Arms a hedged read request for `logical_index`.
    ///
    /// This validates that the requested index lies within the initialized
    /// logical length of the replicated view before delegating to
    /// [`MemHedgeCtrl::arm`].
    ///
    /// # Errors
    /// Returns:
    /// - [`MemHedgeError::OutOfBounds`] if `logical_index >= replicas.len()`.
    /// - any error returned by [`MemHedgeCtrl::arm`].
    pub fn arm(&self, logical_index: usize) -> Result<usize, MemHedgeError> {
        is! { logical_index >= self.replicas.len(), return Err(MemHedgeError::OutOfBounds) }
        self.ctrl.arm(logical_index)
    }

    /// Clears the current request state.
    ///
    /// This delegates to [`MemHedgeCtrl::clear`].
    pub fn clear(&self) {
        self.ctrl.clear();
    }

    /// Returns the current request epoch.
    pub fn epoch(&self) -> usize {
        self.ctrl.epoch()
    }

    /// Returns the currently requested logical index.
    pub fn logical_index(&self) -> usize {
        self.ctrl.logical_index()
    }

    /// Returns the current hedge lifecycle state.
    pub fn state(&self) -> MemHedgeState {
        self.ctrl.state()
    }

    /// Returns the winning replica index, if any.
    pub fn winner(&self) -> Option<usize> {
        self.ctrl.winner()
    }

    /// Reads the first replica for the currently active request.
    ///
    /// This is a convenience alias for [`read_replica`](Self::read_replica)
    /// with `replica = 0`.
    ///
    /// # Errors
    /// Returns:
    /// - [`MemHedgeError::NotArmed`] if no request is active,
    /// - [`MemHedgeError::OutOfBounds`] if the active logical index is not
    ///   readable in the bound replica view.
    pub fn read(&self) -> Result<&T, MemHedgeError> {
        self.read_replica(0)
    }

    /// Reads one replica for the currently active request.
    ///
    /// This uses the logical index published in the bound [`MemHedgeCtrl`].
    ///
    /// # Errors
    /// Returns:
    /// - [`MemHedgeError::NotArmed`] if no request is active,
    /// - [`MemHedgeError::InvalidReplica`] if `replica >= N`,
    /// - [`MemHedgeError::OutOfBounds`] if the active logical index is not
    ///   readable in the bound replica view.
    pub fn read_replica(&self, replica: usize) -> Result<&T, MemHedgeError> {
        is! { self.state() == MemHedgeState::Idle, return Err(MemHedgeError::NotArmed) }
        is! { replica >= N, return Err(MemHedgeError::InvalidReplica) }
        let logical_index = self.logical_index();
        self.replicas.get_replica(replica, logical_index).ok_or(MemHedgeError::OutOfBounds)
    }

    /// Reads one replica and then attempts to claim the active request.
    ///
    /// This is the worker-facing convenience operation:
    ///
    /// - read the replica corresponding to `replica`,
    /// - try to claim victory for that replica,
    /// - return the read value only if this replica won.
    ///
    /// Returns:
    /// - `Ok(Some(&T))` if this replica became the winner,
    /// - `Ok(None)` if another replica won first while the request was still armed,
    /// - `Err(...)` if the request was not active or the indices were invalid.
    pub fn try_read_claim(&self, replica: usize) -> Result<Option<&T>, MemHedgeError> {
        let value = self.read_replica(replica)?;
        match self.ctrl.try_claim(replica)? {
            true => Ok(Some(value)),
            false => Ok(None),
        }
    }
}
