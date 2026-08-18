// devela/src/data/store/pool/seq/define.rs
//
//! Defines [`pool_seq!`].

#[doc = crate::_tags!(construction data_structure)]
/// Defines a generational pool of variable-length contiguous sequences.
#[doc = crate::_doc_meta!{location("data/store")}]
///
/// Sequence handles remain stable while their physical cell spans may be
/// reclaimed or relocated. Cells within each sequence remain ordered and
/// contiguous, but have no independent stable identity.
///
/// The static backend owns fixed capacities for both sequences and cells.
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
