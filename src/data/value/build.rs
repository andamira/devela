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
    all_sizes: DataValue, DataType, DataRaw, // order matters

    // --------------------------------------------------------------- 1-B / 8-b
    copy_variants_1B:
        "8-bit unsigned integer ", U8, u8,
        "8-bit signed integer", I8, i8,
        "1-Byte array of bytes", ByteArray1B, [u8; 1],
        "Boolean value", Bool, bool,
    copy_variants_1B_dep:
    copy_variants_1B_psize:
        "8-bit usize", Usize, usize, target_pointer_width = "8",
        "8-bit isize", Isize, isize, target_pointer_width = "8",
    copy_variants_1B_psize_dep:
    noncopy_variants_1B:
    noncopy_variants_1B_dep:
    noncopy_variants_1B_psize:
    noncopy_variants_1B_psize_dep:

    // -------------------------------------------------------------- 2-B / 16-b
    copy_variants_2B:
        "16-bit unsigned integer ", U16, u16,
        "16-bit signed integer", I16, i16,
        "2-Byte array of bytes", ByteArray2B, [u8; 2],
    copy_variants_2B_dep:
        // "16-bit floating-point number", F16, ::core::f16, "nightly_float", "nightly_float",
    copy_variants_2B_psize:
        "16-bit usize", Usize, usize, target_pointer_width = "16",
        "16-bit isize", Isize, isize, target_pointer_width = "16",
    copy_variants_2B_psize_dep:
    noncopy_variants_2B:
    noncopy_variants_2B_dep:
    noncopy_variants_2B_psize:
    noncopy_variants_2B_psize_dep:

    // -------------------------------------------------------------- 4-B / 32-b
    copy_variants_4B:
        "32-bit unsigned integer ", U32, u32,
        "32-bit signed integer", I32, i32,
        "32-bit floating-point number", F32, f32,
        "4-Byte array of bytes", ByteArray4B, [u8; 4],
        "4-Byte char ", Char, char,
    copy_variants_4B_dep:
    copy_variants_4B_psize:
        "32-bit usize", Usize, usize, target_pointer_width = "32",
        "32-bit isize", Isize, isize, target_pointer_width = "32",
    copy_variants_4B_psize_dep:
    noncopy_variants_4B:
    noncopy_variants_4B_dep:
    noncopy_variants_4B_psize:
    noncopy_variants_4B_psize_dep:

    // -------------------------------------------------------------- 8-B / 64-b
    copy_variants_8B:
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "8-Byte array of bytes", ByteArray8B, [u8; 8],
    copy_variants_8B_dep:
    copy_variants_8B_psize:
        "64-bit usize", Usize, usize, target_pointer_width = "64",
        "64-bit isize", Isize, isize, target_pointer_width = "64",
    copy_variants_8B_psize_dep:
    noncopy_variants_8B:
    noncopy_variants_8B_dep:
    noncopy_variants_8B_psize:
    noncopy_variants_8B_psize_dep:
        "6-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "16", "alloc", "alloc",

    // ------------------------------------------------------------- 16-B /128-b
    copy_variants_16B:
        "128-bit unsigned integer ", U128, u128,
        "128-bit signed integer", I128, i128,
        "16-Byte array of bytes", ByteArray16B, [u8; 16],
        "128-bit Duration", Duration, crate::Duration,
    copy_variants_16B_dep:
        // "128-bit floating-point number", F128, ::core::f128, "nightly_float", "nightly_float",
        "128-bit SystemInstant", SystemInstant, crate::SystemInstant, "std", "std",
        "128-bit SystemTime", SystemTime, crate::SystemTime, "std", "std",
    copy_variants_16B_psize:
        "128-bit usize", Usize, usize, target_pointer_width = "128",
        "128-bit isize", Isize, isize, target_pointer_width = "128",
    copy_variants_16B_psize_dep:
    noncopy_variants_16B:
    noncopy_variants_16B_dep:
    noncopy_variants_16B_psize:
    noncopy_variants_16B_psize_dep:
        "12-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "32", "alloc", "alloc",

    // ------------------------------------------------------------ 32-B / 256-b
    copy_variants_32B:
    "32-Byte array of bytes", ByteArray32B, [u8; 32],
    copy_variants_32B_dep:
    copy_variants_32B_psize:
    copy_variants_32B_psize_dep:
    noncopy_variants_32B:
    noncopy_variants_32B_dep:
    noncopy_variants_32B_psize:
    noncopy_variants_32B_psize_dep:
        "24-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "64", "alloc", "alloc",

    // ------------------------------------------------------------ 64 B / 512-b
    copy_variants_64B:
    "64-Byte array of bytes", ByteArray64B, [u8; 64],
    copy_variants_64B_dep:
    copy_variants_64B_psize:
    copy_variants_64B_psize_dep:
    noncopy_variants_64B:
    noncopy_variants_64B_dep:
    noncopy_variants_64B_psize:
    noncopy_variants_64B_psize_dep:
        "48-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "128", "alloc", "alloc",

    // ---------------------------------------------------------- 128-B / 1024-b
    copy_variants_128B:
    "128-Byte array of bytes", ByteArray128B, [u8; 128],
    copy_variants_128B_dep:
    copy_variants_128B_psize:
    copy_variants_128B_psize_dep:
    noncopy_variants_128B:
    noncopy_variants_128B_dep:
    noncopy_variants_128B_psize:
    noncopy_variants_128B_psize_dep:
}
