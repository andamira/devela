// devela/src/data/store/pool/seq/define.rs
//
//! Defines [`pool_seq!`].

#[doc = crate::_tags!(construction data_structure)]
/// Defines a generational pool of variable-length contiguous sequences.
#[doc = crate::_doc_meta!{
    location("data/store/pool", macro pool_seq),
}]
/// Each inserted sequence receives a generational handle whose identity is
/// independent of the sequence's physical cell span. A sequence may therefore
/// relocate while its handle remains valid.
///
/// Cells within a sequence preserve their order and remain contiguous, but
/// individual cells have no independent stable identity.
///
/// # Capacity
///
/// The static pool has two independent fixed capacities:
///
/// - `SEQS` is the maximum number of simultaneously live sequences.
/// - `CELLS` is the total number of cells available to their reserved spans.
///
/// The generated type is:
/// `Pool<T, const SEQS: usize, const CELLS: usize>`.
///
/// A sequence has both a logical [`seq_len`](#method.seq_len) and a reserved
/// [`seq_capacity`](#method.seq_capacity). Operations such as
/// [`truncate`](#method.truncate) and [`pop`](#method.pop) reduce the logical
/// length without releasing the remaining reservation.
///
/// [`shrink_to_fit`](#method.shrink_to_fit) releases that unused reservation.
///
/// # Contiguity and fragmentation
///
/// Each sequence occupies one contiguous physical span. Consequently,
/// [`cell_remaining`](#method.cell_remaining) may report enough total free cells
/// for an insertion while no individual free span is large enough.
///
/// [`largest_free_span`](#method.largest_free_span),
/// [`can_insert`](#method.can_insert), and
/// [`is_fragmented_for`](#method.is_fragmented_for) expose this distinction.
///
/// Growth first uses existing reserved capacity, then tries to extend the
/// current span, and otherwise relocates the complete sequence. Relocation
/// preserves the sequence handle.
///
/// Growth does not implicitly compact the pool.
///
/// [`compact`](#method.compact) removes gaps while preserving per-sequence
/// reservations. [`pack`](#method.pack) additionally releases unused
/// per-sequence capacity.
///
/// # Representations
///
/// `index` selects the primitive and optional representation used for sequence
/// slot indices.
///
/// `generation` selects the primitive and optional representation used for
/// sequence generations.
///
/// `cell` selects the unsigned primitive used internally to represent cell
/// offsets, lengths, and capacities. It is not the stored cell type `T`.
///
/// The cell representation must be able to represent `CELLS`.
///
/// Omitting a representation after `+` uses the primitive itself.
///
/// # Handle validity
///
/// Handles are relative to the pool instance that produced them. They do not
/// encode a pool identity.
///
/// Removing a sequence advances its slot generation before that slot can be
/// reused, so stale handles are normally rejected. Generations eventually wrap,
/// so this protection is bounded by the configured generation domain.
///
/// # Cell storage
///
/// The current backend is fixed-capacity and fully initialized.
/// [`new_init`](#method.new_init) initializes the backing cell storage with
/// `T::INIT`. Operations that copy or relocate cell contents require `T: Copy`.
///
/// # Examples
/// ```
/// # use devela::{NonMaxU16, pool_seq};
/// pool_seq! {
///     [
///         index: u8;
///         generation: u16 + NonMaxU16;
///         cell: u16;
///     ]
///     pub Sequences;
///     pub SequenceId;
/// }
///
/// let mut pool = Sequences::<u8, 4, 16>::new_init();
///
/// let word = pool.insert(b"cat").unwrap();
/// let other = pool.insert(b"tree").unwrap();
///
/// assert_eq!(pool.get(word), Some(b"cat".as_slice()));
///
/// // `other` physically follows `word`, so growing `word` may relocate it.
/// assert_eq!(pool.push(word, b'!'), Ok(()));
///
/// // Its semantic identity remains unchanged.
/// assert_eq!(pool.get(word), Some(b"cat!".as_slice()));
/// assert_eq!(pool.get(other), Some(b"tree".as_slice()));
/// ```
///
/// See: [`PoolSeqExample`], [`PoolSeqHandleExample`].
///
/// [`PoolSeqExample`]: crate::PoolSeqExample
/// [`PoolSeqHandleExample`]: crate::PoolSeqHandleExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! pool_seq {
    (
        [
            index: $iprim:ident $(+ $Index:ty)?;
            generation: $gprim:ident $(+ $Generation:ty)?;
            cell: $cprim:ident;
        ]

        $(#[$pool_attr:meta])*
        $vis:vis $Pool:ident $( : $kind:ident)?;

        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident $(;)?
    ) => {
        $crate::pool_seq! { %normalize_index
            [kind: $($kind)?]
            [index: $iprim $(+ $Index)?]
            [generation: $gprim $(+ $Generation)?]
            [cell: $cprim]
            [pool: $(#[$pool_attr])* $vis $Pool]
            [handle: $(#[$handle_attr])* $hvis $Handle]
        }
    };
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident]
        $($rest:tt)*
    ) => {
        $crate::pool_seq! { %normalize_generation
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
        $crate::pool_seq! { %normalize_generation
            [kind: $($kind)?]
            [index: $iprim + $Index]
            $($rest)*
        }
    };
    (%normalize_generation
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [generation: $gprim:ident]
        $($rest:tt)*
    ) => {
        $crate::pool_seq! { %generate
            [kind: $($kind)?]
            [index: $iprim + $Index]
            [generation: $gprim + $gprim]
            $($rest)*
        }
    };
    (%normalize_generation
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [generation: $gprim:ident + $Generation:ty]
        $($rest:tt)*
    ) => {
        $crate::pool_seq! { %generate
            [kind: $($kind)?]
            [index: $iprim + $Index]
            [generation: $gprim + $Generation]
            $($rest)*
        }
    };
    (%generate
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [generation: $gprim:ident + $Generation:ty]
        [cell: $cprim:ident]
        [pool: $(#[$pool_attr:meta])* $vis:vis $Pool:ident]
        [handle: $(#[$handle_attr:meta])* $hvis:vis $Handle:ident]
    ) => {
        $crate::handle_gen! {
            [
                index: $iprim + $Index;
                generation: $gprim + $Generation;
            ]
            $(#[$handle_attr])*
            $hvis $Handle;
        }
        $crate::paste! {
            $crate::pool_seq! {
                %backend
                [kind: $($kind)?]
                [
                    index: $iprim + $Index;
                    generation: $gprim + $Generation;
                    cell: $cprim;
                ]
                [pool: $(#[$pool_attr])* $vis $Pool]
                [handle: $hvis $Handle]
                [private:
                    meta: [<_ $Pool Meta>];
                    free_span: [<_ $Pool FreeSpan>];
                    meta_pool: [<_ $Pool MetaPool>];
                ]
            }
        }
    };
    (%backend
        [kind:]
        $($rest:tt)*) => {
        $crate::pool_seq! {%backend [kind: static] $($rest)* }
    };
    (%backend
        [kind: static]
        [
            index: $iprim:ident + $Index:ty;
            generation: $gprim:ident + $Generation:ty;
            cell: $cprim:ident;
        ]
        [pool: $(#[$pool_attr:meta])* $vis:vis $Pool:ident]
        [handle: $hvis:vis $Handle:ident]
        [private:
            meta: $Meta:ident;
            free_span: $FreeSpan:ident;
            meta_pool: $MetaPool:ident;
        ]
    ) => {
        $crate::__pool_impl_array! {
            [
                index: $iprim + $Index;
                generation: $gprim + $Generation;
            ]
            $MetaPool;
            $Handle;
        }
        $crate::__pool_seq_impl_array! {
            [cell: $cprim]
            [private:
                meta: $Meta;
                free_span: $FreeSpan;
                meta_pool: $MetaPool;
            ]
            $(#[$pool_attr])*
            $vis $Pool;
            $hvis $Handle;
        }
    };
}
#[doc(inline)]
pub use pool_seq;
