// devela::sys::arch::namespace
//
//! Target feature-dependant namespaces.
//

use crate::_dep::safe_arch::*;

/// Arch-related functionality.
pub struct Arch;

#[doc = crate::doc_private!()]
/// Helps to re-export standalone functions as namespaced methods of a struct.
macro_rules! arch_fn {
    ( // a function with return type
    $doc:literal, $fn_name:ident($($param:ident: $ty:ty),*) -> $ret:ty) => {
        #[doc = $doc] #[must_use]
        pub fn $fn_name($($param: $ty),*) -> $ret { $fn_name($($param),*) }
    };
    ( // a function without return type
    $doc:literal, $fn_name:ident($($param:ident: $ty:ty),*)) => {
        #[doc = $doc]
        pub fn $fn_name($($param: $ty),*) { $fn_name($($param),*) }
    };
    ( // a list of functions
    $($doc:literal, $fn_name:ident($($param:ident: $ty:ty),*) $(-> $ret:ty)?);+ $(;)?
    ) => { $( arch_fn![$doc, $fn_name($($param: $ty),*) $(-> $ret)?];)+ };
}

#[cfg(target_feature = "popcnt")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(target_feature = "popcnt")))] 
impl Arch {
    arch_fn! {
        "Count the number of bits set within an `i32`.",
        population_count_i32(a: i32) -> i32;
        "Count the number of bits set within an `i64`.",
        population_count_i64(a: i64) -> i32;
        "Deposit contiguous low bits from a u32 according to a mask.",
        population_deposit_u32(a: u32, index: u32) -> u32;
        "Deposit contiguous low bits from a u64 according to a mask.",
        population_deposit_u64(a: u64, index: u64) -> u64;
        "Extract bits from a u32 according to a mask.",
        population_extract_u32(a: u32, index: u32) -> u32;
        "Extract bits from a u64 according to a mask.",
        population_extract_u64(a: u64, index: u64) -> u64
    }
}
