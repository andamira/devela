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

    // ----------------------------------------------------------------- 8b / 1B
    copy@8:
        "8-bit signed integer", I8, i8, [def:true],
        "8-bit unsigned integer ", U8, u8, [def:true],
        "1-Byte byte array", ByteArray1B, [u8; 1], [def:true],
        "Boolean value", Bool, bool, [def:true],
    copy@8_dep:
    copy@8_ptr:
        "8-bit isize", Isize, isize, target_pointer_width = "8", [def:true],
        "8-bit usize", Usize, usize, target_pointer_width = "8", [def:true],
    copy@8_ptrdep:

    noncopy@8:
    noncopy@8_dep:
    noncopy@8_ptr:
    noncopy@8_ptrdep:

    // ---------------------------------------------------------------- 16b / 2B
    copy@16:
        "16-bit signed integer", I16, i16, [def:true],
        "16-bit unsigned integer ", U16, u16, [def:true],
        "2-Byte byte array", ByteArray2B, [u8; 2], [def:true],
    copy@16_dep:
        // WAIT:
        // "16-bit floating-point number", F16, ::core::primiive::f16,
        //     "nightly_float", "nightly_float", [def:true],
    copy@16_ptr:
        "16-bit isize", Isize, isize, target_pointer_width = "16", [def:true],
        "16-bit usize", Usize, usize, target_pointer_width = "16", [def:true],
    copy@16_ptrdep:
    noncopy@16:

    noncopy@16_dep:
    noncopy@16_ptr:
    noncopy@16_ptrdep:

    // ---------------------------------------------------------------- 32b / 4B
    copy@32:
        "32-bit signed integer", I32, i32, [def:true],
        "32-bit unsigned integer ", U32, u32, [def:true],
        "32-bit floating-point number", F32, f32, [def:true],
        "4-Byte byte array", ByteArray4B, [u8; 4], [def:true],
        "32-bit char ", Char, char, [def:true],
    copy@32_dep:
    copy@32_ptr:
        "32-bit isize", Isize, isize, target_pointer_width = "32", [def:true],
        "32-bit usize", Usize, usize, target_pointer_width = "32", [def:true],
    copy@32_ptrdep:

    noncopy@32:
    noncopy@32_dep:
    noncopy@32_ptr:
    noncopy@32_ptrdep:

    // ---------------------------------------------------------------- 64b / 8B
    copy@64:
        "64-bit signed integer", I64, i64, [def:true],
        "64-bit unsigned integer ", U64, u64, [def:true],
        "64-bit floating-point number", F64, f64, [def:true],
        "8-Byte byte array", ByteArray8B, [u8; 8], [def:true],
    copy@64_dep:
    copy@64_ptr:
        "64-bit isize", Isize, isize, target_pointer_width = "64", [def:true],
        "64-bit usize", Usize, usize, target_pointer_width = "64", [def:true],
    copy@64_ptrdep:

    noncopy@64:
    noncopy@64_dep:
    noncopy@64_ptr:
    noncopy@64_ptrdep:
        "6-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "16", "alloc", "alloc", [def:true],

    // --------------------------------------------------------------- 128b / 16B
    copy@128:
        "128-bit signed integer", I128, i128, [def:true],
        "128-bit unsigned integer ", U128, u128, [def:true],
        "16-Byte byte array", ByteArray16B, [u8; 16], [def:true],
        "128-bit Duration", Duration, crate::Duration, [def:true],
    copy@128_dep:
        // WAIT:
        // "128-bit floating-point number", F128, ::core::primitive::f128,
        //     "nightly_float", "nightly_float", [def:true],
        "128-bit SystemInstant", SystemInstant, crate::SystemInstant, "std", "std", [def:false],
        "128-bit SystemTime", SystemTime, crate::SystemTime, "std", "std", [def:false],
    copy@128_ptr:
        "128-bit isize", Isize, isize, target_pointer_width = "128", [def:true],
        "128-bit usize", Usize, usize, target_pointer_width = "128", [def:true],
    copy@128_ptrdep:

    noncopy@128:
    noncopy@128_dep:
    noncopy@128_ptr:
    noncopy@128_ptrdep:
        "12-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "32", "alloc", "alloc", [def:true],

    // -------------------------------------------------------------- 256b / 32B
    copy@256:
        "32-Byte byte array", ByteArray32B, [u8; 32], [def:true],
    copy@256_dep:
    copy@256_ptr:
    copy@256_ptrdep:

    noncopy@256:
    noncopy@256_dep:
    noncopy@256_ptr:
    noncopy@256_ptrdep:
        "24-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "64", "alloc", "alloc", [def:true],

    // -------------------------------------------------------------- 512b / 64B
    copy@512:
        "64-Byte byte array", ByteArray64B, [u8; 64], [def:false/*¡*/],
    copy@512_dep:
    copy@512_ptr:
    copy@512_ptrdep:

    noncopy@512:
    noncopy@512_dep:
    noncopy@512_ptr:
    noncopy@512_ptrdep:
        "48-Byte fat-pointer String", String, crate::String,
            target_pointer_width = "128", "alloc", "alloc", [def:true],

    // ------------------------------------------------------------ 1024b / 128B
    copy@1024:
        "128-Byte byte array", ByteArray128B, [u8; 128], [def:false/*¡*/],
    copy@1024_dep:
    copy@1024_ptr:
    copy@1024_ptrdep:

    noncopy@1024:
    noncopy@1024_dep:
    noncopy@1024_ptr:
    noncopy@1024_ptrdep:
}
