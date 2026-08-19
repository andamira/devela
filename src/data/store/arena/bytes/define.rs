// devela/src/data/store/arena/bytes/define.rs
//
//! Defines [`arena_bytes!`].
//

#[doc = crate::_tags!(construction data_structure)]
/// Defines a fixed-capacity byte arena with compact span handles.
#[doc = crate::_doc_meta!{location("data/store")}]
///
/// The generated arena stores bytes in an append-only initialized prefix.
///
/// Arena positions use the primitive part of the configured `cursor`.
/// The arena's current length and rollback marks use that primitive directly,
/// while generated handles may use a different representation for their
/// byte offsets and lengths.
///
/// Handles describe coordinates only; they do not identify a particular arena.
/// The receiving arena validates that their spans lie within its written prefix.
///
/// # Configuration
/// `cursor` selects the primitive byte-coordinate type and, optionally,
/// the representation used by generated handle fields.
///
/// - `cursor: u16;` uses `u16` for both.
/// - `cursor: u16 + NonMaxU16;` keeps arena cursor state as `u16`
///   while storing handle coordinates as `NonMaxU16`.
///
/// # Optional marks
///
/// Supplying a third generated type enables checkpoint-based reclamation:
///
/// ```text
/// arena_bytes! {
///     [cursor: u8;]
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
/// # Features
/// Uses `unsafe_array` to avoid initializing the full byte capacity,
/// and `unsafe_slice` for additional slice-access optimizations.
///
/// # Example
/// ```
/// # use devela::{NonMaxU16, arena_bytes};
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
/// [`ArenaBytesMarkExample`].
///
/// [`ArenaBytesExample`]: crate::ArenaBytesExample
/// [`ArenaBytesHandleExample`]: crate::ArenaBytesHandleExample
/// [`ArenaBytesMarkExample`]: crate::ArenaBytesMarkExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! arena_bytes {
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

        $crate::arena_bytes! { %backend
            [kind: $($kind)?]
            [cursor: $cprim + $Cursor]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($(#[$mark_attr])* $mvis $Mark)?]
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
        [mark: $($(#[$mark_attr:meta])* $mvis:vis $Mark:ident)?]
    ) => {
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
        $crate::__arena_bytes_impl_array! {
            [cursor: $cprim]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($mvis $Mark)?]
            [internal: $crate::__ArenaBytesArray::<CAP>]
            ($)
        }
    };
}
#[doc(inline)]
pub use arena_bytes;
