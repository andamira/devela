// devela::sys::arch::_reexport_dep

#[allow(unused_imports, reason = "dep_safe_arc feature-gate")]
use crate::{_reexport, _tags};

/* `safe_arch` re-exports */

// FIX: this macro not found at the root. NOTE: re-implemented at the bottom
// #[doc(inline)] #[cfg(feature = "dep_safe_arch")] pub use _dep::safe_arch;
//
// /// <span class='stab portability' title='re-exported from `safe_arch`'>`safe_arch`</span>
// #[cfg(all(feature = "dep_safe_arch", target_feature = "avx"))]
// #[cfg_attr(nightly_doc, doc(cfg(target_feature = "avx")))]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_safe_arch")))]
// pub use crate::_dep::safe_arch::cmp_op as arch_cmp;

#[doc = _tags!(platform)]
/// <span class='stab portability' title='re-exported from `safe_arch`'>`safe_arch`</span>
/// Turns a round operator token to the correct constant value.
#[doc = crate::_doc_location!("sys/arch")]
#[cfg(all(feature = "dep_safe_arch", target_feature = "avx"))]
#[cfg_attr(nightly_doc, doc(cfg(target_feature = "avx")))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_safe_arch")))]
pub use crate::_dep::safe_arch::round_op as arch_round;

_reexport! { "dep_safe_arch", "safe_arch", safe_arch, location: "sys/arch", tag: _tags!(platform),
    doc: "The data for a 128-bit SSE register of four `f32` lanes.", m128
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch, location: "sys/arch", tag: _tags!(platform),
    doc: "The data for a 256-bit AVX register of eight `f32` lanes.", m256
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch, location: "sys/arch", tag: _tags!(platform),
    doc: "The data for a 128-bit SSE register of two `f64` values.", m128d
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch, location: "sys/arch", tag: _tags!(platform),
    doc: "The data for a 128-bit SSE register of integer data.", m128i
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch, location: "sys/arch", tag: _tags!(platform),
    doc: "The data for a 256-bit AVX register of four `f64` values.", m256d
}
_reexport! { "dep_safe_arch", "safe_arch", safe_arch, location: "sys/arch", tag: _tags!(platform),
    doc: "The data for a 256-bit AVX register of integer data.", m256i
}

/* `safe_arch` re-implementations */

#[doc = _tags!(platform)]
/// <span class='stab portability' title='re-exported from `safe_arch`'>`safe_arch`</span>
/// Turns a comparison operator token to the correct constant value.
#[doc = crate::_doc_location!("sys/arch")]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
#[cfg(target_feature = "avx")]
#[cfg_attr(nightly_doc, doc(cfg(target_feature = "avx")))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_safe_arch")))]
macro_rules! arch_cmp {
    (EqualOrdered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_EQ_OQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_EQ_OQ;
        _CMP_EQ_OQ
    }};
    (EqualUnordered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_EQ_UQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_EQ_UQ;
        _CMP_EQ_UQ
    }};
    (False) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_FALSE_OQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_FALSE_OQ;
        _CMP_FALSE_OQ
    }};
    (GreaterEqualOrdered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_GE_OQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_GE_OQ;
        _CMP_GE_OQ
    }};
    (GreaterThanOrdered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_GT_OQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_GT_OQ;
        _CMP_GT_OQ
    }};
    (LessEqualOrdered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_LE_OQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_LE_OQ;
        _CMP_LE_OQ
    }};
    (LessThanOrdered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_LT_OQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_LT_OQ;
        _CMP_LT_OQ
    }};
    (NotEqualOrdered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_NEQ_OQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_NEQ_OQ;
        _CMP_NEQ_OQ
    }};
    (NotEqualUnordered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_NEQ_UQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_NEQ_UQ;
        _CMP_NEQ_UQ
    }};
    (NotGreaterEqualUnordered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_NGE_UQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_NGE_UQ;
        _CMP_NGE_UQ
    }};
    (NotGreaterThanUnordered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_NGT_UQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_NGT_UQ;
        _CMP_NGT_UQ
    }};
    (NotLessEqualUnordered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_NLE_UQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_NLE_UQ;
        _CMP_NLE_UQ
    }};
    (NotLessThanUnordered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_NLT_UQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_NLT_UQ;
        _CMP_NLT_UQ
    }};
    (Ordered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_ORD_Q;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_ORD_Q;
        _CMP_ORD_Q
    }};
    (True) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_TRUE_UQ;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_TRUE_UQ;
        _CMP_TRUE_UQ
    }};
    (Unordered) => {{
        #[cfg(target_arch = "x86")]
        use ::core::arch::x86::_CMP_UNORD_Q;
        #[cfg(target_arch = "x86_64")]
        use ::core::arch::x86_64::_CMP_UNORD_Q;
        _CMP_UNORD_Q
    }};
    ($unknown_op:tt) => {{
        compile_error!("The operation name given is invalid.");
    }};
}
#[doc(inline)]
#[cfg(target_feature = "avx")]
pub use arch_cmp;
