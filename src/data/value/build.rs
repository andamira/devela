// devela::data::value::build
//
//! All the data-value related types are declared here.
//!
//! Firstly some mockups of unimported types are defined in order for the
//! `define_all_sizes` macro to work correctly under any feature combination.
//!
//! Secondly the crate types are built for the following sizes:
//! - bytes: 1, 2, 4, 8, 16, 32, 64, 128
//! - bits: 8, 16, 32, 64, 128, 256, 512, 1024
//

use super::*;

// 1. Mockups replacing unused dependencies
// -----------------------------------------------------------------------------

// 2. TYPES DEFINITIONS
// -----------------------------------------------------------------------------

define_data_value_type_raw! {
    all_sizes: v: DataValue, t: DataType, r: DataRaw,

    // --------------------------------------------------------------- 1-B / 8-b
    copy@1B:
        "8-bit unsigned integer ", U8, u8,
        "8-bit signed integer", I8, i8,
        "1-Byte array of bytes", ByteArray1B, [u8; 1],
        "Boolean value", Bool, bool,
    copy@1B_dep:
    copy@1B_ptr:
        "8-bit usize", Usize, usize, target_pointer_width = "8",
        "8-bit isize", Isize, isize, target_pointer_width = "8",
    copy@1B_ptrdep:

    noncopy@1B:
    noncopy@1B_dep:
    noncopy@1B_ptr:
    noncopy@1B_ptrdep:

    // -------------------------------------------------------------- 2-B / 16-b
    copy@2B:
        "16-bit unsigned integer ", U16, u16,
        "16-bit signed integer", I16, i16,
        "2-Byte array of bytes", ByteArray2B, [u8; 2],
    copy@2B_dep:
        // WAIT:
        // "16-bit floating-point number", F16, ::core::primiive::f16,
        //     "nightly_float", "nightly_float",
    copy@2B_ptr:
        "16-bit usize", Usize, usize, target_pointer_width = "16",
        "16-bit isize", Isize, isize, target_pointer_width = "16",
    copy@2B_ptrdep:
    noncopy@2B:

    noncopy@2B_dep:
    noncopy@2B_ptr:
    noncopy@2B_ptrdep:

    // -------------------------------------------------------------- 4-B / 32-b
    copy@4B:
        "32-bit unsigned integer ", U32, u32,
        "32-bit signed integer", I32, i32,
        "32-bit floating-point number", F32, f32,
        "4-Byte array of bytes", ByteArray4B, [u8; 4],
        "4-Byte char ", Char, char,
    copy@4B_dep:
    copy@4B_ptr:
        "32-bit usize", Usize, usize, target_pointer_width = "32",
        "32-bit isize", Isize, isize, target_pointer_width = "32",
    copy@4B_ptrdep:

    noncopy@4B:
    noncopy@4B_dep:
    noncopy@4B_ptr:
    noncopy@4B_ptrdep:

    // -------------------------------------------------------------- 8-B / 64-b
    copy@8B:
        "64-bit unsigned integer ", U64, u64,
        "64-bit signed integer", I64, i64,
        "64-bit floating-point number", F64, f64,
        "8-Byte array of bytes", ByteArray8B, [u8; 8],
    copy@8B_dep:
    copy@8B_ptr:
        "64-bit usize", Usize, usize, target_pointer_width = "64",
        "64-bit isize", Isize, isize, target_pointer_width = "64",
    copy@8B_ptrdep:

    noncopy@8B:
    noncopy@8B_dep:
    noncopy@8B_ptr:
    noncopy@8B_ptrdep:
        "6-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "16", "alloc", "alloc",

    // ------------------------------------------------------------- 16-B /128-b
    copy@16B:
        "128-bit unsigned integer ", U128, u128,
        "128-bit signed integer", I128, i128,
        "16-Byte array of bytes", ByteArray16B, [u8; 16],
        "128-bit Duration", Duration, crate::Duration,
    copy@16B_dep:
        // WAIT:
        // "128-bit floating-point number", F128, ::core::primitive::f128,
        //     "nightly_float", "nightly_float",
        "128-bit SystemInstant", SystemInstant, crate::SystemInstant, "std", "std",
        "128-bit SystemTime", SystemTime, crate::SystemTime, "std", "std",
    copy@16B_ptr:
        "128-bit usize", Usize, usize, target_pointer_width = "128",
        "128-bit isize", Isize, isize, target_pointer_width = "128",
    copy@16B_ptrdep:

    noncopy@16B:
    noncopy@16B_dep:
    noncopy@16B_ptr:
    noncopy@16B_ptrdep:
        "12-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "32", "alloc", "alloc",

    // ------------------------------------------------------------ 32-B / 256-b
    copy@32B:
        "32-Byte array of bytes", ByteArray32B, [u8; 32],
    copy@32B_dep:
    copy@32B_ptr:
    copy@32B_ptrdep:

    noncopy@32B:
    noncopy@32B_dep:
    noncopy@32B_ptr:
    noncopy@32B_ptrdep:
        "24-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "64", "alloc", "alloc",

    // ------------------------------------------------------------ 64 B / 512-b
    copy@64B:
        "64-Byte array of bytes", ByteArray64B, [u8; 64],
    copy@64B_dep:
    copy@64B_ptr:
    copy@64B_ptrdep:

    noncopy@64B:
    noncopy@64B_dep:
    noncopy@64B_ptr:
    noncopy@64B_ptrdep:
        "48-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "128", "alloc", "alloc",

    // ---------------------------------------------------------- 128-B / 1024-b
    copy@128B:
        "128-Byte array of bytes", ByteArray128B, [u8; 128],
    copy@128B_dep:
    copy@128B_ptr:
    copy@128B_ptrdep:

    noncopy@128B:
    noncopy@128B_dep:
    noncopy@128B_ptr:
    noncopy@128B_ptrdep:
}
