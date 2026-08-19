// devela/src/data/store/pool/item/define.rs
//
//! Defines [`pool!`].
//

#[doc = crate::_tags!(construction data_structure)]
/// Defines an owning generational pool with static or allocating storage.
#[doc = crate::_doc_meta!{location("data/store/pool")}]
///
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
/// [`capacity`](#method.capacity) reports the usable number of slots supported
/// by the current storage:
///
/// - for a static pool, this is the fixed `CAP`;
/// - for an allocating pool, this is the currently reserved usable capacity,
///   bounded by the index representation.
///
/// [`remaining`](#method.remaining) returns `capacity() - len()`.
///
/// For an allocating pool, `remaining() == 0` does not necessarily mean the
/// pool is full: a later insertion may grow the allocation. [`is_full`]
/// indicates that insertion must fail because neither a vacant slot nor a new
/// representable index exists.
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
/// # use devela::{NonMaxU16, pool};
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
/// [`PoolExample`], [`PoolAllocExample`],
/// [`PoolHandleExample`], [`PoolAllocHandleExample`].
///
/// [`is_full`]: crate::PoolExample::is_full
/// [`PoolExample`]: crate::PoolExample
/// [`PoolAllocExample`]: crate::PoolAllocExample
/// [`PoolHandleExample`]: crate::PoolHandleExample
/// [`PoolAllocHandleExample`]: crate::PoolAllocHandleExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! pool {
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
        $crate::pool! { %normalize_index
            [kind: $($kind)?]
            [index: $iprim $(+ $Index)?]
            [generation: $gprim $(+ $Generation)?]
            [pool: $(#[$pool_attr])* $vis $Pool]
            [handle: $(#[$handle_attr])* $hvis $Handle]
        }
    };
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident]
        $($rest:tt)*
    ) => {
        $crate::pool! { %normalize_generation
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
        $crate::pool! { %normalize_generation
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
        $crate::pool! { %generate
            [kind: $($kind)?]
            [index: $iprim + $Index; generation: $gprim + $gprim;]
            $($rest)*
        }
    };
    (%normalize_generation
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [generation: $gprim:ident + $Generation:ty]
        $($rest:tt)*
    ) => {
        $crate::pool! { %generate
            [kind: $($kind)?]
            [index: $iprim + $Index; generation: $gprim + $Generation;]
            $($rest)*
        }
    };
    (%generate
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty; generation: $gprim:ident + $Generation:ty;]
        [pool: $(#[$pool_attr:meta])* $vis:vis $Pool:ident]
        [handle: $(#[$handle_attr:meta])* $hvis:vis $Handle:ident]
    ) => {
        $crate::handle_gen! {
            [index: $iprim + $Index; generation: $gprim + $Generation;]
            $(#[$handle_attr])* $hvis $Handle
        }
        $crate::pool! { %backend
            [kind: $($kind)?]
            [index: $iprim + $Index; generation: $gprim + $Generation;]
            [pool: $(#[$pool_attr])* $vis $Pool]
            [handle: $hvis $Handle]
        }
    };
    (%backend
        [kind:]
        $($rest:tt)*) => {
        $crate::pool! { %backend [kind: static] $($rest)* }
    };
    (%backend
        [kind: static]
        [index: $iprim:ident + $Index:ty; generation: $gprim:ident + $Generation:ty;]
        [pool: $(#[$pool_attr:meta])* $vis:vis $Pool:ident]
        [handle: $hvis:vis $Handle:ident]
    ) => {
        $crate::__pool_impl_array! {
            [index: $iprim + $Index; generation: $gprim + $Generation;]
            $(#[$pool_attr])* $vis $Pool;
            $hvis $Handle;
        }
    };
    (%backend
        [kind: alloc]
        [index: $iprim:ident + $Index:ty; generation: $gprim:ident + $Generation:ty;]
        [pool: $(#[$pool_attr:meta])* $vis:vis $Pool:ident]
        [handle: $hvis:vis $Handle:ident]
    ) => {
        $crate::__pool_impl_vec! {
            [index: $iprim + $Index; generation: $gprim + $Generation;]
            $(#[$pool_attr])* $vis $Pool;
            $hvis $Handle;
        }
    };
    (%impl_common_core
     $(const$($_c:lifetime)?)?
     [index: $iprim:ident + $Index:ty; generation: $gprim:ident + $Generation:ty;]
     $vis:vis $Pool:ident;
     $hvis:vis $Handle:ident;
    ) => {
            /* capacity */

            /// Returns the number of occupied slots.
            #[must_use]
            $vis const fn len(&self) -> usize { self.len }

            /// Returns whether the pool contains no values.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.len == 0 }

            /// Returns how many additional values fit within the current capacity.
            ///
            /// An allocating pool may grow when this reaches zero
            /// unless [`is_full`][Self::is_full].
            #[must_use]
            $vis const fn remaining(&self) -> usize {
                self.capacity() - self.len
            }

            /* private */

            $(const$($_c)?)? fn __resolve_index(&self, handle: $Handle) -> Option<usize> {
                let index = $crate::unwrap![ok_some?
                    $crate::MaybeNiche(handle.index()).try_to_usize()];
                if index >= self.values.len() { return None; }
                if self.generations[index].ne($crate::MaybeNiche(handle.generation())) {
                    return None;
                }
                $crate::is!{ self.values[index].is_none(), return None }
                Some(index)
            }
            $(const$($_c)?)? fn __handle_at(&self, index: usize) -> Option<$Handle> {
                if index >= self.values.len() || self.values[index].is_none() { return None; }
                let encoded =
                    $crate::unwrap![ok_some? $crate::MaybeNiche::<$Index>::try_from_usize(index)];
                let generation = self.generations[index];
                Some($Handle::new(encoded.get(), generation.get()))
            }
            const fn __next_generation(current: $crate::MaybeNiche<$Generation>)
                -> $crate::MaybeNiche<$Generation> {
                let mut candidate = current.get_prim();
                loop {
                    candidate = candidate.wrapping_add(1);
                    if let Ok(next) = $crate::MaybeNiche::<$Generation>::try_from_prim(candidate) {
                        return next;
                    }
                }
            }
            const fn __index_capacity() -> usize {
                match $crate::MaybeNiche::<$Index>::MAX.try_to_usize() {
                    Ok(max) => max.saturating_add(1),
                    Err(_) => usize::MAX,
                }
            }
    };
    (%impl_common_iter
     [index: $iprim:ident + $Index:ty; generation: $gprim:ident + $Generation:ty;]
     $vis:vis $Pool:ident;
     $hvis:vis $Handle:ident;
    ) => {
            /// Iterates over current handles in ascending slot order.
            $hvis fn handles(&self) -> impl Iterator<Item = $Handle> + '_ {
                self.entries().map(|(handle, _)| handle)
            }
            /// Iterates over current handles and shared values in ascending slot order.
            $hvis fn entries(&self) -> impl Iterator<Item = ($Handle, &T)> + '_ {
                self.values.iter().enumerate()
                    .filter_map(|(index, value)| {
                        let value = value.as_ref()?;
                        let handle = self.__handle_at(index)?;
                        Some((handle, value))
                    })
            }
            /// Iterates over current handles and exclusive values in ascending slot order.
            $hvis fn entries_mut(&mut self) -> impl Iterator<Item = ($Handle, &mut T)> + '_ {
                let generations = &self.generations;
                self.values.iter_mut().enumerate()
                    .filter_map(move |(index_usize, value)| {
                        let value = value.as_mut()?;
                        let generation = generations[index_usize];
                        let index = $crate::MaybeNiche::<$Index>::try_from_usize(index_usize).ok()?;
                        Some(($Handle::new(index.get(), generation.get()), value))
                    })
            }
    };
    (%impl_common_iter_traits
     $(const$($_c:lifetime)?)?
     [index: $iprim:ident + $Index:ty; generation: $gprim:ident + $Generation:ty;]
     $vis:vis $Pool:ident;
     $hvis:vis $Handle:ident;
    ) => {
        impl<'a, T $($($_c)?, const CAP: usize)?> $crate::IteratorInto
            for &'a $Pool<T$($($_c)?, CAP)?> {
            type Item = &'a T;
            type IntoIter = $crate::PoolIter<&'a [$crate::Option<T>]>;
            fn into_iter(self) -> Self::IntoIter { self.iter() }
        }
        impl<'a, T $($($_c)?, const CAP: usize)?> $crate::IteratorInto
            for &'a mut $Pool<T$($($_c)?, CAP)?> {
            type Item = &'a mut T;
            type IntoIter = $crate::PoolIter<&'a mut [$crate::Option<T>]>;
            fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
        }
    };
}
#[doc(inline)]
pub use pool;
