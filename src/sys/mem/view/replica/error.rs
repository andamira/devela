// devela::sys::mem::view::replica::error
//
//! Defines [`MemReplicaError`].
//

#[doc = crate::_tags!(error mem layout)]
/// Errors produced by replicated layout construction and access.
#[doc = crate::_doc_location!("sys/mem/layout")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemReplicaError {
    /// The requested replica count `N` is zero.
    ZeroReplicas,
    /// The layout declares zero channels.
    ZeroChannels,
    /// The element type has zero size and cannot participate in byte-spaced layout.
    ZeroSizedType,
    /// The channel offset is smaller than `size_of::<T>()`.
    OffsetTooSmall,
    /// The channel offset is not a multiple of `size_of::<T>()`.
    MisalignedOffset,
    /// The replica count `N` exceeds the available number of channels.
    TooManyReplicas,
    /// The provided storage cannot hold even one logical chunk of the layout.
    StorageTooSmall,
    /// No logical capacity remains for insertion.
    Full,
    /// The requested logical index lies outside the addressable logical capacity.
    OutOfBounds,
}
