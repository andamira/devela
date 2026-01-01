// devela_base_std::work::sync::mpsc::_reexport

use crate::_reexport;
#[allow(unused_imports)]
use crate::{_TAG_CONCURRENCY, _TAG_ERROR, _TAG_ERROR_COMPOSITE, _TAG_ITERATOR};

/* structs */

_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!() _TAG_ITERATOR!(),
    doc: "An owning iterator over messages on an [`MpscReceiver`].",
    @IntoIter as MpscIntoIter
}
_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!() _TAG_ITERATOR!(),
    doc: "An iterator over messages on an [`MpscReceiver`]",
    @Iter as MpscIter
}
_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!(),
    doc: "The receiving half of an [`Mpsc`][crate::Mpsc]
    [`channel`][crate::Mpsc::channel] or [`sync_channel`][crate::Mpsc::sync_channel].",
    @Receiver as MpscReceiver
}
_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!() _TAG_ERROR!(),
    doc: "An error returned from [`MpscReceiver::recv`].",
    @RecvError as MpscReceiveError
}
_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!() _TAG_ERROR!(),
    doc: "An error returned from [`MpscSender::send`] or [`MpscSyncSender::send`].",
    @SendError as MpscSendError
}
_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!(),
    doc: "The sending half of an *async* `Mpsc` channel.",
    @Sender as MpscSender
}
_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!(),
    doc: "The sending half of a *sync* `Mpsc` channel.",
    @SyncSender as MpscSyncSender
}
_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!() _TAG_ITERATOR!(),
    doc: "Attempts to yield all pending values for an [`MpscReceiver`].",
    @TryIter as MpscTryIter
}

/* enums */

_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!() _TAG_ERROR_COMPOSITE!(),
    doc: "Possible errors that made [`recv_timeout`][MpscReceiver::recv_timeout]
unable to return data when called.",
    @RecvTimeoutError as MpscRecvTimeoutError
}
_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!() _TAG_ERROR_COMPOSITE!(),
    doc: "Possible reasons that [`try_recv`][MpscReceiver::try_recv]
could not return data when called.",
    @TryRecvError as MpscTryRecvError
}
_reexport! { rust: std::sync::mpsc,
    tag: _TAG_CONCURRENCY!() _TAG_ERROR_COMPOSITE!(),
    doc: "Possible error outcomes for the [`try_send`][MpscSyncSender::try_send] method.",
    @TrySendError as MpscTrySendError
}
