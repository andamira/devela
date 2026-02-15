// devela_base_core::data::id::uid::registry
//
//! Defines [`IdRegistry`]
//!
//! > A resolver of foreign identities into local identities
//
// IMPROVE: Naming slightly undersells its role
// MAYBE:RENAME: IdInterner | IdMapper | IdTranslator

#[doc = crate::_tags!(uid)]
/// Maps backend/native identifiers into stable, compact internal IDs.
#[doc = crate::_doc_location!("data/id")]
///
/// Used throughout the library anywhere a subsystem needs to allocate
/// a small set of stable identifiers from arbitrary external handles.
#[derive(Clone, Debug)]
pub struct IdRegistry<Id, const MAX: usize> {
    next: u32,
    entries: [(u32, Id); MAX],
}

impl<Id: Copy, const MAX: usize> IdRegistry<Id, MAX> {
    /// Creates an empty registry.
    pub const fn new(default: Id) -> Self {
        Self { next: 0, entries: [(0, default); MAX] }
    }

    /// Returns an existing id for `native`, or inserts a new one.
    pub fn intern(&mut self, native: u32, mk: impl Fn(u32) -> Id) -> Id {
        // lookup
        for (raw, id) in self.entries.iter() {
            if *raw == native {
                return *id;
            }
        }
        // allocate new
        let id = mk(self.next);
        self.next += 1;

        for slot in self.entries.iter_mut() {
            if slot.0 == 0 {
                *slot = (native, id);
                return id;
            }
        }
        panic!("IdRegistry FULL"); // TEMP
    }
}
