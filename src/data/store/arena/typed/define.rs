// devela/src/data/store/arena/typed/define.rs
//
//! Defines [`arena!`].
//

#[doc = crate::_tags!(construction data_structure)]
/// Defines an owning typed arena with static or allocating storage.
#[doc = crate::_doc_meta!{location("data/store/arena")}]
///
/// The generated arena appends values densely and returns compact index handles.
/// Values are reclaimed collectively by clearing the arena. Supplying a third
/// generated type additionally enables marks for reclaiming a suffix by rollback.
/// Arbitrary interior removal is intentionally unsupported.
///
/// # Storage regimes
///
/// The arena declaration supports two ownership regimes:
///
/// - **Static** — the default.
///
///   The arena owns fixed-capacity `[Option<T>; CAP]` storage and has the type
///   `Arena<T, const CAP: usize>`. It does not allocate.
///   Copyable values also receive const-capable insertion and clearing methods,
///   and const-capable rollback when marks are enabled.
///
///   The optional `: static` selector may be written explicitly or omitted.
///
/// - **Allocating** — selected with `: alloc`.
///
///   The arena owns growable `Vec<T>` storage and has the type `Arena<T>`.
///   It requires the `alloc` feature and grows until the configured index
///   representation can no longer represent another value.
///
/// # Optional marks
///
/// Supplying a third generated type enables checkpoint-based reclamation:
///
/// ```text
/// arena! {
///     [index: u8;]
///     pub Arena;
///     pub Handle;
///     pub Mark;
/// }
/// ```
/// [`mark`](#method.mark) snapshots the current insertion frontier and
/// [`rollback`](#method.rollback) removes every value inserted after it.
///
/// The mark stores the frontier as `usize`, independently of the handle index
/// representation, so the frontier after a completely full arena remains representable.
///
/// Marks, like handles, are relative to the arena instance that produced them.
/// Rolling back to a mark ahead of the current frontier is rejected.
///
/// Static marks use the same compact representation as the arena frontier.
/// Allocating marks retain the Vec frontier as usize.
///
/// # Handle validity
///
/// Handles contain only an index and no arena-instance identity or generation.
/// Rolling back or clearing invalidates handles to reclaimed values, but that
/// invalidation is not remembered. If a later insertion reuses the same index,
/// an old handle for that index can resolve again to the new value.
///
/// Use [`pool!`][crate::pool!] when individual reclamation and bounded stale-handle
/// rejection are required.
///
/// # Capacity
///
/// [`capacity`](#method.capacity) reports usable storage without further growth:
///
/// For a static arena, the configured representation stores both handle indices
/// and the current insertion frontier. It must therefore represent every frontier
/// in 0..=CAP; CAP may not exceed the representation's maximum value.
/// The maximum frontier value is not issued as a handle index.
///
/// An allocating arena stores its frontier in Vec and may therefore use
/// the full representable index range.
///
/// [`remaining`](#method.remaining) returns `capacity() - len()`.
/// For an allocating arena, `remaining() == 0` does not necessarily mean that
/// insertion must fail: the vector may grow. [`is_full`](#method.is_full)
/// indicates that no further index can be represented.
///
/// # Representation requirements
///
/// The index primitive must implement [`PrimIndex`](crate::PrimIndex).
/// Its representation must:
/// - be unsigned;
/// - contain zero;
/// - form a contiguous range from zero.
///
/// For a static arena it must represent every index in `0..CAP`. An allocating
/// arena can append values until the next index is no longer representable.
///
/// Omitting the representation after `+` uses the primitive itself.
///
/// # Examples
/// ```
/// # use devela::arena;
/// // Static storage, with rollback marks.
/// arena! {
///     [index: u8;]
///     pub Entities;
///     pub EntityId;
///     pub EntityMark;
/// }
/// let mut entities = Entities::<&str, 8>::new();
/// let origin = entities.mark();
/// let tree = entities.insert("tree").unwrap();
/// let river = entities.insert("river").unwrap();
/// assert_eq!(entities.get(tree), Some(&"tree"));
/// assert_eq!(entities.get(river), Some(&"river"));
/// assert!(entities.rollback(origin));
/// assert!(!entities.contains(tree));
///
/// // Allocating storage, without marks.
/// # #[cfg(feature = "alloc")] {
/// arena! {
///     [index: u32;]
///     pub DynamicEntities: alloc;
///     pub DynamicEntityId;
/// }
/// let mut entities = DynamicEntities::<&str>::with_capacity(8);
/// let cloud = entities.insert("cloud").unwrap();
/// assert_eq!(entities.get(cloud), Some(&"cloud"));
/// # }
/// ```
///
/// See:
/// [`ArenaExample`], [`ArenaAllocExample`],
/// [`ArenaHandleExample`], [`ArenaAllocHandleExample`],
/// [`ArenaMarkExample`], [`ArenaAllocMarkExample`].
///
/// [`ArenaExample`]: crate::ArenaExample
/// [`ArenaAllocExample`]: crate::ArenaAllocExample
/// [`ArenaHandleExample`]: crate::ArenaHandleExample
/// [`ArenaAllocHandleExample`]: crate::ArenaAllocHandleExample
/// [`ArenaMarkExample`]: crate::ArenaMarkExample
/// [`ArenaAllocMarkExample`]: crate::ArenaAllocMarkExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! arena {
    (
        [ index: $iprim:ident $(+ $Index:ty)?; ]

        $(#[$arena_attr:meta])*
        $vis:vis $Arena:ident $( : $kind:ident)?;

        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident;

        $(
            $(#[$mark_attr:meta])*
            $mvis:vis $Mark:ident $(;)?
        )?
    ) => {
        $crate::arena! { %normalize_index
            [kind: $($kind)?]
            [index: $iprim $(+ $Index)?]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $(#[$handle_attr])* $hvis $Handle]
            [mark: $($(#[$mark_attr])* $mvis $Mark)?]
        }
    };
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident]
        $($rest:tt)*
    ) => {
        $crate::arena! { %generate
            [kind: $($kind)?]
            [index: $iprim + $iprim]
            $($rest)*
        }
    };
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        $($rest:tt)*
    ) => {
        $crate::arena! { %generate
            [kind: $($kind)?]
            [index: $iprim + $Index]
            $($rest)*
        }
    };
    (%generate
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $(#[$handle_attr:meta])* $hvis:vis $Handle:ident]
        [mark: $($(#[$mark_attr:meta])* $mvis:vis $Mark:ident)?]
    ) => {
        $crate::handle! {
            [index: $iprim + $Index;]
            $(#[$handle_attr])* $hvis $Handle
        }
        $crate::arena! { %backend
            [kind: $($kind)?]
            [index: $iprim + $Index]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($(#[$mark_attr])* $mvis $Mark)?]
        }
    };
    (%backend
        [kind:]
        $($rest:tt)*) => {
        $crate::arena! { %backend [kind: static] $($rest)* }
    };
    (%backend
        [kind: static]
        [index: $iprim:ident + $Index:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $($(#[$mark_attr:meta])* $mvis:vis $Mark:ident)?]
    ) => {
        $crate::__arena_impl_array! {
            [index: $iprim + $Index;]
            $(#[$arena_attr])* $vis $Arena;
            $hvis $Handle;
            [mark: $($mvis $Mark)?]
        }
        $(
            $(#[$mark_attr])*
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $mvis struct $Mark($crate::MaybeNiche<$Index>);

            #[allow(dead_code)]
            impl $Mark {
                const fn new(mark: $crate::MaybeNiche<$Index>) -> Self { Self(mark) }
            }
        )?
    };
    (%backend
        [kind: alloc]
        [index: $iprim:ident + $Index:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $($(#[$mark_attr:meta])* $mvis:vis $Mark:ident)?]
    ) => {
        $crate::__arena_impl_vec! {
            [index: $iprim + $Index;]
            $(#[$arena_attr])* $vis $Arena;
            $hvis $Handle;
            [mark: $($mvis $Mark)?]
        }
        $(
            $(#[$mark_attr])*
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $mvis struct $Mark(usize);

            #[allow(dead_code)]
            impl $Mark {
                const fn new(mark: usize) -> Self { Self(mark) }
            }
        )?
    };
}
#[doc(inline)]
pub use arena;
