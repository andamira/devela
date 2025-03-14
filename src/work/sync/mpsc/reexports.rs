// devela::work::sync::mpsc::reexports
//
//! Reexported items.
//

use crate::reexport;
#[cfg(feature = "std")]
use crate::{TAG_ERROR, TAG_ERROR_COMPOSITE, TAG_ITERATOR};

/* structs */

reexport! { rust: std::sync::mpsc,
    tag: TAG_ITERATOR!(),
    doc: "An owning iterator over messages on an [`MpscReceiver`].",
    @IntoIter as MpscIntoIter
}
reexport! { rust: std::sync::mpsc,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over messages on an [`MpscReceiver`]",
    @Iter as MpscIter
}
reexport! { rust: std::sync::mpsc,
    doc: "The receiving half of a [`Mpsc`] [`channel`][Mpsc::channel]
or [`sync_channel`][Mpsc::sync_channel].",
    @Receiver as MpscReceiver
}
reexport! { rust: std::sync::mpsc,
    tag: TAG_ERROR!(),
    doc: "An error returned from [`MpscReceiver::recv`].",
    @RecvError as MpscReceiveError
}
reexport! { rust: std::sync::mpsc,
    tag: TAG_ERROR!(),
    doc: "An error returned from [`MpscSender::send`] or [`MpscSyncSender::send`].",
    @SendError as MpscSendError
}
reexport! { rust: std::sync::mpsc,
    doc: "The sending half of an *async* `Mpsc` channel.",
    @Sender as MpscSender
}
reexport! { rust: std::sync::mpsc,
    doc: "The sending half of a *sync* `Mpsc` channel.",
    @SyncSender as MpscSyncSender
}
reexport! { rust: std::sync::mpsc,
    tag: TAG_ITERATOR!(),
    doc: "Attempts to yield all pending values for an [`MpscReceiver`].",
    @TryIter as MpscTryIter
}

/* enums */

reexport! { rust: std::sync::mpsc,
    tag: TAG_ERROR_COMPOSITE!(),
    doc: "Possible errors that made [`recv_timeout`][MpscReceiver::recv_timeout]
unable to return data when called.",
    @RecvTimeoutError as MpscRecvTimeoutError
}
reexport! { rust: std::sync::mpsc,
    tag: TAG_ERROR_COMPOSITE!(),
    doc: "Possible reasons that [`try_recv`][MpscReceiver::try_recv]
could not return data when called.",
    @TryRecvError as MpscTryRecvError
}
reexport! { rust: std::sync::mpsc,
    tag: TAG_ERROR_COMPOSITE!(),
    doc: "Possible error outcomes for the [`try_send`][MpscSyncSender::try_send] method.",
    @TrySendError as MpscTrySendError
}
