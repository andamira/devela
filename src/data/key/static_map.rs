// devela::data::key::map::static
//
//! Defines the [`define_static_map!`] macro.
//

#[cfg(doc)]
define_static_map![ExampleStaticMapU16, KEY: u16];

/// Build a custom static hashmap, compile-time-friendly.
///
/// # Arguments
/// - `$NAME`:      the name of the new hashmap.
/// - `$K`:         the integer primitive type for the keys.
///
/// optional:
/// - `$EMPTY`:     the $K value for empty entries.
/// - `$TOMB`:      the $K value for deleted entries.
/// - `$HASH_ARG`:  the argument representing the byte slice.
/// - `$HASH_EXPR`: the const hasher expression using `$HASH_ARG`.
///
/// # Notes
/// - values `V` have to be `Copy` + `ConstDefault`.
/// - keys `$K` can be any primitive integers, floats or `char`.
/// - Two specific `$K` values are reserved to indicate empty deleted keys.
///   They default to `MIN` and `MAX`, respectively, but can be customized.
/// - The default hasher is [`HasherFx`][crate::HasherFx].
///
/// # Examples
///
/// See the example type: [`ExampleStaticMapU16`].
///
/// Basic usage
/// ```
/// # use devela::define_static_map;
/// // Define a static hashmap with `u16` keys and default hasher
/// define_static_map![ExampleMap, KEY: u16];
///
/// let mut map = ExampleMap::<u16, u32, 8>::new();
///
/// // Insert key-value pairs
/// map.insert(1, 100).unwrap();
/// map.insert(2, 200).unwrap();
///
/// // Retrieve values
/// assert_eq!(map.get(1), Some(100));
/// assert_eq!(map.get(2), Some(200));
/// assert_eq!(map.get(3), None); // Key not found
///
/// // Delete a key
/// assert!(map.remove(1));
/// assert_eq!(map.get(1), None);
///
/// // Check introspection methods
/// assert_eq!(map.len(), 1);
/// assert!(!map.is_empty());
/// assert!(!map.is_full());
///
/// // Rebuild after deletions to optimize probing
/// if map.should_rebuild() {
///     map.rebuild();
/// }
/// ```
///
/// Custom hashers
/// ```
/// # use devela::{define_static_map, HasherFx};
/// // Define a static hashmap using `HasherFx` with a custom seed
/// define_static_map![ExampleMapFxSeeded, KEY: u16,
///     HASHER: |b| HasherFx::<usize>::hash_bytes_with_seed(123, b)
/// ];
/// let mut map = ExampleMapFxSeeded::<u16, u32, 8>::new();
/// map.insert(1, 100).unwrap();
/// assert_eq!(map.get(1), Some(100));
///
/// # #[cfg(feature = "hash")] {
/// # use devela::HasherPengy;
/// // Define a static hashmap using a stateful pengy hasher
/// define_static_map![ExampleMapPengy, KEY: u16,
///     HASHER: |b| {
///         let mut p = HasherPengy::new();
///         p.process(b);
///         p.digest() as usize
///     }
/// ];
/// let mut map = ExampleMapPengy::<u16, u32, 8>::new();
/// map.insert(1, 100).unwrap();
/// assert_eq!(map.get(1), Some(100));
/// # }
/// ```
#[macro_export]
macro_rules! define_static_map {
    (
        // Default constructor
        $NAME:ident, KEY:$K:ty $(,)?
    ) => {
        $crate::define_static_map![ $NAME, KEY:$K,
            EMPTY:<$K>::MIN, TOMB:<$K>::MAX,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];
    };
    (
        // Custom Empty/Tomb, Default Hasher
        $NAME:ident, KEY:$K:ty, EMPTY:$EMPTY:expr, TOMB:$TOMB:expr $(,)?
    ) => {
        $crate::define_static_map![ $NAME, KEY:$K,
            EMPTY:$EMPTY, TOMB:$TOMB,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];
    };
    (
        // Custom Hasher, Default Empty/Tomb
        $NAME:ident, KEY:$K:ty, HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        $crate::define_static_map![$NAME, KEY:$K, EMPTY:<$K>::MIN, TOMB:<$K>::MAX,
            HASHER: | $HASH_ARG | $HASH_EXPR
        ];
    };
    (
        // Fully Customizable
        $NAME:ident, KEY:$K:ty, EMPTY:$EMPTY:expr, TOMB:$TOMB:expr,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        #[doc = concat!("A custom static hashmap with `", stringify!($K), "` keys.")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $NAME<K: Copy, V: Copy + $crate::ConstDefault, const N: usize> {
            keys: [K; N],
            values: [V; N],
        }

        impl<V: Copy + $crate::ConstDefault, const N: usize> Default for $NAME<$K, V, N> {
            fn default() -> Self { Self::new() }
        }

        #[allow(unused)]
        impl<V: Copy + $crate::ConstDefault, const N: usize> $NAME<$K, V, N> {
            /// Special marker value for deleted slots.
            pub const TOMB: $K = $TOMB as $K;
            /// Special marker value for empty slots.
            pub const EMPTY: $K = $EMPTY as $K;

            /// Creates an empty hashmap.
            ///
            /// # Panics
            /// Panics in debug if `EMPTY` and `TOMB` are equal,
            /// or if any of them are out of range for `$K`.
            #[allow(clippy::float_cmp_const)]
            pub const fn new() -> Self {
                debug_assert![$EMPTY != $TOMB, "`$EMPTY` and `$TOMB` must be distinct"];
                debug_assert![($EMPTY as i128) >= (<$K>::MIN as i128)
                    && ($EMPTY as i128) <= (<$K>::MAX as i128),
                    "`$EMPTY` value is out of range for type `$K`"];
                debug_assert![($TOMB as i128) >= (<$K>::MIN as i128)
                    && ($TOMB as i128) <= (<$K>::MAX as i128),
                    "`$TOMB` value is out of range for type `$K`"];
                Self {
                    keys: [Self::EMPTY; N],
                    values: [V::DEFAULT; N],
                }
            }

            /// Inserts a key-value pair.
            ///
            /// # Returns
            /// - `Ok(())` if the insertion succeeds.
            /// - `Err([`NotEnoughSpace`])` if no slots are available.
            ///
            /// # Behavior
            /// - Computes the **hash index** of the key.
            /// - If the slot is `EMPTY`, inserts immediately.
            /// - If the slot contains `TOMB`, the first `TOMB` encountered is
            ///   **used if no empty slots exist earlier in probing**.
            /// - If the slot contains another key, **probes forward** until an open slot is found.
            /// - If no open slots exist, returns an error.
            #[allow(clippy::float_cmp, clippy::float_cmp_const)]
            pub const fn insert(&mut self, key: $K, value: V)
                -> Result<(), $crate::NotEnoughSpace> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                let mut tombstone_index = None;
                while i < N {
                    if self.keys[index] == Self::EMPTY || self.keys[index] == Self::TOMB {
                        let slot = if let Some(tomb) = tombstone_index { tomb } else { index };
                        self.keys[slot] = key;
                        self.values[slot] = value;
                        return Ok(());
                    }
                    if self.keys[index] == key {
                        self.values[index] = value; // Overwrite existing value
                        return Ok(());
                    }
                    if self.keys[index] == Self::TOMB && tombstone_index.is_none() {
                        tombstone_index = Some(index);
                    }
                    index = (index + 1) % N;
                    i += 1;
                }
                Err($crate::NotEnoughSpace(Some(1)))
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
            pub const fn get(&self, key: $K) -> Option<V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { return Some(self.values[index]); }
                    if self.keys[index] == Self::EMPTY { return None; } // end of probe chain
                    index = (index + 1) % N;
                    i += 1;
                }
                None
            }

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
            pub const fn remove(&mut self, key: $K) -> bool {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { self.keys[index] = Self::TOMB; return true; }
                    if self.keys[index] == Self::EMPTY { return false; }
                    index = (index + 1) % N;
                    i += 1;
                }
                false
            }
            /// Removes a key-value pair and optionally rebuilds the table.
            ///
            /// # Behavior
            /// - Calls `remove()`, returning `true` if the key was found.
            /// - If `should_rebuild()` returns `true`, calls `rebuild()`.
            pub const fn remove_rebuild(&mut self, key: $K) -> bool {
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
                let mut i = 0;
                while i < N {
                    if self.keys[i] != Self::EMPTY && self.keys[i] != Self::TOMB {
                        let _ = new_table.insert(self.keys[i], self.values[i]);
                    }
                    i += 1;
                }
                new_table
            }

            /* introspection */

            /// Determines if rebuilding the table would improve efficiency.
            ///
            /// # Heuristic:
            /// - Rebuild if `TOMB` slots exceed `N / 2` (half the table size).
            pub const fn should_rebuild(&self) -> bool {
                self.deleted_count() >= N / 2
            }

            /// Returns the number of occupied slots in the hashmap.
            pub const fn len(&self) -> usize {
                let mut count = 0;
                let mut i = 0;
                while i < N {
                    if self.keys[i] != Self::EMPTY && self.keys[i] != Self::TOMB { count += 1; }
                    i += 1;
                }
                count
            }

            /// Returns the total capacity of the hashmap (fixed at `N`).
            pub const fn capacity(&self) -> usize {
                N
            }

            /// Returns `true` if the hashmap contains no entries.
            pub const fn is_empty(&self) -> bool {
                self.len() == 0
            }

            /// Returns `true` if the hashmap is completely full.
            pub const fn is_full(&self) -> bool {
                self.len() == N
            }

            /// Returns the number of deleted (TOMB) slots.
            pub const fn deleted_count(&self) -> usize {
                let mut count = 0;
                let mut i = 0;
                while i < N {
                    if self.keys[i] == Self::TOMB { count += 1; }
                    i += 1;
                }
                count
            }

            /// Returns the load factor as a fraction of total capacity.
            pub const fn load_factor(&self) -> f32 {
                self.len() as f32 / N as f32
            }

            /* utility */

            /// Computes a hash index.
            #[$crate::compile(not(same($K, char)))] // for integers and floats
            pub const fn hash_index(&self, key: $K) -> usize {
                let $HASH_ARG = &key.to_le_bytes();
                let expr = $HASH_EXPR;
                expr % N
            }
            /// Computes a hash index.
            #[$crate::compile(same($K, char))] // only for chars
            pub const fn hash_index(&self, key: $K) -> usize {
                let $HASH_ARG = &(key as u32).to_le_bytes();
                let expr = $HASH_EXPR;
                expr % N
            }

            /// Ensures the given key is not EMPTY or TOMB.
            const fn debug_assert_valid_key(key: $K) {
                debug_assert!(key != Self::EMPTY, "Key cannot be `EMPTY` marker");
                debug_assert!(key != Self::TOMB, "Key cannot be `TOMB` marker");
            }
        }
    };
}
pub use define_static_map;

