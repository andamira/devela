// devela/src/data/store/arena/string/define.rs
//
//! Defines [`arena_string!`].
//

#[doc = crate::_tags!(construction data_structure)]
/// Defines a packed UTF-8 string arena with static or allocating storage.
#[doc = crate::_doc_meta!{
    location("data/store/arena", macro arena_string),
}]
/// The generated arena appends strings densely and returns compact index handles.
///
/// Strings are stored as packed UTF-8 bytes while their boundaries are retained
/// separately. Each insertion has its own identity: duplicate and empty strings
/// receive distinct handles. This is therefore a string arena, not an interner.
///
/// Strings remain at stable positions while retained. Reclamation is collective:
/// the arena may be cleared, or a retained suffix may be reclaimed by rollback
/// when an optional mark type is generated. Arbitrary interior removal is
/// intentionally unsupported.
///
/// # Storage regimes
///
/// The arena declaration supports two storage regimes:
///
/// - **Static** — the default.
///
///   The arena owns fixed-capacity inline storage and has the type
///   `Arena<const STRINGS: usize, const BYTES: usize>`.
///
///   `STRINGS` is the maximum number of retained string entries and `BYTES` is
///   the maximum number of packed UTF-8 bytes. These capacities are independent:
///   an empty string consumes one entry but no bytes.
///
///   Both capacities must fit their configured coordinate representations.
///
///   The optional `: static` selector may be written explicitly or omitted.
///
/// - **Allocating** — selected with `: alloc`.
///
///   The arena owns growable storage and has the type `Arena`.
///   It requires the `alloc` feature.
///
///   [`capacity`](#method.capacity) and [`byte_capacity`](#method.byte_capacity)
///   report storage currently available without reallocating. The arena may grow
///   until either the configured string-index or byte-cursor representation is
///   exhausted.
///
/// # Coordinates
///
/// The declaration configures two independent coordinate domains:
///
/// - `index` identifies retained strings and determines the representation of
///   generated handles.
/// - `cursor` represents cumulative UTF-8 byte ends in the packed storage.
///
/// Each may be written as `primitive + representation`; omitting the
/// representation uses the primitive itself.
///
/// Both representations must be unsigned, contiguous, and contain zero.
///
/// For static storage, the string insertion frontier is itself stored in the
/// index representation, so the configured string capacity may not exceed that
/// representation's maximum value.
///
/// Allocating storage keeps its frontier separately and can therefore issue every
/// representable index value. Its maximum string count is one greater than the
/// maximum representable index when that addition is machine-representable.
///
/// The packed byte length itself is a cursor value, so its maximum is the
/// maximum value representable by `cursor` in either storage regime.
///
/// # Handles
///
/// Generated handles contain only a string index. They do not contain an
/// arena-instance identity or generation.
///
/// After rollback or clearing, handles into the reclaimed suffix no longer
/// resolve. This invalidation is not remembered: later insertion may reuse the
/// same index, allowing an old handle to resolve again to a new string.
///
/// Arena string handles are therefore storage coordinates rather than
/// generational identities.
///
/// # Packed representation
///
/// [`as_bytes`](#method.as_bytes) exposes the concatenated UTF-8 bytes of all
/// retained strings in insertion order.
///
/// Entry boundaries are not encoded in that byte slice itself; they remain part
/// of the arena's string-indexed structure. Use [`get`](#method.get),
/// [`iter`](#method.iter), or [`entries`](#method.entries) when those boundaries matter.
///
/// # Optional marks
///
/// Supplying a third generated type enables checkpoint-based reclamation:
///
/// ```text
/// arena_string! {
///     [
///         index: u8;
///         cursor: u16;
///     ]
///     pub Strings;
///     pub StringId;
///     pub StringMark;
/// }
/// ```
///
/// [`mark`](#method.mark) records the current string insertion frontier.
/// [`rollback`](#method.rollback) retracts the arena to that frontier and
/// automatically restores the corresponding packed-byte frontier.
///
/// Marks contain frontier coordinates only; they do not identify an arena
/// instance. A rollback is rejected when its mark lies ahead of the receiving
/// arena's current frontier.
///
/// # Example
///
/// ```
/// use devela::{NonMaxU8, NonMaxU16, arena_string};
///
/// arena_string! {
///     [
///         index: u8 + NonMaxU8;
///         cursor: u16 + NonMaxU16;
///     ]
///
///     /// A packed string arena.
///     pub Strings;
///     /// A string handle.
///     pub StringId;
///     /// A rollback mark.
///     pub StringMark;
/// }
///
/// let mut strings = Strings::<8, 64>::new();
///
/// let hello = strings.insert("hello").unwrap();
/// let lambda = strings.insert("λ").unwrap();
/// let hello_again = strings.insert("hello").unwrap();
///
/// assert_eq!(strings.get(hello), Some("hello"));
/// assert_eq!(strings.get(lambda), Some("λ"));
/// assert_ne!(hello, hello_again);
/// assert_eq!(strings.as_bytes(), "helloλhello".as_bytes());
/// ```
///
/// See:
/// [`ArenaStringExample`],
/// [`ArenaStringHandleExample`],
/// [`ArenaStringMarkExample`],
/// [`ArenaStringAllocExample`],
/// [`ArenaStringAllocHandleExample`],
/// [`ArenaStringAllocMarkExample`].
///
/// [`ArenaStringExample`]: crate::ArenaStringExample
/// [`ArenaStringHandleExample`]: crate::ArenaStringHandleExample
/// [`ArenaStringMarkExample`]: crate::ArenaStringMarkExample
/// [`ArenaStringAllocExample`]: crate::ArenaStringAllocExample
/// [`ArenaStringAllocHandleExample`]: crate::ArenaStringAllocHandleExample
/// [`ArenaStringAllocMarkExample`]: crate::ArenaStringAllocMarkExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! arena_string· {
    (
        [
            index: $iprim:ident $(+ $Index:ty)?;
            cursor: $cprim:ident $(+ $Cursor:ty)?;
        ]

        $(#[$arena_attr:meta])*
        $vis:vis $Arena:ident $( : $kind:ident)?;

        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident;

        $(
            $(#[$mark_attr:meta])*
            $mvis:vis $Mark:ident $(;)?
        )?
    ) => {
        $crate::__arena_string! { %normalize_index
            [kind: $($kind)?]
            [index: $iprim $(+ $Index)?]
            [cursor: $cprim $(+ $Cursor)?]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $(#[$handle_attr])* $hvis $Handle]
            [mark: $($(#[$mark_attr])* $mvis $Mark)?]
        }
    };
}
#[doc(inline)]
pub use arena_string· as arena_string;
