// devela/src/data/store/arena/bytes/define.rs
//
//! Defines [`arena_bytes!`].
//

#[doc = crate::_tags!(construction data_structure)]
/// Defines a byte arena with static or allocating storage and compact span handles.
#[doc = crate::_doc_meta!{
    location("data/store/arena", macro arena_bytes),
}]
/// The generated arena stores bytes in an append-only initialized prefix.
/// Handles identify byte spans within that prefix.
///
/// Like handles, marks contain coordinates only; they do not identify
/// a particular arena. A receiving arena accepts a mark only when its
/// frontier is not ahead of the current written prefix.
///
/// Reclamation does not permanently invalidate their coordinates:
/// later writes may reuse the same byte positions.
///
/// # Storage regimes
///
/// The arena declaration supports two storage regimes:
///
/// - **Static** — the default.
///
///   The arena owns fixed-capacity inline storage and has the type
///   `Arena<const CAP: usize>`. It does not allocate.
///
///   The complete capacity must be representable by both the cursor primitive
///   and the handle representation.
///
///   The optional `: static` selector may be written explicitly or omitted.
///
/// - **Allocating** — selected with `: alloc`.
///
///   The arena owns growable `Vec<u8>` storage and has the type `Arena`.
///   It requires the `alloc` feature and may grow until the configured
///   byte-coordinate range is exhausted.
///
///   Its `capacity` and `remaining` methods describe storage available without
///   reallocating. Therefore `remaining() == 0` does not necessarily mean that
///   another write will fail. `is_full()` indicates that no additional byte
///   coordinate can be represented.
///
/// # Cursor and handle representation
///
/// `cursor` selects the primitive byte-coordinate type and, optionally,
/// the representation used by generated handle fields.
///
/// - `cursor: u16;` uses `u16` for both.
/// - `cursor: u16 + NonMaxU16;` keeps arena cursor state as `u16`
///   while storing handle coordinates as `NonMaxU16`.
///
/// The current byte frontier and rollback marks use the cursor primitive
/// directly. Handles may use a different representation for their byte
/// offsets and lengths.
///
/// Both storage regimes keep the written frontier within the coordinate range
/// representable by the cursor primitive and handle representation. This keeps
/// byte-span semantics independent of the storage backend.
///
/// # Optional marks
///
/// Supplying a mark type adds `mark` and `rollback`.
///
/// A mark records a storage frontier, not a snapshot of the retained bytes:
/// mutations before that frontier are not reverted by rollback.
///
/// Like handles, marks are relative to the arena instance that produced them.
/// Reclamation does not permanently invalidate their coordinates: later writes
/// may reuse the same byte positions.
///
/// # Features
///
/// The static backend does not require allocation. With `unsafe_array`,
/// unwritten static capacity can remain uninitialized; `unsafe_slice` enables
/// additional slice-access optimizations where applicable.
///
/// The `: alloc` backend requires the `alloc` feature and uses `Vec<u8>` storage.
///
/// # Example
/// ```
/// use devela::{NonMaxU16, arena_bytes};
///
/// arena_bytes! {
///     [cursor: u16 + NonMaxU16;]
///
///     /// A byte arena.
///     pub Bytes;
///     /// A byte span within `Bytes`.
///     pub BytesHandle;
///     /// A rollback position within `Bytes`.
///     pub BytesMark;
/// }
///
/// let mut arena = Bytes::<64>::new();
/// let handle = arena.push_bytes(b"devela").unwrap();
/// assert_eq!(arena.read_bytes(handle), Some(&b"devela"[..]));
/// ```
/// See:
/// [`ArenaBytesExample`],
/// [`ArenaBytesHandleExample`],
/// [`ArenaBytesMarkExample`],
/// [`ArenaBytesAllocExample`].
///
/// [`ArenaBytesExample`]: crate::ArenaBytesExample
/// [`ArenaBytesHandleExample`]: crate::ArenaBytesHandleExample
/// [`ArenaBytesMarkExample`]: crate::ArenaBytesMarkExample
/// [`ArenaBytesAllocExample`]: crate::ArenaBytesAllocExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! arena_bytes· {
    (
        [cursor: $cprim:ident $(+ $Cursor:ty)?;]

        $(#[$arena_attr:meta])*
        $vis:vis $Arena:ident $( : $kind:ident )?;

        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident;

        $(
            $(#[$mark_attr:meta])*
            $mvis:vis $Mark:ident $(;)?
        )?
    ) => {
        $crate::arena_bytes! { %normalize_cursor
            [kind: $($kind)?]
            [cursor: $cprim $(+ $Cursor)?]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $(#[$handle_attr])* $hvis $Handle]
            [mark: $($(#[$mark_attr])* $mvis $Mark)?]
        }
    };
    (%normalize_cursor
        [kind: $($kind:ident)?]
        [cursor: $cprim:ident]
        $($rest:tt)*
    ) => {
        $crate::arena_bytes! { %generate
            [kind: $($kind)?]
            [cursor: $cprim + $cprim]
            $($rest)*
        }
    };
    (%normalize_cursor
        [kind: $($kind:ident)?]
        [cursor: $cprim:ident + $Cursor:ty]
        $($rest:tt)*
    ) => {
        $crate::arena_bytes! { %generate
            [kind: $($kind)?]
            [cursor: $cprim + $Cursor]
            $($rest)*
        }
    };
    (%generate
        [kind: $($kind:ident)?]
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $(#[$handle_attr:meta])* $hvis:vis $Handle:ident]
        [mark: $($(#[$mark_attr:meta])* $mvis:vis $Mark:ident)?]
    ) => {
        $crate::handle_span! {
            [offset: $cprim + $Cursor;]
            $(#[$handle_attr])*
            $hvis $Handle;
        }
        $(
            $(#[$mark_attr])*
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $mvis struct $Mark($cprim);

            #[allow(dead_code)]
            impl $Mark {
                const fn new(cursor: $cprim) -> Self {
                    Self(cursor)
                }
            }
        )?
        $crate::arena_bytes! { %backend
            [kind: $($kind)?]
            [cursor: $cprim + $Cursor]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($mvis $Mark)?]
        }
    };
    (%backend
        [kind:]
        $($rest:tt)*) => {
        $crate::arena_bytes! { %backend [kind: static] $($rest)* }
    };
    (%backend
        [kind: static]
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $($mvis:vis $Mark:ident)?]
    ) => {
        $crate::paste! { $crate::__arena_bytes_impl_array! {
            [cursor: $cprim]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($mvis $Mark)?]
            [internal: $crate::__ArenaBytesArray::<CAP>]
            [module: [<_arena_bytes_impl_ $Arena>]]
            ($)
        }}
    };
    (%backend
        [kind: alloc]
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $($mvis:vis $Mark:ident)?]
    ) => {
        $crate::paste! { $crate::__arena_bytes_impl_vec! {
            [cursor: $cprim + $Cursor]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($mvis $Mark)?]
            [module: [<_arena_bytes_impl_ $Arena>]]
            ($)
        }}
    };
}
#[doc(inline)]
pub use arena_bytes· as arena_bytes;
