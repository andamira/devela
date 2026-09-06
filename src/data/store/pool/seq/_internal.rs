// devela/src/data/store/pool/seq/_internal.rs
//
//! Defines [`__pool_seq!`].
//

#[doc(hidden)]
#[macro_export]
macro_rules! __pool_seq· {
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident]
        $($rest:tt)*
    ) => {
        $crate::__pool_seq! { %normalize_generation
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
        $crate::__pool_seq! { %normalize_generation
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
        $crate::__pool_seq! { %generate
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
        $crate::__pool_seq! { %generate
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
            $crate::__pool_seq! {
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
        $crate::__pool_seq! {%backend [kind: static] $($rest)* }
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
        // Reuse the ordinary array pool as the private sequence-identity layer:
        // it owns generations, handle validation, and free sequence slots.
        $crate::__pool_impl_array! {
            [
                index: $iprim + $Index;
                generation: $gprim + $Generation;
            ]
            $MetaPool;
            $Handle;
        }
        // Layer contiguous sequence/cell storage over that identity pool.
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
        $crate::__pool_seq! { %impl_common_core
            [index: $iprim + $Index; cell: $cprim;]
            $vis $Pool;
        }
    };
    (%impl_common_core
        [
            index: $iprim:ident + $Index:ty;
            cell: $cprim:ident;
        ]
        $vis:vis $Pool:ident;
    ) => {
        impl<T, const SEQS: usize, const CELLS: usize> $Pool<T, SEQS, CELLS> {
            /// The maximum fixed sequence capacity representable by this pool.
            $vis const MAX_CAPACITY: usize = {
                match $crate::MaybeNiche::<$Index>::MAX.try_to_usize() {
                    Ok(max) => max.saturating_add(1),
                    Err(_) => usize::MAX,
                }
            };

            /// The maximum fixed cell capacity representable by this pool.
            $vis const MAX_CELL_CAPACITY: usize = $cprim::MAX as usize;
        }
    };
}
pub use __pool_seq· as __pool_seq;
