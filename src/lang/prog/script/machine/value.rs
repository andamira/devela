// devela/src/lang/prog/script/machine/value.rs
//
//!
//

#[cfg(doc)]
use crate::ScriptMachine;
use crate::{ConstInit, ValueKind};

#[doc = crate::_tags!(lang value)]
/// A value manipulated by a [`ScriptMachine`].
#[doc = crate::_doc_meta!{
    location("lang/prog/script"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: ScriptValue<u32> = 12|96; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: ScriptValue<u32> = 16|128; niche Option),
}]
/// Values are small, copyable machine-level data.
///
/// [`Ref`][Self::Ref] carries an opaque reference whose meaning and validity
/// are defined by the host rather than by the scripting machine.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ScriptValue<R> {
    /// The absence of a value.
    ///
    /// This is the default.
    #[default]
    Nil,

    /// A boolean value.
    Bool(bool),

    /// A signed integer value.
    Int(i64),

    /// An opaque reference interpreted by the host.
    Ref(R),
}

impl<R> ConstInit for ScriptValue<R> {
    const INIT: Self = Self::Nil;
}

impl<R> ScriptValue<R> {
    /// Returns this value's general semantic category.
    pub const fn kind(&self) -> ValueKind {
        match self {
            Self::Nil => ValueKind::Nil,
            Self::Bool(_) => ValueKind::Bool,
            Self::Int(_) => ValueKind::Int,
            Self::Ref(_) => ValueKind::Ref,
        }
    }
    /// Returns whether this is [`Nil`][Self::Nil].
    pub const fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }
}