#[cfg(test)]
mod k_u8 {
    super::define_static_map![TestMapU8, KEY:u8];

    #[test]
    fn map_custom_empty_tomb() {
        define_static_map![TestMapU8Custom, KEY: u8, EMPTY: 1, TOMB: 66];
        let mut _map = TestMapU8Custom::<u8, u32, 4>::new();
    }
    #[test]
    fn map_insert_get() {
        let mut map = TestMapU8::<u8, u32, 4>::new();
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.get(1), Some(100));
        assert_eq!(map.get(2), None);
    }
    #[test]
    fn map_insert_overwrite() {
        let mut map = TestMapU8::<u8, u32, 4>::new();
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.insert(1, 200), Ok(())); // overwrite
        assert_eq!(map.get(1), Some(200));
    }
    #[test]
    fn map_remove() {
        let mut map = TestMapU8::<u8, u32, 4>::new();
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.remove(1), true);
        assert_eq!(map.get(1), None);
        assert_eq!(map.remove(1), false); // already removed
    }
    #[test]
    fn should_rebuild() {
        let mut map = TestMapU8::<u8, u32, 8>::new();
        assert!(!map.should_rebuild());
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.insert(2, 200), Ok(()));
        assert_eq!(map.insert(3, 300), Ok(()));
        assert_eq!(map.insert(4, 400), Ok(()));
        assert!(!map.should_rebuild());
        // remove some values but not enough to trigger a rebuild
        assert_eq!(map.remove(2), true);
        assert_eq!(map.remove(4), true);
        assert!(!map.should_rebuild());
        // remove more values to pass the threshold
        assert_eq!(map.remove(1), true);
        assert_eq!(map.remove(3), true);
        assert!(map.should_rebuild()); // now it should require a rebuild
    }
    #[test]
    fn map_collision() {
        let mut map = TestMapU8::<u8, u32, 2>::new();
        assert_eq!(map.hash_index(1), map.hash_index(3)); // make sure keys collide
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.insert(3, 200), Ok(())); // collides with 1, probes forward
        assert_eq!(map.get(1), Some(100));
        assert_eq!(map.get(3), Some(200));
    }
}
#[cfg(test)] #[rustfmt::skip]
mod k_u8_panicking {
    super::define_static_map![TestMapU8, KEY:u8];
    #[allow(unused)] type M = TestMapU8::<u8, u32, 4>;
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `EMPTY` marker")]
    fn insert_empty_key() { let mut m = M::new(); let _ = m.insert(M::EMPTY, 42); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `TOMB` marker")]
    fn insert_tombstone_key() { let mut m = M::new(); let _ = m.insert(M::TOMB, 42); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `EMPTY` marker")]
    fn get_empty_key() { let m = M::new(); let _ = m.get(M::EMPTY); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `TOMB` marker")]
    fn get_tombstone_key() { let m = M::new(); let _ = m.get(M::TOMB); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `EMPTY` marker")]
    fn remove_empty_key() { let mut m = M::new(); let _ = m.remove(M::EMPTY); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `TOMB` marker")]
    fn remove_tombstone_key() { let mut m = M::new(); let _ = m.remove(M::TOMB); }
}
#[cfg(test)]
mod k_f32 {
    super::define_static_map![TestMapF32, KEY:f32];

    #[test]
    fn map_custom_empty_tomb() {
        define_static_map![TestMapF32Custom,
            KEY: f32, EMPTY: f32::NEG_INFINITY, TOMB: f32::INFINITY];
        let mut _map = TestMapF32Custom::<f32, u32, 4>::new();
    }
    #[test]
    fn map_insert_get() {
        let mut map = TestMapF32::<f32, u32, 4>::new();
        assert_eq!(map.insert(0.23, 100), Ok(()));
        assert_eq!(map.get(0.23), Some(100));
        assert_eq!(map.get(1.64), None);
    }
    #[test]
    fn map_insert_overwrite() {
        let mut map = TestMapF32::<f32, u32, 4>::new();
        assert_eq!(map.insert(0.51, 100), Ok(()));
        assert_eq!(map.insert(0.51, 200), Ok(())); // overwrite
        assert_eq!(map.get(0.51), Some(200));
    }
    #[test]
    fn map_remove() {
        let mut map = TestMapF32::<f32, u32, 4>::new();
        assert_eq!(map.insert(0.33, 100), Ok(()));
        assert_eq!(map.remove(0.33), true);
        assert_eq!(map.get(0.33), None);
        assert_eq!(map.remove(0.33), false); // already removed
    }
    #[test]
    fn map_collision() {
        let mut map = TestMapF32::<f32, u32, 2>::new();
        assert_eq!(map.hash_index(0.3), map.hash_index(1.3)); // make sure keys collide
        assert_eq!(map.insert(0.3, 100), Ok(()));
        assert_eq!(map.insert(1.3, 200), Ok(())); // collides with 1, probes forward
        assert_eq!(map.get(0.3), Some(100));
        assert_eq!(map.get(1.3), Some(200));
    }
}
#[cfg(test)]
mod k_char {
    super::define_static_map![TestMapChar, KEY:char];

    #[test]
    fn map_custom_empty_tomb() {
        define_static_map![TestMapCharCustom, KEY: char, EMPTY: '\0', TOMB: 'Ŧ'];
        let mut map = TestMapCharCustom::<char, u32, 4>::new();
        assert_eq!(map.insert('a', 100), Ok(()));

        // can use u8 values, because they cast to `char`
        define_static_map![TestMapCharCustom2, KEY: char, EMPTY: 0u8, TOMB: 1u8];
    }
    #[test]
    fn map_insert_get() {
        let mut map = TestMapChar::<char, u32, 4>::new();
        assert_eq!(map.insert('a', 100), Ok(()));
        assert_eq!(map.get('a'), Some(100));
        assert_eq!(map.get('b'), None);
    }
    #[test]
    fn map_insert_overwrite() {
        let mut map = TestMapChar::<char, u32, 4>::new();
        assert_eq!(map.insert('€', 100), Ok(()));
        assert_eq!(map.insert('€', 200), Ok(())); // overwrite
        assert_eq!(map.get('€'), Some(200));
    }
    #[test]
    fn map_remove() {
        let mut map = TestMapChar::<char, u32, 4>::new();
        assert_eq!(map.insert('x', 100), Ok(()));
        assert_eq!(map.remove('x'), true);
        assert_eq!(map.get('x'), None);
        assert_eq!(map.remove('x'), false); // already removed
    }
    #[test]
    fn map_collision() {
        let mut map = TestMapChar::<char, u32, 2>::new();
        assert_eq!(map.hash_index('a'), map.hash_index('c')); // make sure keys collide
        assert_eq!(map.insert('a', 100), Ok(()));
        assert_eq!(map.insert('c', 200), Ok(())); // collides with 'a', probes forward
        assert_eq!(map.get('a'), Some(100));
        assert_eq!(map.get('c'), Some(200));
    }
}
