// devela::data::key::static_map::define
//
//! Defines the [`define_static_map!`] macro.
//

#[cfg(doc)]
define_static_map![ExampleStaticMapU16, KEY: u16];

/// Build a custom static hashmap.
///
/// # Arguments
/// - `$NAME`:      the name of the new hashmap.
/// - `$KEY`:       the integer primitive type for the keys.
///
/// optional:
/// - `$EMPTY`:     the `$KEY` value for empty entries.
/// - `$TOMB`:      the `$KEY` value for deleted entries.
/// - `$HASH_ARG`:  the argument representing the byte slice.
/// - `$HASH_EXPR`: the const hasher expression using `$HASH_ARG`.
///
/// # Notes
/// - values `V` have to be `Copy` + `ConstDefault`.
/// - keys `$KEY` can be any primitive integers, floats or `char`.
/// - Two specific `$KEY` values are reserved to indicate empty deleted keys.
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
/// define_static_map![const ExampleMap, KEY: u16];
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
/// define_static_map![const ExampleMapFxSeeded, KEY: u16,
///     HASHER: |b| HasherFx::<usize>::hash_bytes_with_seed(123, b)
/// ];
/// let mut map = ExampleMapFxSeeded::<u16, u32, 8>::new();
/// map.insert(1, 100).unwrap();
/// assert_eq!(map.get(1), Some(100));
///
/// # #[cfg(feature = "hash")] {
/// # use devela::HasherPengy;
/// // Define a static hashmap using a stateful pengy hasher
/// # #[cfg(feature = "hash")]
/// define_static_map![const ExampleMapPengy, KEY: u16,
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
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! define_static_map {
    // --------------------------------------------------------------------------------------------
    (const // Default constructor
        $NAME:ident, KEY:$KEY:ty $(,)?
    ) => {
        $crate::define_static_map![const $NAME, KEY:$KEY,
            EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];
    };
    (const // Custom Empty/Tomb, Default Hasher
        $NAME:ident, KEY:$KEY:ty, EMPTY:$EMPTY:expr, TOMB:$TOMB:expr $(,)?
    ) => {
        $crate::define_static_map![const $NAME, KEY:$KEY,
            EMPTY:$EMPTY, TOMB:$TOMB,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];
    };
    (const // Custom Hasher, Default Empty/Tomb
        $NAME:ident, KEY:$KEY:ty, HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        $crate::define_static_map![const $NAME, KEY:$KEY, EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER: | $HASH_ARG | $HASH_EXPR
        ];
    };
    (const // Fully customizable
        $NAME:ident, KEY:$KEY:ty, EMPTY:$EMPTY:expr, TOMB:$TOMB:expr,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        #[doc = concat!("A custom static hashmap with `", stringify!($KEY), "` keys.")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $NAME<K: Copy, V, const N: usize> {
            keys: [K; N],
            values: [V; N],
        }

        $crate::define_static_map![@shared $NAME, KEY:$KEY,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];

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

        impl<V: Copy + $crate::ConstDefault, const N: usize>
            $crate::ConstDefault for $NAME<$KEY, V, N> {
            const DEFAULT: Self = Self::new();
        }
        impl<V: Default, const N: usize> Default for $NAME<$KEY, V, N> {
            /// Creates an empty hashmap.
            ///
            /// # Panics
            /// Panics in debug if `EMPTY` and `TOMB` are equal,
            /// or if any of them are out of range for `$KEY`.
            #[allow(unexpected_cfgs, reason = "array_init")]
            fn default() -> Self {
                Self:: debug_assert_invariants();
                Self {
                    keys: [$EMPTY; N],
                    values: $crate::array_init![default [V; N], "safe_data", "unsafe_array"],
                }
            }
        }

        #[allow(unused)]
        impl<V: Copy + $crate::ConstDefault, const N: usize> $NAME<$KEY, V, N> {
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
                    values: [V::DEFAULT; N],
                }
            }
        }

        #[allow(unused)]
        impl<V, const N: usize> $NAME<$KEY, V, N> {
            /// Retrieves some shared reference to the value associated with the given key.
            pub const fn get_ref(&self, key: $KEY) -> Option<&V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { return Some(&self.values[index]); }
                    if self.keys[index] == self.empty() { return None; }
                    index = (index + 1) % N;
                    i += 1;
                }
                None
            }

            /// Retrieves some exclusive reference to the value associated with the given key.
            pub const fn get_mut(&mut self, key: $KEY) -> Option<&mut V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { return Some(&mut self.values[index]); }
                    if self.keys[index] == self.empty() { return None; }
                    index = (index + 1) % N;
                    i += 1;
                }
                None
            }

            /// Retrieves an entry for a given key.
            pub const fn entry(&mut self, key: $KEY) -> $crate::StaticMapEntry<'_, V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                let mut tombstone_index = None;
                while i < N {
                    if self.keys[index] == self.empty() {
                        return $crate::StaticMapEntry::Vacant(index);
                    }
                    if self.keys[index] == key {
                        return $crate::StaticMapEntry::Occupied(&mut self.values[index]);
                    }
                    if self.keys[index] == self.tomb() && tombstone_index.is_none() {
                        tombstone_index = Some(index);
                    }
                    index = (index + 1) % N;
                    i += 1;
                }
                // If full, return N (invalid index)
                $crate::StaticMapEntry::Vacant($crate::unwrap![some_or tombstone_index, N])
            }

            /* intropection */

            /// Returns the number of occupied slots in the hashmap.
            pub const fn len(&self) -> usize {
                let mut count = 0;
                let mut i = 0;
                while i < N {
                    if self.keys[i] != self.empty() && self.keys[i] != self.tomb() { count += 1; }
                    i += 1;
                }
                count
            }

            /// Returns `true` if the hashmap contains no entries.
            pub const fn is_empty(&self) -> bool {
                self.len() == 0
            }

            /// Returns `true` if the hashmap is completely full.
            pub const fn is_full(&self) -> bool {
                self.len() == N
            }

            /// Determines if rebuilding the table would improve efficiency.
            ///
            /// # Heuristic:
            /// - Rebuild if `TOMB` slots exceed `N / 2` (half the table size).
            pub const fn should_rebuild(&self) -> bool {
                self.deleted_count() >= N / 2
            }

            /// Returns the number of deleted (TOMB) slots.
            pub const fn deleted_count(&self) -> usize {
                let mut count = 0;
                let mut i = 0;
                while i < N {
                    if self.keys[i] == self.tomb() { count += 1; }
                    i += 1;
                }
                count
            }

            /// Returns the load factor as a fraction of total capacity.
            pub const fn load_factor(&self) -> f32 {
                self.len() as f32 / N as f32
            }

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
                let mut i = 0;
                let mut tombstone_index = None;
                while i < N {
                    if self.keys[index] == self.empty() || self.keys[index] == self.tomb() {
                        let slot = if let Some(tomb) = tombstone_index { tomb } else { index };
                        self.keys[slot] = key;
                        self.values[slot] = value;
                        return Ok(());
                    }
                    if self.keys[index] == key {
                        self.values[index] = value; // Overwrite existing value
                        return Ok(());
                    }
                    if self.keys[index] == self.tomb() && tombstone_index.is_none() {
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
            pub const fn get(&self, key: $KEY) -> Option<V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { return Some(self.values[index]); }
                    if self.keys[index] == self.empty() { return None; } // end of probe chain
                    index = (index + 1) % N;
                    i += 1;
                }
                None
            }
        }

        #[allow(unused)]
        impl<V: Copy + $crate::ConstDefault, const N: usize> $NAME<$KEY, V, N> {
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
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { self.keys[index] = self.tomb(); return true; }
                    if self.keys[index] == self.empty() { return false; }
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
                let mut i = 0;
                while i < N {
                    if self.keys[i] != self.empty() && self.keys[i] != self.tomb() {
                        let _ = new_table.insert(self.keys[i], self.values[i]);
                    }
                    i += 1;
                }
                new_table
            }
        }
    };
    // --------------------------------------------------------------------------------------------
    ( // Default constructor
        $NAME:ident, KEY:$KEY:ty $(,)?
    ) => {
        $crate::define_static_map![$NAME, KEY:$KEY,
            EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];
    };
    ( // Custom Empty/Tomb, Default Hasher
        $NAME:ident, KEY:$KEY:ty, EMPTY:$EMPTY:expr, TOMB:$TOMB:expr $(,)?
    ) => {
        $crate::define_static_map![ $NAME, KEY:$KEY,
            EMPTY:$EMPTY, TOMB:$TOMB,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];
    };
    ( // Custom Hasher, Default Empty/Tomb
        $NAME:ident, KEY:$KEY:ty, HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        $crate::define_static_map![NAME, KEY:$KEY, EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER: | $HASH_ARG | $HASH_EXPR
        ];
    };
    ( // Fully customizable
        $NAME:ident, KEY:$KEY:ty, EMPTY:$EMPTY:expr, TOMB:$TOMB:expr,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        ///
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $NAME<K: Copy, V, const N: usize> {
            keys: [K; N],
            values: [V; N],
            empty: K,
            tomb: K,
        }

        // implement shared methods
        $crate::define_static_map![@shared $NAME, KEY:$KEY,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];

        #[allow(unused)]
        impl<V, const N: usize> $NAME<$KEY, V, N> {
            /// Returns the key value used to mark empty slots.
            pub fn empty(&self) -> $KEY { self.empty }
            /// Returns the key value used to mark deleted slots.
            pub fn tomb(&self) -> $KEY { self.tomb }
        }

        impl<V: Default, const N: usize> Default for $NAME<$KEY, V, N> {
            /// Creates an empty hashmap.
            ///
            /// # Panics
            /// Panics in debug if `EMPTY` and `TOMB` are equal,
            /// or if any of them are out of range for `$KEY`.
            #[allow(unexpected_cfgs, reason = "array_init")]
            fn default() -> Self {
                Self:: debug_assert_invariants();
                Self {
                    keys: [$EMPTY; N],
                    values: $crate::array_init![default [V; N], "safe_data", "unsafe_array"],
                    empty: $EMPTY,
                    tomb: $TOMB,
                }
            }
        }

        #[allow(unused)]
        impl<V, const N: usize> $NAME<$KEY, V, N> {
            /// Constructs a new static map with runtime EMPTY and TOMB values.
            pub fn new() -> Self where V: Default {
                Self::default()
            }

            /// Creates an empty hashmap, by cloning a `value`.
            ///
            /// # Panics
            /// Panics in debug if `EMPTY` and `TOMB` are equal,
            /// or if any of them are out of range for `$KEY`.
            #[allow(unexpected_cfgs, reason = "array_init")]
            fn new_cloned(value: V) -> Self where V: Clone {
                Self:: debug_assert_invariants();
                Self {
                    keys: [$EMPTY; N],
                    values: $crate::array_init![clone [V; N], "safe_data", "unsafe_array", value],
                    empty: $EMPTY,
                    tomb: $TOMB,
                }
            }

            /// Retrieves some shared reference to the value associated with the given key.
            pub fn get_ref(&self, key: $KEY) -> Option<&V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { return Some(&self.values[index]); }
                    if self.keys[index] == self.empty() { return None; }
                    index = (index + 1) % N;
                    i += 1;
                }
                None
            }

            /// Retrieves some exclusive reference to the value associated with the given key.
            pub fn get_mut(&mut self, key: $KEY) -> Option<&mut V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { return Some(&mut self.values[index]); }
                    if self.keys[index] == self.empty() { return None; }
                    index = (index + 1) % N;
                    i += 1;
                }
                None
            }

            /// Retrieves an entry for a given key.
            pub fn entry(&mut self, key: $KEY) -> $crate::StaticMapEntry<'_, V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                let mut tombstone_index = None;
                while i < N {
                    if self.keys[index] == self.empty() {
                        return $crate::StaticMapEntry::Vacant(index);
                    }
                    if self.keys[index] == key {
                        return $crate::StaticMapEntry::Occupied(&mut self.values[index]);
                    }
                    if self.keys[index] == self.tomb() && tombstone_index.is_none() {
                        tombstone_index = Some(index);
                    }
                    index = (index + 1) % N;
                    i += 1;
                }
                // If full, return N (invalid index)
                $crate::StaticMapEntry::Vacant($crate::unwrap![some_or tombstone_index, N])
            }

            /* intropection */

            /// Returns the number of occupied slots in the hashmap.
            pub fn len(&self) -> usize {
                let mut count = 0;
                let mut i = 0;
                while i < N {
                    if self.keys[i] != self.empty() && self.keys[i] != self.tomb() { count += 1; }
                    i += 1;
                }
                count
            }

            /// Returns `true` if the hashmap contains no entries.
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            /// Returns `true` if the hashmap is completely full.
            pub fn is_full(&self) -> bool {
                self.len() == N
            }

            /// Determines if rebuilding the table would improve efficiency.
            ///
            /// # Heuristic:
            /// - Rebuild if `TOMB` slots exceed `N / 2` (half the table size).
            pub fn should_rebuild(&self) -> bool {
                self.deleted_count() >= N / 2
            }

            /// Returns the number of deleted (TOMB) slots.
            pub fn deleted_count(&self) -> usize {
                let mut count = 0;
                let mut i = 0;
                while i < N {
                    if self.keys[i] == self.tomb() { count += 1; }
                    i += 1;
                }
                count
            }

            /// Returns the load factor as a fraction of total capacity.
            pub fn load_factor(&self) -> f32 {
                self.len() as f32 / N as f32
            }

            /* utility */

            /// Ensures the given key is not EMPTY or TOMB.
            fn debug_assert_valid_key(key: $KEY) {
                debug_assert!(key != $EMPTY, "Key cannot be `EMPTY` marker");
                debug_assert!(key != $TOMB, "Key cannot be `TOMB` marker");
            }
            /// Ensures the type invariants hold.
            fn debug_assert_invariants() {
                debug_assert![$EMPTY != $TOMB, "`$EMPTY` and `$TOMB` must be distinct"];
                debug_assert![($EMPTY as i128) >= (<$KEY>::MIN as i128)
                    && ($EMPTY as i128) <= (<$KEY>::MAX as i128),
                    "`$EMPTY` value is out of range for type `$KEY`"];
                debug_assert![($TOMB as i128) >= (<$KEY>::MIN as i128)
                    && ($TOMB as i128) <= (<$KEY>::MAX as i128),
                    "`$TOMB` value is out of range for type `$KEY`"];
            }

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
            pub fn insert(&mut self, key: $KEY, value: V)
                -> Result<(), $crate::NotEnoughSpace> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                let mut tombstone_index = None;
                while i < N {
                    if self.keys[index] == self.empty() || self.keys[index] == self.tomb() {
                        let slot = if let Some(tomb) = tombstone_index { tomb } else { index };
                        self.keys[slot] = key;
                        self.values[slot] = value;
                        return Ok(());
                    }
                    if self.keys[index] == key {
                        self.values[index] = value; // Overwrite existing value
                        return Ok(());
                    }
                    if self.keys[index] == self.tomb() && tombstone_index.is_none() {
                        tombstone_index = Some(index);
                    }
                    index = (index + 1) % N;
                    i += 1;
                }
                Err($crate::NotEnoughSpace(Some(1)))
            }
        }

        #[allow(unused)]
        impl<V: Copy, const N: usize> $NAME<$KEY, V, N> {
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
            pub fn get(&self, key: $KEY) -> Option<V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { return Some(self.values[index]); }
                    if self.keys[index] == self.empty() { return None; } // end of probe chain
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
            pub fn remove(&mut self, key: $KEY) -> bool {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key { self.keys[index] = self.tomb(); return true; }
                    if self.keys[index] == self.empty() { return false; }
                    index = (index + 1) % N;
                    i += 1;
                }
                false
            }
        }

        #[allow(unused)]
        impl<V: Copy + Default, const N: usize> $NAME<$KEY, V, N> {
            /// Removes a key-value pair and optionally rebuilds the table.
            ///
            /// # Behavior
            /// - Calls `remove()`, returning `true` if the key was found.
            /// - If `should_rebuild()` returns `true`, calls `rebuild()`.
            pub fn remove_rebuild(&mut self, key: $KEY) -> bool {
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
            pub fn rebuild(&mut self) { *self = self.rebuilt(); }

            /// Returns a rebuilt version of the table with `TOMB` slots removed.
            ///
            /// Creates a new table and reinserts all valid keys while preserving the probe order.
            ///
            /// # Complexity
            /// - **O(N)** worst-case when all slots are occupied.
            pub fn rebuilt(&self) -> Self {
                let mut new_table = Self::new();
                let mut i = 0;
                while i < N {
                    if self.keys[i] != self.empty() && self.keys[i] != self.tomb() {
                        let _ = new_table.insert(self.keys[i], self.values[i]);
                    }
                    i += 1;
                }
                new_table
            }
        }
    };
    // --------------------------------------------------------------------------------------------
    (typeid // uses 64-bit hashes of `TypeId`s for the keys
     $NAME:ident $(,)?) => {
        $crate::define_static_map![$NAME, KEY: u64,
            EMPTY: type_id_hash::<Empty>(), TOMB: type_id_hash::<Tomb>(),
            HASHER: |bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];

        struct Empty;
        struct Tomb;
        fn type_id_hash<T: 'static>() -> u64 {
            let mut hasher = HasherFx::<u64>::new();
            $crate::TypeId::of::<T>().hash(&mut hasher);
            hasher.finish()
        }

        #[allow(unused)]
        /// Convenience methods for when the keys are `TypeId`s.
        impl<V, const N: usize> $NAME<u64, V, N> {
            /// Returns the hash of `T`'s `TypeId`.
            pub fn type_id_hash<T: 'static>() -> u64 { type_id_hash::<T>() }

            /// Retrieves some exclusive reference to the value associated with the given type `T`.
            ///
            /// Calls [`get_ref`][Self::get_ref] with the hash of its type id.
            pub fn get_ref_type<T: 'static>(&self) -> Option<&V> {
                let key = Self::type_id_hash::<T>();
                self.get_ref(key)
            }
            /// Retrieves some exclusive reference to the value associated with the given type `T`.
            ///
            /// Calls [`get_mut`][Self::get_mut] with the hash of its type id.
            pub fn get_mut_type<T: 'static>(&mut self) -> Option<&mut V> {
                let key = Self::type_id_hash::<T>();
                self.get_mut(key)
            }

            /// Inserts a value paired with the given type `T`.
            ///
            /// Calls [`insert`][Self::insert] with the hash of its type id.
            ///
            /// # Returns
            /// - `Ok(())` if the insertion succeeds.
            /// - `Err(`[`NotEnoughSpace`][crate::NotEnoughSpace]`)` if no slots are available.
            pub fn insert_type<T: 'static>(&mut self, value: V)
                -> Result<(), $crate::NotEnoughSpace> {
                let key = Self::type_id_hash::<T>();
                self.insert(key, value)
            }
        }
        #[allow(unused)]
        impl<V: Copy, const N: usize> $NAME<u64, V, N> {
            /// Retrieves some value associated with the given type `T`.
            ///
            /// Calls [`get`][Self::get] with the hash of its type id.
            pub fn get_type<T: 'static>(&self) -> Option<V> {
                let key = Self::type_id_hash::<T>();
                self.get(key)
            }
            /// Removes the value paired with the given type `T`.
            ///
            /// Calls [`remove`][Self::remove] with the hash of its type id.
            pub fn remove_type<T: 'static>(&mut self) -> bool {
                let key = Self::type_id_hash::<T>();
                self.remove(key)
            }
        }
    };

    // --------------------------------------------------------------------------------------------
    (@shared $NAME:ident, KEY:$KEY:ty, HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?) => {
        #[allow(unused)]
        impl<V, const N: usize> $NAME<$KEY, V, N> {
            /// Inserts a key-value pair, consuming the value.
            pub fn insert_move(&mut self, key: $KEY, value: V)
                -> Result<(), $crate::NotEnoughSpace> {
                match self.entry(key) {
                    $crate::StaticMapEntry::Occupied(slot) => {
                        *slot = value; // Overwrite existing value
                        Ok(())
                    }
                    $crate::StaticMapEntry::Vacant(index) if index < N => {
                        self.keys[index] = key;
                        self.values[index] = value;
                        Ok(())
                    }
                    _ => Err($crate::NotEnoughSpace(Some(1))),
                }
            }

            /// Removes and returns the value for a given key, replacing it with a provided value.
            #[rustfmt::skip]
            pub fn replace(&mut self, key: $KEY, replacement: V) -> Option<V> {
                match self._replace_internal(key) {
                    Some(slot) => Some($crate::Mem::replace(slot, replacement)),
                    None => None,
                }
            }
            /// Removes and returns the value for a given key, replacing it with `V::default()`.
            #[rustfmt::skip]
            pub fn replace_default(&mut self, key: $KEY) -> Option<V> where V: Default {
                self._replace_internal(key).map(|v| $crate::Mem::replace(v, V::default()))
            }
            /// Removes and returns the value for a given key, replacing it with a custom value.
            #[rustfmt::skip]
            pub fn replace_with<F>(&mut self, key: $KEY, replacement: F) -> Option<V>
            where F: FnOnce() -> V {
                self._replace_internal(key).map(|v| $crate::Mem::replace(v, replacement()))
            }
            /// Internal function to locate and mark a key as removed.
            ///
            /// Returns a mutable reference to the value slot for replacement.
            /* const */ fn _replace_internal(&mut self, key: $KEY) -> Option<&mut V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                let mut i = 0;
                while i < N {
                    if self.keys[index] == key {
                        self.keys[index] = self.tomb();
                        return Some(&mut self.values[index]);
                    }
                    if self.keys[index] == self.empty() { return None; }
                    index = (index + 1) % N;
                    i += 1;
                }
                None
            }

            /* introspection */

            /// Returns the total capacity of the hashmap (fixed at `N`).
            pub const fn capacity(&self) -> usize {
                N
            }

            /* utility */

            /// Computes a hash index.
            #[$crate::compile(not(same($KEY, char)))] // for integers and floats
            pub const fn hash_index(&self, key: $KEY) -> usize {
                let $HASH_ARG = &key.to_le_bytes();
                let expr = $HASH_EXPR;
                expr % N
            }
            /// Computes a hash index.
            #[$crate::compile(same($KEY, char))] // only for chars
            pub const fn hash_index(&self, key: $KEY) -> usize {
                let $HASH_ARG = &(key as u32).to_le_bytes();
                let expr = $HASH_EXPR;
                expr % N
            }
        }
    };
}
#[doc(inline)]
pub use define_static_map;
