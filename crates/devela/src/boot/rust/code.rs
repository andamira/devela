use crate::{_reexport, _tags};

pub(crate) use {
    any::*, convert::*, hint::*, init::*, marker::*, ops::*, ops_call::*, ops_overload::*,
    ops_range::*, panic::*, result::*, source::*, util_assert::*, util_cfg::*, util_debug::*,
    util_token::*,
};

// https://doc.rust-lang.org/stable/core/any/
pub(crate) mod any {
    use super::*;

    _reexport! { rust: core::any,
        location: "code/any" => trait Any, tag: _tags!(code),
        doc: "A trait to emulate dynamic typing.",
        Any
    }
    _reexport! { rust: core::any,
        location: "code/any" => struct TypeId, tag: _tags!(code id),
        doc: "Represents a globally unique identifier for a type.",
        TypeId
    }
    _reexport! { rust: core::any,
        location: "code/any" => fn any_type_name, tag: _tags!(code),
        doc: "Returns the name of a type as a string slice.",
        @type_name as any_type_name
    }
}

// https://doc.rust-lang.org/stable/core/convert/
pub(crate) mod convert {
    use super::*;

    /* traits */

    _reexport! { rust: core::convert,
        location: "code/convert" => trait AsMut, tag: _tags!(lifetime),
        doc: "Used to do a cheap mutable-to-mutable reference conversion.",
        AsMut
    }
    _reexport! { rust: core::convert,
        location: "code/convert" => trait AsRef, tag: _tags!(lifetime),
        doc: "Used to do a cheap reference-to-reference conversion.",
        AsRef
    }
    _reexport! { rust: core::convert,
        location: "code/convert" => trait From, tag: _tags!(value),
        doc: "Used to do value-to-value conversions while consuming the input value.",
        From
    }
    _reexport! { rust: core::convert,
        location: "code/convert" => trait Into, tag: _tags!(value),
        doc: "A value-to-value conversion that consumes the input value.",
        Into
    }
    _reexport! { rust: core::convert,
        location: "code/convert" => trait TryFrom, tag: _tags!(value),
        doc: "Simple and safe type conversions that may fail in a controlled way.",
        TryFrom
    }
    _reexport! { rust: core::convert,
        location: "code/convert" => trait TryInto, tag: _tags!(value),
        doc: "An attempted conversion that consumes self, which may be expensive.",
        TryInto
    }

    /* functions */

    _reexport! { rust: core::convert,
        location: "code/convert" => fn identity, tag: _tags!(no),
        doc: "The identity function. Just returns back its input.",
        identity
    }
}

// https://doc.rust-lang.org/stable/core/hint/
pub(crate) mod hint {
    use super::*;
    #[cfg(all(doc, root_sys))]
    use crate::Arch;

    _reexport! { rust: core::hint,
        location: "code/hint" => fn assert_unchecked, tag: _tags!(assert),
        doc: "Makes a *soundness* promise to the compiler that the `cond`ition holds.",
        assert_unchecked
    }
    _reexport! { rust: core::hint,
        location: "code/hint" => fn black_box, tag: _tags!(code),
        doc: "Hints the compiler to be maximally pessimistic about what black_box could do.",
        black_box
    }
    _reexport! { rust: core::hint,
        location: "code/hint" => fn cold_path, tag: _tags!(code),
        doc: "Hints to the compiler that given path is cold, i.e., unlikely to be taken.",
        cold_path
    }
    _reexport! { rust: core::hint,
        location: "code/hint" => fn select_unpredictable, tag: _tags!(code),
        doc: "Hints the compiler that the `condition` is branch-unpredictable.",
        select_unpredictable
    }
    _reexport! { rust: core::hint,
        location: "code/hint" => fn spin_loop, tag: _tags!(code),
        doc: "Signals the processor that it is running in a busy-wait spin-loop.\n\n
This is a best-effort hint and may do nothing. In current Rust implementations
it emits an architecture-specific instruction on x86/x86_64, RISC-V, AArch64,
LoongArch, and supported ARM configurations. On AVR, MSP430, wasm32, Xtensa,
and other unsupported architectures, the fallback is empty and may emit no
instruction at all.\n\n
It must not be used as a timing or delay primitive.\n\n
See also [`Arch::relax`] for a busy-wait step with an instruction-emitting
fallback, and [`Arch::nop`] for executing an actual no-operation instruction.",
        spin_loop
    }
    _reexport! { rust: core::hint,
        location: "code/hint" => fn unreachable_unchecked, tag: _tags!(assert),
        doc: "Informs the compiler that the current calling site is not reachable.",
        unreachable_unchecked
    }
}

