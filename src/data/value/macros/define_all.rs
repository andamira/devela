// devela::data::value::macros::define_all
//!
//

/// Defines all sizes.
///
/// # Features
/// Specific sizes are feature-gated with the corresponding `_value*` feature.
///
/// # Arms
/// - all_sizes:
/// - single_size:
macro_rules! define_data_value_type_raw {
    (
    // Defines all sizes at the same time.
    //
    // This arm receives all the types ordered by size,
    // classified according to the following groupings,
    // where # represents the number of bits:
    //
    // - copy@#:              (name, type),*
    // - copy@#_dep:          (doc, name, type, dep1, dep2),*
    // - copy@#_ptr:          (doc, name, type, ptr),*
    // - copy@#_ptrdep:       (doc, name, type, ptr, dep1, dep2),*
    // - noncopy@#:           (doc, name, type),*
    // - noncopy@#_dep:       (doc, name, type, dep1, dep2),*
    // - noncopy@#_ptr:       (doc, name, type, ptr),*
    // - noncopy@#_ptrdep:    (doc, name, type, ptr, dep1, dep2),*
    //
    // where:
    // - the `copy` prefix indicates the types are `Copy`,
    //   otherwise the `noncopy` prefix is used.
    // - `nB` indicates the number of bytes of the types in the current group.
    // - the `dep` suffix indicates a dependency on 2 features (dep1 & dep2)
    //   (pass the same dependency twice in order to only depend on one).
    // - the `ptr` suffix indicates a dependency on a specific pointer size.
    //
    // The `single_size` arm is called making sure each size contains
    // all variants with a size less than or equal to the current size.

    all_sizes: v: $Value:ident, t: $Type:ident, r: $Raw:ident,

    // 8-bit / 1-Byte
    copy@8: $(
        $C_doc_8:literal, $C_name_8:ident, $C_type_8:ty,
        [def:$C_def_8:stmt],
        )*
    copy@8_dep: $(
        $C_doc_8_dep:literal, $C_name_8_dep:ident, $C_type_8_dep:ty,
        $C_dep1_8_dep:literal, $C_dep2_8_dep:literal,
        [def:$C_def_8_dep:stmt],
        )*
    copy@8_ptr: $(
        $C_doc_8_ptr:literal, $C_name_8_ptr:ident, $C_type_8_ptr:ty,
        $C_ptr_8_ptr:meta,
        [def:$C_def_8_ptr:stmt],
        )*
    copy@8_ptrdep: $(
        $C_doc_8_ptrdep:literal, $C_name_8_ptrdep:ident, $C_type_8_ptrdep:ty,
        $C_ptrdep_8_ptrdep:meta,
        [def:$C_def_8_ptrdep:stmt],
        )*

    noncopy@8: $(
        $N_doc_8:literal, $N_name_8:ident, $N_type_8:ty,
        [def:$N_def_8:stmt],
        )*
    noncopy@8_dep: $(
        $N_doc_8_dep:literal, $N_name_8_dep:ident, $N_type_8_dep:ty,
        $N_dep2_8_dep:literal,
        [def:$N_def_8_dep:stmt],
        )*
    noncopy@8_ptr: $(
        $N_doc_8_ptr:literal, $N_name_8_ptr:ident, $N_type_8_ptr:ty,
        $N_ptr_8_ptr:meta,
        [def:$N_def_8_ptr:stmt],
        )*
    noncopy@8_ptrdep: $(
        $N_doc_8_ptrdep:literal, $N_name_8_ptrdep:ident, $N_type_8_ptrdep:ty,
        $N_ptr_8_ptrdep:meta,
        $N_dep1_8_ptrdep:literal, $N_dep2_8_ptrdep:literal
        [def:$N_def_8_ptrdep:stmt],
        ),*

    // 16-bit / 2-Byte
    copy@16: $(
        $C_doc_16:literal, $C_name_16:ident, $C_type_16:ty,
        [def:$C_def_16:stmt],
        )*
    copy@16_dep: $(
        $C_doc_16_dep:literal, $C_name_16_dep:ident, $C_type_16_dep:ty,
        $C_dep1_16_dep:literal, $C_dep2_16_dep:literal,
        [def:$C_def_16_dep:stmt],
        )*
    copy@16_ptr: $(
        $C_doc_16_ptr:literal, $C_name_16_ptr:ident, $C_type_16_ptr:ty,
        $C_ptr_16_ptr:meta,
        [def:$C_def_16_ptr:stmt],
        )*
    copy@16_ptrdep: $(
        $C_doc_16_ptrdep:literal, $C_name_16_ptrdep:ident, $C_type_16_ptrdep:ty,
        $C_ptrdep_16_ptrdep:meta,
        [def:$C_def_16_ptrdep:stmt],
        )*

    noncopy@16: $(
        $N_doc_16:literal, $N_name_16:ident, $N_type_16:ty,
        [def:$N_def_16:stmt],
        )*
    noncopy@16_dep: $(
        $N_doc_16_dep:literal, $N_name_16_dep:ident, $N_type_16_dep:ty,
        $N_dep1_16_dep:literal,
        [def:$N_def_16_dep:stmt],
        )*
    noncopy@16_ptr: $(
        $N_doc_16_ptr:literal, $N_name_16_ptr:ident, $N_type_16_ptr:ty,
        $N_ptr_16_ptr:meta,
        [def:$N_def_16_ptr:stmt],
        )*
    noncopy@16_ptrdep: $(
        $N_doc_16_ptrdep:literal, $N_name_16_ptrdep:ident, $N_type_16_ptrdep:ty,
        $N_ptr_16_ptrdep:meta,
        $N_dep1_16_ptrdep:literal, $N_dep2_16_ptrdep:literal,
        [def:$N_def_16_ptrdep:stmt],
        )*

    // 32-bit / 4-Byte
    copy@32: $(
        $C_doc_32:literal, $C_name_32:ident, $C_type_32:ty,
        [def:$C_def_32:stmt],
        )*
    copy@32_dep: $(
        $C_doc_32_dep:literal, $C_name_32_dep:ident, $C_type_32_dep:ty,
        $C_dep1_32_dep:literal, $C_dep2_32_dep:literal,
        [def:$C_def_32_dep:stmt],
        )*
    copy@32_ptr: $(
        $C_doc_32_ptr:literal, $C_name_32_ptr:ident, $C_type_32_ptr:ty,
        $C_ptr_32_ptr:meta,
        [def:$C_def_32_ptr:stmt],
        )*
    copy@32_ptrdep: $(
        $C_doc_32_ptrdep:literal, $C_name_32_ptrdep:ident, $C_type_32_ptrdep:ty,
        $C_ptrdep_32_ptrdep:meta,
        [def:$C_def_32_ptrdep:stmt],
        )*

    noncopy@32: $(
        $N_doc_32:literal, $N_name_32:ident, $N_type_32:ty,
        [def:$N_def_32:stmt],
        )*
    noncopy@32_dep: $(
        $N_doc_32_dep:literal, $N_name_32_dep:ident, $N_type_32_dep:ty,
        $N_dep1_32_dep:literal, $N_dep2_32_dep:literal,
        [def:$N_def_32_dep:stmt],
        )*
    noncopy@32_ptr: $(
        $N_doc_32_ptr:literal, $N_name_32_ptr:ident, $N_type_32_ptr:ty,
        $N_ptr_32_ptr:meta,
        [def:$N_def_32_ptr:stmt],
        )*
    noncopy@32_ptrdep: $(
        $N_doc_32_ptrdep:literal, $N_name_32_ptrdep:ident, $N_type_32_ptrdep:ty,
        $N_ptr_32_ptrdep:meta,
        $N_dep1_32_ptrdep:literal, $N_dep2_32_ptrdep:literal,
        [def:$N_def_32_ptrdep:stmt],
        )*

    // 64-bit / 8-Byte
    copy@64: $(
        $C_doc_64:literal, $C_name_64:ident, $C_type_64:ty,
        [def:$C_def_64:stmt],
        )*
    copy@64_dep: $(
        $C_doc_64_dep:literal, $C_name_64_dep:ident, $C_type_64_dep:ty,
        $C_dep1_64_dep:literal, $C_dep2_64_dep:literal,
        [def:$C_def_64_dep:stmt],
        )*
    copy@64_ptr: $(
        $C_doc_64_ptr:literal, $C_name_64_ptr:ident, $C_type_64_ptr:ty,
        $C_ptr_64_ptr:meta,
        [def:$C_def_64_ptr:stmt],
        )*
    copy@64_ptrdep: $(
        $C_doc_64_ptrdep:literal, $C_name_64_ptrdep:ident, $C_type_64_ptrdep:ty,
        $C_ptrdep_64_ptrdep:meta,
        [def:$C_def_64_ptrdep:stmt],
        )*

    noncopy@64: $(
        $N_doc_64:literal, $N_name_64:ident, $N_type_64:ty,
        [def:$N_def_64:stmt],
        )*
    noncopy@64_dep: $(
        $N_doc_64_dep:literal, $N_name_64_dep:ident, $N_type_64_dep:ty,
        $N_dep1_64_dep:literal, $N_dep2_64_dep:literal,
        [def:$N_def_64_dep:stmt],
        )*
    noncopy@64_ptr: $(
        $N_doc_64_ptr:literal, $N_name_64_ptr:ident, $N_type_64_ptr:ty,
        $N_ptr_64_ptr:meta,
        [def:$N_def_64_ptr:stmt],
        )*
    noncopy@64_ptrdep: $(
        $N_doc_64_ptrdep:literal, $N_name_64_ptrdep:ident, $N_type_64_ptrdep:ty,
        $N_ptr_64_ptrdep:meta,
        $N_dep1_64_ptrdep:literal, $N_dep2_64_ptrdep:literal,
        [def:$N_def_64_ptrdep:stmt],
        )*

    // 128-bit / 16-Byte
    copy@128: $(
        $C_doc_128:literal, $C_name_128:ident, $C_type_128:ty,
        [def:$C_def_128:stmt],
        )*
    copy@128_dep: $(
        $C_doc_128_dep:literal, $C_name_128_dep:ident, $C_type_128_dep:ty,
        $C_dep1_128_dep:literal, $C_dep2_128_dep:literal,
        [def:$C_def_128_dep:stmt],
        )*
    copy@128_ptr: $(
        $C_doc_128_ptr:literal, $C_name_128_ptr:ident, $C_type_128_ptr:ty,
        $C_ptr_128_ptr:meta,
        [def:$C_def_128_ptr:stmt],
        )*
    copy@128_ptrdep: $(
        $C_doc_128_ptrdep:literal, $C_name_128_ptrdep:ident, $C_type_128_ptrdep:ty,
        $C_ptrdep_128_ptrdep:meta,
        [def:$C_def_128_ptrdep:stmt],
        )*

    noncopy@128: $(
        $N_doc_128:literal, $N_name_128:ident, $N_type_128:ty,
        [def:$N_def_128:stmt],
        )*
    noncopy@128_dep: $(
        $N_doc_128_dep:literal, $N_name_128_dep:ident, $N_type_128_dep:ty,
        $N_dep1_128_dep:literal, $N_dep2_128_dep:literal,
        [def:$N_def_128_dep:stmt],
        )*
    noncopy@128_ptr: $(
        $N_doc_128_ptr:literal, $N_name_128_ptr:ident, $N_type_128_ptr:ty,
        $N_ptr_128_ptr:meta,
        [def:$N_def_128_ptr:stmt],
        )*
    noncopy@128_ptrdep: $(
        $N_doc_128_ptrdep:literal, $N_name_128_ptrdep:ident, $N_type_128_ptrdep:ty,
        $N_ptr_128_ptrdep:meta,
        $N_dep1_128_ptrdep:literal, $N_dep2_128_ptrdep:literal,
        [def:$N_def_128_ptrdep:stmt],
        )*

    // 256-bit / 32-Byte
    copy@256: $(
        $C_doc_256:literal, $C_name_256:ident, $C_type_256:ty,
        [def:$C_def_256:stmt],
        )*
    copy@256_dep: $(
        $C_doc_256_dep:literal, $C_name_256_dep:ident, $C_type_256_dep:ty,
        $C_dep1_256_dep:literal, $C_dep2_256_dep:literal,
        [def:$C_def_256_dep:stmt],
        )*
    copy@256_ptr: $(
        $C_doc_256_ptr:literal, $C_name_256_ptr:ident, $C_type_256_ptr:ty,
        $C_ptr_256_ptr:meta,
        [def:$C_def_256_ptr:stmt],
        )*
    copy@256_ptrdep: $(
        $C_doc_256_ptrdep:literal, $C_name_256_ptrdep:ident, $C_type_256_ptrdep:ty,
        $C_ptrdep_256_ptrdep:meta,
        [def:$C_def_256_ptrdep:stmt],
        )*

    noncopy@256: $(
        $N_doc_256:literal, $N_name_256:ident, $N_type_256:ty,
        [def:$N_def_256:stmt],
        )*
    noncopy@256_dep: $(
        $N_doc_256_dep:literal, $N_name_256_dep:ident, $N_type_256_dep:ty,
        $N_dep1_256_dep:literal, $N_dep2_256_dep:literal,
        [def:$N_def_256_dep:stmt],
        )*
    noncopy@256_ptr: $(
        $N_doc_256_ptr:literal, $N_name_256_ptr:ident, $N_type_256_ptr:ty,
        $N_ptr_256_ptr:meta,
        [def:$N_def_256_ptr:stmt],
        )*
    noncopy@256_ptrdep: $(
        $N_doc_256_ptrdep:literal, $N_name_256_ptrdep:ident, $N_type_256_ptrdep:ty,
        $N_ptr_256_ptrdep:meta,
        $N_dep1_256_ptrdep:literal, $N_dep2_256_ptrdep:literal,
        [def:$N_def_256_ptrdep:stmt],
        )*

    // 512-bit / 64-Byte
    copy@512: $(
        $C_doc_512:literal, $C_name_512:ident, $C_type_512:ty,
        [def:$C_def_512:stmt],
        )*
    copy@512_dep: $(
        $C_doc_512_dep:literal, $C_name_512_dep:ident, $C_type_512_dep:ty,
        $C_dep1_512_dep:literal, $C_dep2_512_dep:literal,
        [def:$C_def_512_dep:stmt],
        )*
    copy@512_ptr: $(
        $C_doc_512_ptr:literal, $C_name_512_ptr:ident, $C_type_512_ptr:ty,
        $C_ptr_512_ptr:meta,
        [def:$C_def_512_ptr:stmt],
        )*
    copy@512_ptrdep: $(
        $C_doc_512_ptrdep:literal, $C_name_512_ptrdep:ident, $C_type_512_ptrdep:ty,
        $C_ptrdep_512_ptrdep:meta,
        [def:$C_def_512_ptrdep:stmt],
        )*

    noncopy@512: $(
        $N_doc_512:literal, $N_name_512:ident, $N_type_512:ty,
        [def:$N_def_512:stmt],
        )*
    noncopy@512_dep: $(
        $N_doc_512_dep:literal, $N_name_512_dep:ident, $N_type_512_dep:ty,
        $N_dep1_512_dep:literal, $N_dep2_512_dep:literal,
        [def:$N_def_512_dep:stmt],
        )*
    noncopy@512_ptr: $(
        $N_doc_512_ptr:literal, $N_name_512_ptr:ident, $N_type_512_ptr:ty,
        $N_ptr_512_ptr:meta,
        [def:$N_def_512_ptr:stmt],
        )*
    noncopy@512_ptrdep: $(
        $N_doc_512_ptrdep:literal, $N_name_512_ptrdep:ident, $N_type_512_ptrdep:ty,
        $N_ptr_512_ptrdep:meta,
        $N_dep1_512_ptrdep:literal, $N_dep2_512_ptrdep:literal,
        [def:$N_def_512_ptrdep:stmt],
        )*

    // 1024-bit / 128-Byte
    copy@1024: $(
        $C_doc_1264:literal, $C_name_1264:ident, $C_type_1264:ty,
        [def:$C_def_1024:stmt],
        )*
    copy@1024_dep: $(
        $C_doc_1024_dep:literal, $C_name_1024_dep:ident, $C_type_1024_dep:ty,
        $C_dep1_1024_dep:literal, $C_dep2_1024_dep:literal,
        [def:$C_def_1024_dep:stmt],
        )*
    copy@1024_ptr: $(
        $C_doc_1024_ptr:literal, $C_name_1024_ptr:ident, $C_type_1024_ptr:ty,
        $C_ptr_1024_ptr:meta,
        [def:$C_def_1024_ptr:stmt],
        )*
    copy@1024_ptrdep: $(
        $C_doc_1024_ptrdep:literal, $C_name_1024_ptrdep:ident, $C_type_1024_ptrdep:ty,
        $C_ptrdep_1024_ptrdep:meta,
        [def:$C_def_1024_ptrdep:stmt],
        )*

    noncopy@1024: $(
        $N_doc_1264:literal, $N_name_1264:ident, $N_type_1264:ty,
        [def:$N_def_1264:stmt],
        )*
    noncopy@1024_dep: $(
        $N_doc_1024_dep:literal, $N_name_1024_dep:ident, $N_type_1024_dep:ty,
        $N_dep1_1024_dep:literal, $N_dep2_1024_dep:literal,
        [def:$N_def_1024_dep:stmt],
        )*
    noncopy@1024_ptr: $(
        $N_doc_1024_ptr:literal, $N_name_1024_ptr:ident, $N_type_1024_ptr:ty,
        $N_ptr_1024_ptr:meta,
        [def:$N_def_1024_ptr:stmt],
        )*
    noncopy@1024_ptrdep: $(
        $N_doc_1024_ptrdep:literal, $N_name_1024_ptrdep:ident, $N_type_1024_ptrdep:ty,
        $N_ptr_1024_ptrdep:meta,
        $N_dep1_1024_ptrdep:literal, $N_dep2_1024_ptrdep:literal,
        [def:$N_def_1024_ptrdep:stmt],
        )*

    ) => {
        // 8-bit / 1-Byte
        #[cfg(feature = "_value8")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 8, feature: "_value8",

            copy:
                $( $C_doc_8, $C_name_8, $C_type_8,
                   [def:$C_def_8]
                )* ;
            copy@dep:
                $( $C_doc_8_dep, $C_name_8_dep, $C_type_8_dep,
                   $C_dep1_8_dep, $C_dep2_8_dep,
                   [def:$C_def_8_dep]
                )* ;
            copy@ptr:
                $( $C_doc_8_ptr, $C_name_8_ptr, $C_type_8_ptr,
                   $C_ptr_8_ptr,
                   [def:$C_def_8_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_8_ptrdep, $C_name_8_ptrdep, $C_type_8_ptrdep,
                   $C_ptrdep_8_ptrdep,
                   [def:$C_def_8_ptrdep]
                )* ;

            noncopy:
                $( $N_doc_8, $N_name_8, $N_type_8,
                   [def:$N_def_8]
                )* ;
            noncopy@dep:
                $( $N_doc_8_dep, $N_name_8_dep, $N_type_8_dep,
                   $N_dep1_8_dep, $N_dep2_8_dep,
                   [def:$N_def_8_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_8_ptr, $N_name_8_ptr, $N_type_8_ptr,
                   $N_ptr_8_ptr, $N_dep1_8_ptr, $N_dep2_8_ptr,
                   [def:$N_def_8_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_8_ptrdep, $N_name_8_ptrdep, $N_type_8_ptrdep,
                   $N_ptr_8_ptrdep,
                   $N_dep1_8_ptrdep, $N_dep2_8_ptrdep,
                   [def:$N_def_8_ptrdep]
                )* ;
        }
        // 16-bit / 2-Byte
        #[cfg(feature = "_value16")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 16, feature: "_value16",

            copy:
                $( $C_doc_8, $C_name_8, $C_type_8,
                   [def:$C_def_8]
                )*
                $( $C_doc_16, $C_name_16, $C_type_16,
                   [def:$C_def_16]
                )* ;
            copy@dep:
                $( $C_doc_8_dep, $C_name_8_dep, $C_type_8_dep,
                   $C_dep1_8_dep, $C_dep2_8_dep,
                   [def:$C_def_8_dep]
                )*
                $( $C_doc_16_dep, $C_name_16_dep, $C_type_16_dep,
                   $C_dep1_16_dep, $C_dep2_16_dep,
                   [def:$C_def_16_dep]
                )* ;
            copy@ptr:
                $( $C_doc_8_ptr, $C_name_8_ptr, $C_type_8_ptr,
                   $C_ptr_8_ptr,
                   [def:$C_def_8_ptr]
                )*
                $( $C_doc_16_ptr, $C_name_16_ptr, $C_type_16_ptr,
                   $C_ptr_16_ptr,
                   [def:$C_def_16_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_8_ptrdep, $C_name_8_ptrdep, $C_type_8_ptrdep,
                   $C_ptrdep_8_ptrdep,
                   [def:$C_def_8_ptrdep]
                )*
                $( $C_doc_16_ptrdep, $C_name_16_ptrdep, $C_type_16_ptrdep,
                   $C_ptrdep_16_ptrdep,
                   [def:$C_def_16_ptrdep]
                )* ;

            noncopy:
                $( $N_doc_8, $N_name_8, $N_type_8,
                   [def:$N_def_8]
                )*
                $( $N_doc_16, $N_name_16, $N_type_16,
                   [def:$N_def_16]
                )* ;
            noncopy@dep:
                $( $N_doc_8_dep, $N_name_8_dep, $N_type_8_dep,
                   $N_dep1_8_dep, $N_dep2_8_dep,
                   [def:$N_def_8_dep]
                )*
                $( $N_doc_16_dep, $N_name_16_dep, $N_type_16_dep,
                   $N_dep1_16_dep, $N_dep2_16_dep,
                   [def:$N_def_16_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_8_ptr, $N_name_8_ptr, $N_type_8_ptr,
                   $N_ptr_8_ptr, $N_dep1_8_ptr, $N_dep2_8_ptr,
                   [def:$N_def_8_ptr]
                )*
                $( $N_doc_16_ptr, $N_name_16_ptr, $N_type_16_ptr,
                   $N_ptr_16_ptr, $N_dep1_16_ptr, $N_dep2_16_ptr,
                   [def:$N_def_16_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_8_ptrdep, $N_name_8_ptrdep, $N_type_8_ptrdep,
                   $N_ptr_8_ptrdep, $N_dep1_8_ptrdep, $N_dep2_8_ptrdep,
                   [def:$N_def_8_ptrdep]
                )*
                $( $N_doc_16_ptrdep, $N_name_16_ptrdep, $N_type_16_ptrdep,
                   $N_ptr_16_ptrdep, $N_dep1_16_ptrdep, $N_dep2_16_ptrdep,
                   [def:$N_def_16_ptrdep]
                )* ;
        }
        // 32-bit / 4-Byte
        #[cfg(feature = "_value32")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 32, feature: "_value32",

            copy:
                $( $C_doc_8, $C_name_8, $C_type_8,
                   [def:$C_def_8]
                )*
                $( $C_doc_16, $C_name_16, $C_type_16,
                   [def:$C_def_16]
                )*
                $( $C_doc_32, $C_name_32, $C_type_32,
                   [def:$C_def_32]
                )* ;
            copy@dep:
                $( $C_doc_8_dep, $C_name_8_dep, $C_type_8_dep,
                   $C_dep1_8_dep, $C_dep2_8_dep,
                   [def:$C_def_8_dep]
                )*
                $( $C_doc_16_dep, $C_name_16_dep, $C_type_16_dep,
                   $C_dep1_16_dep, $C_dep2_16_dep,
                   [def:$C_def_16_dep]
                )*
                $( $C_doc_32_dep, $C_name_32_dep, $C_type_32_dep,
                   $C_dep1_32_dep, $C_dep2_32_dep,
                   [def:$C_def_32_dep]
                )* ;
            copy@ptr:
                $( $C_doc_8_ptr, $C_name_8_ptr, $C_type_8_ptr,
                   $C_ptr_8_ptr,
                   [def:$C_def_8_ptr]
                )*
                $( $C_doc_16_ptr, $C_name_16_ptr, $C_type_16_ptr,
                   $C_ptr_16_ptr,
                   [def:$C_def_16_ptr]
                )*
                $( $C_doc_32_ptr, $C_name_32_ptr, $C_type_32_ptr,
                   $C_ptr_32_ptr,
                   [def:$C_def_32_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_8_ptrdep, $C_name_8_ptrdep, $C_type_8_ptrdep,
                   $C_ptrdep_8_ptrdep,
                   [def:$C_def_8_ptrdep]
                )*
                $( $C_doc_16_ptrdep, $C_name_16_ptrdep, $C_type_16_ptrdep,
                   $C_ptrdep_16_ptrdep,
                   [def:$C_def_16_ptrdep]
                )*
                $( $C_doc_32_ptrdep, $C_name_32_ptrdep, $C_type_32_ptrdep,
                   $C_ptrdep_32_ptrdep,
                   [def:$C_def_32_ptrdep]
                )* ;

            noncopy:
                $( $N_doc_8, $N_name_8, $N_type_8,
                   [def:$N_def_8]
                )*
                $( $N_doc_16, $N_name_16, $N_type_16,
                   [def:$N_def_16]
                )*
                $( $N_doc_32, $N_name_32, $N_type_32,
                   [def:$N_def_32]
                )* ;
            noncopy@dep:
                $( $N_doc_8_dep, $N_name_8_dep, $N_type_8_dep,
                   $N_dep1_8_dep, $N_dep2_8_dep,
                   [def:$N_def_8_dep]
                )*
                $( $N_doc_16_dep, $N_name_16_dep, $N_type_16_dep,
                   $N_dep1_16_dep, $N_dep2_16_dep,
                   [def:$N_def_16_dep]
                )*
                $( $N_doc_32_dep, $N_name_32_dep, $N_type_32_dep,
                   $N_dep1_32_dep, $N_dep2_32_dep,
                   [def:$N_def_32_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_8_ptr, $N_name_8_ptr, $N_type_8_ptr,
                   $N_ptr_8_ptr, $N_dep1_8_ptr, $N_dep2_8_ptr,
                   [def:$N_def_8_ptr]
                )*
                $( $N_doc_16_ptr, $N_name_16_ptr, $N_type_16_ptr,
                   $N_ptr_16_ptr, $N_dep1_16_ptr, $N_dep2_16_ptr,
                   [def:$N_def_16_ptr]
                )*
                $( $N_doc_32_ptr, $N_name_32_ptr, $N_type_32_ptr,
                   $N_ptr_32_ptr, $N_dep1_32_ptr, $N_dep2_32_ptr,
                   [def:$N_def_32_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_8_ptrdep, $N_name_8_ptrdep, $N_type_8_ptrdep,
                   $N_ptr_8_ptrdep, $N_dep1_8_ptrdep, $N_dep2_8_ptrdep,
                   [def:$N_def_8_ptrdep]
                )*
                $( $N_doc_16_ptrdep, $N_name_16_ptrdep, $N_type_16_ptrdep,
                   $N_ptr_16_ptrdep, $N_dep1_16_ptrdep, $N_dep2_16_ptrdep,
                   [def:$N_def_16_ptrdep]
                )*
                $( $N_doc_32_ptrdep, $N_name_32_ptrdep, $N_type_32_ptrdep,
                   $N_ptr_32_ptrdep, $N_dep1_32_ptrdep, $N_dep2_32_ptrdep,
                   [def:$N_def_32_ptrdep]
                )* ;
        }
        // 64-bit / 8-Byte
        #[cfg(feature = "_value64")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 64, feature: "_value64",

            copy:
                $( $C_doc_8, $C_name_8, $C_type_8,
                   [def:$C_def_8]
                )*
                $( $C_doc_16, $C_name_16, $C_type_16,
                   [def:$C_def_16]
                )*
                $( $C_doc_32, $C_name_32, $C_type_32,
                   [def:$C_def_32]
                )*
                $( $C_doc_64, $C_name_64, $C_type_64,
                   [def:$C_def_64]
                )* ;
            copy@dep:
                $( $C_doc_8_dep, $C_name_8_dep, $C_type_8_dep,
                   $C_dep1_8_dep, $C_dep2_8_dep,
                   [def:$C_def_8_dep]
                )*
                $( $C_doc_16_dep, $C_name_16_dep, $C_type_16_dep,
                   $C_dep1_16_dep, $C_dep2_16_dep,
                   [def:$C_def_16_dep]
                )*
                $( $C_doc_32_dep, $C_name_32_dep, $C_type_32_dep,
                   $C_dep1_32_dep, $C_dep2_32_dep,
                   [def:$C_def_32_dep]
                )*
                $( $C_doc_64_dep, $C_name_64_dep, $C_type_64_dep,
                   $C_dep1_64_dep, $C_dep2_64_dep,
                   [def:$C_def_64_dep]
                )* ;
            copy@ptr:
                $( $C_doc_8_ptr, $C_name_8_ptr, $C_type_8_ptr,
                   $C_ptr_8_ptr,
                   [def:$C_def_8_ptr]
                )*
                $( $C_doc_16_ptr, $C_name_16_ptr, $C_type_16_ptr,
                   $C_ptr_16_ptr,
                   [def:$C_def_16_ptr]
                )*
                $( $C_doc_32_ptr, $C_name_32_ptr, $C_type_32_ptr,
                   $C_ptr_32_ptr,
                   [def:$C_def_32_ptr]
                )*
                $( $C_doc_64_ptr, $C_name_64_ptr, $C_type_64_ptr,
                   $C_ptr_64_ptr,
                   [def:$C_def_64_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_8_ptrdep, $C_name_8_ptrdep, $C_type_8_ptrdep,
                   $C_ptrdep_8_ptrdep,
                   [def:$C_def_8_ptrdep]
                )*
                $( $C_doc_16_ptrdep, $C_name_16_ptrdep, $C_type_16_ptrdep,
                   $C_ptrdep_16_ptrdep,
                   [def:$C_def_16_ptrdep]
                )*
                $( $C_doc_32_ptrdep, $C_name_32_ptrdep, $C_type_32_ptrdep,
                   $C_ptrdep_32_ptrdep,
                   [def:$C_def_32_ptrdep]
                )*
                $( $C_doc_64_ptrdep, $C_name_64_ptrdep, $C_type_64_ptrdep,
                   $C_ptrdep_64_ptrdep,
                   [def:$C_def_64_ptrdep]
                ),* ;

            noncopy:
                $( $N_doc_8, $N_name_8, $N_type_8,
                   [def:$N_def_8]
                )*
                $( $N_doc_16, $N_name_16, $N_type_16,
                   [def:$N_def_16]
                )*
                $( $N_doc_32, $N_name_32, $N_type_32,
                   [def:$N_def_32]
                )*
                $( $N_doc_64, $N_name_64, $N_type_64,
                   [def:$N_def_64]
                )* ;

            noncopy@dep:
                $( $N_doc_8_dep, $N_name_8_dep, $N_type_8_dep,
                   $N_dep1_8_dep, $N_dep2_8_dep,
                   [def:$N_def_8_dep]
                )*
                $( $N_doc_16_dep, $N_name_16_dep, $N_type_16_dep,
                   $N_dep1_16_dep, $N_dep2_16_dep,
                   [def:$N_def_16_dep]
                )*
                $( $N_doc_32_dep, $N_name_32_dep, $N_type_32_dep,
                   $N_dep1_32_dep, $N_dep2_32_dep,
                   [def:$N_def_32_dep]
                )*
                $( $N_doc_64_dep, $N_name_64_dep, $N_type_64_dep,
                   $N_dep1_64_dep, $N_dep2_64_dep,
                   [def:$N_def_64_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_8_ptr, $N_name_8_ptr, $N_type_8_ptr,
                   $N_ptr_8_ptr, $N_dep1_8_ptr, $N_dep2_8_ptr,
                   [def:$N_def_8_ptr]
                )*
                $( $N_doc_16_ptr, $N_name_16_ptr, $N_type_16_ptr,
                   $N_ptr_16_ptr, $N_dep1_16_ptr, $N_dep2_16_ptr,
                   [def:$N_def_16_ptr]
                )*
                $( $N_doc_32_ptr, $N_name_32_ptr, $N_type_32_ptr,
                   $N_ptr_32_ptr, $N_dep1_32_ptr, $N_dep2_32_ptr,
                   [def:$N_def_32_ptr]
                )*
                $( $N_doc_64_ptr, $N_name_64_ptr, $N_type_64_ptr,
                   $N_ptr_64_ptr, $N_dep1_64_ptr, $N_dep2_64_ptr,
                   [def:$N_def_64_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_8_ptrdep, $N_name_8_ptrdep, $N_type_8_ptrdep,
                   $N_ptr_8_ptrdep, $N_dep1_8_ptrdep, $N_dep2_8_ptrdep,
                   [def:$N_def_8_ptrdep]
                )*
                $( $N_doc_16_ptrdep, $N_name_16_ptrdep, $N_type_16_ptrdep,
                   $N_ptr_16_ptrdep, $N_dep1_16_ptrdep, $N_dep2_16_ptrdep,
                   [def:$N_def_16_ptrdep]
                )*
                $( $N_doc_32_ptrdep, $N_name_32_ptrdep, $N_type_32_ptrdep,
                   $N_ptr_32_ptrdep, $N_dep1_32_ptrdep, $N_dep2_32_ptrdep,
                   [def:$N_def_32_ptrdep]
                )*
                $( $N_doc_64_ptrdep, $N_name_64_ptrdep, $N_type_64_ptrdep,
                   $N_ptr_64_ptrdep, $N_dep1_64_ptrdep, $N_dep2_64_ptrdep,
                   [def:$N_def_64_ptrdep]
                )* ;
        }
        // 128-bit / 16-Byte
        #[cfg(feature = "_value128")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 128, feature: "_value128",

            copy:
                $( $C_doc_8, $C_name_8, $C_type_8,
                   [def:$C_def_8]
                )*
                $( $C_doc_16, $C_name_16, $C_type_16,
                   [def:$C_def_16]
                )*
                $( $C_doc_32, $C_name_32, $C_type_32,
                   [def:$C_def_32]
                )*
                $( $C_doc_64, $C_name_64, $C_type_64,
                   [def:$C_def_64]
                )*
                $( $C_doc_128, $C_name_128, $C_type_128,
                   [def:$C_def_128]
                )* ;
            copy@dep:
                $( $C_doc_8_dep, $C_name_8_dep, $C_type_8_dep,
                   $C_dep1_8_dep, $C_dep2_8_dep,
                   [def:$C_def_8_dep]
                )*
                $( $C_doc_16_dep, $C_name_16_dep, $C_type_16_dep,
                   $C_dep1_16_dep, $C_dep2_16_dep,
                   [def:$C_def_16_dep]
                )*
                $( $C_doc_32_dep, $C_name_32_dep, $C_type_32_dep,
                   $C_dep1_32_dep, $C_dep2_32_dep,
                   [def:$C_def_32_dep]
                )*
                $( $C_doc_64_dep, $C_name_64_dep, $C_type_64_dep,
                   $C_dep1_64_dep, $C_dep2_64_dep,
                   [def:$C_def_64_dep]
                )*
                $( $C_doc_128_dep, $C_name_128_dep, $C_type_128_dep,
                   $C_dep1_128_dep, $C_dep2_128_dep,
                   [def:$C_def_128_dep]
                )* ;
            copy@ptr:
                $( $C_doc_8_ptr, $C_name_8_ptr, $C_type_8_ptr,
                   $C_ptr_8_ptr,
                   [def:$C_def_8_ptr]
                )*
                $( $C_doc_16_ptr, $C_name_16_ptr, $C_type_16_ptr,
                   $C_ptr_16_ptr,
                   [def:$C_def_16_ptr]
                )*
                $( $C_doc_32_ptr, $C_name_32_ptr, $C_type_32_ptr,
                   $C_ptr_32_ptr,
                   [def:$C_def_32_ptr]
                )*
                $( $C_doc_64_ptr, $C_name_64_ptr, $C_type_64_ptr,
                   $C_ptr_64_ptr,
                   [def:$C_def_64_ptr]
                )*
                $( $C_doc_128_ptr, $C_name_128_ptr, $C_type_128_ptr,
                   $C_ptr_128_ptr,
                   [def:$C_def_128_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_8_ptrdep, $C_name_8_ptrdep, $C_type_8_ptrdep,
                   $C_ptrdep_8_ptrdep,
                   [def:$C_def_8_ptrdep]
                )*
                $( $C_doc_16_ptrdep, $C_name_16_ptrdep, $C_type_16_ptrdep,
                   $C_ptrdep_16_ptrdep,
                   [def:$C_def_16_ptrdep]
                )*
                $( $C_doc_32_ptrdep, $C_name_32_ptrdep, $C_type_32_ptrdep,
                   $C_ptrdep_32_ptrdep,
                   [def:$C_def_32_ptrdep]
                )*
                $( $C_doc_64_ptrdep, $C_name_64_ptrdep, $C_type_64_ptrdep,
                   $C_ptrdep_64_ptrdep,
                   [def:$C_def_64_ptrdep]
                )*
                $( $C_doc_128_ptrdep, $C_name_128_ptrdep, $C_type_128_ptrdep,
                   $C_ptrdep_128_ptrdep,
                   [def:$C_def_128_ptrdep]
                )* ;

            noncopy:
                $( $N_doc_8, $N_name_8, $N_type_8,
                   [def:$N_def_8]
                )*
                $( $N_doc_16, $N_name_16, $N_type_16,
                   [def:$N_def_16]
                )*
                $( $N_doc_32, $N_name_32, $N_type_32,
                   [def:$N_def_32]
                )*
                $( $N_doc_64, $N_name_64, $N_type_64,
                   [def:$N_def_64]
                )*
                $( $N_doc_128, $N_name_128, $N_type_128,
                   [def:$N_def_128]
                )* ;
            noncopy@dep:
                $( $N_doc_8_dep, $N_name_8_dep, $N_type_8_dep,
                   $N_dep1_8_dep, $N_dep2_8_dep,
                   [def:$N_def_8_dep]
                )*
                $( $N_doc_16_dep, $N_name_16_dep, $N_type_16_dep,
                   $N_dep1_16_dep, $N_dep2_16_dep,
                   [def:$N_def_16_dep]
                )*
                $( $N_doc_32_dep, $N_name_32_dep, $N_type_32_dep,
                   $N_dep1_32_dep, $N_dep2_32_dep,
                   [def:$N_def_32_dep]
                )*
                $( $N_doc_64_dep, $N_name_64_dep, $N_type_64_dep,
                   $N_dep1_64_dep, $N_dep2_64_dep,
                   [def:$N_def_64_dep]
                )*
                $( $N_doc_128_dep, $N_name_128_dep, $N_type_128_dep,
                   $N_dep1_128_dep, $N_dep2_128_dep,
                   [def:$N_def_128_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_8_ptr, $N_name_8_ptr, $N_type_8_ptr,
                   $N_ptr_8_ptr, $N_dep1_8_ptr, $N_dep2_8_ptr,
                   [def:$N_def_8_ptr]
                )*
                $( $N_doc_16_ptr, $N_name_16_ptr, $N_type_16_ptr,
                   $N_ptr_16_ptr, $N_dep1_16_ptr, $N_dep2_16_ptr,
                   [def:$N_def_16_ptr]
                )*
                $( $N_doc_32_ptr, $N_name_32_ptr, $N_type_32_ptr,
                   $N_ptr_32_ptr, $N_dep1_32_ptr, $N_dep2_32_ptr,
                   [def:$N_def_32_ptr]
                )*
                $( $N_doc_64_ptr, $N_name_64_ptr, $N_type_64_ptr,
                   $N_ptr_64_ptr, $N_dep1_64_ptr, $N_dep2_64_ptr,
                   [def:$N_def_64_ptr]
                )*
                $( $N_doc_128_ptr, $N_name_128_ptr, $N_type_128_ptr,
                   $N_ptr_128_ptr, $N_dep1_128_ptr, $N_dep2_128_ptr,
                   [def:$N_def_128_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_8_ptrdep, $N_name_8_ptrdep, $N_type_8_ptrdep,
                    $N_ptr_8_ptrdep, $N_dep1_8_ptrdep, $N_dep2_8_ptrdep,
                   [def:$N_def_8_ptrdep]
                )*
                $( $N_doc_16_ptrdep, $N_name_16_ptrdep, $N_type_16_ptrdep,
                    $N_ptr_16_ptrdep, $N_dep1_16_ptrdep, $N_dep2_16_ptrdep,
                   [def:$N_def_16_ptrdep]
                )*
                $( $N_doc_32_ptrdep, $N_name_32_ptrdep, $N_type_32_ptrdep,
                    $N_ptr_32_ptrdep, $N_dep1_32_ptrdep, $N_dep2_32_ptrdep,
                   [def:$N_def_32_ptrdep]
                )*
                $( $N_doc_64_ptrdep, $N_name_64_ptrdep, $N_type_64_ptrdep,
                    $N_ptr_64_ptrdep, $N_dep1_64_ptrdep, $N_dep2_64_ptrdep,
                   [def:$N_def_64_ptrdep]
                )*
                $( $N_doc_128_ptrdep, $N_name_128_ptrdep, $N_type_128_ptrdep,
                    $N_ptr_128_ptrdep, $N_dep1_128_ptrdep, $N_dep2_128_ptrdep,
                   [def:$N_def_128_ptrdep]
                )* ;
        }
        // 256-bit / 32-Byte
        #[cfg(feature = "_value256")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 256, feature: "_value256",

            copy:
                $( $C_doc_8, $C_name_8, $C_type_8,
                   [def:$C_def_8]
                )*
                $( $C_doc_16, $C_name_16, $C_type_16,
                   [def:$C_def_16]
                )*
                $( $C_doc_32, $C_name_32, $C_type_32,
                   [def:$C_def_32]
                )*
                $( $C_doc_64, $C_name_64, $C_type_64,
                   [def:$C_def_64]
                )*
                $( $C_doc_128, $C_name_128, $C_type_128,
                   [def:$C_def_128]
                )*
                $( $C_doc_256, $C_name_256, $C_type_256,
                   [def:$C_def_256]
                )* ;
            copy@dep:
                $( $C_doc_8_dep, $C_name_8_dep, $C_type_8_dep,
                   $C_dep1_8_dep, $C_dep2_8_dep,
                   [def:$C_def_8_dep]
                )*
                $( $C_doc_16_dep, $C_name_16_dep, $C_type_16_dep,
                   $C_dep1_16_dep, $C_dep2_16_dep,
                   [def:$C_def_16_dep]
                )*
                $( $C_doc_32_dep, $C_name_32_dep, $C_type_32_dep,
                   $C_dep1_32_dep, $C_dep2_32_dep,
                   [def:$C_def_32_dep]
                )*
                $( $C_doc_64_dep, $C_name_64_dep, $C_type_64_dep,
                   $C_dep1_64_dep, $C_dep2_64_dep,
                   [def:$C_def_64_dep]
                )*
                $( $C_doc_128_dep, $C_name_128_dep, $C_type_128_dep,
                   $C_dep1_128_dep, $C_dep2_128_dep,
                   [def:$C_def_128_dep]
                )*
                $( $C_doc_256_dep, $C_name_256_dep, $C_type_256_dep,
                   $C_dep1_256_dep, $C_dep2_256_dep,
                   [def:$C_def_256_dep]
                )* ;
            copy@ptr:
                $( $C_doc_8_ptr, $C_name_8_ptr, $C_type_8_ptr,
                   $C_ptr_8_ptr,
                   [def:$C_def_8_ptr]
                )*
                $( $C_doc_16_ptr, $C_name_16_ptr, $C_type_16_ptr,
                   $C_ptr_16_ptr,
                   [def:$C_def_16_ptr]
                )*
                $( $C_doc_32_ptr, $C_name_32_ptr, $C_type_32_ptr,
                   $C_ptr_32_ptr,
                   [def:$C_def_32_ptr]
                )*
                $( $C_doc_64_ptr, $C_name_64_ptr, $C_type_64_ptr,
                   $C_ptr_64_ptr,
                   [def:$C_def_64_ptr]
                )*
                $( $C_doc_128_ptr, $C_name_128_ptr, $C_type_128_ptr,
                   $C_ptr_128_ptr,
                   [def:$C_def_128_ptr]
                )*
                $( $C_doc_256_ptr, $C_name_256_ptr, $C_type_256_ptr,
                   $C_ptr_256_ptr,
                   [def:$C_def_256_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_8_ptrdep, $C_name_8_ptrdep, $C_type_8_ptrdep,
                   $C_ptrdep_8_ptrdep,
                   [def:$C_def_8_ptrdep]
                )*
                $( $C_doc_16_ptrdep, $C_name_16_ptrdep, $C_type_16_ptrdep,
                   $C_ptrdep_16_ptrdep,
                   [def:$C_def_16_ptrdep]
                )*
                $( $C_doc_32_ptrdep, $C_name_32_ptrdep, $C_type_32_ptrdep,
                   $C_ptrdep_32_ptrdep,
                   [def:$C_def_32_ptrdep]
                )*
                $( $C_doc_64_ptrdep, $C_name_64_ptrdep, $C_type_64_ptrdep,
                   $C_ptrdep_64_ptrdep,
                   [def:$C_def_64_ptrdep]
                )*
                $( $C_doc_128_ptrdep, $C_name_128_ptrdep, $C_type_128_ptrdep,
                   $C_ptrdep_128_ptrdep,
                   [def:$C_def_128_ptrdep]
                )*
                $( $C_doc_256_ptrdep, $C_name_256_ptrdep, $C_type_256_ptrdep,
                   $C_ptrdep_256_ptrdep,
                   [def:$C_def_256_ptrdep]
                )* ;

            noncopy:
                $( $N_doc_8, $N_name_8, $N_type_8,
                   [def:$N_def_8]
                )*
                $( $N_doc_16, $N_name_16, $N_type_16,
                   [def:$N_def_16]
                )*
                $( $N_doc_32, $N_name_32, $N_type_32,
                   [def:$N_def_32]
                )*
                $( $N_doc_64, $N_name_64, $N_type_64,
                   [def:$N_def_64]
                )*
                $( $N_doc_128, $N_name_128, $N_type_128,
                   [def:$N_def_128]
                )*
                $( $N_doc_256, $N_name_256, $N_type_256,
                   [def:$N_def_256]
                )* ;
            noncopy@dep:
                $( $N_doc_8_dep, $N_name_8_dep, $N_type_8_dep,
                   $N_dep1_8_dep, $N_dep2_8_dep,
                   [def:$N_def_8_dep]
                )*
                $( $N_doc_16_dep, $N_name_16_dep, $N_type_16_dep,
                   $N_dep1_16_dep, $N_dep2_16_dep,
                   [def:$N_def_16_dep]
                )*
                $( $N_doc_32_dep, $N_name_32_dep, $N_type_32_dep,
                   $N_dep1_32_dep, $N_dep2_32_dep,
                   [def:$N_def_32_dep]
                )*
                $( $N_doc_64_dep, $N_name_64_dep, $N_type_64_dep,
                   $N_dep1_64_dep, $N_dep2_64_dep,
                   [def:$N_def_64_dep]
                )*
                $( $N_doc_128_dep, $N_name_128_dep, $N_type_128_dep,
                   $N_dep1_128_dep, $N_dep2_128_dep,
                   [def:$N_def_128_dep]
                )*
                $( $N_doc_256_dep, $N_name_256_dep, $N_type_256_dep,
                   $N_dep1_256_dep, $N_dep2_256_dep,
                   [def:$N_def_256_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_8_ptr, $N_name_8_ptr, $N_type_8_ptr,
                   $N_ptr_8_ptr, $N_dep1_8_ptr, $N_dep2_8_ptr,
                   [def:$N_def_8_ptr]
                )*
                $( $N_doc_16_ptr, $N_name_16_ptr, $N_type_16_ptr,
                   $N_ptr_16_ptr, $N_dep1_16_ptr, $N_dep2_16_ptr,
                   [def:$N_def_16_ptr]
                )*
                $( $N_doc_32_ptr, $N_name_32_ptr, $N_type_32_ptr,
                   $N_ptr_32_ptr, $N_dep1_32_ptr, $N_dep2_32_ptr,
                   [def:$N_def_32_ptr]
                )*
                $( $N_doc_64_ptr, $N_name_64_ptr, $N_type_64_ptr,
                   $N_ptr_64_ptr, $N_dep1_64_ptr, $N_dep2_64_ptr,
                   [def:$N_def_64_ptr]
                )*
                $( $N_doc_128_ptr, $N_name_128_ptr, $N_type_128_ptr,
                   $N_ptr_128_ptr, $N_dep1_128_ptr, $N_dep2_128_ptr,
                   [def:$N_def_128_ptr]
                )*
                $( $N_doc_256_ptr, $N_name_256_ptr, $N_type_256_ptr,
                   $N_ptr_256_ptr, $N_dep1_256_ptr, $N_dep2_256_ptr,
                   [def:$N_def_256_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_8_ptrdep, $N_name_8_ptrdep, $N_type_8_ptrdep,
                  $N_ptr_8_ptrdep, $N_dep1_8_ptrdep, $N_dep2_8_ptrdep,
                   [def:$N_def_8_ptrdep]
                )*
                $( $N_doc_16_ptrdep, $N_name_16_ptrdep, $N_type_16_ptrdep,
                  $N_ptr_16_ptrdep, $N_dep1_16_ptrdep, $N_dep2_16_ptrdep,
                   [def:$N_def_16_ptrdep]
                )*
                $( $N_doc_32_ptrdep, $N_name_32_ptrdep, $N_type_32_ptrdep,
                  $N_ptr_32_ptrdep, $N_dep1_32_ptrdep, $N_dep2_32_ptrdep,
                   [def:$N_def_32_ptrdep]
                )*
                $( $N_doc_64_ptrdep, $N_name_64_ptrdep, $N_type_64_ptrdep,
                  $N_ptr_64_ptrdep, $N_dep1_64_ptrdep, $N_dep2_64_ptrdep,
                   [def:$N_def_64_ptrdep]
                )*
                $( $N_doc_128_ptrdep, $N_name_128_ptrdep, $N_type_128_ptrdep,
                  $N_ptr_128_ptrdep, $N_dep1_128_ptrdep, $N_dep2_128_ptrdep,
                   [def:$N_def_128_ptrdep]
                )*
                $( $N_doc_256_ptrdep, $N_name_256_ptrdep, $N_type_256_ptrdep,
                   $N_ptr_256_ptrdep, $N_dep1_256_ptrdep, $N_dep2_256_ptrdep,
                   [def:$N_def_256_ptrdep]
                )* ;
        }
        // 512-bit / 64-Byte
        #[cfg(feature = "_value512")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 512, feature: "_value512",

            copy:
                $( $C_doc_8, $C_name_8, $C_type_8,
                   [def:$C_def_8]
                )*
                $( $C_doc_16, $C_name_16, $C_type_16,
                   [def:$C_def_16]
                )*
                $( $C_doc_32, $C_name_32, $C_type_32,
                   [def:$C_def_32]
                )*
                $( $C_doc_64, $C_name_64, $C_type_64,
                   [def:$C_def_64]
                )*
                $( $C_doc_128, $C_name_128, $C_type_128,
                   [def:$C_def_128]
                )*
                $( $C_doc_256, $C_name_256, $C_type_256,
                   [def:$C_def_256]
                )*
                $( $C_doc_512, $C_name_512, $C_type_512,
                   [def:$C_def_512]
                )* ;
            copy@dep:
                $( $C_doc_8_dep, $C_name_8_dep, $C_type_8_dep,
                   $C_dep1_8_dep, $C_dep2_8_dep,
                   [def:$C_def_8_dep]
                )*
                $( $C_doc_16_dep, $C_name_16_dep, $C_type_16_dep,
                   $C_dep1_16_dep, $C_dep2_16_dep,
                   [def:$C_def_16_dep]
                )*
                $( $C_doc_32_dep, $C_name_32_dep, $C_type_32_dep,
                   $C_dep1_32_dep, $C_dep2_32_dep,
                   [def:$C_def_32_dep]
                )*
                $( $C_doc_64_dep, $C_name_64_dep, $C_type_64_dep,
                   $C_dep1_64_dep, $C_dep2_64_dep,
                   [def:$C_def_64_dep]
                )*
                $( $C_doc_128_dep, $C_name_128_dep, $C_type_128_dep,
                   $C_dep1_128_dep, $C_dep2_128_dep,
                   [def:$C_def_128_dep]
                )*
                $( $C_doc_256_dep, $C_name_256_dep, $C_type_256_dep,
                   $C_dep1_256_dep, $C_dep2_256_dep,
                   [def:$C_def_256_dep]
                )*
                $( $C_doc_512_dep, $C_name_512_dep, $C_type_512_dep,
                   $C_dep1_512_dep, $C_dep2_512_dep,
                   [def:$C_def_512_dep]
                )* ;
            copy@ptr:
                $( $C_doc_8_ptr, $C_name_8_ptr, $C_type_8_ptr,
                   $C_ptr_8_ptr,
                   [def:$C_def_8_ptr]
                )*
                $( $C_doc_16_ptr, $C_name_16_ptr, $C_type_16_ptr,
                   $C_ptr_16_ptr,
                   [def:$C_def_16_ptr]
                )*
                $( $C_doc_32_ptr, $C_name_32_ptr, $C_type_32_ptr,
                   $C_ptr_32_ptr,
                   [def:$C_def_32_ptr]
                )*
                $( $C_doc_64_ptr, $C_name_64_ptr, $C_type_64_ptr,
                   $C_ptr_64_ptr,
                   [def:$C_def_64_ptr]
                )*
                $( $C_doc_128_ptr, $C_name_128_ptr, $C_type_128_ptr,
                   $C_ptr_128_ptr,
                   [def:$C_def_128_ptr]
                )*
                $( $C_doc_256_ptr, $C_name_256_ptr, $C_type_256_ptr,
                   $C_ptr_256_ptr,
                   [def:$C_def_256_ptr]
                )*
                $( $C_doc_512_ptr, $C_name_512_ptr, $C_type_512_ptr,
                   $C_ptr_512_ptr,
                   [def:$C_def_512_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_8_ptrdep, $C_name_8_ptrdep, $C_type_8_ptrdep,
                   $C_ptrdep_8_ptrdep,
                   [def:$C_def_8_ptrdep]
                )*
                $( $C_doc_16_ptrdep, $C_name_16_ptrdep, $C_type_16_ptrdep,
                   $C_ptrdep_16_ptrdep,
                   [def:$C_def_16_ptrdep]
                )*
                $( $C_doc_32_ptrdep, $C_name_32_ptrdep, $C_type_32_ptrdep,
                   $C_ptrdep_32_ptrdep,
                   [def:$C_def_32_ptrdep]
                )*
                $( $C_doc_64_ptrdep, $C_name_64_ptrdep, $C_type_64_ptrdep,
                   $C_ptrdep_64_ptrdep,
                   [def:$C_def_64_ptrdep]
                )*
                $( $C_doc_128_ptrdep, $C_name_128_ptrdep, $C_type_128_ptrdep,
                   $C_ptrdep_128_ptrdep,
                   [def:$C_def_128_ptrdep]
                )*
                $( $C_doc_256_ptrdep, $C_name_256_ptrdep, $C_type_256_ptrdep,
                   $C_ptrdep_256_ptrdep,
                   [def:$C_def_256_ptrdep]
                )*
                $( $C_doc_512_ptrdep, $C_name_512_ptrdep, $C_type_512_ptrdep,
                   $C_ptrdep_512_ptrdep,
                   [def:$C_def_512_ptrdep]
                )* ;

            noncopy:
                $( $N_doc_8, $N_name_8, $N_type_8,
                   [def:$N_def_8]
                )*
                $( $N_doc_16, $N_name_16, $N_type_16,
                   [def:$N_def_16]
                )*
                $( $N_doc_32, $N_name_32, $N_type_32,
                   [def:$N_def_32]
                )*
                $( $N_doc_64, $N_name_64, $N_type_64,
                   [def:$N_def_64]
                )*
                $( $N_doc_128, $N_name_128, $N_type_128,
                   [def:$N_def_128]
                )*
                $( $N_doc_256, $N_name_256, $N_type_256,
                   [def:$N_def_256]
                )*
                $( $N_doc_512, $N_name_512, $N_type_512,
                   [def:$N_def_512]
                )* ;
            noncopy@dep:
                $( $N_doc_8_dep, $N_name_8_dep, $N_type_8_dep,
                   $N_dep1_8_dep, $N_dep2_8_dep,
                   [def:$N_def_8_dep]
                )*
                $( $N_doc_16_dep, $N_name_16_dep, $N_type_16_dep,
                   $N_dep1_16_dep, $N_dep2_16_dep,
                   [def:$N_def_16_dep]
                )*
                $( $N_doc_32_dep, $N_name_32_dep, $N_type_32_dep,
                   $N_dep1_32_dep, $N_dep2_32_dep,
                   [def:$N_def_32_dep]
                )*
                $( $N_doc_64_dep, $N_name_64_dep, $N_type_64_dep,
                   $N_dep1_64_dep, $N_dep2_64_dep,
                   [def:$N_def_64_dep]
                )*
                $( $N_doc_128_dep, $N_name_128_dep, $N_type_128_dep,
                   $N_dep1_128_dep, $N_dep2_128_dep,
                   [def:$N_def_128_dep]
                )*
                $( $N_doc_256_dep, $N_name_256_dep, $N_type_256_dep,
                   $N_dep1_256_dep, $N_dep2_256_dep,
                   [def:$N_def_256_dep]
                )*
                $( $N_doc_512_dep, $N_name_512_dep, $N_type_512_dep,
                   $N_dep1_512_dep, $N_dep2_512_dep,
                   [def:$N_def_512_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_8_ptr, $N_name_8_ptr, $N_type_8_ptr,
                   $N_ptr_8_ptr, $N_dep1_8_ptr, $N_dep2_8_ptr,
                   [def:$N_def_8_ptr]
                )*
                $( $N_doc_16_ptr, $N_name_16_ptr, $N_type_16_ptr,
                   $N_ptr_16_ptr, $N_dep1_16_ptr, $N_dep2_16_ptr,
                   [def:$N_def_16_ptr]
                )*
                $( $N_doc_32_ptr, $N_name_32_ptr, $N_type_32_ptr,
                   $N_ptr_32_ptr, $N_dep1_32_ptr, $N_dep2_32_ptr,
                   [def:$N_def_32_ptr]
                )*
                $( $N_doc_64_ptr, $N_name_64_ptr, $N_type_64_ptr,
                   $N_ptr_64_ptr, $N_dep1_64_ptr, $N_dep2_64_ptr,
                   [def:$N_def_64_ptr]
                )*
                $( $N_doc_128_ptr, $N_name_128_ptr, $N_type_128_ptr,
                   $N_ptr_128_ptr, $N_dep1_128_ptr, $N_dep2_128_ptr,
                   [def:$N_def_128_ptr]
                )*
                $( $N_doc_256_ptr, $N_name_256_ptr, $N_type_256_ptr,
                   $N_ptr_256_ptr, $N_dep1_256_ptr, $N_dep2_256_ptr,
                   [def:$N_def_256_ptr]
                )*
                $( $N_doc_512_ptr, $N_name_512_ptr, $N_type_512_ptr,
                   $N_ptr_512_ptr, $N_dep1_512_ptr, $N_dep2_512_ptr,
                   [def:$N_def_512_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_8_ptrdep, $N_name_8_ptrdep, $N_type_8_ptrdep,
                   $N_ptr_8_ptrdep, $N_dep1_8_ptrdep, $N_dep2_8_ptrdep,
                   [def:$N_def_8_ptrdep]
                )*
                $( $N_doc_16_ptrdep, $N_name_16_ptrdep, $N_type_16_ptrdep,
                   $N_ptr_16_ptrdep, $N_dep1_16_ptrdep, $N_dep2_16_ptrdep,
                   [def:$N_def_16_ptrdep]
                )*
                $( $N_doc_32_ptrdep, $N_name_32_ptrdep, $N_type_32_ptrdep,
                   $N_ptr_32_ptrdep, $N_dep1_32_ptrdep, $N_dep2_32_ptrdep,
                   [def:$N_def_32_ptrdep]
                )*
                $( $N_doc_64_ptrdep, $N_name_64_ptrdep, $N_type_64_ptrdep,
                   $N_ptr_64_ptrdep, $N_dep1_64_ptrdep, $N_dep2_64_ptrdep,
                   [def:$N_def_64_ptrdep]
                )*
                $( $N_doc_128_ptrdep, $N_name_128_ptrdep, $N_type_128_ptrdep,
                   $N_ptr_128_ptrdep, $N_dep1_128_ptrdep, $N_dep2_128_ptrdep,
                   [def:$N_def_128_ptrdep]
                )*
                $( $N_doc_256_ptrdep, $N_name_256_ptrdep, $N_type_256_ptrdep,
                   $N_ptr_256_ptrdep, $N_dep1_256_ptrdep, $N_dep2_256_ptrdep,
                   [def:$N_def_256_ptrdep]
                )*
                $( $N_doc_512_ptrdep, $N_name_512_ptrdep, $N_type_512_ptrdep,
                   $N_ptr_512_ptrdep, $N_dep1_512_ptrdep, $N_dep2_512_ptrdep,
                   [def:$N_def_512_ptrdep]
                )* ;
        }
        // 1024-bit / 128-Byte
        #[cfg(feature = "_value1024")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 1024, feature: "_value1024",

            copy:
                $( $C_doc_8, $C_name_8, $C_type_8,
                   [def:$C_def_8]
                )*
                $( $C_doc_16, $C_name_16, $C_type_16,
                   [def:$C_def_16]
                )*
                $( $C_doc_32, $C_name_32, $C_type_32,
                   [def:$C_def_32]
                )*
                $( $C_doc_64, $C_name_64, $C_type_64,
                   [def:$C_def_64]
                )*
                $( $C_doc_128, $C_name_128, $C_type_128,
                   [def:$C_def_128]
                )*
                $( $C_doc_256, $C_name_256, $C_type_256,
                   [def:$C_def_256]
                )*
                $( $C_doc_512, $C_name_512, $C_type_512,
                   [def:$C_def_512]
                )*
                $( $C_doc_1264, $C_name_1264, $C_type_1264,
                   [def:$C_def_1024]
                )* ;
            copy@dep:
                $( $C_doc_8_dep, $C_name_8_dep, $C_type_8_dep,
                   $C_dep1_8_dep, $C_dep2_8_dep,
                   [def:$C_def_8_dep]
                )*
                $( $C_doc_16_dep, $C_name_16_dep, $C_type_16_dep,
                   $C_dep1_16_dep, $C_dep2_16_dep,
                   [def:$C_def_16_dep]
                )*
                $( $C_doc_32_dep, $C_name_32_dep, $C_type_32_dep,
                   $C_dep1_32_dep, $C_dep2_32_dep,
                   [def:$C_def_32_dep]
                )*
                $( $C_doc_64_dep, $C_name_64_dep, $C_type_64_dep,
                   $C_dep1_64_dep, $C_dep2_64_dep,
                   [def:$C_def_64_dep]
                )*
                $( $C_doc_128_dep, $C_name_128_dep, $C_type_128_dep,
                   $C_dep1_128_dep, $C_dep2_128_dep,
                   [def:$C_def_128_dep]
                )*
                $( $C_doc_256_dep, $C_name_256_dep, $C_type_256_dep,
                   $C_dep1_256_dep, $C_dep2_256_dep,
                   [def:$C_def_256_dep]
                )*
                $( $C_doc_512_dep, $C_name_512_dep, $C_type_512_dep,
                   $C_dep1_512_dep, $C_dep2_512_dep,
                   [def:$C_def_512_dep]
                )*
                $( $C_doc_1024_dep, $C_name_1024_dep, $C_type_1024_dep,
                   $C_dep1_1024_dep, $C_dep2_1024_dep,
                   [def:$C_def_1024_dep]
                )* ;
            copy@ptr:
                $( $C_doc_8_ptr, $C_name_8_ptr, $C_type_8_ptr,
                   $C_ptr_8_ptr,
                   [def:$C_def_8_ptr]
                )*
                $( $C_doc_16_ptr, $C_name_16_ptr, $C_type_16_ptr,
                   $C_ptr_16_ptr,
                   [def:$C_def_16_ptr]
                )*
                $( $C_doc_32_ptr, $C_name_32_ptr, $C_type_32_ptr,
                   $C_ptr_32_ptr,
                   [def:$C_def_32_ptr]
                )*
                $( $C_doc_64_ptr, $C_name_64_ptr, $C_type_64_ptr,
                   $C_ptr_64_ptr,
                   [def:$C_def_64_ptr]
                )*
                $( $C_doc_128_ptr, $C_name_128_ptr, $C_type_128_ptr,
                   $C_ptr_128_ptr,
                   [def:$C_def_128_ptr]
                )*
                $( $C_doc_256_ptr, $C_name_256_ptr, $C_type_256_ptr,
                   $C_ptr_256_ptr,
                   [def:$C_def_256_ptr]
                )*
                $( $C_doc_512_ptr, $C_name_512_ptr, $C_type_512_ptr,
                   $C_ptr_512_ptr,
                   [def:$C_def_512_ptr]
                )*
                $( $C_doc_1024_ptr, $C_name_1024_ptr, $C_type_1024_ptr,
                   $C_ptr_1024_ptr,
                   [def:$C_def_1024_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_8_ptrdep, $C_name_8_ptrdep, $C_type_8_ptrdep,
                   $C_ptrdep_8_ptrdep,
                   [def:$C_def_8_ptrdep]
                )*
                $( $C_doc_16_ptrdep, $C_name_16_ptrdep, $C_type_16_ptrdep,
                   $C_ptrdep_16_ptrdep,
                   [def:$C_def_16_ptrdep]
                )*
                $( $C_doc_32_ptrdep, $C_name_32_ptrdep, $C_type_32_ptrdep,
                   $C_ptrdep_32_ptrdep,
                   [def:$C_def_32_ptrdep]
                )*
                $( $C_doc_64_ptrdep, $C_name_64_ptrdep, $C_type_64_ptrdep,
                   $C_ptrdep_64_ptrdep,
                   [def:$C_def_64_ptrdep]
                )*
                $( $C_doc_128_ptrdep, $C_name_128_ptrdep, $C_type_128_ptrdep,
                   $C_ptrdep_128_ptrdep,
                   [def:$C_def_128_ptrdep]
                )*
                $( $C_doc_256_ptrdep, $C_name_256_ptrdep, $C_type_256_ptrdep,
                   $C_ptrdep_256_ptrdep,
                   [def:$C_def_256_ptrdep]
                )*
                $( $C_doc_512_ptrdep, $C_name_512_ptrdep, $C_type_512_ptrdep,
                   $C_ptrdep_512_ptrdep,
                   [def:$C_def_512_ptrdep]
                )*
                $( $C_doc_1024_ptrdep, $C_name_1024_ptrdep, $C_type_1024_ptrdep,
                   $C_ptrdep_1024_ptrdep,
                   [def:$C_def_1024_ptrdep]
                )* ;

            noncopy:
                $( $N_doc_8, $N_name_8, $N_type_8,
                   [def:$N_def_8]
                )*
                $( $N_doc_16, $N_name_16, $N_type_16,
                   [def:$N_def_16]
                )*
                $( $N_doc_32, $N_name_32, $N_type_32,
                   [def:$N_def_32]
                )*
                $( $N_doc_64, $N_name_64, $N_type_64,
                   [def:$N_def_64]
                )*
                $( $N_doc_128, $N_name_128, $N_type_128,
                   [def:$N_def_128]
                )*
                $( $N_doc_256, $N_name_256, $N_type_256,
                   [def:$N_def_256]
                )*
                $( $N_doc_512, $N_name_512, $N_type_512,
                   [def:$N_def_512]
                )*
                $( $N_doc_1264, $N_name_1264, $N_type_1264,
                   [def:$N_def_1024]
                )* ;
            noncopy@dep:
                $( $N_doc_8_dep, $N_name_8_dep, $N_type_8_dep,
                   $N_dep1_8_dep, $N_dep2_8_dep,
                   [def:$N_def_8_dep]
                )*
                $( $N_doc_16_dep, $N_name_16_dep, $N_type_16_dep,
                   $N_dep1_16_dep, $N_dep2_16_dep,
                   [def:$N_def_16_dep]
                )*
                $( $N_doc_32_dep, $N_name_32_dep, $N_type_32_dep,
                   $N_dep1_32_dep, $N_dep2_32_dep,
                   [def:$N_def_32_dep]
                )*
                $( $N_doc_64_dep, $N_name_64_dep, $N_type_64_dep,
                   $N_dep1_64_dep, $N_dep2_64_dep,
                   [def:$N_def_64_dep]
                )*
                $( $N_doc_128_dep, $N_name_128_dep, $N_type_128_dep,
                   $N_dep1_128_dep, $N_dep2_128_dep,
                   [def:$N_def_128_dep]
                )*
                $( $N_doc_256_dep, $N_name_256_dep, $N_type_256_dep,
                   $N_dep1_256_dep, $N_dep2_256_dep,
                   [def:$N_def_256_dep]
                )*
                $( $N_doc_512_dep, $N_name_512_dep, $N_type_512_dep,
                   $N_dep1_512_dep, $N_dep2_512_dep,
                   [def:$N_def_512_dep]
                )*
                $( $N_doc_1024_dep, $N_name_1024_dep, $N_type_1024_dep,
                   $N_dep1_1024_dep, $N_dep2_1024_dep,
                   [def:$N_def_1024_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_8_ptr, $N_name_8_ptr, $N_type_8_ptr,
                   $N_ptr_8_ptr, $N_dep1_8_ptr, $N_dep2_8_ptr,
                   [def:$N_def_8_ptr]
                )*
                $( $N_doc_16_ptr, $N_name_16_ptr, $N_type_16_ptr,
                   $N_ptr_16_ptr, $N_dep1_16_ptr, $N_dep2_16_ptr,
                   [def:$N_def_16_ptr]
                )*
                $( $N_doc_32_ptr, $N_name_32_ptr, $N_type_32_ptr,
                   $N_ptr_32_ptr, $N_dep1_32_ptr, $N_dep2_32_ptr,
                   [def:$N_def_32_ptr]
                )*
                $( $N_doc_64_ptr, $N_name_64_ptr, $N_type_64_ptr,
                   $N_ptr_64_ptr, $N_dep1_64_ptr, $N_dep2_64_ptr,
                   [def:$N_def_64_ptr]
                )*
                $( $N_doc_128_ptr, $N_name_128_ptr, $N_type_128_ptr,
                   $N_ptr_128_ptr, $N_dep1_128_ptr, $N_dep2_128_ptr,
                   [def:$N_def_128_ptr]
                )*
                $( $N_doc_256_ptr, $N_name_256_ptr, $N_type_256_ptr,
                   $N_ptr_256_ptr, $N_dep1_256_ptr, $N_dep2_256_ptr,
                   [def:$N_def_256_ptr]
                )*
                $( $N_doc_512_ptr, $N_name_512_ptr, $N_type_512_ptr,
                   $N_ptr_512_ptr, $N_dep1_512_ptr, $N_dep2_512_ptr,
                   [def:$N_def_512_ptr]
                )*
                $( $N_doc_1024_ptr, $N_name_1024_ptr, $N_type_1024_ptr,
                   $N_ptr_1024_ptr, $N_dep1_1024_ptr, $N_dep2_1024_ptr,
                   [def:$N_def_1024_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_8_ptrdep, $N_name_8_ptrdep, $N_type_8_ptrdep,
                   $N_ptr_8_ptrdep, $N_dep1_8_ptrdep, $N_dep2_8_ptrdep,
                   [def:$N_def_8_ptrdep]
                )*
                $( $N_doc_16_ptrdep, $N_name_16_ptrdep, $N_type_16_ptrdep,
                   $N_ptr_16_ptrdep, $N_dep1_16_ptrdep, $N_dep2_16_ptrdep,
                   [def:$N_def_16_ptrdep]
                )*
                $( $N_doc_32_ptrdep, $N_name_32_ptrdep, $N_type_32_ptrdep,
                   $N_ptr_32_ptrdep, $N_dep1_32_ptrdep, $N_dep2_32_ptrdep,
                   [def:$N_def_32_ptrdep]
                )*
                $( $N_doc_64_ptrdep, $N_name_64_ptrdep, $N_type_64_ptrdep,
                   $N_ptr_64_ptrdep, $N_dep1_64_ptrdep, $N_dep2_64_ptrdep,
                   [def:$N_def_64_ptrdep]
                )*
                $( $N_doc_128_ptrdep, $N_name_128_ptrdep, $N_type_128_ptrdep,
                   $N_ptr_128_ptrdep, $N_dep1_128_ptrdep, $N_dep2_128_ptrdep,
                   [def:$N_def_128_ptrdep]
                )*
                $( $N_doc_256_ptrdep, $N_name_256_ptrdep, $N_type_256_ptrdep,
                   $N_ptr_256_ptrdep, $N_dep1_256_ptrdep, $N_dep2_256_ptrdep,
                   [def:$N_def_256_ptrdep]
                )*
                $( $N_doc_512_ptrdep, $N_name_512_ptrdep, $N_type_512_ptrdep,
                   $N_ptr_512_ptrdep, $N_dep1_512_ptrdep, $N_dep2_512_ptrdep,
                   [def:$N_def_512_ptrdep]
                )*
                $( $N_doc_1024_ptrdep, $N_name_1024_ptrdep, $N_type_1024_ptrdep,
                   $N_ptr_1024_ptrdep, $N_dep1_1024_ptrdep, $N_dep2_1024_ptrdep,
                   [def:$N_def_1024_ptrdep]
                )* ;
        }
    };
    (
    // -------------------------------------------------------------------------

    // This arm defines in one pass a single size `DataValue*`, `DataType*`, `DataRaw*`.
    //
    // It calls the macros: `define_data_value!`, `define_data_type!` and `define_data_raw!`.
    single_size: $Value:ident, $Type:ident, $Raw:ident,
    size: $b:literal,
    feature: $feature:literal,

    copy:
        $( $C_doc:literal, $C_name:ident, $C_type:ty,
           [def:$C_def:stmt]
        )* ;
    copy@dep:
        $( $C_doc_dep:literal, $C_name_dep:ident, $C_type_dep:ty,
           $C_dep1_dep:literal, $C_dep2_dep:literal,
           [def:$C_def_dep:stmt]
        )* ;
    copy@ptr:
        $( $C_doc_ptr:literal, $C_name_ptr:ident, $C_type_ptr:ty,
           $C_ptr_ptr:meta,
           [def:$C_def_ptr:stmt]
        )* ;
    copy@ptrdep:
        $( $C_doc_ptrdep:literal, $C_name_ptrdep:ident, $C_type_ptrdep:ty,
           $C_ptr_ptrdep:meta, $C_dep1_ptrdep:literal, $C_dep2_ptrdep:literal,
           [def:$C_def_ptrdep:stmt]
        )* ;

    noncopy:
        $( $N_doc:literal, $N_name:ident, $N_type:ty,
           [def:$N_def:stmt]
        )* ;
    noncopy@dep:
        $( $N_doc_dep:literal, $N_name_dep:ident, $N_type_dep:ty,
           $N_dep1_dep:literal, $N_dep2_dep:literal,
           [def:$N_def_dep:stmt]
        )* ;
    noncopy@ptr:
        $( $N_doc_ptr:literal, $N_name_ptr:ident, $N_type_ptr:ty,
           $N_ptr_ptr:meta, $N_dep1_ptr:literal, $N_dep2_ptr:literal,
           [def:$N_def_ptr:stmt]
        )* ;
    noncopy@ptrdep:
        $( $N_doc_ptrdep:literal, $N_name_ptrdep:ident, $N_type_ptrdep:ty,
           $N_ptr_ptrdep:meta, $N_dep1_ptrdep:literal, $N_dep2_ptrdep:literal,
           [def:$N_def_ptrdep:stmt]
        )* ;
    ) => {
        $crate::define_data_value! {
            v: $Value, t: $Type, r: $Raw,
            size: $b, feature: $feature,

            copy:
                $( $C_doc, $C_name, $C_type,
                   [def:$C_def]
                )* ;
            copy@dep:
                $( $C_doc_dep, $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep,
                   [def:$C_def_dep]
                )* ;
            copy@ptr:
                $( $C_doc_ptr, $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr,
                   [def:$C_def_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_ptrdep, $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep, $C_dep1_ptrdep, $C_dep2_ptrdep,
                   [def:$C_def_ptrdep]
                )* ;

            noncopy:
                $( $N_doc, $N_name, $N_type,
                   [def:$N_def]
                )* ;
            noncopy@dep:
                $( $N_doc_dep, $N_name_dep, $N_type_dep,
                   $N_dep1_dep, $N_dep2_dep,
                   [def:$N_def_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_ptr, $N_name_ptr, $N_type_ptr,
                   $N_ptr_ptr, $N_dep1_ptr, $N_dep2_ptr,
                   [def:$N_def_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_ptrdep, $N_name_ptrdep, $N_type_ptrdep,
                   $N_ptr_ptrdep, $N_dep1_ptrdep, $N_dep2_ptrdep,
                   [def:$N_def_ptrdep]
                )* ;
        }
        $crate::define_data_type! {
            v: $Value, t: $Type, r: $Raw,
            size: $b, feature: $feature,

            copy:
                $( $C_doc, $C_name, $C_type,
                   [def:$C_def]
                )* ;
            copy@dep:
                $( $C_doc_dep, $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep,
                   [def:$C_def_dep]
                )* ;
            copy@ptr:
                $( $C_doc_ptr, $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr,
                   [def:$C_def_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_ptrdep, $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep, $C_dep1_ptrdep, $C_dep2_ptrdep,
                   [def:$C_def_ptrdep]
                )* ;

            noncopy:
                $( $N_doc, $N_name, $N_type,
                   [def:$N_def]
                )* ;
            noncopy@dep:
                $( $N_doc_dep, $N_name_dep, $N_type_dep,
                   $N_dep1_dep, $N_dep2_dep,
                   [def:$N_def_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_ptr, $N_name_ptr, $N_type_ptr,
                   $N_ptr_ptr, $N_dep1_ptr, $N_dep2_ptr,
                   [def:$N_def_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_ptrdep, $N_name_ptrdep, $N_type_ptrdep,
                   $N_ptr_ptrdep, $N_dep1_ptrdep, $N_dep2_ptrdep,
                   [def:$N_def_ptrdep]
                )* ;
        }
        $crate::define_data_raw! {
            v: $Value, t: $Type, r: $Raw,
            size: $b, feature: $feature,

            copy:
                $( $C_doc, $C_name, $C_type,
                   [def:$C_def]
                )* ;
            copy@dep:
                $( $C_doc_dep, $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep,
                   [def:$C_def_dep]
                )* ;
            copy@ptr:
                $( $C_doc_ptr, $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr,
                   [def:$C_def_ptr]
                )* ;
            copy@ptrdep:
                $( $C_doc_ptrdep, $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep, $C_dep1_ptrdep, $C_dep2_ptrdep,
                   [def:$C_def_ptrdep]
                )* ;

            noncopy:
                $( $N_doc, $N_name, $N_type,
                   [def:$N_def]
                )* ;
            noncopy@dep:
                $( $N_doc_dep, $N_name_dep, $N_type_dep,
                   $N_dep1_dep, $N_dep2_dep,
                   [def:$N_def_dep]
                )* ;
            noncopy@ptr:
                $( $N_doc_ptr, $N_name_ptr, $N_type_ptr,
                   $N_ptr_ptr, $N_dep1_ptr, $N_dep2_ptr,
                   [def:$N_def_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_doc_ptrdep, $N_name_ptrdep, $N_type_ptrdep,
                   $N_ptr_ptrdep, $N_dep1_ptrdep, $N_dep2_ptrdep,
                   [def:$N_def_ptrdep]
                )* ;
        }
    };
}
pub(crate) use define_data_value_type_raw;
