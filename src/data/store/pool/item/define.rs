// devela/src/data/store/pool/item/define.rs
//
//! Defines [`pool!`].
//

#[cfg(all(doc, feature = "_docs_examples"))]
use crate::PoolExample;

#[doc = crate::_tags!(construction data_structure)]
/// Defines an owning generational pool with static or allocating storage.
#[doc = crate::_doc_meta!{
    location("data/store/pool", macro pool),
}]
/// The generated pool stores values in indexed slots and accesses them
/// through generated handles containing a slot index and generation.
///
/// Removing a value advances that slot's generation before the slot can be
/// reused, so handles previously issued for that slot no longer resolve.
///
/// # Storage regimes
///
/// The pool declaration supports two ownership regimes:
///
/// - **Static** — the default.
///
///   The pool owns fixed-capacity array storage and has the type
///   `Pool<T, const CAP: usize>`. It does not allocate, and many operations
///   are const-capable.
///
///   The optional `: static` selector may be written explicitly or omitted.
///
/// - **Allocating** — selected with `: alloc`.
///
///   The pool owns growable vector storage and has the type `Pool<T>`.
///   It requires the `alloc` feature and grows until the configured index
///   representation can no longer represent another slot.
///
/// # Capacity
///
/// [`capacity`][PoolExample::capacity] reports the usable number of slots supported
/// by the current storage:
///
/// - for a static pool, this is the fixed `CAP`;
/// - for an allocating pool, this is the currently reserved usable capacity,
///   bounded by the index representation.
///
/// [`remaining`][PoolExample::remaining] returns `capacity() - len()`.
///
/// For an allocating pool, `remaining() == 0` does not necessarily mean the
/// pool is full: a later insertion may grow the allocation.
/// [`is_full`][PoolExample::is_full] indicates that insertion must fail
/// because neither a vacant slot nor a new representable index exists.
///
/// # Handle validity
///
/// Handles are relative to the pool instance that produced them. They do not
/// contain a pool identity, so resolving a handle against another instance of
/// the same generated pool type—including a cloned state—may also succeed.
///
/// Generations wrap through the valid values of their configured
/// representation. A sufficiently old stale handle can therefore become valid
/// again after a complete generation cycle for the same slot.
///
/// # Representation requirements
///
/// The index representation must:
///
/// - be unsigned;
/// - contain zero;
/// - form a contiguous range from zero.
///
/// For a static pool it must represent every index in `0..CAP`. An allocating
/// pool can introduce slots until the next index is no longer representable.
///
/// The generation representation must contain at least two distinct values.
///
/// Omitting the representation after `+` uses the primitive itself.
///
/// # Examples
/// ```
/// use devela::{NonMaxU16, pool};
///
/// // Static storage is the default.
/// pool! {
///     [
///         index: u8;
///         generation: u16 + NonMaxU16;
///     ]
///     pub Entities;
///     pub EntityId;
/// }
/// let mut entities = Entities::<&str, 8>::new();
/// let tree = entities.insert("tree").unwrap();
/// assert_eq!(entities.get(tree), Some(&"tree"));
///
/// // Allocating storage.
/// # #[cfg(feature = "alloc")] {
/// pool! {
///     [
///         index: u32;
///         generation: u16 + NonMaxU16;
///     ]
///     pub DynamicEntities: alloc;
///     pub DynamicEntityId;
/// }
/// let mut entities = DynamicEntities::<&str>::new();
/// let river = entities.insert("river").unwrap();
/// assert_eq!(entities.get(river), Some(&"river"));
/// # }
/// ```
///
/// See:
/// - [`PoolExample`], [`PoolHandleExample`].
/// - [`PoolAllocExample`], [`PoolAllocHandleExample`].
///
/// [`PoolAllocExample`]: crate::PoolAllocExample
/// [`PoolHandleExample`]: crate::PoolHandleExample
/// [`PoolAllocHandleExample`]: crate::PoolAllocHandleExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! pool· {
    (
        [
            index: $iprim:ident $(+ $Index:ty)?;
            generation: $gprim:ident $(+ $Generation:ty)?;
        ]

        $(#[$pool_attr:meta])*
        $vis:vis $Pool:ident $( : $kind:ident)?;

        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident $(;)?

    ) => {
        $crate::__pool! { %normalize_index
            [kind: $($kind)?]
            [index: $iprim $(+ $Index)?]
            [generation: $gprim $(+ $Generation)?]
            [pool: $(#[$pool_attr])* $vis $Pool]
            [handle: $(#[$handle_attr])* $hvis $Handle]
        }
    };
}
#[doc(inline)]
pub use pool· as pool;