pub(crate) mod init {
    use super::*;

    /* traits */

    // NOTE: the following trait and the corresponding derive macro have the same name:
    _reexport! { rust: core::default,
        location: "code/init" => trait Default, tag: _tags!(init),
        doc: "A trait for giving a type a useful default value. (Derivable)",
        Default
    }
}

pub(crate) mod marker {
    use super::*;

    // See: <https://doc.rust-lang.org/nomicon/phantom-data.html#table-of-phantomdata-patterns>
    _reexport! { rust: core::marker,
        location: "code/marker" => struct PhantomData, tag: _tags!(code),
        doc: "Zero-sized type used to mark things that \"act like\" they own a `T`.",
        PhantomData
    }
    _reexport! { rust: core::marker,
        location: "code/marker" => struct PhantomPinned, tag: _tags!(code),
        doc: "A marker type which does not implement `Unpin`.",
        PhantomPinned
    }

    /* traits */

    // NOTE: the following trait and the corresponding derive macro have the same name:
    _reexport! { rust: core::marker,
        location: "code/marker" => trait Copy, tag: _tags!(code),
        doc: "Types whose values can be duplicated simply by copying bits. (Derivable)",
        Copy
    }

    _reexport! { rust: core::marker,
        location: "code/marker" => trait Send, tag: _tags!(code concurrency),
        doc: "Types that can be transferred across thread boundaries.",
        Send
    }
    _reexport! { rust: core::marker,
        location: "code/marker" => trait Sync, tag: _tags!(code concurrency),
        doc: "Types for which it is safe to share references between threads.",
        Sync
    }

    _reexport! { rust: core::marker,
        location: "code/marker" => trait Sized, tag: _tags!(code),
        doc: "Types with a constant size known at compile time.",
        Sized
    }
    _reexport! { rust: core::marker,
        location: "code/marker" => trait Unpin, tag: _tags!(code),
        doc: "Types that do not require any pinning guarantees.",
        Unpin
    }

    // /// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
    // See also:
    // - https://dabstractions whose primary subject is logical truth, relations, or inference.ev-doc.rust-lang.org/stable/unstable-book/library-features/fn-ptr-trait.html
    // - [implement FnPtr for all fn pointers](https://github.com/rust-lang/rust/pull/108080)
    // #[cfg(nightly_fn)]
    // pub use ::core::marker::FnPtr;
}

pub(crate) mod ops {
    use super::*;

    /* enums */

    _reexport! { rust: core::ops,
        location: "code/ops" => enum ControlFlow, tag: _tags!(code),
        doc: "Used to tell an operation whether it should exit early or go on as usual.",
        ControlFlow
    }

    /* `core::clone` */

    // NOTE: the following trait and the corresponding derive macro have the same name:
    _reexport! { rust: core::clone,
        location: "code/ops" => trait Clone, tag: _tags!(value),
        doc: "A common trait that allows explicit creation of a duplicate value. (Derivable)",
        Clone
    }
}
pub(crate) mod ops_call {
    use super::*;

    /* traits */

    _reexport! { rust: core::ops,
        location: "code/ops/call" => trait Fn, tag: _tags!(code),
        doc: "The version of the call operator that takes an immutable receiver.",
        Fn
    }
    _reexport! { rust: core::ops,
        location: "code/ops/call" => trait FnMut, tag: _tags!(code),
        doc: "The version of the call operator that takes a mutable receiver.",
        FnMut
    }
    _reexport! { rust: core::ops,
        location: "code/ops/call" => trait FnOnce, tag: _tags!(code),
        doc: "The version of the call operator that takes a by-value receiver.",
        FnOnce
    }
}
pub(crate) mod ops_overload {
    use super::*;

    /* traits */

