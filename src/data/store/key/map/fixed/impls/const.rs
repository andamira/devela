// devela/src/data/store/key/map/fixed/impls/constrs

#[doc(hidden)]
#[macro_export]
macro_rules! __map_impl_const {
    (
        $(#[$attr:meta])*
        $vis:vis $NAME:ident, KEY:$KEY:ty,
        EMPTY:$EMPTY:expr, TOMB:$TOMB:expr,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        $(#[$attr])*
        #[doc = concat!("A fully `const` static hashmap with compile-time `",
            stringify!($KEY), "` keys.")]
        ///
        /// This variant defines its `EMPTY` and `TOMB` markers as **associated constants**
        /// and exposes only `const fn` methods, allowing construction and use in
        /// compile-time contexts.
        ///
        /// All hashing, probing, and insertion logic mirror the runtime variant,
        /// but with stricter compile-time guarantees and no stored marker fields.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis struct $NAME<K: Copy, V, const N: usize> {
            keys: [K; N],
            values: [V; N],
        }

        $crate::__map_impl_shared![$NAME, KEY:$KEY, HASHER: | $HASH_ARG | $HASH_EXPR];

        #[allow(unused)]
        impl<V, const N: usize> $NAME<$KEY, V, N> {
            /// Compile-time key value used to mark empty slots.
            pub const EMPTY: $KEY = $EMPTY as $KEY;
            /// Compile-time key value used to mark deleted slots.
            pub const TOMB: $KEY = $TOMB as $KEY;

            /// Returns the key value used to mark empty slots.
            pub const fn empty(&self) -> $KEY { $EMPTY }
            /// Returns the key value used to mark deleted slots.
            pub const fn tomb(&self) -> $KEY { $TOMB }
        }

        impl<V: Copy + $crate::ConstInit, const N: usize>
            $crate::ConstInit for $NAME<$KEY, V, N> {
            const INIT: Self = Self::new();
        }
        impl<V: Default, const N: usize> Default for $NAME<$KEY, V, N> {
            /// Creates an empty hashmap.
            ///
            /// # Panics
            /// Panics in debug if `EMPTY` and `TOMB` are equal,
            /// or if any of them are out of range for `$KEY`.
            #[allow(unexpected_cfgs, reason = "init_array")]
            fn default() -> Self {
                Self:: debug_assert_invariants();
                Self {
                    keys: [$EMPTY; N],
                    values: $crate::init_array![default [V; N], "safe_data", "unsafe_array"],
                }
            }
        }

        #[allow(unused)]
        impl<V: Copy + $crate::ConstInit, const N: usize> $NAME<$KEY, V, N> {
            /// Creates an empty hashmap.
            ///
            /// # Panics
            /// Panics in debug if `EMPTY` and `TOMB` are equal,
            /// or if any of them are out of range for `$KEY`.
            #[allow(clippy::float_cmp_const)]
            pub const fn new() -> Self {
                Self:: debug_assert_invariants();
                Self {
                    keys: [$EMPTY; N],
                    values: [V::INIT; N],
                }
            }
        }

        #[allow(unused)]
        impl<V, const N: usize> $NAME<$KEY, V, N> {
            /// Retrieves some shared reference to the value associated with the given key.
            pub const fn get_ref(&self, key: $KEY) -> Option<&V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                $crate::whilst! { i in 0..N; {
                    if self.keys[index] == key { return Some(&self.values[index]); }
                    if self.keys[index] == self.empty() { return None; }
                    index = (index + 1) % N;
                }}
                None
            }
            /// Retrieves some exclusive reference to the value associated with the given key.
            pub const fn get_mut(&mut self, key: $KEY) -> Option<&mut V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                $crate::whilst! { i in 0..N; {
                    if self.keys[index] == key { return Some(&mut self.values[index]); }
                    if self.keys[index] == self.empty() { return None; }
                    index = (index + 1) % N;
                }}
                None
            }
            /// Retrieves an entry for a given key.
            pub const fn entry(&mut self, key: $KEY) -> $crate::MapFixedEntry<'_, V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut tombstone_index = None;
                $crate::whilst! { i in 0..N; {
                    if self.keys[index] == self.empty() {
                        return $crate::MapFixedEntry::Vacant(
                            $crate::unwrap![some_or tombstone_index, index]);
                    }
                    if self.keys[index] == key {
                        return $crate::MapFixedEntry::Occupied(&mut self.values[index]);
                    }
                    if self.keys[index] == self.tomb() && tombstone_index.is_none() {
                        tombstone_index = Some(index);
                    }
                    index = (index + 1) % N;
                }}
                // If full, return N (invalid index)
                $crate::MapFixedEntry::Vacant($crate::unwrap![some_or tombstone_index, N])
            }

            /* introspection */

            /// Returns the number of occupied slots in the hashmap.
            pub const fn len(&self) -> usize {
                let mut count = 0;
                $crate::whilst! { i in 0..N; {
                    if self.keys[i] != self.empty() && self.keys[i] != self.tomb() { count += 1; }
                }}
                count
            }
            /// Returns `true` if the hashmap contains no entries.
            pub const fn is_empty(&self) -> bool { self.len() == 0 }

            /// Returns `true` if the hashmap is completely full.
            pub const fn is_full(&self) -> bool { self.len() == N }

            /// Determines if rebuilding the table would improve efficiency.
            ///
            /// # Heuristic:
            /// - Rebuild if `TOMB` slots exceed `N / 2` (half the table size).
            pub const fn should_rebuild(&self) -> bool { self.deleted_count() >= N / 2 }

            /// Returns the number of deleted (TOMB) slots.
            pub const fn deleted_count(&self) -> usize {
                let mut count = 0;
                $crate::whilst! { i in 0..N; {
                    if self.keys[i] == self.tomb() { count += 1; }
                }}
                count
            }

            /// Returns the load factor as a fraction of total capacity.
            pub const fn load_factor(&self) -> f32 { self.len() as f32 / N as f32 }

            /* utility */

            /// Ensures the given key is not EMPTY or TOMB.
            const fn debug_assert_valid_key(key: $KEY) {
                debug_assert!(key != $EMPTY, "Key cannot be `EMPTY` marker");
                debug_assert!(key != $TOMB, "Key cannot be `TOMB` marker");
            }
            /// Ensures the type invariants hold.
            const fn debug_assert_invariants() {
                debug_assert![$EMPTY != $TOMB, "`$EMPTY` and `$TOMB` must be distinct"];
                debug_assert![($EMPTY as i128) >= (<$KEY>::MIN as i128)
                    && ($EMPTY as i128) <= (<$KEY>::MAX as i128),
                    "`$EMPTY` value is out of range for type `$KEY`"];
                debug_assert![($TOMB as i128) >= (<$KEY>::MIN as i128)
                    && ($TOMB as i128) <= (<$KEY>::MAX as i128),
                    "`$TOMB` value is out of range for type `$KEY`"];
            }
        }

        #[allow(unused)]
        impl<V: Copy, const N: usize> $NAME<$KEY, V, N> {
            /// Inserts a key-value pair.
            ///
            /// # Returns
            /// - `Ok(())` if the insertion succeeds.
            /// - `Err(`[`NotEnoughSpace`][crate::NotEnoughSpace]`)` if no slots are available.
            ///
            /// # Behavior
            /// - Computes the **hash index** of the key.
            /// - If the slot is `EMPTY`, inserts immediately.
            /// - If the slot contains `TOMB`, the first `TOMB` encountered is
            ///   **used if no empty slots exist earlier in probing**.
            /// - If the slot contains another key, **probes forward** until an open slot is found.
            /// - If no open slots exist, returns an error.
            #[allow(clippy::float_cmp, clippy::float_cmp_const)]
            pub const fn insert(&mut self, key: $KEY, value: V)
                -> Result<(), $crate::NotEnoughSpace> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut tombstone_index = None;
                $crate::whilst! { i in 0..N; {
                    if self.keys[index] == key {
                        self.values[index] = value;
                        return Ok(());
                    }
                    if self.keys[index] == self.empty() {
                        let slot = $crate::unwrap![some_or tombstone_index, index];
                        self.keys[slot] = key;
                        self.values[slot] = value;
                        return Ok(());
                    }
                    if self.keys[index] == self.tomb() && tombstone_index.is_none() {
                        tombstone_index = Some(index);
                    }
                    index = (index + 1) % N;
                }}
                if let Some(slot) = tombstone_index {
                    self.keys[slot] = key;
                    self.values[slot] = value;
                    Ok(())
                } else {
                    Err($crate::NotEnoughSpace(Some(1)))
                }
            }
            /// Retrieves a value by key.
            ///
            /// # Returns
            /// - `Some(value)` if the key exists.
            /// - `None` if the key is missing.
            ///
            /// # Behavior
            /// - Searches for the key using **linear probing**.
            /// - If a `TOMB` (deleted slot) is encountered, it **continues probing**.
            /// - If an `EMPTY` slot is reached, the key is **not in the table**.
            #[allow(clippy::float_cmp, clippy::float_cmp_const)]
            pub const fn get(&self, key: $KEY) -> Option<V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                $crate::whilst! { i in 0..N; {
                    if self.keys[index] == key { return Some(self.values[index]); }
                    if self.keys[index] == self.empty() { return None; } // end of probe chain
                    index = (index + 1) % N;
                }}
                None
            }
        }

        #[allow(unused)]
        impl<V: Copy + $crate::ConstInit, const N: usize> $NAME<$KEY, V, N> {
            /// Removes a key-value pair.
            ///
            /// # Returns
            /// - `true` if the key was found and removed.
            /// - `false` if the key was not found in the map.
            ///
            /// # Behavior
            /// - Marks the slot as deleted (`TOMB`).
            /// - Future lookups will continue probing past deleted entries.
            /// - **Does NOT free the slot for immediate reuse**.
            /// - New insertions only reuse a `TOMB` slot if no earlier `EMPTY` slots exist.
            #[allow(clippy::float_cmp, clippy::float_cmp_const)]
            pub const fn remove(&mut self, key: $KEY) -> bool {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                $crate::whilst! { i in 0..N; {
                    if self.keys[index] == key { self.keys[index] = self.tomb(); return true; }
                    if self.keys[index] == self.empty() { return false; }
                    index = (index + 1) % N;
                }}
                false
            }
            /// Removes a key-value pair and optionally rebuilds the table.
            ///
            /// # Behavior
            /// - Calls `remove()`, returning `true` if the key was found.
            /// - If `should_rebuild()` returns `true`, calls `rebuild()`.
            pub const fn remove_rebuild(&mut self, key: $KEY) -> bool {
                let removed = self.remove(key);
                if removed && self.should_rebuild() { self.rebuild(); }
                removed
            }
            /// Rebuilds the table by removing `TOMB` slots and optimizing key placement.
            ///
            /// Calls [`Self::rebuilt()`] and replaces `self` with the optimized table.
            ///
            /// # When to Call?
            /// - When **many deletions have occurred**.
            /// - If lookups start taking significantly longer.
            pub const fn rebuild(&mut self) { *self = self.rebuilt(); }

            /// Returns a rebuilt version of the table with `TOMB` slots removed.
            ///
            /// Creates a new table and reinserts all valid keys while preserving the probe order.
            ///
            /// # Complexity
            /// - **O(N)** worst-case when all slots are occupied.
            pub const fn rebuilt(&self) -> Self {
                let mut new_table = Self::new();
                $crate::whilst! { i in 0..N; {
                    if self.keys[i] != self.empty() && self.keys[i] != self.tomb() {
                        let _ = new_table.insert(self.keys[i], self.values[i]);
                    }
                }}
                new_table
            }
        }
    };
}
