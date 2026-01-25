// devela::sys::mem::storage::boxed
//
//! *Boxed* storage
//

#[cfg(doc)]
use crate::{Bare, BareBox};
use crate::{Box, ConstInit, Storage};
// #[cfg(feature = "dep_rkyv")] // DEP_DISABLED
// use rkyv::{Archive, Deserialize, Serialize};

#[doc = crate::_tags!(mem)]
/// A zero-sized marker for a [`Storage`] type that wraps its data in a [`Box`].
#[doc = crate::_doc_location!("sys/mem")]
///
/// Equivalent to the [`Bare`] marker struct which uses a [`BareBox`] for the underlying storage.
// #[cfg_attr(feature = "dep_rkyv", derive(Archive, Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Boxed;

/// This implementation is equivalent to the one for [`Bare`] which uses [`BareBox`] for storage.
impl Storage for Boxed {
    type Stored<T> = Box<T>;

    fn name() -> &'static str {
        "Boxed"
    }
}
impl ConstInit for Boxed {
    const INIT: Self = Boxed;
}