    // logic ops
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait BitAnd, tag: _tags!(logic),
        doc: "The bitwise AND operator `&`.",
        BitAnd
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait BitAndAssign, tag: _tags!(logic),
        doc: "The bitwise AND assignment operator `&=`.",
        BitAndAssign
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait BitOr, tag: _tags!(logic),
        doc: "The bitwise OR operator `|`.",
        BitOr
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait BitOrAssign, tag: _tags!(logic),
        doc: "The bitwise OR assignment operator `|=`.",
        BitOrAssign
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait BitXor, tag: _tags!(logic),
        doc: "The bitwise XOR operator `^`.",
        BitXor
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait BitXorAssign, tag: _tags!(logic),
        doc: "The bitwise XOR assignment operator `^=`.",
        BitXorAssign
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Not, tag: _tags!(logic),
        doc: "The unary logical negation operator `!`.",
        Not
    }

    // num ops
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Add, tag: _tags!(num),
        doc: "The addition operator `+`.",
        Add
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait AddAssign, tag: _tags!(num),
        doc: "The addition assignment operator `+=`.",
        AddAssign
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Div, tag: _tags!(num),
        doc: "The division operator `/`.",
        Div
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait DivAssign, tag: _tags!(num),
        doc: "The division assignment operator `/=`.",
        DivAssign
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Mul, tag: _tags!(num),
        doc: "The multiplication operator `*`.",
        Mul
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait MulAssign, tag: _tags!(num),
        doc: "The multiplication assignment operator `*=`.",
        MulAssign
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Rem, tag: _tags!(num),
        doc: "The remainder operator `%`.",
        Rem
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait RemAssign, tag: _tags!(num),
        doc: "The remainder assignment operator `%=`.",
        RemAssign
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Sub, tag: _tags!(num),
        doc: "The subtraction operator `-`.",
        Sub
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait SubAssign, tag: _tags!(num),
        doc: "The subtraction assignment operator `-=`.",
        SubAssign
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Neg, tag: _tags!(num),
        doc: "The unary negation operator `-`.",
        Neg
    }

    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Deref, tag: _tags!(lifetime),
        doc: "Used for immutable dereferencing operations, like `*v`.",
        Deref
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait DerefMut, tag: _tags!(lifetime),
        doc: "Used for mutable dereferencing operations, like in `*v = 1;`.",
        DerefMut
    }

    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Drop, tag: _tags!(guard),
        doc: "Custom code within the destructor.",
        Drop
    }

    // NOTE: Fn* traits re-exported from code/ops/call

    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait Index, tag: _tags!(value lifetime),
        doc: "Used for indexing operations (`container[index]`) in immutable contexts.",
        Index
    }
    _reexport! { rust: core::ops,
        location: "code/ops/overload" => trait IndexMut, tag: _tags!(value lifetime),
        doc: "Used for indexing operations (`container[index]`) in mutable contexts.",
        IndexMut
    }
}
pub(crate) mod ops_range {
    use super::*;

    /* enums */

    _reexport! { rust: core::ops,
        location: "code/ops/range" => enum Bound, tag: _tags!(quant),
        doc: "An endpoint of a range of keys.",
        +doc: "See also `num::`[`Interval`][crate::Interval].",
        Bound
    }

    /* structs */

    // new range API
    _reexport! { rust: core::range,
        location: "code/ops/range" => struct Range, tag: _tags!(quant),
        doc: "A range value bounded inclusively below and exclusively above (`start..end`).",
        Range
    }
    _reexport! { rust: core::range,
        location: "code/ops/range" => struct RangeFrom, tag: _tags!(quant),
        doc: "A range value bounded inclusively below (`start..`).",
        RangeFrom
    }
    _reexport! { rust: core::ops,
        location: "code/ops/range" => struct RangeFull, tag: _tags!(quant),
        doc: "An unbounded range (`..`).",
        RangeFull // WAIT:new-range-api
    }
    _reexport! { rust: core::range,
        location: "code/ops/range" => struct RangeInclusive, tag: _tags!(quant),
        doc: "A range value bounded inclusively below and above (`start..=end`).",
        RangeInclusive
    }
    _reexport! { rust: core::ops,
        location: "code/ops/range" => struct RangeTo, tag: _tags!(quant),
        doc: "A range bounded exclusively above (`..end`).",
        RangeTo // WAIT:new-range-api
    }
    _reexport! { rust: core::range,
        location: "code/ops/range" => struct RangeToInclusive, tag: _tags!(quant),
        doc: "A range value bounded inclusively above (`..=end`).",
        RangeToInclusive
    }

    /* traits */

    _reexport! { rust: core::ops,
        location: "code/ops/range" => trait RangeBounds, tag: _tags!(quant),
        doc: "Implemented by Rust's built-in range types",
        RangeBounds
    }
}

