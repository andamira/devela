// devela/src/data/store/intern/string/impls/vec.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __intern_string_impl_vec· {
    (
        [index: $iprim:ident + $Index:ty;]
        [cursor: $cprim:ident + $Cursor:ty;]
        [arena: $StringArena:ident;]

        $(#[$interner_attr:meta])*
        $vis:vis $Interner:ident;

        $svis:vis $Symbol:ident;
    ) => {
        /* packed canonical strings */

        $crate::__arena_string_impl_vec! {
            [index: $iprim + $Index;]
            [cursor: $cprim + $Cursor;]

            $StringArena;
            $Symbol;
            [mark:]
        }

        /* interner */

        $(#[$interner_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Interner {
            strings: $StringArena,

            /// Active open-addressed lookup slots.
            ///
            /// `EMPTY` represents vacancy; otherwise the value is a
            /// canonical-string index.
            slots: $crate::Vec<$crate::MaybeNiche<$Index>>,
        }

        impl Default for $Interner {
            fn default() -> Self { Self::new() }
        }
        impl $crate::ConstInit for $Interner {
            const INIT: Self = Self::new();
        }

        #[allow(dead_code)]
        impl $Interner {
            /// Slot value representing an unoccupied probe position.
            const EMPTY: $crate::MaybeNiche<$Index> = $crate::MaybeNiche::<$Index>::MAX;

            /// Maximum number of canonical strings supported by the
            /// symbol representation.
            ///
            /// One representable index value is reserved for `EMPTY`.
            $vis const MAX_CAPACITY: usize = {
                $crate::unwrap![ok_or $crate::MaybeNiche::<$Index>::MAX.try_to_usize(), usize::MAX]
            };
            /// Maximum packed-byte length supported by the cursor representation.
            $vis const MAX_BYTE_CAPACITY: usize = $StringArena::MAX_BYTE_CAPACITY;

            /* construction */

            /// Creates an empty allocating string interner.
            #[must_use]
            $vis const fn new() -> Self {
                Self { strings: $StringArena::new(), slots: $crate::Vec::new() }
            }

            /// Creates an empty interner with initial string, byte, and
            /// lookup-slot capacities.
            ///
            /// The capacities describe initially reserved storage and do not
            /// impose fixed limits. Lookup slots grow and rehash as necessary.
            ///
            /// # Panics
            /// Panics if the requested string or byte capacity exceeds its
            /// configured representation, or if allocation fails.
            #[must_use]
            $vis fn with_capacity(
                string_capacity: usize,
                byte_capacity: usize,
                slot_capacity: usize,
            ) -> Self {
                assert!(string_capacity <= Self::MAX_CAPACITY,
                    "intern_string! string capacity exceeds its symbol representation");
                let strings = $StringArena::with_capacity(string_capacity, byte_capacity);
                let mut slots = $crate::Vec::with_capacity(slot_capacity);
                slots.resize(slot_capacity, Self::EMPTY);
                Self { strings, slots }
            }

            /* canonical-string capacity */

            /// Returns the canonical-string capacity available without reallocating.
            #[must_use]
            $vis fn capacity(&self) -> usize {
                ::core::cmp::min(self.strings.capacity(), Self::MAX_CAPACITY)
            }
            /// Returns the number of canonical strings.
            #[must_use]
            $vis const fn len(&self) -> usize { self.strings.len() }
            /// Returns whether no strings have been interned.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.strings.is_empty() }

            /* lookup capacity */

            /// Returns the current number of active lookup slots.
            #[must_use]
            $vis const fn slot_capacity(&self) -> usize { self.slots.len() }

            /* byte capacity */

            /// Returns the packed-byte capacity available without reallocating.
            #[must_use]
            $vis fn byte_capacity(&self) -> usize {
                self.strings.byte_capacity()
            }
            /// Returns the packed UTF-8 byte length.
            #[must_use]
            $vis const fn byte_len(&self) -> usize {
                self.strings.byte_len()
            }
            /// Returns how many packed bytes fit without reallocating.
            #[must_use]
            $vis fn byte_remaining(&self) -> usize {
                self.strings.byte_remaining()
            }
            /// Returns whether `string` can be represented by this interner.
            ///
            /// An already canonical string is always accepted.
            ///
            /// For a new string this checks the symbol and byte coordinate
            /// representations. Lookup storage may grow and rehash.
            #[must_use]
            $vis fn can_intern(&self, string: &str) -> bool {
                if self.find(string).is_some() { true } else { self._can_insert_new(string) }
            }

            /* access */

            /// Returns whether `symbol` currently resolves to a canonical string.
            #[must_use]
            $svis const fn contains(&self, symbol: $Symbol) -> bool {
                self.strings.contains(symbol)
            }
            /// Returns the canonical string identified by `symbol`.
            #[must_use]
            $svis fn get(&self, symbol: $Symbol) -> Option<&str> {
                self.strings.get(symbol)
            }
            /// Returns the canonical symbol for `string`, if already interned.
            #[must_use]
            $svis fn find(&self, string: &str) -> Option<$Symbol> {
                self._probe(string).0
            }
            /// Returns all packed canonical UTF-8 bytes.
            #[must_use]
            $vis fn as_bytes(&self) -> &[u8] {
                self.strings.as_bytes()
            }

            /* mutation */

            /// Returns the canonical symbol for `string`, inserting it if absent.
            ///
            /// Equal strings always resolve to the same symbol.
            ///
            /// Lookup storage grows and rehashes when every active slot is
            /// occupied. Existing symbols remain unchanged by rehashing.
            ///
            /// Returns `None` when a new canonical string cannot be represented.
            $svis fn intern(&mut self, string: &str) -> Option<$Symbol> {
                let (found, vacant) = self._probe(string);
                if let Some(symbol) = found { return Some(symbol); }
                // Check semantic representability before growing lookup storage.
                if !self._can_insert_new(string) { return None; }
                let slot = match vacant {
                    Some(slot) => slot,
                    None => {
                        // No active slot exists, or the lookup table is full.
                        if !self._grow_slots() { return None; }
                        let (_, vacant) = self._probe(string);
                        $crate::unwrap![some? vacant]
                    }
                };
                // Establish a lookup destination before mutating packed storage.
                let symbol = self.strings.insert(string)?;
                self.slots[slot] = $crate::MaybeNiche::<$Index>::new(symbol.get_index());
                Some(symbol)
            }
            /// Removes every canonical string and resets the lookup table.
            ///
            /// Allocated storage and the current number of lookup slots are retained.
            ///
            /// Previously issued symbols may resolve again after later interning.
            $vis fn clear(&mut self) {
                self.strings.clear();
                self.slots.fill(Self::EMPTY);
            }

            /* iteration */

            /// Iterates over canonical strings in insertion order.
            $vis fn iter(&self) -> impl Iterator<Item = &str> + '_ {
                self.strings.iter()
            }
            /// Iterates over canonical symbols in insertion order.
            $svis fn symbols(&self) -> impl Iterator<Item = $Symbol> + '_ {
                self.strings.ids()
            }
            /// Iterates over canonical symbols and strings in insertion order.
            $svis fn entries(&self) -> impl Iterator<Item = ($Symbol, &str)> + '_ {
                self.strings.entries()
            }

            /* private */

            /// Returns whether one new canonical string can be represented.
            fn _can_insert_new(&self, string: &str) -> bool {
                self.len() < Self::MAX_CAPACITY && self.strings.can_insert(string)
            }
            /// Returns the matching symbol or first vacant lookup slot.
            fn _probe(&self, string: &str) -> (Option<$Symbol>, Option<usize>) {
                let nn = (None, None);
                let slots = self.slots.len();
                if slots == 0 { return nn; }
                let mut slot =
                    $crate::HasherFx::<usize>::hash_bytes_native(string.as_bytes()) % slots;
                $crate::whilst! { probed in 0..slots; {
                    let stored = self.slots[slot];
                    if stored.get_prim() == Self::EMPTY.get_prim() { return (None, Some(slot)); }
                    let index = $crate::unwrap![ok_or stored.try_to_usize(), return nn];
                    let symbol = $crate::unwrap![ok_or <$Symbol>::try_from_usize(index), return nn];
                    let candidate = $crate::unwrap![some_or self.strings.get(symbol), return nn];
                    if $crate::Slice::<u8>::eq(candidate.as_bytes(), string.as_bytes()) {
                        return (Some(symbol), None);
                    }
                    slot += 1;
                    if slot == slots { slot = 0; }
                }}
                nn
            }
            /// Expands the active lookup table and rehashes all canonical strings.
            fn _grow_slots(&mut self) -> bool {
                let old = self.slots.len();
                let new = $crate::is![old == 0, 1,
                    $crate::unwrap![some_or old.checked_mul(2), return false]];
                self._rehash(new);
                true
            }
            /// Rebuilds the lookup table with `slot_count` active slots.
            ///
            /// Canonical strings and their symbols remain unchanged.
            fn _rehash(&mut self, slot_count: usize) {
                debug_assert!(slot_count >= self.len());
                self.slots.resize(slot_count, Self::EMPTY);
                self.slots.fill(Self::EMPTY);
                for (symbol, string) in self.strings.entries() {
                    let mut slot = $crate::HasherFx::<usize>::hash_bytes_native(string.as_bytes())
                        % slot_count;
                    loop {
                        if self.slots[slot].get_prim() == Self::EMPTY.get_prim() {
                            self.slots[slot] =
                                $crate::MaybeNiche::<$Index>::new(symbol.get_index());
                            break;
                        }
                        slot += 1;
                        if slot == slot_count { slot = 0; }
                    }
                }
            }
        }
    };
}
pub use __intern_string_impl_vec· as __intern_string_impl_vec;
