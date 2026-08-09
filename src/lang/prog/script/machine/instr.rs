// devela/src/lang/prog/script/machine/instr.rs
//
//!
//

use crate::ScriptValue;

#[doc = crate::_tags!(lang)]
/// An operation executed by a [`ScriptMachine`][crate::ScriptMachine].
#[doc = crate::_doc_meta!{
    location("lang/prog/script"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: ScriptOp<u32> = 12|96; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: ScriptOp<u32> = 16|128; niche Option),
}]
/// Operations form the machine-level executable representation.
/// They are intentionally independent of any
/// particular scripting language syntax.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScriptOp<R> {
    /// Pushes a value onto the stack.
    Push(ScriptValue<R>),

    /// Removes the top stack value.
    Drop,

    /// Duplicates the top stack value.
    Dup,

    /// Adds the top two integer values.
    Add,

    /// Compares the top two values for equality.
    Eq,

    /// Jumps by the given signed operation offset.
    Jump(isize),

    /// Jumps by the given signed operation offset if the popped value is `true`.
    JumpIf(isize),

    /// Suspends execution until the machine is resumed.
    Yield,

    /// Finishes execution, returning the top stack value when present.
    Return,
}