pub(crate) mod panic {
    use super::*;

    /* structs */

    _reexport! { rust: core::panic,
        location: "code/panic" => struct PanicAssertUnwindSafe, tag: _tags!(code assert),
        doc: "A simple wrapper around a type to assert that it is unwind safe.",
        @AssertUnwindSafe as PanicAssertUnwindSafe
    }
    _reexport! { rust: std::panic,
        location: "code/panic" => struct PanicHookInfo, tag: _tags!(code),
        doc: "Passed to `std::panic::set_hook` where panics can have arbitrary payloads.",
        PanicHookInfo
    }
    _reexport! { rust: core::panic,
        location: "code/panic" => struct PanicInfo, tag: _tags!(code),
        doc: "Passed to `#[panic_handler]` in `no_std`, carries a formatted message.",
        PanicInfo
    }
    _reexport! { rust: core::panic,
        location: "code/panic" => struct PanicLocation, tag: _tags!(code),
        doc: "A struct containing information about the location of a panic.",
        @Location as PanicLocation
    }

    /* traits */

    _reexport! { rust: core::panic,
        location: "code/panic" => trait PanicRefUnwindSafe, tag: _tags!(code lifetime),
        doc: "A marker trait which represents a shared reference considered unwind safe.",
        @RefUnwindSafe as PanicRefUnwindSafe
    }
    _reexport! { rust: core::panic,
        location: "code/panic" => trait PanicUnwindSafe, tag: _tags!(code),
        doc: "A marker trait which represents \"panic safe\" types in Rust.",
        @UnwindSafe as PanicUnwindSafe
    }

    /* macros */

    _reexport! { rust: core,
        location: "code/panic" => macro todo, tag: _tags!(code),
        doc: "Indicates unfinished code.",
        todo
    }
    _reexport! { rust: core,
        location: "code/panic" => macro unreachable, tag: _tags!(code),
        doc: "Indicates unreachable code.",
        unreachable
    }
    _reexport! { rust: core,
        location: "code/panic" => macro unimplemented, tag: _tags!(code),
        doc: "Indicates unimplemented code.",
        unimplemented
    }

    // NOTE: the following macro and the corresponding module have the same name:
    #[doc = crate::_tags!(code)]
    /// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
    /// Panics the current thread.
    #[doc = crate::_doc_meta!{
        location("code/panic", macro panic_),
    }]
    #[doc = "*Re-exported from [`core::panic`][macro@::core::panic]*."]
    #[doc = "\n\n---"]
    ///
    /// The reason of the `_` suffix is to avoid conflicting with the Rust's prelude
    /// when glob importing from this crate. Since this macro has the same name
    /// as its sibling module `core::panic`, to be able to re-export
    /// just the macro we have to wrap it with our own.
    #[macro_export]
    #[cfg_attr(cargo_primary_package, doc(hidden))]
    macro_rules! panic_ { ($($tt:tt)*) => { core::panic![$($tt)*] } }
    #[doc(inline)]
    pub use panic_;
}

pub(crate) mod result {
    use super::*;

    /* enums */

    _reexport! { rust: core::option,
        location: "code/result" => enum Option, tag: _tags!(niche),
        doc: "A type that represents an optional value.",
        Option
    }
    _reexport! { rust: core::result,
        location: "code/result" => enum Result, tag: _tags!(result),
        doc: "A type that represents either success ([`Ok`]) or failure ([`Err`]).",
        Result
    }
}

pub(crate) mod source {
    use super::*;

    /* macros */

    // source code
    _reexport! { rust: core,
        location: "code/source" => macro code_column, tag: _tags!(code),
        doc: "Expands to the column number at which it was invoked.",
        @column as code_column
    }
    _reexport! { rust: core,
        location: "code/source" => macro code_line, tag: _tags!(code),
        doc: "Expands to the line number at which it was invoked.",
        @line as code_line
    }
    _reexport! { rust: core,
        location: "code/source" => macro code_file, tag: _tags!(code),
        doc: "Expands to the file name at which it was invoked.",
        @file as code_file
    }
    _reexport! { rust: core,
        location: "code/source" => macro code_module, tag: _tags!(code string),
        doc: "Expands to a string representing the current module path.",
        @module_path as code_module
    }

