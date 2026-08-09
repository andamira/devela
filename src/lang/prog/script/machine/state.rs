// devela/src/lang/prog/script/machine/state.rs
//
//!
//

use crate::{ConstInit, ScriptValue};

#[doc = crate::_tags!(lang state)]
/// A fixed-stack hosted scripting machine.
#[doc = crate::_doc_meta!{
    location("lang/prog/script"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: ScriptMachine<u32, 4> = 56|448; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: ScriptMachine<u32, 4> = 80|640; niche Option),
}]
/// The machine stores suspended execution state independently of the
/// program being executed. A host may therefore retain a machine between
/// calls and resume it against the same loaded program.
///
/// `STACK` is the maximum number of live stack values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptMachine<R: Copy, const STACK: usize> {
    ip: usize,
    len: usize,
    stack: [ScriptValue<R>; STACK],
}

impl<R: Copy, const STACK: usize> Default for ScriptMachine<R, STACK> {
    fn default() -> Self {
        Self::new()
    }
}
impl<R: Copy, const STACK: usize> ConstInit for ScriptMachine<R, STACK> {
    const INIT: Self = Self::new();
}

impl<R: Copy, const STACK: usize> ScriptMachine<R, STACK> {
    /// Creates an empty machine positioned at the first operation.
    pub const fn new() -> Self {
        Self { ip: 0, len: 0, stack: [ScriptValue::Nil; STACK] }
    }

    /* queries */

    /// Returns the current instruction position.
    pub const fn ip(&self) -> usize {
        self.ip
    }
    /// Returns the number of live stack values.
    pub const fn stack_len(&self) -> usize {
        self.len
    }
    /// Returns the maximum number of live stack values.
    pub const fn stack_capacity(&self) -> usize {
        STACK
    }
    /// Returns whether the stack contains no live values.
    pub const fn stack_is_empty(&self) -> bool {
        self.len == 0
    }
    /// Returns whether the stack is full.
    pub const fn stack_is_full(&self) -> bool {
        self.len == STACK
    }

    /* */

    /// Resets execution and clears the logical stack.
    pub const fn reset(&mut self) {
        self.ip = 0;
        self.len = 0;
    }
    /// Attempts to push a value onto the stack.
    ///
    /// Returns `false` when the stack is full.
    pub const fn push(&mut self, value: ScriptValue<R>) -> bool {
        if self.len == STACK {
            false
        } else {
            self.stack[self.len] = value;
            self.len += 1;
            true
        }
    }
    /// Removes and returns the top stack value.
    pub const fn pop(&mut self) -> Option<ScriptValue<R>> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.stack[self.len])
        }
    }

    /* */

    /// Returns the top stack value.
    pub const fn peek(&self) -> Option<&ScriptValue<R>> {
        if self.len == 0 { None } else { Some(&self.stack[self.len - 1]) }
    }
    /// Returns the live portion of the stack.
    pub fn stack(&self) -> &[ScriptValue<R>] {
        &self.stack[..self.len]
    }
}
