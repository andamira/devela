// devela/src/data/store/key/map/fixed/impls/shared.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __map_impl_shared {
    (
        $NAME:ident, KEY:$KEY:ty,
        HASHER: | $HASH_ARG:ident | $HASH_EXPR:expr $(,)?
    ) => {
        #[allow(unused)]
        impl<V, const N: usize> $NAME<$KEY, V, N> {
            /// Inserts a key-value pair, consuming the value.
            pub fn insert_move(
                &mut self,
                key: $KEY,
                value: V,
            ) -> Result<(), $crate::NotEnoughSpace> {
                match self.entry(key) {
                    $crate::MapFixedEntry::Occupied(slot) => {
                        *slot = value; // Overwrite existing value
                        Ok(())
                    }
                    $crate::MapFixedEntry::Vacant(index) if index < N => {
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
            /* const */
            fn _replace_internal(&mut self, key: $KEY) -> Option<&mut V> {
                Self::debug_assert_valid_key(key);
                let mut index = self.hash_index(key);
                $crate::whilst! { i in 0..N; {
                    if self.keys[index] == key {
                        self.keys[index] = self.tomb();
                        return Some(&mut self.values[index]);
                    }
                    if self.keys[index] == self.empty() { return None; }
                    index = (index + 1) % N;
                }}
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
