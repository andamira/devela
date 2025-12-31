// devela::sys::mem::borrow::ownership
//
//! Defines the [`Ownership`] trait.
//

#[cfg(feature = "alloc")]
use devela::{Borrow, String, Vec};

#[doc = crate::_TAG_LIFETIME!()]
/// Defines the relationship between a borrowed type and its owned counterpart.
///
/// This enables abstracting over allocation strategies in [`MaybeOwned`][crate::MaybeOwned].
/// Implement this for types where:
/// - The borrowed form is `&T`
/// - The owned form implements `Borrow<T>` (can be viewed as `&T`)
///
/// # Allocation Requirement
/// This trait's associated type **always requires `alloc`** because:
/// 1. The `Owned` type must implement `Borrow<Self>`
/// 2. Non-alloc owned types would violate the borrowing relationship
///
/// However, the trait itself is `core`-compatible
/// # Examples
/// ```ignore
/// # use devela::{Ownership, MaybeOwned};
/// #[cfg(feature = "alloc")]
/// impl Ownership for str {
///     type Owned = String;  // String implements Borrow<str>
/// }
/// ```
///
/// # Alloc Consideration
/// The trait itself is `core`-compatible, but most implementations will require
/// the `alloc` feature for their `Owned` types.
pub trait Ownership {
    /// The owned counterpart.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    type Backing: Borrow<Self>;
}

// TODO: helper macro

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl Ownership for crate::Path {
    type Backing = crate::PathBuf;
}
impl Ownership for str {
    #[cfg(feature = "alloc")]
    type Backing = String;
}
#[cfg(feature = "alloc")]
impl<T> Ownership for [T] {
    #[cfg(feature = "alloc")]
    type Backing = Vec<T>;
}
