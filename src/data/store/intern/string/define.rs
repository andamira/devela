// devela/src/data/store/intern/string/define.rs
//
//! Defines [`intern_string!`].
//

#[cfg(all(doc, feature = "_docs_examples"))]
use crate::InternStringExample;

#[doc = crate::_tags!(construction data_structure)]
/// Defines a UTF-8 string interner with compact canonical symbols.
#[doc = crate::_doc_meta! {
    location("data/store/intern", macro intern_string),
}]
/// The generated interner stores each distinct string once and returns a compact
/// symbol identifying its canonical copy. Interning an equal string returns the
/// existing symbol without storing another copy.
///
/// Unlike [`arena_string!`][crate::arena_string], where every insertion receives
/// a distinct identity, `intern_string!` gives equal strings one shared identity.
///
/// # Storage regimes
///
/// Two storage regimes are available:
///
/// - **Static** — the default.
///
///   The generated type is
///   `Interner<const STRINGS: usize, const BYTES: usize, const SLOTS: usize>`.
///   It owns fixed-capacity packed string storage and a fixed open-addressed
///   lookup table, and does not allocate.
///
///   `STRINGS` limits canonical strings, `BYTES` limits packed UTF-8 bytes,
///   and `SLOTS` sets the lookup-table size. `SLOTS` must be at least `STRINGS`.
///
///   The optional `: static` selector may be written explicitly or omitted.
///
/// - **Allocating** — selected with `: alloc`.
///
///   The generated type is `Interner`. It requires the `alloc` feature.
///   Canonical strings and packed bytes grow as needed, while lookup slots grow
///   and rehash when the active table is full. Rehashing never changes symbols.
///
///   [`with_capacity`][crate::InternStringAllocExample::with_capacity]
///   reserves initial string, byte, and lookup-slot capacities independently.
///
/// # Representations
///
/// The declaration configures two compact representations:
///
/// - `index` determines the canonical-string symbol representation.
/// - `cursor` represents cumulative UTF-8 byte ends in packed storage.
///
/// Each may be written as `primitive + representation`;
/// omitting the representation uses the primitive itself.
/// Both representations must be unsigned, contiguous, and contain zero.
///
/// The maximum `index` value is reserved as the empty lookup-slot sentinel and
/// is never issued as a symbol. Static and allocating interners therefore share
/// the same symbol domain. [`MAX_CAPACITY`][InternStringExample::MAX_CAPACITY]
/// reports its maximum canonical-string count.
///
/// [`MAX_BYTE_CAPACITY`][InternStringExample::MAX_BYTE_CAPACITY]
/// reports the maximum packed-byte frontier representable by `cursor`.
///
/// # Canonicalization
///
/// Hashing selects the initial lookup slot; it does not define equality.
/// Probe collisions are resolved by comparing the actual UTF-8 bytes.
///
/// [`find`][InternStringExample::find] performs lookup without mutation.
/// [`intern`][InternStringExample::intern] inserts only when no equal canonical
/// string exists. An already canonical string can therefore still be interned
/// when no new canonical string would fit.
///
/// # Symbols and clearing
///
/// Symbols contain only the canonical string index.
/// They carry no interner identity or generation.
///
/// [`clear`][InternStringExample::clear] removes all canonical strings.
/// Previously issued symbols stop resolving, but later interning may reuse
/// the same index, so an old symbol may resolve again to a different string.
///
/// # Packed bytes
///
/// [`as_bytes`][InternStringExample::as_bytes] exposes canonical UTF-8 bytes
/// concatenated in insertion order. String boundaries are not encoded in that
/// byte slice; [`get`][InternStringExample::get] resolves a symbol to its string.
///
/// # Example
///
/// ```
/// use devela::{NonMaxU8, NonMaxU16, intern_string};
///
/// intern_string! {
///     [
///         index: u8 + NonMaxU8;
///         cursor: u16 + NonMaxU16;
///     ]
///
///     pub Symbols;
///     pub Symbol;
/// }
///
/// let mut symbols = Symbols::<8, 64, 16>::new();
///
/// let alpha = symbols.intern("alpha").unwrap();
/// let beta = symbols.intern("beta").unwrap();
///
/// assert_eq!(symbols.intern("alpha"), Some(alpha));
/// assert_ne!(alpha, beta);
/// assert_eq!(symbols.get(alpha), Some("alpha"));
/// assert_eq!(symbols.find("beta"), Some(beta));
/// assert_eq!(symbols.len(), 2);
/// ```
///
/// See:
/// - [`InternStringExample`], [`InternStringSymbolExample`].
/// - [`InternStringAllocExample`], [`InternStringAllocSymbolExample`].
///
/// [`InternStringSymbolExample`]: crate::InternStringSymbolExample
/// [`InternStringAllocExample`]: crate::InternStringAllocExample
/// [`InternStringAllocSymbolExample`]: crate::InternStringAllocSymbolExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! intern_string· {
    (
        [
            index: $iprim:ident $(+ $Index:ty)?;
            cursor: $cprim:ident $(+ $Cursor:ty)?;
        ]

        $(#[$interner_attr:meta])*
        $vis:vis $Interner:ident $( : $kind:ident)?;

        $(#[$symbol_attr:meta])*
        $svis:vis $Symbol:ident $(;)?
    ) => {
        $crate::__intern_string! { %normalize_index
            [kind: $($kind)?]
            [index: $iprim $(+ $Index)?]
            [cursor: $cprim $(+ $Cursor)?]
            [interner: $(#[$interner_attr])* $vis $Interner]
            [symbol: $(#[$symbol_attr])* $svis $Symbol]
        }
    };
}
#[doc(inline)]
pub use intern_string· as intern_string;