    // include
    _reexport! { rust: core,
        location: "code/source" => macro include, tag: _tags!(code),
        doc: "Parses a file as an expression or an item according to the context.",
        include
    }
    _reexport! { rust: core,
        location: "code/source" => macro include_bytes, tag: _tags!(code),
        doc: "Includes a file as a reference to a byte array.",
        include_bytes
    }
    _reexport! { rust: core,
        location: "code/source" => macro include_str, tag: _tags!(code string),
        doc: "Includes a UTF-8 encoded file as a string.",
        include_str
    }
}

pub(crate) mod util_assert {
    use super::*;

    /* macros */

    // assert
    _reexport! { rust: core,
        location: "code/util/assert" => macro assert, tag: _tags!(assert),
        doc: "Asserts that a boolean expression is true at runtime.", assert
    }
    _reexport! { rust: core,
        location: "code/util/assert" => macro assert_eq, tag: _tags!(assert),
        doc: "Asserts that two expressions are equal to each other.", assert_eq
    }
    _reexport! { rust: core,
        location: "code/util/assert" => macro assert_ne, tag: _tags!(assert),
        doc: "Asserts that two expressions are not equal to each other.", assert_ne
    }
    _reexport! { rust: core,
        location: "code/util/assert" => macro assert_matches, tag: _tags!(assert),
        doc: "Asserts that an expression matches the provided pattern.", assert_matches
    }
    // debug_assert
    _reexport! { rust: core,
        location: "code/util/assert" => macro debug_assert, tag: _tags!(assert),
        doc: "Asserts that a boolean expression is true at runtime.", debug_assert
    }
    _reexport! { rust: core,
        location: "code/util/assert" => macro debug_assert_eq, tag: _tags!(assert),
        doc: "Asserts that two expressions are equal to each other.", debug_assert_eq
    }
    _reexport! { rust: core,
        location: "code/util/assert" => macro debug_assert_ne, tag: _tags!(assert),
        doc: "Asserts that two expressions are not equal to each other.", debug_assert_ne
    }
    _reexport! { rust: core,
        location: "code/util/assert" => macro debug_assert_matches, tag: _tags!(assert),
        doc: "Asserts that an expression matches the provided pattern.", debug_assert_matches
    }
    // compile
    _reexport! { rust: core,
        location: "code/util/assert" => macro compile_error, tag: _tags!(code assert),
        doc: "Causes compilation to fail with the given message when encountered.",
        compile_error
    }
}
pub(crate) mod util_cfg {
    use super::*;

    /* macros */

    _reexport! { rust: core, location: "code/util/cfg" => macro cfg, tag: _tags!(code),
        doc: "Evaluates boolean combinations of configuration flags at compile-time.", cfg
    }
    _reexport! { rust: core, location: "code/util/cfg" => macro cfg_select, tag: _tags!(code),
        doc: "Selects code at compile-time based on cfg predicates.", cfg_select
    }
}
pub(crate) mod util_debug {
    use super::*;

    /* structs */

    _reexport! { rust: std::backtrace,
        location: "code/util/debug" => struct Backtrace, tag: _tags!(code debug),
        doc: "A captured OS thread stack backtrace.",
        Backtrace
    }

    /* enums */

    _reexport! { rust: std::backtrace,
        location: "code/util/debug" => enum BacktraceStatus, tag: _tags!(code debug),
        doc: "The current status of a backtrace.",
        BacktraceStatus
    }
}
pub(crate) mod util_token {
    use super::*;

    /* macros */

    // concatenating
    _reexport! { rust: core,
        location: "code/util/token" => macro concat, tag: _tags!(code text),
        doc: "Concatenates literals into a static string slice.", concat
    }
    // WAIT: [macro_metavar_expr_concat](https://github.com/rust-lang/rust/issues/124225)
    // _reexport! { rust: core,
    //     location: "code/util/token" => macro concat_idents, tag: _tags!(code text),
    //     doc: "Concatenates identifiers into one identifier.", concat_idents
    // }
    // WAIT: [concat_bytes](https://github.com/rust-lang/rust/issues/87555)
    // _reexport! { rust: core,
    //     location: "code/util/token" => macro concat_bytes, tag: _tags!(code text),
    //     doc: "Concatenates literals into a byte slice.", concat_bytes
    // }
    _reexport! { rust: core,
        location: "code/util/token" => macro stringify, tag: _tags!(code text),
        doc: "Stringifies its arguments.", stringify
    }
}
