// devela/src/data/store/pool/item/_internal.rs
//
//! Defines [`__pool!`].
//

#[doc(hidden)]
#[macro_export]
macro_rules! __pool· {
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident]
        $($rest:tt)*
    ) => {
        $crate::__pool! { %normalize_generation
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
        $crate::__pool! { %normalize_generation
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
        $crate::__pool! { %generate
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
        $crate::__pool! { %generate
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
        $crate::__pool! { %backend
            [kind: $($kind)?]
            [index: $iprim + $Index; generation: $gprim + $Generation;]
            [pool: $(#[$pool_attr])* $vis $Pool]
            [handle: $hvis $Handle]
        }
    };
    (%backend
        [kind:]
        $($rest:tt)*) => {
        $crate::__pool! { %backend [kind: static] $($rest)* }
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

            /// The maximum fixed capacity representable by this pool.
            $vis const MAX_CAPACITY: usize = {
                match $crate::MaybeNiche::<$Index>::MAX.try_to_usize() {
                    Ok(max) => max.saturating_add(1),
                    Err(_) => usize::MAX,
                }
            };

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

            $(const$($_c)?)? fn _resolve_index(&self, handle: $Handle) -> Option<usize> {
                let index = $crate::unwrap![ok_some?
                    $crate::MaybeNiche(handle.get_index()).try_to_usize()];
                if index >= self.values.len() { return None; }
                if self.generations[index].ne($crate::MaybeNiche(handle.get_generation())) {
                    return None;
                }
                $crate::is!{ self.values[index].is_none(), return None }
                Some(index)
            }
            $(const$($_c)?)? fn _handle_at(&self, index: usize) -> Option<$Handle> {
                if index >= self.values.len() || self.values[index].is_none() { return None; }
                let encoded =
                    $crate::unwrap![ok_some? $crate::MaybeNiche::<$Index>::try_from_usize(index)];
                let generation = self.generations[index];
                Some($Handle::new(encoded.get(), generation.get()))
            }
            const fn _next_generation(current: $crate::MaybeNiche<$Generation>)
                -> $crate::MaybeNiche<$Generation> {
                let mut candidate = current.get_prim();
                loop {
                    candidate = candidate.wrapping_add(1);
                    if let Ok(next) = $crate::MaybeNiche::<$Generation>::try_from_prim(candidate) {
                        return next;
                    }
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
                        let handle = self._handle_at(index)?;
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
pub use __pool· as __pool;
