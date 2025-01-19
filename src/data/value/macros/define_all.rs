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
    // already classified according to the following grouping:
    //
    // - copy@nB:              (name, type),*
    // - copy@nB_dep:          (doc, name, type, dep1, dep2),*
    // - copy@nB_ptr:          (doc, name, type, ptr),*
    // - copy@nB_ptrdep:       (doc, name, type, ptr, dep1, dep2),*
    // - noncopy@nB:           (doc, name, type),*
    // - noncopy@nB_dep:       (doc, name, type, dep1, dep2),*
    // - noncopy@nB_ptr:       (doc, name, type, ptr),*
    // - noncopy@nB_ptrdep:    (doc, name, type, ptr, dep1, dep2),*
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

    // 1-Byte / 8-bit
    copy@1B: $(
        $C_doc_1B:literal, $C_name_1B:ident, $C_type_1B:ty,
        [def:$C_def_1B:stmt],
        )*
    copy@1B_dep: $(
        $C_doc_1B_dep:literal, $C_name_1B_dep:ident, $C_type_1B_dep:ty,
        $C_dep1_1B_dep:literal, $C_dep2_1B_dep:literal,
        [def:$C_def_1B_dep:stmt],
        )*
    copy@1B_ptr: $(
        $C_doc_1B_ptr:literal, $C_name_1B_ptr:ident, $C_type_1B_ptr:ty,
        $C_ptr_1B_ptr:meta,
        [def:$C_def_1B_ptr:stmt],
        )*
    copy@1B_ptrdep: $(
        $C_doc_1B_ptrdep:literal, $C_name_1B_ptrdep:ident, $C_type_1B_ptrdep:ty,
        $C_ptrdep_1B_ptrdep:meta,
        [def:$C_def_1B_ptrdep:stmt],
        )*

    noncopy@1B: $(
        $N_doc_1B:literal, $N_name_1B:ident, $N_type_1B:ty,
        [def:$N_def_1B:stmt],
        )*
    noncopy@1B_dep: $(
        $N_doc_1B_dep:literal, $N_name_1B_dep:ident, $N_type_1B_dep:ty,
        $N_dep2_1B_dep:literal,
        [def:$N_def_1B_dep:stmt],
        )*
    noncopy@1B_ptr: $(
        $N_doc_1B_ptr:literal, $N_name_1B_ptr:ident, $N_type_1B_ptr:ty,
        $N_ptr_1B_ptr:meta,
        [def:$N_def_1B_ptr:stmt],
        )*
    noncopy@1B_ptrdep: $(
        $N_doc_1B_ptrdep:literal, $N_name_1B_ptrdep:ident, $N_type_1B_ptrdep:ty,
        $N_ptr_1B_ptrdep:meta,
        $N_dep1_1B_ptrdep:literal, $N_dep2_1B_ptrdep:literal
        [def:$N_def_1B_ptrdep:stmt],
        ),*

    // 2-Byte / 16-bit
    copy@2B: $(
        $C_doc_2B:literal, $C_name_2B:ident, $C_type_2B:ty,
        [def:$C_def_2B:stmt],
        )*
    copy@2B_dep: $(
        $C_doc_2B_dep:literal, $C_name_2B_dep:ident, $C_type_2B_dep:ty,
        $C_dep1_2B_dep:literal, $C_dep2_2B_dep:literal,
        [def:$C_def_2B_dep:stmt],
        )*
    copy@2B_ptr: $(
        $C_doc_2B_ptr:literal, $C_name_2B_ptr:ident, $C_type_2B_ptr:ty,
        $C_ptr_2B_ptr:meta,
        [def:$C_def_2B_ptr:stmt],
        )*
    copy@2B_ptrdep: $(
        $C_doc_2B_ptrdep:literal, $C_name_2B_ptrdep:ident, $C_type_2B_ptrdep:ty,
        $C_ptrdep_2B_ptrdep:meta,
        [def:$C_def_2B_ptrdep:stmt],
        )*

    noncopy@2B: $(
        $N_doc_2B:literal, $N_name_2B:ident, $N_type_2B:ty,
        [def:$N_def_2B:stmt],
        )*
    noncopy@2B_dep: $(
        $N_doc_2B_dep:literal, $N_name_2B_dep:ident, $N_type_2B_dep:ty,
        $N_dep1_2B_dep:literal,
        [def:$N_def_2B_dep:stmt],
        )*
    noncopy@2B_ptr: $(
        $N_doc_2B_ptr:literal, $N_name_2B_ptr:ident, $N_type_2B_ptr:ty,
        $N_ptr_2B_ptr:meta,
        [def:$N_def_2B_ptr:stmt],
        )*
    noncopy@2B_ptrdep: $(
        $N_doc_2B_ptrdep:literal, $N_name_2B_ptrdep:ident, $N_type_2B_ptrdep:ty,
        $N_ptr_2B_ptrdep:meta,
        $N_dep1_2B_ptrdep:literal, $N_dep2_2B_ptrdep:literal,
        [def:$N_def_2B_ptrdep:stmt],
        )*

    // 4-Byte / 32-bit
    copy@4B: $(
        $C_doc_4B:literal, $C_name_4B:ident, $C_type_4B:ty,
        [def:$C_def_4B:stmt],
        )*
    copy@4B_dep: $(
        $C_doc_4B_dep:literal, $C_name_4B_dep:ident, $C_type_4B_dep:ty,
        $C_dep1_4B_dep:literal, $C_dep2_4B_dep:literal,
        [def:$C_def_4B_dep:stmt],
        )*
    copy@4B_ptr: $(
        $C_doc_4B_ptr:literal, $C_name_4B_ptr:ident, $C_type_4B_ptr:ty,
        $C_ptr_4B_ptr:meta,
        [def:$C_def_4B_ptr:stmt],
        )*
    copy@4B_ptrdep: $(
        $C_doc_4B_ptrdep:literal, $C_name_4B_ptrdep:ident, $C_type_4B_ptrdep:ty,
        $C_ptrdep_4B_ptrdep:meta,
        [def:$C_def_4B_ptrdep:stmt],
        )*

    noncopy@4B: $(
        $N_doc_4B:literal, $N_name_4B:ident, $N_type_4B:ty,
        [def:$N_def_4B:stmt],
        )*
    noncopy@4B_dep: $(
        $N_doc_4B_dep:literal, $N_name_4B_dep:ident, $N_type_4B_dep:ty,
        $N_dep1_4B_dep:literal, $N_dep2_4B_dep:literal,
        [def:$N_def_4B_dep:stmt],
        )*
    noncopy@4B_ptr: $(
        $N_doc_4B_ptr:literal, $N_name_4B_ptr:ident, $N_type_4B_ptr:ty,
        $N_ptr_4B_ptr:meta,
        [def:$N_def_4B_ptr:stmt],
        )*
    noncopy@4B_ptrdep: $(
        $N_doc_4B_ptrdep:literal, $N_name_4B_ptrdep:ident, $N_type_4B_ptrdep:ty,
        $N_ptr_4B_ptrdep:meta,
        $N_dep1_4B_ptrdep:literal, $N_dep2_4B_ptrdep:literal,
        [def:$N_def_4B_ptrdep:stmt],
        )*

    // 8-Byte / 64-bit
    copy@8B: $(
        $C_doc_8B:literal, $C_name_8B:ident, $C_type_8B:ty,
        [def:$C_def_8B:stmt],
        )*
    copy@8B_dep: $(
        $C_doc_8B_dep:literal, $C_name_8B_dep:ident, $C_type_8B_dep:ty,
        $C_dep1_8B_dep:literal, $C_dep2_8B_dep:literal,
        [def:$C_def_8B_dep:stmt],
        )*
    copy@8B_ptr: $(
        $C_doc_8B_ptr:literal, $C_name_8B_ptr:ident, $C_type_8B_ptr:ty,
        $C_ptr_8B_ptr:meta,
        [def:$C_def_8B_ptr:stmt],
        )*
    copy@8B_ptrdep: $(
        $C_doc_8B_ptrdep:literal, $C_name_8B_ptrdep:ident, $C_type_8B_ptrdep:ty,
        $C_ptrdep_8B_ptrdep:meta,
        [def:$C_def_8B_ptrdep:stmt],
        )*

    noncopy@8B: $(
        $N_doc_8B:literal, $N_name_8B:ident, $N_type_8B:ty,
        [def:$N_def_8B:stmt],
        )*
    noncopy@8B_dep: $(
        $N_doc_8B_dep:literal, $N_name_8B_dep:ident, $N_type_8B_dep:ty,
        $N_dep1_8B_dep:literal, $N_dep2_8B_dep:literal,
        [def:$N_def_8B_dep:stmt],
        )*
    noncopy@8B_ptr: $(
        $N_doc_8B_ptr:literal, $N_name_8B_ptr:ident, $N_type_8B_ptr:ty,
        $N_ptr_8B_ptr:meta,
        [def:$N_def_8B_ptr:stmt],
        )*
    noncopy@8B_ptrdep: $(
        $N_doc_8B_ptrdep:literal, $N_name_8B_ptrdep:ident, $N_type_8B_ptrdep:ty,
        $N_ptr_8B_ptrdep:meta,
        $N_dep1_8B_ptrdep:literal, $N_dep2_8B_ptrdep:literal,
        [def:$N_def_8B_ptrdep:stmt],
        )*

    // 16-Byte / 128-bit
    copy@16B: $(
        $C_doc_16B:literal, $C_name_16B:ident, $C_type_16B:ty,
        [def:$C_def_16B:stmt],
        )*
    copy@16B_dep: $(
        $C_doc_16B_dep:literal, $C_name_16B_dep:ident, $C_type_16B_dep:ty,
        $C_dep1_16B_dep:literal, $C_dep2_16B_dep:literal,
        [def:$C_def_16B_dep:stmt],
        )*
    copy@16B_ptr: $(
        $C_doc_16B_ptr:literal, $C_name_16B_ptr:ident, $C_type_16B_ptr:ty,
        $C_ptr_16B_ptr:meta,
        [def:$C_def_16B_ptr:stmt],
        )*
    copy@16B_ptrdep: $(
        $C_doc_16B_ptrdep:literal, $C_name_16B_ptrdep:ident, $C_type_16B_ptrdep:ty,
        $C_ptrdep_16B_ptrdep:meta,
        [def:$C_def_16B_ptrdep:stmt],
        )*

    noncopy@16B: $(
        $N_doc_16B:literal, $N_name_16B:ident, $N_type_16B:ty,
        [def:$N_def_16B:stmt],
        )*
    noncopy@16B_dep: $(
        $N_doc_16B_dep:literal, $N_name_16B_dep:ident, $N_type_16B_dep:ty,
        $N_dep1_16B_dep:literal, $N_dep2_16B_dep:literal,
        [def:$N_def_16B_dep:stmt],
        )*
    noncopy@16B_ptr: $(
        $N_doc_16B_ptr:literal, $N_name_16B_ptr:ident, $N_type_16B_ptr:ty,
        $N_ptr_16B_ptr:meta,
        [def:$N_def_16B_ptr:stmt],
        )*
    noncopy@16B_ptrdep: $(
        $N_doc_16B_ptrdep:literal, $N_name_16B_ptrdep:ident, $N_type_16B_ptrdep:ty,
        $N_ptr_16B_ptrdep:meta,
        $N_dep1_16B_ptrdep:literal, $N_dep2_16B_ptrdep:literal,
        [def:$N_def_16B_ptrdep:stmt],
        )*

    // 32-Byte / 256-bit
    copy@32B: $(
        $C_doc_32B:literal, $C_name_32B:ident, $C_type_32B:ty,
        [def:$C_def_32B:stmt],
        )*
    copy@32B_dep: $(
        $C_doc_32B_dep:literal, $C_name_32B_dep:ident, $C_type_32B_dep:ty,
        $C_dep1_32B_dep:literal, $C_dep2_32B_dep:literal,
        [def:$C_def_32B_dep:stmt],
        )*
    copy@32B_ptr: $(
        $C_doc_32B_ptr:literal, $C_name_32B_ptr:ident, $C_type_32B_ptr:ty,
        $C_ptr_32B_ptr:meta,
        [def:$C_def_32B_ptr:stmt],
        )*
    copy@32B_ptrdep: $(
        $C_doc_32B_ptrdep:literal, $C_name_32B_ptrdep:ident, $C_type_32B_ptrdep:ty,
        $C_ptrdep_32B_ptrdep:meta,
        [def:$C_def_32B_ptrdep:stmt],
        )*

    noncopy@32B: $(
        $N_doc_32B:literal, $N_name_32B:ident, $N_type_32B:ty,
        [def:$N_def_32B:stmt],
        )*
    noncopy@32B_dep: $(
        $N_doc_32B_dep:literal, $N_name_32B_dep:ident, $N_type_32B_dep:ty,
        $N_dep1_32B_dep:literal, $N_dep2_32B_dep:literal,
        [def:$N_def_32B_dep:stmt],
        )*
    noncopy@32B_ptr: $(
        $N_doc_32B_ptr:literal, $N_name_32B_ptr:ident, $N_type_32B_ptr:ty,
        $N_ptr_32B_ptr:meta,
        [def:$N_def_32B_ptr:stmt],
        )*
    noncopy@32B_ptrdep: $(
        $N_doc_32B_ptrdep:literal, $N_name_32B_ptrdep:ident, $N_type_32B_ptrdep:ty,
        $N_ptr_32B_ptrdep:meta,
        $N_dep1_32B_ptrdep:literal, $N_dep2_32B_ptrdep:literal,
        [def:$N_def_32B_ptrdep:stmt],
        )*

    // 64-Byte / 512-bit
    copy@64B: $(
        $C_doc_64B:literal, $C_name_64B:ident, $C_type_64B:ty,
        [def:$C_def_64B:stmt],
        )*
    copy@64B_dep: $(
        $C_doc_64B_dep:literal, $C_name_64B_dep:ident, $C_type_64B_dep:ty,
        $C_dep1_64B_dep:literal, $C_dep2_64B_dep:literal,
        [def:$C_def_64B_dep:stmt],
        )*
    copy@64B_ptr: $(
        $C_doc_64B_ptr:literal, $C_name_64B_ptr:ident, $C_type_64B_ptr:ty,
        $C_ptr_64B_ptr:meta,
        [def:$C_def_64B_ptr:stmt],
        )*
    copy@64B_ptrdep: $(
        $C_doc_64B_ptrdep:literal, $C_name_64B_ptrdep:ident, $C_type_64B_ptrdep:ty,
        $C_ptrdep_64B_ptrdep:meta,
        [def:$C_def_64B_ptrdep:stmt],
        )*

    noncopy@64B: $(
        $N_doc_64B:literal, $N_name_64B:ident, $N_type_64B:ty,
        [def:$N_def_64B:stmt],
        )*
    noncopy@64B_dep: $(
        $N_doc_64B_dep:literal, $N_name_64B_dep:ident, $N_type_64B_dep:ty,
        $N_dep1_64B_dep:literal, $N_dep2_64B_dep:literal,
        [def:$N_def_64B_dep:stmt],
        )*
    noncopy@64B_ptr: $(
        $N_doc_64B_ptr:literal, $N_name_64B_ptr:ident, $N_type_64B_ptr:ty,
        $N_ptr_64B_ptr:meta,
        [def:$N_def_64B_ptr:stmt],
        )*
    noncopy@64B_ptrdep: $(
        $N_doc_64B_ptrdep:literal, $N_name_64B_ptrdep:ident, $N_type_64B_ptrdep:ty,
        $N_ptr_64B_ptrdep:meta,
        $N_dep1_64B_ptrdep:literal, $N_dep2_64B_ptrdep:literal,
        [def:$N_def_64B_ptrdep:stmt],
        )*

    // 128-Byte / 1024-bit
    copy@128B: $(
        $C_doc_128B:literal, $C_name_128B:ident, $C_type_128B:ty,
        [def:$C_def_128B:stmt],
        )*
    copy@128B_dep: $(
        $C_doc_128B_dep:literal, $C_name_128B_dep:ident, $C_type_128B_dep:ty,
        $C_dep1_128B_dep:literal, $C_dep2_128B_dep:literal,
        [def:$C_def_128B_dep:stmt],
        )*
    copy@128B_ptr: $(
        $C_doc_128B_ptr:literal, $C_name_128B_ptr:ident, $C_type_128B_ptr:ty,
        $C_ptr_128B_ptr:meta,
        [def:$C_def_128B_ptr:stmt],
        )*
    copy@128B_ptrdep: $(
        $C_doc_128B_ptrdep:literal, $C_name_128B_ptrdep:ident, $C_type_128B_ptrdep:ty,
        $C_ptrdep_128B_ptrdep:meta,
        [def:$C_def_128B_ptrdep:stmt],
        )*

    noncopy@128B: $(
        $N_doc_128B:literal, $N_name_128B:ident, $N_type_128B:ty,
        [def:$N_def_128B:stmt],
        )*
    noncopy@128B_dep: $(
        $N_doc_128B_dep:literal, $N_name_128B_dep:ident, $N_type_128B_dep:ty,
        $N_dep1_128B_dep:literal, $N_dep2_128B_dep:literal,
        [def:$N_def_128B_dep:stmt],
        )*
    noncopy@128B_ptr: $(
        $N_doc_128B_ptr:literal, $N_name_128B_ptr:ident, $N_type_128B_ptr:ty,
        $N_ptr_128B_ptr:meta,
        [def:$N_def_128B_ptr:stmt],
        )*
    noncopy@128B_ptrdep: $(
        $N_doc_128B_ptrdep:literal, $N_name_128B_ptrdep:ident, $N_type_128B_ptrdep:ty,
        $N_ptr_128B_ptrdep:meta,
        $N_dep1_128B_ptrdep:literal, $N_dep2_128B_ptrdep:literal,
        [def:$N_def_128B_ptrdep:stmt],
        )*

    ) => {
        // 1-Byte / 8-bit
        #[cfg(feature = "_value8")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 1, 8, feature: "_value8",

            copy:
                $( $C_doc_1B, $C_name_1B, $C_type_1B,
                )* ;
            copy@dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep,
                )* ;
            copy@ptr:
                $( $C_doc_1B_ptr, $C_name_1B_ptr, $C_type_1B_ptr,
                    $C_ptr_1B_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_1B_ptrdep, $C_name_1B_ptrdep, $C_type_1B_ptrdep,
                    $C_ptrdep_1B_ptrdep,
                )* ;

            noncopy:
                $( $N_doc_1B, $N_name_1B, $N_type_1B,
                )* ;
            noncopy@dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_1B_ptr, $N_name_1B_ptr, $N_type_1B_ptr,
                    $N_ptr_1B_ptr, $N_dep1_1B_ptr, $N_dep2_1B_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_1B_ptrdep, $N_name_1B_ptrdep, $N_type_1B_ptrdep,
                    $N_ptr_1B_ptrdep,
                    $N_dep1_1B_ptrdep, $N_dep2_1B_ptrdep,
                )* ;
        }
        // 2-Byte / 16-bit
        #[cfg(feature = "_value16")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 2, 16, feature: "_value16",

            copy:
                $( $C_doc_1B, $C_name_1B, $C_type_1B,
                )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B,
                )* ;
            copy@dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep,
                )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep,

                )* ;
            copy@ptr:
                $( $C_doc_1B_ptr, $C_name_1B_ptr, $C_type_1B_ptr,
                    $C_ptr_1B_ptr,
                )*
                $( $C_doc_2B_ptr, $C_name_2B_ptr, $C_type_2B_ptr,
                    $C_ptr_2B_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_1B_ptrdep, $C_name_1B_ptrdep, $C_type_1B_ptrdep,
                    $C_ptrdep_1B_ptrdep,
                )*
                $( $C_doc_2B_ptrdep, $C_name_2B_ptrdep, $C_type_2B_ptrdep,
                    $C_ptrdep_2B_ptrdep,
                )* ;

            noncopy:
                $( $N_doc_1B, $N_name_1B, $N_type_1B,
                )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B,
                )* ;
            noncopy@dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep,
                )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_1B_ptr, $N_name_1B_ptr, $N_type_1B_ptr,
                    $N_ptr_1B_ptr, $N_dep1_1B_ptr, $N_dep2_1B_ptr,
                )*
                $( $N_doc_2B_ptr, $N_name_2B_ptr, $N_type_2B_ptr,
                    $N_ptr_2B_ptr, $N_dep1_2B_ptr, $N_dep2_2B_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_1B_ptrdep, $N_name_1B_ptrdep, $N_type_1B_ptrdep,
                    $N_ptr_1B_ptrdep, $N_dep1_1B_ptrdep, $N_dep2_1B_ptrdep,
                )*
                $( $N_doc_2B_ptrdep, $N_name_2B_ptrdep, $N_type_2B_ptrdep,
                    $N_ptr_2B_ptrdep, $N_dep1_2B_ptrdep, $N_dep2_2B_ptrdep,
                )* ;
        }
        // 4-Byte / 32-bit
        #[cfg(feature = "_value32")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 4, 32, feature: "_value32",

            copy:
                $( $C_doc_1B, $C_name_1B, $C_type_1B,
                )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B,
                )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B,
                )* ;
            copy@dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep,
                )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep,
                )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                    $C_dep1_4B_dep, $C_dep2_4B_dep,
                )* ;
            copy@ptr:
                $( $C_doc_1B_ptr, $C_name_1B_ptr, $C_type_1B_ptr,
                    $C_ptr_1B_ptr,
                )*
                $( $C_doc_2B_ptr, $C_name_2B_ptr, $C_type_2B_ptr,
                    $C_ptr_2B_ptr,
                )*
                $( $C_doc_4B_ptr, $C_name_4B_ptr, $C_type_4B_ptr,
                    $C_ptr_4B_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_1B_ptrdep, $C_name_1B_ptrdep, $C_type_1B_ptrdep,
                    $C_ptrdep_1B_ptrdep,
                )*
                $( $C_doc_2B_ptrdep, $C_name_2B_ptrdep, $C_type_2B_ptrdep,
                    $C_ptrdep_2B_ptrdep,
                )*
                $( $C_doc_4B_ptrdep, $C_name_4B_ptrdep, $C_type_4B_ptrdep,
                    $C_ptrdep_4B_ptrdep,
                )* ;

            noncopy:
                $( $N_doc_1B, $N_name_1B, $N_type_1B,
                )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B,
                )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B,
                )* ;
            noncopy@dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep,
                )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep,
                )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                    $N_dep1_4B_dep, $N_dep2_4B_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_1B_ptr, $N_name_1B_ptr, $N_type_1B_ptr,
                    $N_ptr_1B_ptr, $N_dep1_1B_ptr, $N_dep2_1B_ptr,
                )*
                $( $N_doc_2B_ptr, $N_name_2B_ptr, $N_type_2B_ptr,
                    $N_ptr_2B_ptr, $N_dep1_2B_ptr, $N_dep2_2B_ptr,
                )*
                $( $N_doc_4B_ptr, $N_name_4B_ptr, $N_type_4B_ptr,
                    $N_ptr_4B_ptr, $N_dep1_4B_ptr, $N_dep2_4B_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_1B_ptrdep, $N_name_1B_ptrdep, $N_type_1B_ptrdep,
                    $N_ptr_1B_ptrdep, $N_dep1_1B_ptrdep, $N_dep2_1B_ptrdep,
                )*
                $( $N_doc_2B_ptrdep, $N_name_2B_ptrdep, $N_type_2B_ptrdep,
                    $N_ptr_2B_ptrdep, $N_dep1_2B_ptrdep, $N_dep2_2B_ptrdep,
                )*
                $( $N_doc_4B_ptrdep, $N_name_4B_ptrdep, $N_type_4B_ptrdep,
                    $N_ptr_4B_ptrdep, $N_dep1_4B_ptrdep, $N_dep2_4B_ptrdep,
                )* ;
        }
        // 8-Byte / 64-bit
        #[cfg(feature = "_value64")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 8, 64, feature: "_value64",

            copy:
                $( $C_doc_1B, $C_name_1B, $C_type_1B,
                )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B,
                )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B,
                )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B,
                )* ;
            copy@dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep,
                )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep,
                )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                    $C_dep1_4B_dep, $C_dep2_4B_dep,
                )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                    $C_dep1_8B_dep, $C_dep2_8B_dep,
                )* ;
            copy@ptr:
                $( $C_doc_1B_ptr, $C_name_1B_ptr, $C_type_1B_ptr,
                    $C_ptr_1B_ptr,
                )*
                $( $C_doc_2B_ptr, $C_name_2B_ptr, $C_type_2B_ptr,
                    $C_ptr_2B_ptr,
                )*
                $( $C_doc_4B_ptr, $C_name_4B_ptr, $C_type_4B_ptr,
                    $C_ptr_4B_ptr,
                )*
                $( $C_doc_8B_ptr, $C_name_8B_ptr, $C_type_8B_ptr,
                    $C_ptr_8B_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_1B_ptrdep, $C_name_1B_ptrdep, $C_type_1B_ptrdep,
                    $C_ptrdep_1B_ptrdep,
                )*
                $( $C_doc_2B_ptrdep, $C_name_2B_ptrdep, $C_type_2B_ptrdep,
                    $C_ptrdep_2B_ptrdep,
                )*
                $( $C_doc_4B_ptrdep, $C_name_4B_ptrdep, $C_type_4B_ptrdep,
                    $C_ptrdep_4B_ptrdep,
                )*
                $( $C_doc_8B_ptrdep, $C_name_8B_ptrdep, $C_type_8B_ptrdep,
                    $C_ptrdep_8B_ptrdep, ),* ;
            noncopy:
                $( $N_doc_1B, $N_name_1B, $N_type_1B,
                )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B,
                )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B,
                )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B,
                )* ;

            noncopy@dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep,
                )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep,
                )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                    $N_dep1_4B_dep, $N_dep2_4B_dep,
                )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                    $N_dep1_8B_dep, $N_dep2_8B_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_1B_ptr, $N_name_1B_ptr, $N_type_1B_ptr,
                    $N_ptr_1B_ptr, $N_dep1_1B_ptr, $N_dep2_1B_ptr,
                )*
                $( $N_doc_2B_ptr, $N_name_2B_ptr, $N_type_2B_ptr,
                    $N_ptr_2B_ptr, $N_dep1_2B_ptr, $N_dep2_2B_ptr,
                )*
                $( $N_doc_4B_ptr, $N_name_4B_ptr, $N_type_4B_ptr,
                    $N_ptr_4B_ptr, $N_dep1_4B_ptr, $N_dep2_4B_ptr,
                )*
                $( $N_doc_8B_ptr, $N_name_8B_ptr, $N_type_8B_ptr,
                    $N_ptr_8B_ptr, $N_dep1_8B_ptr, $N_dep2_8B_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_1B_ptrdep, $N_name_1B_ptrdep, $N_type_1B_ptrdep,
                    $N_ptr_1B_ptrdep, $N_dep1_1B_ptrdep, $N_dep2_1B_ptrdep,
                )*
                $( $N_doc_2B_ptrdep, $N_name_2B_ptrdep, $N_type_2B_ptrdep,
                    $N_ptr_2B_ptrdep, $N_dep1_2B_ptrdep, $N_dep2_2B_ptrdep,
                )*
                $( $N_doc_4B_ptrdep, $N_name_4B_ptrdep, $N_type_4B_ptrdep,
                    $N_ptr_4B_ptrdep, $N_dep1_4B_ptrdep, $N_dep2_4B_ptrdep,
                )*
                $( $N_doc_8B_ptrdep, $N_name_8B_ptrdep, $N_type_8B_ptrdep,
                    $N_ptr_8B_ptrdep, $N_dep1_8B_ptrdep, $N_dep2_8B_ptrdep,
                )* ;
        }
        // 16-Byte / 128-bit
        #[cfg(feature = "_value128")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 16, 128, feature: "_value128",

            copy:
                $( $C_doc_1B, $C_name_1B, $C_type_1B,
                )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B,
                )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B,
                )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B,
                )*
                $( $C_doc_16B, $C_name_16B, $C_type_16B,
                )* ;
            copy@dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep,
                )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep,
                )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                    $C_dep1_4B_dep, $C_dep2_4B_dep,
                )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                    $C_dep1_8B_dep, $C_dep2_8B_dep,
                )*
                $( $C_doc_16B_dep, $C_name_16B_dep, $C_type_16B_dep,
                    $C_dep1_16B_dep, $C_dep2_16B_dep,
                )* ;
            copy@ptr:
                $( $C_doc_1B_ptr, $C_name_1B_ptr, $C_type_1B_ptr,
                    $C_ptr_1B_ptr,
                )*
                $( $C_doc_2B_ptr, $C_name_2B_ptr, $C_type_2B_ptr,
                    $C_ptr_2B_ptr,
                )*
                $( $C_doc_4B_ptr, $C_name_4B_ptr, $C_type_4B_ptr,
                    $C_ptr_4B_ptr,
                )*
                $( $C_doc_8B_ptr, $C_name_8B_ptr, $C_type_8B_ptr,
                    $C_ptr_8B_ptr,
                )*
                $( $C_doc_16B_ptr, $C_name_16B_ptr, $C_type_16B_ptr,
                    $C_ptr_16B_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_1B_ptrdep, $C_name_1B_ptrdep, $C_type_1B_ptrdep,
                    $C_ptrdep_1B_ptrdep,
                )*
                $( $C_doc_2B_ptrdep, $C_name_2B_ptrdep, $C_type_2B_ptrdep,
                    $C_ptrdep_2B_ptrdep,
                )*
                $( $C_doc_4B_ptrdep, $C_name_4B_ptrdep, $C_type_4B_ptrdep,
                    $C_ptrdep_4B_ptrdep,
                )*
                $( $C_doc_8B_ptrdep, $C_name_8B_ptrdep, $C_type_8B_ptrdep,
                    $C_ptrdep_8B_ptrdep,
                )*
                $( $C_doc_16B_ptrdep, $C_name_16B_ptrdep, $C_type_16B_ptrdep,
                    $C_ptrdep_16B_ptrdep,
                )* ;

            noncopy:
                $( $N_doc_1B, $N_name_1B, $N_type_1B,
                )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B,
                )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B,
                )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B,
                )*
                $( $N_doc_16B, $N_name_16B, $N_type_16B,
                )* ;
            noncopy@dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep,
                )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep,
                )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                    $N_dep1_4B_dep, $N_dep2_4B_dep,
                )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                    $N_dep1_8B_dep, $N_dep2_8B_dep,
                )*
                $( $N_doc_16B_dep, $N_name_16B_dep, $N_type_16B_dep,
                    $N_dep1_16B_dep, $N_dep2_16B_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_1B_ptr, $N_name_1B_ptr, $N_type_1B_ptr,
                    $N_ptr_1B_ptr, $N_dep1_1B_ptr, $N_dep2_1B_ptr,
                )*
                $( $N_doc_2B_ptr, $N_name_2B_ptr, $N_type_2B_ptr,
                    $N_ptr_2B_ptr, $N_dep1_2B_ptr, $N_dep2_2B_ptr,
                )*
                $( $N_doc_4B_ptr, $N_name_4B_ptr, $N_type_4B_ptr,
                    $N_ptr_4B_ptr, $N_dep1_4B_ptr, $N_dep2_4B_ptr,
                )*
                $( $N_doc_8B_ptr, $N_name_8B_ptr, $N_type_8B_ptr,
                    $N_ptr_8B_ptr, $N_dep1_8B_ptr, $N_dep2_8B_ptr,
                )*
                $( $N_doc_16B_ptr, $N_name_16B_ptr, $N_type_16B_ptr,
                    $N_ptr_16B_ptr, $N_dep1_16B_ptr, $N_dep2_16B_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_1B_ptrdep, $N_name_1B_ptrdep, $N_type_1B_ptrdep,
                    $N_ptr_1B_ptrdep, $N_dep1_1B_ptrdep, $N_dep2_1B_ptrdep,
                )*
                $( $N_doc_2B_ptrdep, $N_name_2B_ptrdep, $N_type_2B_ptrdep,
                    $N_ptr_2B_ptrdep, $N_dep1_2B_ptrdep, $N_dep2_2B_ptrdep,
                )*
                $( $N_doc_4B_ptrdep, $N_name_4B_ptrdep, $N_type_4B_ptrdep,
                    $N_ptr_4B_ptrdep, $N_dep1_4B_ptrdep, $N_dep2_4B_ptrdep,
                )*
                $( $N_doc_8B_ptrdep, $N_name_8B_ptrdep, $N_type_8B_ptrdep,
                    $N_ptr_8B_ptrdep, $N_dep1_8B_ptrdep, $N_dep2_8B_ptrdep,
                )*
                $( $N_doc_16B_ptrdep, $N_name_16B_ptrdep, $N_type_16B_ptrdep,
                    $N_ptr_16B_ptrdep, $N_dep1_16B_ptrdep, $N_dep2_16B_ptrdep,
                )* ;
        }
        // 32-Byte / 256-bit
        #[cfg(feature = "_value256")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 32, 256, feature: "_value256",

            copy:
                $( $C_doc_1B, $C_name_1B, $C_type_1B,
                )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B,
                )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B,
                )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B,
                )*
                $( $C_doc_16B, $C_name_16B, $C_type_16B,
                )*
                $( $C_doc_32B, $C_name_32B, $C_type_32B,
                )* ;
            copy@dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                   $C_dep1_1B_dep, $C_dep2_1B_dep,
                )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                   $C_dep1_2B_dep, $C_dep2_2B_dep,
                )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                   $C_dep1_4B_dep, $C_dep2_4B_dep,
                )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                   $C_dep1_8B_dep, $C_dep2_8B_dep,
                )*
                $( $C_doc_16B_dep, $C_name_16B_dep, $C_type_16B_dep,
                   $C_dep1_16B_dep, $C_dep2_16B_dep,
                )*
                $( $C_doc_32B_dep, $C_name_32B_dep, $C_type_32B_dep,
                   $C_dep1_32B_dep, $C_dep2_32B_dep,
                )* ;
            copy@ptr:
                $( $C_doc_1B_ptr, $C_name_1B_ptr, $C_type_1B_ptr,
                   $C_ptr_1B_ptr,
                )*
                $( $C_doc_2B_ptr, $C_name_2B_ptr, $C_type_2B_ptr,
                   $C_ptr_2B_ptr,
                )*
                $( $C_doc_4B_ptr, $C_name_4B_ptr, $C_type_4B_ptr,
                   $C_ptr_4B_ptr,
                )*
                $( $C_doc_8B_ptr, $C_name_8B_ptr, $C_type_8B_ptr,
                   $C_ptr_8B_ptr,
                )*
                $( $C_doc_16B_ptr, $C_name_16B_ptr, $C_type_16B_ptr,
                   $C_ptr_16B_ptr,
                )*
                $( $C_doc_32B_ptr, $C_name_32B_ptr, $C_type_32B_ptr,
                   $C_ptr_32B_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_1B_ptrdep, $C_name_1B_ptrdep, $C_type_1B_ptrdep,
                   $C_ptrdep_1B_ptrdep,
                )*
                $( $C_doc_2B_ptrdep, $C_name_2B_ptrdep, $C_type_2B_ptrdep,
                   $C_ptrdep_2B_ptrdep,
                )*
                $( $C_doc_4B_ptrdep, $C_name_4B_ptrdep, $C_type_4B_ptrdep,
                   $C_ptrdep_4B_ptrdep,
                )*
                $( $C_doc_8B_ptrdep, $C_name_8B_ptrdep, $C_type_8B_ptrdep,
                   $C_ptrdep_8B_ptrdep,
                )*
                $( $C_doc_16B_ptrdep, $C_name_16B_ptrdep, $C_type_16B_ptrdep,
                   $C_ptrdep_16B_ptrdep,
                )*
                $( $C_doc_32B_ptrdep, $C_name_32B_ptrdep, $C_type_32B_ptrdep,
                   $C_ptrdep_32B_ptrdep,
                )* ;

            noncopy:
                $( $N_doc_1B, $N_name_1B, $N_type_1B,
                )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B,
                )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B,
                )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B,
                )*
                $( $N_doc_16B, $N_name_16B, $N_type_16B,
                )*
                $( $N_doc_32B, $N_name_32B, $N_type_32B,
                )* ;
            noncopy@dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                   $N_dep1_1B_dep, $N_dep2_1B_dep,
                )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                   $N_dep1_2B_dep, $N_dep2_2B_dep,
                )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                   $N_dep1_4B_dep, $N_dep2_4B_dep,
                )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                   $N_dep1_8B_dep, $N_dep2_8B_dep,
                )*
                $( $N_doc_16B_dep, $N_name_16B_dep, $N_type_16B_dep,
                   $N_dep1_16B_dep, $N_dep2_16B_dep,
                )*
                $( $N_doc_32B_dep, $N_name_32B_dep, $N_type_32B_dep,
                   $N_dep1_32B_dep, $N_dep2_32B_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_1B_ptr, $N_name_1B_ptr, $N_type_1B_ptr,
                   $N_ptr_1B_ptr, $N_dep1_1B_ptr, $N_dep2_1B_ptr,
                )*
                $( $N_doc_2B_ptr, $N_name_2B_ptr, $N_type_2B_ptr,
                   $N_ptr_2B_ptr, $N_dep1_2B_ptr, $N_dep2_2B_ptr,
                )*
                $( $N_doc_4B_ptr, $N_name_4B_ptr, $N_type_4B_ptr,
                   $N_ptr_4B_ptr, $N_dep1_4B_ptr, $N_dep2_4B_ptr,
                )*
                $( $N_doc_8B_ptr, $N_name_8B_ptr, $N_type_8B_ptr,
                   $N_ptr_8B_ptr, $N_dep1_8B_ptr, $N_dep2_8B_ptr,
                )*
                $( $N_doc_16B_ptr, $N_name_16B_ptr, $N_type_16B_ptr,
                   $N_ptr_16B_ptr, $N_dep1_16B_ptr, $N_dep2_16B_ptr,
                )*
                $( $N_doc_32B_ptr, $N_name_32B_ptr, $N_type_32B_ptr,
                   $N_ptr_32B_ptr, $N_dep1_32B_ptr, $N_dep2_32B_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_1B_ptrdep, $N_name_1B_ptrdep, $N_type_1B_ptrdep,
                  $N_ptr_1B_ptrdep, $N_dep1_1B_ptrdep, $N_dep2_1B_ptrdep,
                )*
                $( $N_doc_2B_ptrdep, $N_name_2B_ptrdep, $N_type_2B_ptrdep,
                  $N_ptr_2B_ptrdep, $N_dep1_2B_ptrdep, $N_dep2_2B_ptrdep,
                )*
                $( $N_doc_4B_ptrdep, $N_name_4B_ptrdep, $N_type_4B_ptrdep,
                  $N_ptr_4B_ptrdep, $N_dep1_4B_ptrdep, $N_dep2_4B_ptrdep,
                )*
                $( $N_doc_8B_ptrdep, $N_name_8B_ptrdep, $N_type_8B_ptrdep,
                  $N_ptr_8B_ptrdep, $N_dep1_8B_ptrdep, $N_dep2_8B_ptrdep,
                )*
                $( $N_doc_16B_ptrdep, $N_name_16B_ptrdep, $N_type_16B_ptrdep,
                  $N_ptr_16B_ptrdep, $N_dep1_16B_ptrdep, $N_dep2_16B_ptrdep,
                )*
                $( $N_doc_32B_ptrdep, $N_name_32B_ptrdep, $N_type_32B_ptrdep,
                   $N_ptr_32B_ptrdep, $N_dep1_32B_ptrdep, $N_dep2_32B_ptrdep,
                )* ;
        }
        // 64-Byte / 512-bit
        #[cfg(feature = "_value512")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 64, 512, feature: "_value512",

            copy:
                $( $C_doc_1B, $C_name_1B, $C_type_1B,
                )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B,
                )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B,
                )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B,
                )*
                $( $C_doc_16B, $C_name_16B, $C_type_16B,
                )*
                $( $C_doc_32B, $C_name_32B, $C_type_32B,
                )*
                $( $C_doc_64B, $C_name_64B, $C_type_64B,
                )* ;
            copy@dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                   $C_dep1_1B_dep, $C_dep2_1B_dep,
                )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                   $C_dep1_2B_dep, $C_dep2_2B_dep,
                )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                   $C_dep1_4B_dep, $C_dep2_4B_dep,
                )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                   $C_dep1_8B_dep, $C_dep2_8B_dep,
                )*
                $( $C_doc_16B_dep, $C_name_16B_dep, $C_type_16B_dep,
                   $C_dep1_16B_dep, $C_dep2_16B_dep,
                )*
                $( $C_doc_32B_dep, $C_name_32B_dep, $C_type_32B_dep,
                   $C_dep1_32B_dep, $C_dep2_32B_dep,
                )*
                $( $C_doc_64B_dep, $C_name_64B_dep, $C_type_64B_dep,
                   $C_dep1_64B_dep, $C_dep2_64B_dep,
                )* ;
            copy@ptr:
                $( $C_doc_1B_ptr, $C_name_1B_ptr, $C_type_1B_ptr,
                   $C_ptr_1B_ptr,
                )*
                $( $C_doc_2B_ptr, $C_name_2B_ptr, $C_type_2B_ptr,
                   $C_ptr_2B_ptr,
                )*
                $( $C_doc_4B_ptr, $C_name_4B_ptr, $C_type_4B_ptr,
                   $C_ptr_4B_ptr,
                )*
                $( $C_doc_8B_ptr, $C_name_8B_ptr, $C_type_8B_ptr,
                   $C_ptr_8B_ptr,
                )*
                $( $C_doc_16B_ptr, $C_name_16B_ptr, $C_type_16B_ptr,
                   $C_ptr_16B_ptr,
                )*
                $( $C_doc_32B_ptr, $C_name_32B_ptr, $C_type_32B_ptr,
                   $C_ptr_32B_ptr,
                )*
                $( $C_doc_64B_ptr, $C_name_64B_ptr, $C_type_64B_ptr,
                   $C_ptr_64B_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_1B_ptrdep, $C_name_1B_ptrdep, $C_type_1B_ptrdep,
                   $C_ptrdep_1B_ptrdep,
                )*
                $( $C_doc_2B_ptrdep, $C_name_2B_ptrdep, $C_type_2B_ptrdep,
                   $C_ptrdep_2B_ptrdep,
                )*
                $( $C_doc_4B_ptrdep, $C_name_4B_ptrdep, $C_type_4B_ptrdep,
                   $C_ptrdep_4B_ptrdep,
                )*
                $( $C_doc_8B_ptrdep, $C_name_8B_ptrdep, $C_type_8B_ptrdep,
                   $C_ptrdep_8B_ptrdep,
                )*
                $( $C_doc_16B_ptrdep, $C_name_16B_ptrdep, $C_type_16B_ptrdep,
                   $C_ptrdep_16B_ptrdep,
                )*
                $( $C_doc_32B_ptrdep, $C_name_32B_ptrdep, $C_type_32B_ptrdep,
                   $C_ptrdep_32B_ptrdep,
                )*
                $( $C_doc_64B_ptrdep, $C_name_64B_ptrdep, $C_type_64B_ptrdep,
                   $C_ptrdep_64B_ptrdep,
                )* ;

            noncopy:
                $( $N_doc_1B, $N_name_1B, $N_type_1B,
                )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B,
                )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B,
                )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B,
                )*
                $( $N_doc_16B, $N_name_16B, $N_type_16B,
                )*
                $( $N_doc_32B, $N_name_32B, $N_type_32B,
                )*
                $( $N_doc_64B, $N_name_64B, $N_type_64B,
                )* ;
            noncopy@dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                   $N_dep1_1B_dep, $N_dep2_1B_dep,
                )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                   $N_dep1_2B_dep, $N_dep2_2B_dep,
                )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                   $N_dep1_4B_dep, $N_dep2_4B_dep,
                )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                   $N_dep1_8B_dep, $N_dep2_8B_dep,
                )*
                $( $N_doc_16B_dep, $N_name_16B_dep, $N_type_16B_dep,
                   $N_dep1_16B_dep, $N_dep2_16B_dep,
                )*
                $( $N_doc_32B_dep, $N_name_32B_dep, $N_type_32B_dep,
                   $N_dep1_32B_dep, $N_dep2_32B_dep,
                )*
                $( $N_doc_64B_dep, $N_name_64B_dep, $N_type_64B_dep,
                   $N_dep1_64B_dep, $N_dep2_64B_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_1B_ptr, $N_name_1B_ptr, $N_type_1B_ptr,
                   $N_ptr_1B_ptr, $N_dep1_1B_ptr, $N_dep2_1B_ptr,
                )*
                $( $N_doc_2B_ptr, $N_name_2B_ptr, $N_type_2B_ptr,
                   $N_ptr_2B_ptr, $N_dep1_2B_ptr, $N_dep2_2B_ptr,
                )*
                $( $N_doc_4B_ptr, $N_name_4B_ptr, $N_type_4B_ptr,
                   $N_ptr_4B_ptr, $N_dep1_4B_ptr, $N_dep2_4B_ptr,
                )*
                $( $N_doc_8B_ptr, $N_name_8B_ptr, $N_type_8B_ptr,
                   $N_ptr_8B_ptr, $N_dep1_8B_ptr, $N_dep2_8B_ptr,
                )*
                $( $N_doc_16B_ptr, $N_name_16B_ptr, $N_type_16B_ptr,
                   $N_ptr_16B_ptr, $N_dep1_16B_ptr, $N_dep2_16B_ptr,
                )*
                $( $N_doc_32B_ptr, $N_name_32B_ptr, $N_type_32B_ptr,
                   $N_ptr_32B_ptr, $N_dep1_32B_ptr, $N_dep2_32B_ptr,
                )*
                $( $N_doc_64B_ptr, $N_name_64B_ptr, $N_type_64B_ptr,
                   $N_ptr_64B_ptr, $N_dep1_64B_ptr, $N_dep2_64B_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_1B_ptrdep, $N_name_1B_ptrdep, $N_type_1B_ptrdep,
                   $N_ptr_1B_ptrdep, $N_dep1_1B_ptrdep, $N_dep2_1B_ptrdep,
                )*
                $( $N_doc_2B_ptrdep, $N_name_2B_ptrdep, $N_type_2B_ptrdep,
                   $N_ptr_2B_ptrdep, $N_dep1_2B_ptrdep, $N_dep2_2B_ptrdep,
                )*
                $( $N_doc_4B_ptrdep, $N_name_4B_ptrdep, $N_type_4B_ptrdep,
                   $N_ptr_4B_ptrdep, $N_dep1_4B_ptrdep, $N_dep2_4B_ptrdep,
                )*
                $( $N_doc_8B_ptrdep, $N_name_8B_ptrdep, $N_type_8B_ptrdep,
                   $N_ptr_8B_ptrdep, $N_dep1_8B_ptrdep, $N_dep2_8B_ptrdep,
                )*
                $( $N_doc_16B_ptrdep, $N_name_16B_ptrdep, $N_type_16B_ptrdep,
                   $N_ptr_16B_ptrdep, $N_dep1_16B_ptrdep, $N_dep2_16B_ptrdep,
                )*
                $( $N_doc_32B_ptrdep, $N_name_32B_ptrdep, $N_type_32B_ptrdep,
                   $N_ptr_32B_ptrdep, $N_dep1_32B_ptrdep, $N_dep2_32B_ptrdep,
                )*
                $( $N_doc_64B_ptrdep, $N_name_64B_ptrdep, $N_type_64B_ptrdep,
                   $N_ptr_64B_ptrdep, $N_dep1_64B_ptrdep, $N_dep2_64B_ptrdep,
                )* ;
        }
        // 128-Byte / 1024-bit
        #[cfg(feature = "_value1024")]
        $crate::define_data_value_type_raw! {
            single_size: $Value, $Type, $Raw, size: 128, 1024, feature: "_value1024",

            copy:
                $( $C_doc_1B, $C_name_1B, $C_type_1B,
                )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B,
                )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B,
                )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B,
                )*
                $( $C_doc_16B, $C_name_16B, $C_type_16B,
                )*
                $( $C_doc_32B, $C_name_32B, $C_type_32B,
                )*
                $( $C_doc_64B, $C_name_64B, $C_type_64B,
                )*
                $( $C_doc_128B, $C_name_128B, $C_type_128B,
                )* ;
            copy@dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                   $C_dep1_1B_dep, $C_dep2_1B_dep,
                )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                   $C_dep1_2B_dep, $C_dep2_2B_dep,
                )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                   $C_dep1_4B_dep, $C_dep2_4B_dep,
                )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                   $C_dep1_8B_dep, $C_dep2_8B_dep,
                )*
                $( $C_doc_16B_dep, $C_name_16B_dep, $C_type_16B_dep,
                   $C_dep1_16B_dep, $C_dep2_16B_dep,
                )*
                $( $C_doc_32B_dep, $C_name_32B_dep, $C_type_32B_dep,
                   $C_dep1_32B_dep, $C_dep2_32B_dep,
                )*
                $( $C_doc_64B_dep, $C_name_64B_dep, $C_type_64B_dep,
                   $C_dep1_64B_dep, $C_dep2_64B_dep,
                )*
                $( $C_doc_128B_dep, $C_name_128B_dep, $C_type_128B_dep,
                   $C_dep1_128B_dep, $C_dep2_128B_dep,
                )* ;
            copy@ptr:
                $( $C_doc_1B_ptr, $C_name_1B_ptr, $C_type_1B_ptr,
                   $C_ptr_1B_ptr,
                )*
                $( $C_doc_2B_ptr, $C_name_2B_ptr, $C_type_2B_ptr,
                   $C_ptr_2B_ptr,
                )*
                $( $C_doc_4B_ptr, $C_name_4B_ptr, $C_type_4B_ptr,
                   $C_ptr_4B_ptr,
                )*
                $( $C_doc_8B_ptr, $C_name_8B_ptr, $C_type_8B_ptr,
                   $C_ptr_8B_ptr,
                )*
                $( $C_doc_16B_ptr, $C_name_16B_ptr, $C_type_16B_ptr,
                   $C_ptr_16B_ptr,
                )*
                $( $C_doc_32B_ptr, $C_name_32B_ptr, $C_type_32B_ptr,
                   $C_ptr_32B_ptr,
                )*
                $( $C_doc_64B_ptr, $C_name_64B_ptr, $C_type_64B_ptr,
                   $C_ptr_64B_ptr,
                )*
                $( $C_doc_128B_ptr, $C_name_128B_ptr, $C_type_128B_ptr,
                   $C_ptr_128B_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_1B_ptrdep, $C_name_1B_ptrdep, $C_type_1B_ptrdep,
                   $C_ptrdep_1B_ptrdep,
                )*
                $( $C_doc_2B_ptrdep, $C_name_2B_ptrdep, $C_type_2B_ptrdep,
                   $C_ptrdep_2B_ptrdep,
                )*
                $( $C_doc_4B_ptrdep, $C_name_4B_ptrdep, $C_type_4B_ptrdep,
                   $C_ptrdep_4B_ptrdep,
                )*
                $( $C_doc_8B_ptrdep, $C_name_8B_ptrdep, $C_type_8B_ptrdep,
                   $C_ptrdep_8B_ptrdep,
                )*
                $( $C_doc_16B_ptrdep, $C_name_16B_ptrdep, $C_type_16B_ptrdep,
                   $C_ptrdep_16B_ptrdep,
                )*
                $( $C_doc_32B_ptrdep, $C_name_32B_ptrdep, $C_type_32B_ptrdep,
                   $C_ptrdep_32B_ptrdep,
                )*
                $( $C_doc_64B_ptrdep, $C_name_64B_ptrdep, $C_type_64B_ptrdep,
                   $C_ptrdep_64B_ptrdep,
                )*
                $( $C_doc_128B_ptrdep, $C_name_128B_ptrdep, $C_type_128B_ptrdep,
                   $C_ptrdep_128B_ptrdep,
                )* ;

            noncopy:
                $( $N_doc_1B, $N_name_1B, $N_type_1B,
                )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B,
                )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B,
                )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B,
                )*
                $( $N_doc_16B, $N_name_16B, $N_type_16B,
                )*
                $( $N_doc_32B, $N_name_32B, $N_type_32B,
                )*
                $( $N_doc_64B, $N_name_64B, $N_type_64B,
                )*
                $( $N_doc_128B, $N_name_128B, $N_type_128B,
                )* ;
            noncopy@dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                   $N_dep1_1B_dep, $N_dep2_1B_dep,
                )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                   $N_dep1_2B_dep, $N_dep2_2B_dep,
                )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                   $N_dep1_4B_dep, $N_dep2_4B_dep,
                )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                   $N_dep1_8B_dep, $N_dep2_8B_dep,
                )*
                $( $N_doc_16B_dep, $N_name_16B_dep, $N_type_16B_dep,
                   $N_dep1_16B_dep, $N_dep2_16B_dep,
                )*
                $( $N_doc_32B_dep, $N_name_32B_dep, $N_type_32B_dep,
                   $N_dep1_32B_dep, $N_dep2_32B_dep,
                )*
                $( $N_doc_64B_dep, $N_name_64B_dep, $N_type_64B_dep,
                   $N_dep1_64B_dep, $N_dep2_64B_dep,
                )*
                $( $N_doc_128B_dep, $N_name_128B_dep, $N_type_128B_dep,
                   $N_dep1_128B_dep, $N_dep2_128B_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_1B_ptr, $N_name_1B_ptr, $N_type_1B_ptr,
                   $N_ptr_1B_ptr, $N_dep1_1B_ptr, $N_dep2_1B_ptr,
                )*
                $( $N_doc_2B_ptr, $N_name_2B_ptr, $N_type_2B_ptr,
                   $N_ptr_2B_ptr, $N_dep1_2B_ptr, $N_dep2_2B_ptr,
                )*
                $( $N_doc_4B_ptr, $N_name_4B_ptr, $N_type_4B_ptr,
                   $N_ptr_4B_ptr, $N_dep1_4B_ptr, $N_dep2_4B_ptr,
                )*
                $( $N_doc_8B_ptr, $N_name_8B_ptr, $N_type_8B_ptr,
                   $N_ptr_8B_ptr, $N_dep1_8B_ptr, $N_dep2_8B_ptr,
                )*
                $( $N_doc_16B_ptr, $N_name_16B_ptr, $N_type_16B_ptr,
                   $N_ptr_16B_ptr, $N_dep1_16B_ptr, $N_dep2_16B_ptr,
                )*
                $( $N_doc_32B_ptr, $N_name_32B_ptr, $N_type_32B_ptr,
                   $N_ptr_32B_ptr, $N_dep1_32B_ptr, $N_dep2_32B_ptr,
                )*
                $( $N_doc_64B_ptr, $N_name_64B_ptr, $N_type_64B_ptr,
                   $N_ptr_64B_ptr, $N_dep1_64B_ptr, $N_dep2_64B_ptr,
                )*
                $( $N_doc_128B_ptr, $N_name_128B_ptr, $N_type_128B_ptr,
                   $N_ptr_128B_ptr, $N_dep1_128B_ptr, $N_dep2_128B_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_1B_ptrdep, $N_name_1B_ptrdep, $N_type_1B_ptrdep,
                   $N_ptr_1B_ptrdep, $N_dep1_1B_ptrdep, $N_dep2_1B_ptrdep,
                )*
                $( $N_doc_2B_ptrdep, $N_name_2B_ptrdep, $N_type_2B_ptrdep,
                   $N_ptr_2B_ptrdep, $N_dep1_2B_ptrdep, $N_dep2_2B_ptrdep,
                )*
                $( $N_doc_4B_ptrdep, $N_name_4B_ptrdep, $N_type_4B_ptrdep,
                   $N_ptr_4B_ptrdep, $N_dep1_4B_ptrdep, $N_dep2_4B_ptrdep,
                )*
                $( $N_doc_8B_ptrdep, $N_name_8B_ptrdep, $N_type_8B_ptrdep,
                   $N_ptr_8B_ptrdep, $N_dep1_8B_ptrdep, $N_dep2_8B_ptrdep,
                )*
                $( $N_doc_16B_ptrdep, $N_name_16B_ptrdep, $N_type_16B_ptrdep,
                   $N_ptr_16B_ptrdep, $N_dep1_16B_ptrdep, $N_dep2_16B_ptrdep,
                )*
                $( $N_doc_32B_ptrdep, $N_name_32B_ptrdep, $N_type_32B_ptrdep,
                   $N_ptr_32B_ptrdep, $N_dep1_32B_ptrdep, $N_dep2_32B_ptrdep,
                )*
                $( $N_doc_64B_ptrdep, $N_name_64B_ptrdep, $N_type_64B_ptrdep,
                   $N_ptr_64B_ptrdep, $N_dep1_64B_ptrdep, $N_dep2_64B_ptrdep,
                )*
                $( $N_doc_128B_ptrdep, $N_name_128B_ptrdep, $N_type_128B_ptrdep,
                   $N_ptr_128B_ptrdep, $N_dep1_128B_ptrdep, $N_dep2_128B_ptrdep,
                )* ;
        }
    };
    (
    // -------------------------------------------------------------------------

    // This arm defines in one pass a single size `DataValue*`, `DataType*`, `DataRaw*`.
    //
    // It calls the macros: `define_data_value!`, `define_data_type!` and `define_data_raw!`.
    single_size: $Value:ident, $Type:ident, $Raw:ident,
    size: $B:literal, $b:literal,
    feature: $feature:literal,

    copy:
        $( $C_doc:literal, $C_name:ident, $C_type:ty,
        )* ;
    copy@dep:
        $( $C_doc_dep:literal, $C_name_dep:ident, $C_type_dep:ty,
           $C_dep1_dep:literal, $C_dep2_dep:literal,
        )* ;
    copy@ptr:
        $( $C_doc_ptr:literal, $C_name_ptr:ident, $C_type_ptr:ty,
           $C_ptr_ptr:meta,
        )* ;
    copy@ptrdep:
        $( $C_doc_ptrdep:literal, $C_name_ptrdep:ident, $C_type_ptrdep:ty,
           $C_ptr_ptrdep:meta, $C_dep1_ptrdep:literal, $C_dep2_ptrdep:literal,
        )* ;

    noncopy:
        $( $N_doc:literal, $N_name:ident, $N_type:ty,
        )* ;
    noncopy@dep:
        $( $N_doc_dep:literal, $N_name_dep:ident, $N_type_dep:ty,
           $N_dep1_dep:literal, $N_dep2_dep:literal,
        )* ;
    noncopy@ptr:
        $( $N_doc_ptr:literal, $N_name_ptr:ident, $N_type_ptr:ty,
           $N_ptr_ptr:meta, $N_dep1_ptr:literal, $N_dep2_ptr:literal,
        )* ;
    noncopy@ptrdep:
        $( $N_doc_ptrdep:literal, $N_name_ptrdep:ident, $N_type_ptrdep:ty,
           $N_ptr_ptrdep:meta, $N_dep1_ptrdep:literal, $N_dep2_ptrdep:literal,
        )* ;
    ) => {
        $crate::define_data_value! {
            v: $Value, t: $Type, r: $Raw,
            size: $B, $b, feature: $feature,

            copy:
                $( $C_doc, $C_name, $C_type,
                )* ;
            copy@dep:
                $( $C_doc_dep, $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep,
                )* ;
            copy@ptr:
                $( $C_doc_ptr, $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_ptrdep, $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep, $C_dep1_ptrdep, $C_dep2_ptrdep,
                )* ;

            noncopy:
                $( $N_doc, $N_name, $N_type,
                )* ;
            noncopy@dep:
                $( $N_doc_dep, $N_name_dep, $N_type_dep,
                   $N_dep1_dep, $N_dep2_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_ptr, $N_name_ptr, $N_type_ptr,
                   $N_ptr_ptr, $N_dep1_ptr, $N_dep2_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_ptrdep, $N_name_ptrdep, $N_type_ptrdep,
                   $N_ptr_ptrdep, $N_dep1_ptrdep, $N_dep2_ptrdep,
                )* ;
        }
        $crate::define_data_type! {
            v: $Value, t: $Type, r: $Raw,
            size: $B, $b, feature: $feature,

            copy:
                $( $C_doc, $C_name, $C_type,
                )* ;
            copy@dep:
                $( $C_doc_dep, $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep,
                )* ;
            copy@ptr:
                $( $C_doc_ptr, $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_ptrdep, $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep, $C_dep1_ptrdep, $C_dep2_ptrdep,
                )* ;

            noncopy:
                $( $N_doc, $N_name, $N_type,
                )* ;
            noncopy@dep:
                $( $N_doc_dep, $N_name_dep, $N_type_dep,
                   $N_dep1_dep, $N_dep2_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_ptr, $N_name_ptr, $N_type_ptr,
                   $N_ptr_ptr, $N_dep1_ptr, $N_dep2_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_ptrdep, $N_name_ptrdep, $N_type_ptrdep,
                   $N_ptr_ptrdep, $N_dep1_ptrdep, $N_dep2_ptrdep,
                )* ;
        }
        $crate::define_data_raw! {
            v: $Value, t: $Type, r: $Raw,
            size: $B, $b, feature: $feature,

            copy:
                $( $C_doc, $C_name, $C_type,
                )* ;
            copy@dep:
                $( $C_doc_dep, $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep,
                )* ;
            copy@ptr:
                $( $C_doc_ptr, $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr,
                )* ;
            copy@ptrdep:
                $( $C_doc_ptrdep, $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep, $C_dep1_ptrdep, $C_dep2_ptrdep,
                )* ;

            noncopy:
                $( $N_doc, $N_name, $N_type,
                )* ;
            noncopy@dep:
                $( $N_doc_dep, $N_name_dep, $N_type_dep,
                   $N_dep1_dep, $N_dep2_dep,
                )* ;
            noncopy@ptr:
                $( $N_doc_ptr, $N_name_ptr, $N_type_ptr,
                   $N_ptr_ptr, $N_dep1_ptr, $N_dep2_ptr,
                )* ;
            noncopy@ptrdep:
                $( $N_doc_ptrdep, $N_name_ptrdep, $N_type_ptrdep,
                   $N_ptr_ptrdep, $N_dep1_ptrdep, $N_dep2_ptrdep,
                )* ;
        }
    };
}
pub(crate) use define_data_value_type_raw;
