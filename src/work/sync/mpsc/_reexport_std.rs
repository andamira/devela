// devela/src/work/sync/mpsc/_reexport_std.rs

use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => struct MpscIntoIter,
    tag: _tags!(concurrency iterator),
    doc: "An owning iterator over messages on an [`MpscReceiver`].",
    @IntoIter as MpscIntoIter
}
_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => struct MpscIter,
    tag: _tags!(concurrency iterator),
    doc: "An iterator over messages on an [`MpscReceiver`]",
    @Iter as MpscIter
}
_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => struct MpscReceiver,
    tag: _tags!(concurrency),
    doc: "The receiving half of an [`Mpsc`][crate::Mpsc]
    [`channel`][crate::Mpsc::channel] or [`sync_channel`][crate::Mpsc::sync_channel].",
    @Receiver as MpscReceiver
}
_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => struct MpscReceiveError,
    tag: _tags!(concurrency error),
    doc: "An error returned from [`MpscReceiver::recv`].",
    @RecvError as MpscReceiveError
}
_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => struct MpscSendError,
    tag: _tags!(concurrency error),
    doc: "An error returned from [`MpscSender::send`] or [`MpscSyncSender::send`].",
    @SendError as MpscSendError
}
_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => struct MpscSender,
    tag: _tags!(concurrency),
    doc: "The sending half of an *async* `Mpsc` channel.",
    @Sender as MpscSender
}
_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => struct MpscSyncSender,
    tag: _tags!(concurrency),
    doc: "The sending half of a *sync* `Mpsc` channel.",
    @SyncSender as MpscSyncSender
}
_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => struct MpscTryIter,
    tag: _tags!(concurrency iterator),
    doc: "Attempts to yield all pending values for an [`MpscReceiver`].",
    @TryIter as MpscTryIter
}

/* enums */

_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => enum MpscRecvTimeoutError,
    tag: _tags!(concurrency error_composite),
    doc: "Possible errors that made [`recv_timeout`][MpscReceiver::recv_timeout]
unable to return data.",
    @RecvTimeoutError as MpscRecvTimeoutError
}
_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => enum MpscTryRecvError,
    tag: _tags!(concurrency error_composite),
    doc: "Possible reasons that [`try_recv`][MpscReceiver::try_recv]
could not return data when called.",
    @TryRecvError as MpscTryRecvError
}
_reexport! { rust: std::sync::mpsc,
    location: "work/sync/mpsc" => enum MpscTrySendError,
    tag: _tags!(concurrency error_composite),
    doc: "Possible error outcomes for the [`try_send`][MpscSyncSender::try_send] method.",
    @TrySendError as MpscTrySendError
}
