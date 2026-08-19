// devela/src/data/store/pool/item/_model.rs
//
//! Model-based tests for pool state transitions.
//!
//! Deterministically explores all operation sequences up to a bounded depth
//! against a small reference model, checking observable state
//! and stale-handle behavior after every transition.
//

use crate::{PoolExample as Pool, PoolHandleExample as Handle, Vec};

const CAP: usize = 2;
const REGS: usize = 2;

enum Op {
    Insert { register: usize },
    Remove { register: usize },
    Replace { register: usize },
    Clear,
    BumpAll,
}

#[derive(Clone)]
struct State {
    pool: Pool<u8, CAP>,

    // Handles available to future operations. Registers deliberately retain
    // removed handles, allowing stale-handle operations to be explored.
    regs: [Option<Handle>; REGS],

    // Abstract live handle/value associations.
    live: [Option<(Handle, u8)>; CAP],
}
impl State {
    fn new() -> Self {
        Self {
            pool: Pool::new(),
            regs: [None; REGS],
            live: [None; CAP],
        }
    }
    fn live_index(&self, handle: Handle) -> Option<usize> {
        self.live
            .iter()
            .position(|entry| matches!(entry, Some((stored, _)) if *stored == handle))
    }
    fn apply(&mut self, op: Op, value: u8) {
        match op {
            Op::Insert { register } => self.apply_insert(register, value),
            Op::Remove { register } => self.apply_remove(register),
            Op::Replace { register } => self.apply_replace(register, value),
            Op::Clear => self.apply_clear(),
            Op::BumpAll => self.apply_bump_all(),
        }
    }
    fn apply_insert(&mut self, register: usize, value: u8) {
        assert!(register < REGS);
        let expected_success = self.live.iter().any(Option::is_none);
        let actual = self.pool.insert(value);
        match (expected_success, actual) {
            (true, Ok(handle)) => {
                let slot = self
                    .live
                    .iter_mut()
                    .find(|slot| slot.is_none())
                    .expect("successful insertion requires a vacant model slot");
                *slot = Some((handle, value));
                self.regs[register] = Some(handle);
            }
            (false, Err(returned)) => {
                assert_eq!(returned, value);
            }
            (true, Err(returned)) => {
                panic!("pool rejected {returned} while the model had a vacant slot");
            }
            (false, Ok(handle)) => {
                panic!("pool inserted {handle:?} beyond the model capacity");
            }
        }
    }
    fn apply_remove(&mut self, register: usize) {
        assert!(register < REGS);
        let Some(handle) = self.regs[register] else {
            return;
        };
        let expected = self
            .live_index(handle)
            .map(|index| self.live[index].take().expect("located live entry must exist").1);
        assert_eq!(self.pool.remove(handle), expected);
        // Intentionally retain the register. It now exercises stale-handle
        // operations until overwritten by another successful insertion.
    }
    fn apply_replace(&mut self, register: usize, value: u8) {
        assert!(register < REGS);
        let Some(handle) = self.regs[register] else {
            return;
        };
        match self.live_index(handle) {
            Some(index) => {
                let old = self.live[index].expect("located live entry must exist").1;
                assert_eq!(self.pool.replace(handle, value), Ok(old));
                self.live[index] = Some((handle, value));
            }
            None => {
                assert_eq!(self.pool.replace(handle, value), Err(value));
            }
        }
    }
    fn apply_clear(&mut self) {
        self.pool.clear();
        self.live = [None; CAP];
        // Keep all registers so every previously recorded handle becomes stale.
    }
    fn apply_bump_all(&mut self) {
        // Exercise handle-bearing exclusive iteration.
        let actual_handles = self
            .pool
            .entries_mut()
            .map(|(handle, value)| {
                *value = (*value).wrapping_add(1);
                handle
            })
            .collect::<Vec<_>>();
        // Update the model independently of what the pool iterator yielded.
        for entry in self.live.iter_mut().flatten() {
            entry.1 = entry.1.wrapping_add(1);
        }
        let mut expected_handles =
            self.live.iter().flatten().map(|(handle, _)| *handle).collect::<Vec<_>>();
        expected_handles.sort_by_key(|handle| handle.index_prim());
        assert_eq!(actual_handles, expected_handles);
    }
    fn assert_equivalent(&self) {
        let live_len = self.live.iter().flatten().count();
        assert_eq!(self.pool.len(), live_len);
        assert_eq!(self.pool.remaining(), CAP - live_len);
        assert_eq!(self.pool.is_empty(), live_len == 0);
        assert_eq!(self.pool.is_full(), live_len == CAP);
        // Check every live association, including handles no longer held in a register.
        for &(handle, value) in self.live.iter().flatten() {
            assert!(self.pool.contains(handle));
            assert_eq!(self.pool.get(handle), Some(&value));
        }
        // Registers may contain either live or stale handles.
        for handle in self.regs.into_iter().flatten() {
            let expected = self
                .live
                .iter()
                .flatten()
                .find_map(|(stored, value)| (*stored == handle).then_some(*value));
            assert_eq!(self.pool.contains(handle), expected.is_some());
            assert_eq!(self.pool.get(handle).copied(), expected);
        }
        let mut expected_entries = self.live.iter().flatten().copied().collect::<Vec<_>>();
        expected_entries.sort_by_key(|(handle, _)| handle.index_prim());
        let actual_entries =
            self.pool.entries().map(|(handle, value)| (handle, *value)).collect::<Vec<_>>();
        assert_eq!(actual_entries, expected_entries);
        let actual_handles = self.pool.handles().collect::<Vec<_>>();
        let expected_handles =
            expected_entries.iter().map(|(handle, _)| *handle).collect::<Vec<_>>();
        assert_eq!(actual_handles, expected_handles);
        let iterated = self.pool.iter().copied().collect::<Vec<_>>();
        let expected_values = expected_entries.iter().map(|(_, value)| *value).collect::<Vec<_>>();
        assert_eq!(iterated, expected_values);
    }
}

fn explore(state: State, depth: usize, seed: u8) {
    state.assert_equivalent();
    if depth == 0 {
        return;
    }
    for register in 0..REGS {
        let mut next = state.clone();
        next.apply(Op::Insert { register }, seed);
        explore(next, depth - 1, seed.wrapping_add(1));
        let mut next = state.clone();
        next.apply(Op::Remove { register }, seed);
        explore(next, depth - 1, seed.wrapping_add(3));
        let mut next = state.clone();
        next.apply(Op::Replace { register }, seed);
        explore(next, depth - 1, seed.wrapping_add(5));
    }
    let mut next = state.clone();
    next.apply(Op::Clear, seed);
    explore(next, depth - 1, seed.wrapping_add(7));
    let mut next = state;
    next.apply(Op::BumpAll, seed);
    explore(next, depth - 1, seed.wrapping_add(11));
}

#[test]
fn check_all_short_operation_sequences() {
    explore(State::new(), 5, 1);
}
