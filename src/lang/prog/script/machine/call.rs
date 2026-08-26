// devela/src/lang/prog/script/machine/call.rs
//
//!
//

#[cfg(doc)]
use crate::ScriptMachine;
use crate::{ConstInit, InvalidValue, NonMaxU16, WordTry, unwrap};

#[doc = crate::_tags!(lang uid)]
/// A compact contextual identifier for a host operation.
#[doc = crate::_doc_meta!{
    location("lang/prog/script/machine"),
    test_size_of(ScriptCallId = 2|16; niche Option),
}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScriptCallId(NonMaxU16);

impl ConstInit for ScriptCallId {
    /// Uses the largest representable identifier as a conspicuous initializer.
    ///
    /// This value is not reserved and may still be assigned meaning by a host.
    const INIT: Self = Self(NonMaxU16::MAX);
}

impl ScriptCallId {
    /// Constructs a host-call identifier from its raw value.
    #[must_use]
    pub const fn new(raw: u16) -> Option<Self> {
        Some(Self(unwrap![some? NonMaxU16::new(raw)]))
    }
    // MAYBE pub const fn from_repr(raw: NonMaxU16) -> Self { Self(raw) }

    /// Returns the raw identifier value.
    #[must_use]
    pub const fn raw(self) -> u16 {
        self.0.get()
    }
}

impl WordTry for ScriptCallId {
    type Repr = u16;
    type Error = InvalidValue;
    fn raw(self) -> Self::Repr {
        ScriptCallId::raw(self)
    }
    fn try_from_raw(raw: Self::Repr) -> Result<Self, Self::Error> {
        unwrap![some_ok_or ScriptCallId::new(raw), InvalidValue]
    }
}

#[doc = crate::_tags!(lang runtime)]
/// A host operation awaiting resolution by the caller.
#[doc = crate::_doc_meta!{
    location("lang/prog/script/machine"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(ScriptCall = 12|96; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(ScriptCall = 24|192; niche Option),
}]
/// A call remains valid while the originating machine is left unchanged.
/// It may be inspected and then completed with [`ScriptMachine::complete_call`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScriptCall {
    pub(super) ip: usize,
    pub(super) stack_len: usize,
    id: ScriptCallId,
    arity: u8,
}
impl ConstInit for ScriptCall {
    /// An inactive call token that cannot match a valid machine state.
    const INIT: Self = Self {
        ip: usize::MAX,
        stack_len: usize::MAX,
        id: ScriptCallId::INIT,
        arity: u8::MAX,
    };
}
impl ScriptCall {
    pub(crate) const fn new(ip: usize, stack_len: usize, id: ScriptCallId, arity: u8) -> Self {
        Self { ip, stack_len, id, arity }
    }
    /// Returns the host-operation identifier.
    pub const fn id(self) -> ScriptCallId {
        self.id
    }
    /// Returns the number of arguments supplied to the call.
    #[must_use]
    pub const fn arity(self) -> u8 {
        self.arity
    }
    /// Returns the instruction position requesting the call.
    #[must_use]
    pub const fn ip(self) -> usize {
        self.ip
    }
}
