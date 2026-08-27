// devela/src/data/store/key/map/define.rs
//
//! Defines the [`map!`] macro.
//
// IMPROVE: do not generate docs, make them specific of the examples.

#[doc = crate::_tags!(construction data_structure)]
/// Defines a custom static hashmap.
#[doc = crate::_doc_meta!{
    location("data/store/key", macro map),
}]
/// # Arguments
/// - `$NAME`:      the name of the new hashmap.
/// - `$KEY`:       the primitive keys type.
///
/// optional:
/// - `$EMPTY`:     the `$KEY` value for empty entries.
/// - `$TOMB`:      the `$KEY` value for deleted entries.
/// - `$HASH_ARG`:  the argument representing the byte slice.
/// - `$HASH_EXPR`: the const hasher expression using `$HASH_ARG`.
///
/// # Notes
/// - values `V` have to be `Copy` + `ConstInit`.
/// - keys `$KEY` can be any primitive integers, floats or `char`.
/// - Two specific `$KEY` values are reserved to indicate empty deleted keys.
///   They default to `MIN` and `MAX`, respectively, but can be customized.
/// - The default hasher is [`HasherFx`][crate::HasherFx].
///
/// # Variants
/// The macro supports three construction modes:
///
/// | Kind | Invocation | Description |
/// |------|-------------|--------------|
#[doc = concat!["| **Const** | `map![const MyMap, KEY: u16]` |",
"Generates a fully `const` hashmap with compile-time operations and const methods. |"]]
#[doc = concat!["| **Runtime** | `map![MyMap, KEY: u16]` |",
"Generates a non-const variant storing markers as struct fields, suitable for runtime mutation. |"]]
#[doc = concat!["| **TypeId-based** | `map![typeid MyMap]` |",
"Uses `TypeId` hashes as keys and provides type-oriented helper methods. |"]]
///
/// # Examples
/// See [`MapStaticConstU8Example`], [`MapStaticU8Example`], [`MapStaticTypeIdExample`].
///
/// [`MapStaticConstU8Example`]: crate::MapStaticConstU8Example
/// [`MapStaticU8Example`]: crate::MapStaticU8Example
/// [`MapStaticTypeIdExample`]: crate::MapStaticTypeIdExample
///
/// Overview
/// ```
/// # use devela::map;
/// // 1. Const hashmap
/// map![
///     #[doc(hidden)] // supports attributes
///     pub const MapConst, KEY: u16
/// ];
///
/// // 2. Runtime hashmap
/// map![pub(crate) MapRuntime, KEY: u16];
///
/// // 3. TypeId-keyed hashmap
/// map![typeid MapTypeId];
/// ```
///
/// Basic usage
/// ```
/// # use devela::map;
/// // Define a static hashmap with `u16` keys and default hasher
/// map![const ExampleMap, KEY: u16];
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
/// # use devela::{map, HasherFx};
/// // Define a static hashmap using `HasherFx` with a custom seed
/// map![const ExampleMapFxSeeded, KEY: u16,
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
/// map![const ExampleMapPengy, KEY: u16,
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
macro_rules! map {
    (
    // Const variant
    // ----------------------------------------------------------------------------------------
    // Default constructor:
        $(#[$attr:meta])*
        $vis:vis const $NAME:ident, KEY:$KEY:ty $(,)?
    ) => {
        $crate::map![
            $(#[$attr])*
            $vis const $NAME, KEY:$KEY,
            EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_primitive_bytes(bytes)
        ];
    };
    (// Custom Empty/Tomb, Default Hasher:
        $(#[$attr:meta])*
        $vis:vis const $NAME:ident, KEY:$KEY:ty,
        EMPTY:$EMPTY:expr, TOMB:$TOMB:expr $(,)?
    ) => {
        $crate::map![
            $(#[$attr])*
            $vis const $NAME, KEY:$KEY,
            EMPTY:$EMPTY, TOMB:$TOMB,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_primitive_bytes(bytes)
        ];
    };
    (// Custom Hasher, Default Empty/Tomb:
        $(#[$attr:meta])*
        $vis:vis const $NAME:ident, KEY:$KEY:ty,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        $crate::map![
            $(#[$attr])*
            $vis const $NAME, KEY:$KEY,
            EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER: | $HASH_ARG | $HASH_EXPR
        ];
    };
    (
        $(#[$attr:meta])*
        $vis:vis const $NAME:ident, KEY:$KEY:ty,
        EMPTY:$EMPTY:expr, TOMB:$TOMB:expr,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        $crate::__map_impl_const! {
            $(#[$attr])*
            $vis $NAME, KEY:$KEY,
            EMPTY:$EMPTY, TOMB:$TOMB,
            HASHER: | $HASH_ARG | $HASH_EXPR
        }
    };
    (
    // Runtime variant
    // ----------------------------------------------------------------------------------------
    // Default constructor:
        $(#[$attr:meta])*
        $vis:vis $NAME:ident, KEY:$KEY:ty $(,)?
    ) => {
        $crate::map![
            $(#[$attr])*
            $vis $NAME, KEY:$KEY, EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_primitive_bytes(bytes)
        ];
    };
    (// Custom Empty/Tomb, Default Hasher:
        $(#[$attr:meta])*
        $vis:vis $NAME:ident, KEY:$KEY:ty,
        EMPTY:$EMPTY:expr, TOMB:$TOMB:expr $(,)?
    ) => {
        $crate::map![
            $(#[$attr])*
            $vis $NAME, KEY:$KEY, EMPTY:$EMPTY, TOMB:$TOMB,
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_primitive_bytes(bytes)
        ];
    };
    (// Custom Hasher, Default Empty/Tomb:
        $(#[$attr:meta])*
        $vis:vis $NAME:ident, KEY:$KEY:ty,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        $crate::map![
            $(#[$attr])*
            $vis $NAME, KEY:$KEY,
            EMPTY:<$KEY>::MIN, TOMB:<$KEY>::MAX,
            HASHER: | $HASH_ARG | $HASH_EXPR
        ];
    };
    (
        $(#[$attr:meta])*
        $vis:vis $NAME:ident, KEY:$KEY:ty,
        EMPTY:$EMPTY:expr, TOMB:$TOMB:expr,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        $crate::__map_impl_runtime! {
            $(#[$attr])*
            $vis $NAME, KEY:$KEY,
            EMPTY:$EMPTY, TOMB:$TOMB,
            HASHER: | $HASH_ARG | $HASH_EXPR
        }
    };
    (
    // TypeId runtime variant
    // ----------------------------------------------------------------------------------------
    // Uses 64-bit hashes of `TypeId`s for the keys:
        $(#[$attr:meta])*
        $vis:vis typeid $NAME:ident $(,)?) => {
        $crate::map![
            $(#[$attr])*
            #[doc = "A `TypeId`-keyed static hashmap.\n\n\
            This variant uses 64-bit hashes of Rust `TypeId`s as keys and adds \
            type-oriented methods such as `insert_type`, `get_type`, and `remove_type`. \
            It is built on the runtime hashmap variant, inheriting its stored `empty` \
            and `tomb` markers and behavior.\n\n"]
            $vis $NAME, KEY: u64,
            EMPTY: type_id_hash::<Empty>(), TOMB: type_id_hash::<Tomb>(),
            HASHER:|bytes| $crate::HasherFx::<usize>::hash_primitive_bytes(bytes)
        ];

        struct Empty;
        struct Tomb;
        fn type_id_hash<T: 'static>() -> u64 {
            let mut hasher = $crate::HasherFx::<u64>::new();
            let id = $crate::TypeId::of::<T>();
            $crate::Hash::hash(&id, &mut hasher);
            $crate::Hasher::finish(&hasher)
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
}
#[doc(inline)]
pub use map;
