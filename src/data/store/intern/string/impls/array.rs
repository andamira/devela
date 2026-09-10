// devela/src/data/store/intern/string/impls/array.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __intern_string_impl_array· {
    (
        [index: $iprim:ident + $Index:ty;]
        [cursor: $cprim:ident + $Cursor:ty;]
        [arena: $StringArena:ident;]

        $(#[$interner_attr:meta])*
        $vis:vis $Interner:ident;

        $svis:vis $Symbol:ident;
    ) => {
        /* packed canonical strings */

        $crate::__arena_string_impl_array! {
            [index: $iprim + $Index;]
            [cursor: $cprim + $Cursor;]

            $StringArena;
            $Symbol;
            [mark:]
        }

        /* interner */

        $(#[$interner_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Interner<
            const STRINGS: usize,
            const BYTES: usize,
            const SLOTS: usize,
        > {
            strings: $StringArena<STRINGS, BYTES>,
            slots: [$crate::MaybeNiche<$Index>; SLOTS],
        }

        impl<
            const STRINGS: usize,
            const BYTES: usize,
            const SLOTS: usize,
        > $crate::ConstInit for $Interner<STRINGS, BYTES, SLOTS> {
            const INIT: Self = Self::new();
        }
        impl<
            const STRINGS: usize,
            const BYTES: usize,
            const SLOTS: usize,
        > Default for $Interner<STRINGS, BYTES, SLOTS> {
            fn default() -> Self {
                Self::new()
            }
        }

        #[allow(dead_code)]
        impl<
            const STRINGS: usize,
            const BYTES: usize,
            const SLOTS: usize,
        > $Interner<STRINGS, BYTES, SLOTS> {
            const _VALID_CONFIG: () = {
                assert!(SLOTS >= STRINGS,
                    "intern_string! slot capacity must cover string capacity");
            };

            /// Slot value representing an unoccupied probe position.
            const EMPTY: $crate::MaybeNiche<$Index> = $crate::MaybeNiche::<$Index>::MAX;

            /// Maximum canonical-string capacity supported by the symbol representation.
            $vis const MAX_CAPACITY: usize = $StringArena::<STRINGS, BYTES>::MAX_CAPACITY;

            /// Maximum packed-byte capacity supported by the cursor representation.
            $vis const MAX_BYTE_CAPACITY: usize = $StringArena::<STRINGS, BYTES>::MAX_BYTE_CAPACITY;
            /* construction */

            /// Creates an empty fixed-capacity string interner.
            #[must_use]
            $vis const fn new() -> Self {
                let () = Self::_VALID_CONFIG;
                Self {
                    strings: $StringArena::new(),
                    slots: [Self::EMPTY; SLOTS],
                }
            }

            /* capacity */

            /// Returns the maximum number of canonical strings.
            #[must_use]
            $vis const fn capacity(&self) -> usize { self.strings.capacity() }

            /// Returns the number of canonical strings.
            #[must_use]
            $vis const fn len(&self) -> usize { self.strings.len() }

            /// Returns whether no strings have been interned.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.strings.is_empty() }

            /// Returns the lookup-slot capacity.
            #[must_use]
            $vis const fn slot_capacity(&self) -> usize { SLOTS }

            /// Returns the fixed packed-byte capacity.
            #[must_use]
            $vis const fn byte_capacity(&self) -> usize { self.strings.byte_capacity() }

            /// Returns the packed UTF-8 byte length.
            #[must_use]
            $vis const fn byte_len(&self) -> usize { self.strings.byte_len() }

            /// Returns the remaining packed-byte capacity.
            #[must_use]
            $vis const fn byte_remaining(&self) -> usize { self.strings.byte_remaining() }

            /// Returns whether `string` can be interned.
            ///
            /// An already canonical string is always accepted.
            ///
            /// A new string requires an available lookup slot
            /// and sufficient canonical-string and packed-byte capacity.
            #[must_use]
            $vis const fn can_intern(&self, string: &str) -> bool {
                let (found, vacant) = self._probe(string);
                if found.is_some() { true }
                else if vacant.is_none() { false }
                else { self.strings.can_insert(string) }
            }

            /* access */

            /// Returns whether `symbol` currently resolves to a canonical string.
            #[must_use]
            $svis const fn contains(&self, symbol: $Symbol) -> bool {
                self.strings.contains(symbol)
            }

            /// Returns the canonical string identified by `symbol`.
            #[must_use]
            $svis const fn get(&self, symbol: $Symbol) -> Option<&str> {
                self.strings.get(symbol)
            }
            /// Returns the canonical symbol for `string`, if already interned.
            #[must_use]
            $svis const fn find(&self, string: &str) -> Option<$Symbol> {
                self._probe(string).0
            }
            /// Returns all packed canonical UTF-8 bytes.
            #[must_use]
            $vis const fn as_bytes(&self) -> &[u8] {
                self.strings.as_bytes()
            }

            /* mutation */

            /// Returns the canonical symbol for `string`, inserting it if absent.
            ///
            /// Equal strings always resolve to the same symbol.
            ///
            /// Returns `None` if a new canonical string cannot fit.
            $svis const fn intern(&mut self, string: &str) -> Option<$Symbol> {
                let (found, vacant) = self._probe(string);
                if let Some(symbol) = found { return Some(symbol); }
                let slot = $crate::unwrap![some? vacant];
                // Establish a lookup destination before mutating storage.
                let symbol = $crate::unwrap![some? self.strings.insert(string)];
                self.slots[slot] = $crate::MaybeNiche::<$Index>::new(symbol.get_index());
                Some(symbol)
            }
            /// Removes every canonical string and resets the lookup table.
            ///
            /// Previously issued symbols may resolve again after later interning.
            $vis const fn clear(&mut self) {
                self.strings.clear();
                $crate::whilst! { slot in 0..SLOTS; {
                    self.slots[slot] = Self::EMPTY;
                }}
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

            /// Returns the matching symbol or first vacant probe slot.
            const fn _probe(&self, string: &str) -> (Option<$Symbol>, Option<usize>) {
                let nn = (None, None);
                if SLOTS == 0 { return nn; }
                let mut slot =
                    $crate::HasherFx::<usize>::hash_bytes_native(string.as_bytes()) % SLOTS;
                $crate::whilst! { probed in 0..SLOTS; {
                    let stored = self.slots[slot];
                    if stored.get_prim() == Self::EMPTY.get_prim() { return (None, Some(slot)); }
                    let index = $crate::unwrap![ok_or stored.try_to_usize(), return nn];
                    let symbol = $crate::unwrap![ok_or <$Symbol>::try_from_usize(index), return nn];
                    let candidate = $crate::unwrap![some_or self.strings.get(symbol), return nn];
                    if $crate::Slice::<u8>::eq(candidate.as_bytes(), string.as_bytes()) {
                        return (Some(symbol), None);
                    }
                    slot += 1;
                    if slot == SLOTS { slot = 0; }
                }}
                nn
            }
        }
    };
}
pub use __intern_string_impl_array· as __intern_string_impl_array;
