// devela_base_core::code::ops::call::semantics
//
//! Structural axes for reasoning about invocation semantics and dispatch
//
// TOC
// - struct CallSemantics
// - enum CallBindTime
// - enum CallContext
// - enum CallDispatch
// - enum CallOpenness
// - enum CallStorage

use crate::Ordering;

#[doc = crate::_tags!(exec)]
/// Structural semantics of a call edge.
#[doc = crate::_doc_location!("core/ops")]
///
/// The axes describe how a call is bound, dispatched, contextualized,
/// and represented; not merely the callable value itself.
///
/// Effective behavior arises at the invocation boundary as the combination of:
/// - the callable's intrinsic semantics, and
/// - the dispatcher or container semantics.
///
/// Ordered axes form a product partial order (ignoring `CallOpenness`).
///
/// ## Invocation Categories
///
/// All invocation reduces to one of:
/// - Direct call
/// - Branch-based dispatch
/// - Indirect function pointer call
/// - Vtable dispatch
///
/// Callable representations reduce to:
/// - Function items
/// - Enums (closed sets)
/// - Function pointers
/// - Trait objects
/// - Type-erased closures
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallSemantics {
    /// When callee identity is fixed.
    pub bind: CallBindTime,

    /// Where the execution environment resides.
    pub context: CallContext,

    /// Mechanism of control transfer.
    pub dispatch: CallDispatch,

    /// Whether the behavior set is fixed or extensible.
    pub open: CallOpenness,

    /// Where the callable representation is stored.
    pub storage: CallStorage,
}
impl PartialOrd for CallSemantics {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::{Equal, Greater, Less};
        let axes = [
            self.bind.cmp(&other.bind),
            self.dispatch.cmp(&other.dispatch),
            self.context.cmp(&other.context),
            self.storage.cmp(&other.storage),
        ];
        let mut less = false;
        let mut greater = false;
        for ord in axes {
            match ord {
                Less => less = true,
                Greater => greater = true,
                Equal => {}
            }
        }
        match (less, greater) {
            (true, true) => None, // incomparable
            (true, false) => Some(Less),
            (false, true) => Some(Greater),
            (false, false) => Some(Equal),
        }
    }
}

#[doc = crate::_tags!(exec)]
/// When the callee identity becomes fixed.
#[doc = crate::_doc_location!("core/ops")]
///
/// Ordered: `Compile < Build < Run`.
///
/// Later binding increases runtime dynamism and reduces static visibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CallBindTime {
    /// Callee identity fixed by the compiler.
    ///
    /// Boundary: the call graph edge is known statically and may be inlined.
    ///
    /// Example: direct function call or monomorphized generic.
    Compile = 0,

    /// Callee selected by build-time fixed data.
    ///
    /// Boundary: selection occurs via branch on a closed set
    /// (e.g. enum or bytecode), but cannot change without rebuilding.
    Build = 1,

    /// Callee selected by runtime-provided data.
    ///
    /// Boundary: changing runtime data alone can alter the invoked function
    /// (e.g. function pointers or trait objects).
    Run = 2,
}

#[doc = crate::_tags!(exec)]
/// Where the callable's execution environment resides.
#[doc = crate::_doc_location!("core/ops")]
///
/// Ordered: `None < Receiver < Captured`.
///
/// This axis describes structural coupling, not argument types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CallContext {
    /// No distinguished execution environment.
    ///
    /// Boundary: all required inputs are ordinary parameters.
    ///
    /// Example: `fn add(a: u32, b: u32) -> u32`.
    None = 0,

    /// Execution environment supplied explicitly by the caller.
    ///
    /// Boundary: a persistent state object participates in the invocation
    /// protocol (e.g. `&mut Vm`, `&World`, or `&self`).
    Receiver = 1,

    /// Execution environment embedded inside the callable object.
    ///
    /// Boundary: the callable owns or encloses state not provided at call site.
    ///
    /// Example: `move |x| acc + x`.
    Captured = 2,
}

#[doc = crate::_tags!(exec)]
/// Mechanism by which control transfers to the callee.
#[doc = crate::_doc_location!("core/ops")]
///
/// Ordered: `Direct < Branch < Indirect < Vtable`.
///
/// Later variants introduce more indirection and reduce static visibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CallDispatch {
    /// Statically resolved call.
    ///
    /// Boundary: the call site encodes the callee directly.
    Direct = 0,

    /// Selection among a finite set via conditional branch.
    ///
    /// Boundary: dispatch occurs by matching a discriminant.
    Branch = 1,

    /// Call via function pointer.
    ///
    /// Boundary: callee address is loaded from memory at runtime.
    Indirect = 2,

    /// Dynamic dispatch via trait object.
    ///
    /// Boundary: method pointer resolved through a vtable.
    Vtable = 3,
}

#[doc = crate::_tags!(exec)]
/// Whether the behavior set is fixed or extensible.
#[doc = crate::_doc_location!("core/ops")]
///
/// Not ordered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CallOpenness {
    /// Behavior universe fixed at build time.
    ///
    /// Boundary: new behaviors require recompilation.
    Closed = 0,

    /// Behavior universe extensible at runtime.
    ///
    /// Boundary: new behaviors may be introduced without rebuilding.
    Open = 1,
}

#[doc = crate::_tags!(exec)]
/// Where the callable representation resides.
#[doc = crate::_doc_location!("core/ops")]
///
/// Ordered: `Static < Inline < Arena < Heap`.
///
/// Later variants increase allocation dynamism and indirection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CallStorage {
    /// Representation baked into the binary.
    ///
    /// Boundary: no allocation and program-long lifetime.
    Static = 0,

    /// Stored inline with known size.
    ///
    /// Boundary: concrete layout known at compile time.
    Inline = 1,

    /// Stored in type-erased managed buffer.
    ///
    /// Boundary: size erased; lifetime managed externally.
    Arena = 2,

    /// Stored in heap allocation.
    ///
    /// Boundary: allocation-managed memory.
    Heap = 3,
}
