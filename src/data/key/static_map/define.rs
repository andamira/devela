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
        $NAME:ident, KEY:$KEY:ty $(,)?
    ) => {
        $crate::define_static_map![ $NAME, KEY:$KEY,
            EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];
    };
    (
        // Custom Empty/Tomb, Default Hasher
        $NAME:ident, KEY:$KEY:ty, EMPTY:$EMPTY:expr, TOMB:$TOMB:expr $(,)?
    ) => {
        $crate::define_static_map![ $NAME, KEY:$KEY,
            EMPTY:$EMPTY, TOMB:$TOMB,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_bytes(bytes)
        ];
    };
    (
        // Custom Hasher, Default Empty/Tomb
        $NAME:ident, KEY:$KEY:ty, HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        $crate::define_static_map![$NAME, KEY:$KEY, EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER: | $HASH_ARG | $HASH_EXPR
        ];
    };
    (
        // Fully Customizable
        $NAME:ident, KEY:$KEY:ty, EMPTY:$EMPTY:expr, TOMB:$TOMB:expr,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        #[doc = concat!("A custom static hashmap with `", stringify!($KEY), "` keys.")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $NAME<K: Copy, V, const N: usize> {
            keys: [K; N],
            values: [V; N],
        }

        /* V */

        #[allow(unused)]
        impl<V, const N: usize> $NAME<$KEY, V, N> {
            /// Special marker value for deleted slots.
            pub const TOMB: $KEY = $TOMB as $KEY;
            /// Special marker value for empty slots.
            pub const EMPTY: $KEY = $EMPTY as $KEY;

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

            /// Ensures the given key is not EMPTY or TOMB.
            const fn debug_assert_valid_key(key: $KEY) {
                debug_assert!(key != Self::EMPTY, "Key cannot be `EMPTY` marker");
                debug_assert!(key != Self::TOMB, "Key cannot be `TOMB` marker");
            }
        }

        /* V: Copy */

        impl<V: Copy + $crate::ConstDefault, const N: usize> Default for $NAME<$KEY, V, N> {
            fn default() -> Self { Self::new() }
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
                debug_assert![$EMPTY != $TOMB, "`$EMPTY` and `$TOMB` must be distinct"];
                debug_assert![($EMPTY as i128) >= (<$KEY>::MIN as i128)
                    && ($EMPTY as i128) <= (<$KEY>::MAX as i128),
                    "`$EMPTY` value is out of range for type `$KEY`"];
                debug_assert![($TOMB as i128) >= (<$KEY>::MIN as i128)
                    && ($TOMB as i128) <= (<$KEY>::MAX as i128),
                    "`$TOMB` value is out of range for type `$KEY`"];
                Self {
                    keys: [Self::EMPTY; N],
                    values: [V::DEFAULT; N],
                }
            }

            /// Inserts a key-value pair.
            ///
            /// # Returns
            /// - `Ok(())` if the insertion succeeds.
            /// - `Err(`[`NotEnoughSpace`]`)` if no slots are available.
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
            pub const fn get(&self, key: $KEY) -> Option<V> {
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
            pub const fn remove(&mut self, key: $KEY) -> bool {
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
                    if self.keys[i] != Self::EMPTY && self.keys[i] != Self::TOMB {
                        let _ = new_table.insert(self.keys[i], self.values[i]);
                    }
                    i += 1;
                }
                new_table
            }
        }
    };
}
pub use define_static_map;
