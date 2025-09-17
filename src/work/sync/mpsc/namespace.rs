// devela::sys::work::sync::mpsc::namespace
//
//! Defines the [`Mpsc`] namespace.
//

#[cfg(feature = "std")]
use crate::{MpscReceiver, MpscSender, MpscSyncSender};
#[cfg(feature = "std")]
use std::sync::mpsc::{channel, sync_channel};

#[doc = crate::_TAG_NAMESPACE!()]
/// Multi-producer, single-consumer channel operations.
#[derive(Debug)]
pub struct Mpsc;

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl Mpsc {
    /// Creates a new asynchronous channel, returning the sender/receiver halves.
    ///
    /// See `std::sync::mpsc::`[`channel`].
    #[must_use]
    pub fn channel<T>() -> (MpscSender<T>, MpscReceiver<T>) {
        channel()
    }

    /// Creates a new synchronous, bounded channel.
    ///
    /// See `std::sync::mpsc::`[`sync_channel`].
    #[must_use]
    pub fn sync_channel<T>(bound: usize) -> (MpscSyncSender<T>, MpscReceiver<T>) {
        sync_channel(bound)
    }
}
