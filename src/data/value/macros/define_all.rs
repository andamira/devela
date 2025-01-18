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
    // - copy_variants_nB:              (name, type),*
    // - copy_variants_nB_dep:          (doc, name, type, dep1, dep2),*
    // - copy_variants_nB_psize:        (doc, name, type, psize),*
    // - copy_variants_nB_psize_dep:    (doc, name, type, psize, dep1, dep2),*
    // - noncopy_variants_nB:           (doc, name, type),*
    // - noncopy_variants_nB_dep:       (doc, name, type, dep1, dep2),*
    // - noncopy_variants_nB_psize:     (doc, name, type, psize),*
    // - noncopy_variants_nB_psize_dep: (doc, name, type, psize, dep1, dep2),*
    //
    // where:
    // - the `copy` prefix indicates the types are `Copy`,
    //   otherwise the `noncopy` prefix is used.
    // - `nB` indicates the number of bytes of the types in the current group.
    // - the `dep` suffix indicates a dependency on 2 features (dep1 & dep2)
    //   (pass the same dependency twice in order to only depend on one).
    // - the `psize` suffix indicates a dependency on a specific pointer size.
    //
    // The `single_size` arm is called making sure each size contains
    // all variants with a size less than or equal to the current size.

    all_sizes: v: $Vname:ident, t: $Tname:ident, r: $Rname:ident,

    // 1-Byte / 8-bit
    copy_variants_1B: $(
        $C_doc_1B:literal, $C_name_1B:ident, $C_type_1B:ty,
        )*
    copy_variants_1B_dep: $(
        $C_doc_1B_dep:literal, $C_name_1B_dep:ident, $C_type_1B_dep:ty,
        $C_dep1_1B_dep:literal, $C_dep2_1B_dep:literal,
        )*
    copy_variants_1B_psize: $(
        $C_doc_1B_psize:literal, $C_name_1B_psize:ident, $C_type_1B_psize:ty,
        $C_psize_1B_psize:meta,
        )*
    copy_variants_1B_psize_dep: $(
        $C_doc_1B_psize_dep:literal, $C_name_1B_psize_dep:ident, $C_type_1B_psize_dep:ty,
        $C_psize_dep_1B_psize_dep:meta,
        )*
    noncopy_variants_1B: $(
        $N_doc_1B:literal, $N_name_1B:ident, $N_type_1B:ty,
        )*
    noncopy_variants_1B_dep: $(
        $N_doc_1B_dep:literal, $N_name_1B_dep:ident, $N_type_1B_dep:ty,
        $N_dep2_1B_dep:literal,
        )*
    noncopy_variants_1B_psize: $(
        $N_doc_1B_psize:literal, $N_name_1B_psize:ident, $N_type_1B_psize:ty,
        $N_psize_1B_psize:meta,
        )*
    noncopy_variants_1B_psize_dep: $(
        $N_doc_1B_psize_dep:literal, $N_name_1B_psize_dep:ident, $N_type_1B_psize_dep:ty,
        $N_psize_1B_psize_dep:meta, $N_dep1_1B_psize_dep:literal, $N_dep2_1B_psize_dep:literal
        ),*

    // 2-Byte / 16-bit
    copy_variants_2B: $(
        $C_doc_2B:literal, $C_name_2B:ident, $C_type_2B:ty,
        )*
    copy_variants_2B_dep: $(
        $C_doc_2B_dep:literal, $C_name_2B_dep:ident, $C_type_2B_dep:ty,
        $C_dep1_2B_dep:literal, $C_dep2_2B_dep:literal,
        )*
    copy_variants_2B_psize: $(
        $C_doc_2B_psize:literal, $C_name_2B_psize:ident,
        $C_type_2B_psize:ty, $C_psize_2B_psize:meta,
        )*
    copy_variants_2B_psize_dep: $(
        $C_doc_2B_psize_dep:literal, $C_name_2B_psize_dep:ident,
        $C_type_2B_psize_dep:ty, $C_psize_dep_2B_psize_dep:meta,
        )*
    noncopy_variants_2B: $(
        $N_doc_2B:literal, $N_name_2B:ident, $N_type_2B:ty,
        )*
    noncopy_variants_2B_dep: $(
        $N_doc_2B_dep:literal, $N_name_2B_dep:ident, $N_type_2B_dep:ty,
        $N_dep1_2B_dep:literal,
        )*
    noncopy_variants_2B_psize: $(
        $N_doc_2B_psize:literal, $N_name_2B_psize:ident,
        $N_type_2B_psize:ty, $N_psize_2B_psize:meta,
        )*
    noncopy_variants_2B_psize_dep: $(
        $N_doc_2B_psize_dep:literal, $N_name_2B_psize_dep:ident,
        $N_type_2B_psize_dep:ty, $N_psize_2B_psize_dep:meta, $N_dep1_2B_psize_dep:literal,
        $N_dep2_2B_psize_dep:literal,

        )*

    // 4-Byte / 32-bit
    copy_variants_4B: $(
        $C_doc_4B:literal, $C_name_4B:ident, $C_type_4B:ty,
        )*
    copy_variants_4B_dep: $(
        $C_doc_4B_dep:literal, $C_name_4B_dep:ident, $C_type_4B_dep:ty,
        $C_dep1_4B_dep:literal, $C_dep2_4B_dep:literal,
        )*
    copy_variants_4B_psize: $(
        $C_doc_4B_psize:literal, $C_name_4B_psize:ident,
        $C_type_4B_psize:ty, $C_psize_4B_psize:meta,
        )*
    copy_variants_4B_psize_dep: $(
        $C_doc_4B_psize_dep:literal, $C_name_4B_psize_dep:ident,
        $C_type_4B_psize_dep:ty, $C_psize_dep_4B_psize_dep:meta,
        )*
    noncopy_variants_4B: $(
        $N_doc_4B:literal, $N_name_4B:ident, $N_type_4B:ty,
        )*
    noncopy_variants_4B_dep: $(
        $N_doc_4B_dep:literal, $N_name_4B_dep:ident, $N_type_4B_dep:ty,
        $N_dep1_4B_dep:literal, $N_dep2_4B_dep:literal,
        )*
    noncopy_variants_4B_psize: $(
        $N_doc_4B_psize:literal, $N_name_4B_psize:ident,
        $N_type_4B_psize:ty, $N_psize_4B_psize:meta,
        )*
    noncopy_variants_4B_psize_dep: $(
        $N_doc_4B_psize_dep:literal, $N_name_4B_psize_dep:ident,
        $N_type_4B_psize_dep:ty, $N_psize_4B_psize_dep:meta, $N_dep1_4B_psize_dep:literal,
        $N_dep2_4B_psize_dep:literal,
        )*

    // 8-Byte / 64-bit
    copy_variants_8B: $(
        $C_doc_8B:literal, $C_name_8B:ident, $C_type_8B:ty,
        )*
    copy_variants_8B_dep: $(
        $C_doc_8B_dep:literal, $C_name_8B_dep:ident, $C_type_8B_dep:ty,
        $C_dep1_8B_dep:literal, $C_dep2_8B_dep:literal,
        )*
    copy_variants_8B_psize: $(
        $C_doc_8B_psize:literal, $C_name_8B_psize:ident,
        $C_type_8B_psize:ty, $C_psize_8B_psize:meta,
        )*
    copy_variants_8B_psize_dep: $(
        $C_doc_8B_psize_dep:literal, $C_name_8B_psize_dep:ident,
        $C_type_8B_psize_dep:ty, $C_psize_dep_8B_psize_dep:meta,
        )*
    noncopy_variants_8B: $(
        $N_doc_8B:literal, $N_name_8B:ident, $N_type_8B:ty,
        )*
    noncopy_variants_8B_dep: $(
        $N_doc_8B_dep:literal, $N_name_8B_dep:ident, $N_type_8B_dep:ty,
        $N_dep1_8B_dep:literal, $N_dep2_8B_dep:literal,
        )*
    noncopy_variants_8B_psize: $(
        $N_doc_8B_psize:literal, $N_name_8B_psize:ident,
        $N_type_8B_psize:ty, $N_psize_8B_psize:meta,
        )*
    noncopy_variants_8B_psize_dep: $(
        $N_doc_8B_psize_dep:literal, $N_name_8B_psize_dep:ident,
        $N_type_8B_psize_dep:ty, $N_psize_8B_psize_dep:meta, $N_dep1_8B_psize_dep:literal,
        $N_dep2_8B_psize_dep:literal,
        )*

    // 16-Byte / 128-bit
    copy_variants_16B: $(
        $C_doc_16B:literal, $C_name_16B:ident, $C_type_16B:ty,
        )*
    copy_variants_16B_dep: $(
        $C_doc_16B_dep:literal, $C_name_16B_dep:ident, $C_type_16B_dep:ty,
        $C_dep1_16B_dep:literal, $C_dep2_16B_dep:literal,
        )*
    copy_variants_16B_psize: $(
        $C_doc_16B_psize:literal, $C_name_16B_psize:ident,
        $C_type_16B_psize:ty, $C_psize_16B_psize:meta,
        )*
    copy_variants_16B_psize_dep: $(
        $C_doc_16B_psize_dep:literal, $C_name_16B_psize_dep:ident,
        $C_type_16B_psize_dep:ty, $C_psize_dep_16B_psize_dep:meta,
        )*
    noncopy_variants_16B: $(
        $N_doc_16B:literal, $N_name_16B:ident, $N_type_16B:ty,
        )*
    noncopy_variants_16B_dep: $(
        $N_doc_16B_dep:literal, $N_name_16B_dep:ident,
        $N_type_16B_dep:ty, $N_dep1_16B_dep:literal, $N_dep2_16B_dep:literal,
        )*
    noncopy_variants_16B_psize: $(
        $N_doc_16B_psize:literal, $N_name_16B_psize:ident,
        $N_type_16B_psize:ty, $N_psize_16B_psize:meta,
        )*
    noncopy_variants_16B_psize_dep: $(
        $N_doc_16B_psize_dep:literal, $N_name_16B_psize_dep:ident,
        $N_type_16B_psize_dep:ty, $N_psize_16B_psize_dep:meta, $N_dep1_16B_psize_dep:literal,
        $N_dep2_16B_psize_dep:literal,
        )*

    // 32-Byte / 256-bit
    copy_variants_32B: $(
        $C_doc_32B:literal, $C_name_32B:ident, $C_type_32B:ty,
        )*
    copy_variants_32B_dep: $(
        $C_doc_32B_dep:literal, $C_name_32B_dep:ident,
        $C_type_32B_dep:ty, $C_dep1_32B_dep:literal, $C_dep2_32B_dep:literal,
        )*
    copy_variants_32B_psize: $(
        $C_doc_32B_psize:literal, $C_name_32B_psize:ident,
        $C_type_32B_psize:ty, $C_psize_32B_psize:meta,
        )*
    copy_variants_32B_psize_dep: $(
        $C_doc_32B_psize_dep:literal, $C_name_32B_psize_dep:ident,
        $C_type_32B_psize_dep:ty, $C_psize_dep_32B_psize_dep:meta,
        )*
    noncopy_variants_32B: $(
        $N_doc_32B:literal, $N_name_32B:ident, $N_type_32B:ty,
        )*
    noncopy_variants_32B_dep: $(
        $N_doc_32B_dep:literal, $N_name_32B_dep:ident,
        $N_type_32B_dep:ty, $N_dep1_32B_dep:literal, $N_dep2_32B_dep:literal,
        )*
    noncopy_variants_32B_psize: $(
        $N_doc_32B_psize:literal, $N_name_32B_psize:ident,
        $N_type_32B_psize:ty, $N_psize_32B_psize:meta,
        )*
    noncopy_variants_32B_psize_dep: $(
        $N_doc_32B_psize_dep:literal, $N_name_32B_psize_dep:ident,
        $N_type_32B_psize_dep:ty, $N_psize_32B_psize_dep:meta, $N_dep1_32B_psize_dep:literal,
        $N_dep2_32B_psize_dep:literal,
        )*

    // 64-Byte / 512-bit
    copy_variants_64B: $(
        $C_doc_64B:literal, $C_name_64B:ident, $C_type_64B:ty,
        )*
    copy_variants_64B_dep: $(
        $C_doc_64B_dep:literal, $C_name_64B_dep:ident,
        $C_type_64B_dep:ty, $C_dep1_64B_dep:literal, $C_dep2_64B_dep:literal,
        )*
    copy_variants_64B_psize: $(
        $C_doc_64B_psize:literal, $C_name_64B_psize:ident,
        $C_type_64B_psize:ty, $C_psize_64B_psize:meta,
        )*
    copy_variants_64B_psize_dep: $(
        $C_doc_64B_psize_dep:literal, $C_name_64B_psize_dep:ident,
        $C_type_64B_psize_dep:ty, $C_psize_dep_64B_psize_dep:meta,
        )*
    noncopy_variants_64B: $(
        $N_doc_64B:literal, $N_name_64B:ident, $N_type_64B:ty,
        )*
    noncopy_variants_64B_dep: $(
        $N_doc_64B_dep:literal, $N_name_64B_dep:ident,
        $N_type_64B_dep:ty, $N_dep1_64B_dep:literal, $N_dep2_64B_dep:literal,
        )*
    noncopy_variants_64B_psize: $(
        $N_doc_64B_psize:literal, $N_name_64B_psize:ident,
        $N_type_64B_psize:ty, $N_psize_64B_psize:meta,
        )*
    noncopy_variants_64B_psize_dep: $(
        $N_doc_64B_psize_dep:literal, $N_name_64B_psize_dep:ident,
        $N_type_64B_psize_dep:ty, $N_psize_64B_psize_dep:meta, $N_dep1_64B_psize_dep:literal,
        $N_dep2_64B_psize_dep:literal,
        )*

    // 128-Byte / 1024-bit
    copy_variants_128B: $(
        $C_doc_128B:literal, $C_name_128B:ident, $C_type_128B:ty,
        )*
    copy_variants_128B_dep: $(
        $C_doc_128B_dep:literal, $C_name_128B_dep:ident,
        $C_type_128B_dep:ty, $C_dep1_128B_dep:literal, $C_dep2_128B_dep:literal,
        )*
    copy_variants_128B_psize: $(
        $C_doc_128B_psize:literal, $C_name_128B_psize:ident,
        $C_type_128B_psize:ty, $C_psize_128B_psize:meta,
        )*
    copy_variants_128B_psize_dep: $(
        $C_doc_128B_psize_dep:literal,
        $C_name_128B_psize_dep:ident, $C_type_128B_psize_dep:ty,
        $C_psize_dep_128B_psize_dep:meta,
        )*
    noncopy_variants_128B: $(
        $N_doc_128B:literal, $N_name_128B:ident, $N_type_128B:ty,
        )*
    noncopy_variants_128B_dep: $(
        $N_doc_128B_dep:literal, $N_name_128B_dep:ident,
        $N_type_128B_dep:ty, $N_dep1_128B_dep:literal, $N_dep2_128B_dep:literal,
        )*
    noncopy_variants_128B_psize: $(
        $N_doc_128B_psize:literal, $N_name_128B_psize:ident,
        $N_type_128B_psize:ty, $N_psize_128B_psize:meta,
        )*
    noncopy_variants_128B_psize_dep: $(
        $N_doc_128B_psize_dep:literal,
        $N_name_128B_psize_dep:ident, $N_type_128B_psize_dep:ty, $N_psize_128B_psize_dep:meta,
        $N_dep1_128B_psize_dep:literal, $N_dep2_128B_psize_dep:literal,
        )*

    ) => {
        // 1-Byte / 8-bit
        #[cfg(feature = "_value8")]
        $crate::define_data_value_type_raw! {
            single_size: $Vname, $Tname, $Rname, size: 1, 8, feature: "_value8",
            copy_variants:
                $( $C_doc_1B, $C_name_1B, $C_type_1B,
                )* ;
            copy_variants_dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep,
                )* ;
            copy_variants_psize:
                $( $C_doc_1B_psize, $C_name_1B_psize, $C_type_1B_psize,
                    $C_psize_1B_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_1B_psize_dep, $C_name_1B_psize_dep, $C_type_1B_psize_dep,
                    $C_psize_dep_1B_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc_1B, $N_name_1B, $N_type_1B, )* ;
            noncopy_variants_dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_1B_psize, $N_name_1B_psize, $N_type_1B_psize,
                    $N_psize_1B_psize, $N_dep1_1B_psize, $N_dep2_1B_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_1B_psize_dep, $N_name_1B_psize_dep, $N_type_1B_psize_dep,
                    $N_psize_1B_psize_dep, $N_dep1_1B_psize_dep, $N_dep2_1B_psize_dep, )* ;
        }
        // 2-Byte / 16-bit
        #[cfg(feature = "_value16")]
        $crate::define_data_value_type_raw! {
            single_size: $Vname, $Tname, $Rname, size: 2, 16, feature: "_value16",
            copy_variants:
                $( $C_doc_1B, $C_name_1B, $C_type_1B, )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B, )* ;
            copy_variants_dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep,
                )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep,
                )* ;
            copy_variants_psize:
                $( $C_doc_1B_psize, $C_name_1B_psize, $C_type_1B_psize,
                    $C_psize_1B_psize, )*
                $( $C_doc_2B_psize, $C_name_2B_psize, $C_type_2B_psize,
                    $C_psize_2B_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_1B_psize_dep, $C_name_1B_psize_dep, $C_type_1B_psize_dep,
                    $C_psize_dep_1B_psize_dep, )*
                $( $C_doc_2B_psize_dep, $C_name_2B_psize_dep, $C_type_2B_psize_dep,
                    $C_psize_dep_2B_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc_1B, $N_name_1B, $N_type_1B, )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B, )* ;
            noncopy_variants_dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep, )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_1B_psize, $N_name_1B_psize, $N_type_1B_psize,
                    $N_psize_1B_psize, $N_dep1_1B_psize, $N_dep2_1B_psize, )*
                $( $N_doc_2B_psize, $N_name_2B_psize, $N_type_2B_psize,
                    $N_psize_2B_psize, $N_dep1_2B_psize, $N_dep2_2B_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_1B_psize_dep, $N_name_1B_psize_dep, $N_type_1B_psize_dep,
                    $N_psize_1B_psize_dep, $N_dep1_1B_psize_dep, $N_dep2_1B_psize_dep, )*
                $( $N_doc_2B_psize_dep, $N_name_2B_psize_dep, $N_type_2B_psize_dep,
                    $N_psize_2B_psize_dep, $N_dep1_2B_psize_dep, $N_dep2_2B_psize_dep, )* ;
        }
        // 4-Byte / 32-bit
        #[cfg(feature = "_value32")]
        $crate::define_data_value_type_raw! {
            single_size: $Vname, $Tname, $Rname, size: 4, 32, feature: "_value32",
            copy_variants:
                $( $C_doc_1B, $C_name_1B, $C_type_1B, )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B, )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B, )* ;
            copy_variants_dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep, )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep, )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                    $C_dep1_4B_dep, $C_dep2_4B_dep, )* ;
            copy_variants_psize:
                $( $C_doc_1B_psize, $C_name_1B_psize, $C_type_1B_psize,
                    $C_psize_1B_psize, )*
                $( $C_doc_2B_psize, $C_name_2B_psize, $C_type_2B_psize,
                    $C_psize_2B_psize, )*
                $( $C_doc_4B_psize, $C_name_4B_psize, $C_type_4B_psize,
                    $C_psize_4B_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_1B_psize_dep, $C_name_1B_psize_dep, $C_type_1B_psize_dep,
                    $C_psize_dep_1B_psize_dep, )*
                $( $C_doc_2B_psize_dep, $C_name_2B_psize_dep, $C_type_2B_psize_dep,
                    $C_psize_dep_2B_psize_dep, )*
                $( $C_doc_4B_psize_dep, $C_name_4B_psize_dep, $C_type_4B_psize_dep,
                    $C_psize_dep_4B_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc_1B, $N_name_1B, $N_type_1B, )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B, )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B, )* ;
            noncopy_variants_dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep, )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep, )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                    $N_dep1_4B_dep, $N_dep2_4B_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_1B_psize, $N_name_1B_psize, $N_type_1B_psize,
                    $N_psize_1B_psize, $N_dep1_1B_psize, $N_dep2_1B_psize, )*
                $( $N_doc_2B_psize, $N_name_2B_psize, $N_type_2B_psize,
                    $N_psize_2B_psize, $N_dep1_2B_psize, $N_dep2_2B_psize, )*
                $( $N_doc_4B_psize, $N_name_4B_psize, $N_type_4B_psize,
                    $N_psize_4B_psize, $N_dep1_4B_psize, $N_dep2_4B_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_1B_psize_dep, $N_name_1B_psize_dep, $N_type_1B_psize_dep,
                    $N_psize_1B_psize_dep, $N_dep1_1B_psize_dep, $N_dep2_1B_psize_dep, )*
                $( $N_doc_2B_psize_dep, $N_name_2B_psize_dep, $N_type_2B_psize_dep,
                    $N_psize_2B_psize_dep, $N_dep1_2B_psize_dep, $N_dep2_2B_psize_dep, )*
                $( $N_doc_4B_psize_dep, $N_name_4B_psize_dep, $N_type_4B_psize_dep,
                    $N_psize_4B_psize_dep, $N_dep1_4B_psize_dep, $N_dep2_4B_psize_dep, )* ;
        }
        // 8-Byte / 64-bit
        #[cfg(feature = "_value64")]
        $crate::define_data_value_type_raw! {
            single_size: $Vname, $Tname, $Rname, size: 8, 64, feature: "_value64",
            copy_variants:
                $( $C_doc_1B, $C_name_1B, $C_type_1B, )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B, )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B, )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B, )* ;
            copy_variants_dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep, )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep, )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                    $C_dep1_4B_dep, $C_dep2_4B_dep, )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                    $C_dep1_8B_dep, $C_dep2_8B_dep, )* ;
            copy_variants_psize:
                $( $C_doc_1B_psize, $C_name_1B_psize, $C_type_1B_psize,
                    $C_psize_1B_psize, )*
                $( $C_doc_2B_psize, $C_name_2B_psize, $C_type_2B_psize,
                    $C_psize_2B_psize, )*
                $( $C_doc_4B_psize, $C_name_4B_psize, $C_type_4B_psize,
                    $C_psize_4B_psize, )*
                $( $C_doc_8B_psize, $C_name_8B_psize, $C_type_8B_psize,
                    $C_psize_8B_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_1B_psize_dep, $C_name_1B_psize_dep, $C_type_1B_psize_dep,
                    $C_psize_dep_1B_psize_dep, )*
                $( $C_doc_2B_psize_dep, $C_name_2B_psize_dep, $C_type_2B_psize_dep,
                    $C_psize_dep_2B_psize_dep, )*
                $( $C_doc_4B_psize_dep, $C_name_4B_psize_dep, $C_type_4B_psize_dep,
                    $C_psize_dep_4B_psize_dep, )*
                $( $C_doc_8B_psize_dep, $C_name_8B_psize_dep, $C_type_8B_psize_dep,
                    $C_psize_dep_8B_psize_dep, ),* ;
            noncopy_variants:
                $( $N_doc_1B, $N_name_1B, $N_type_1B, )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B, )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B, )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B, )* ;
            noncopy_variants_dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep, )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep, )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                    $N_dep1_4B_dep, $N_dep2_4B_dep, )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                    $N_dep1_8B_dep, $N_dep2_8B_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_1B_psize, $N_name_1B_psize, $N_type_1B_psize,
                    $N_psize_1B_psize, $N_dep1_1B_psize, $N_dep2_1B_psize, )*
                $( $N_doc_2B_psize, $N_name_2B_psize, $N_type_2B_psize,
                    $N_psize_2B_psize, $N_dep1_2B_psize, $N_dep2_2B_psize, )*
                $( $N_doc_4B_psize, $N_name_4B_psize, $N_type_4B_psize,
                    $N_psize_4B_psize, $N_dep1_4B_psize, $N_dep2_4B_psize, )*
                $( $N_doc_8B_psize, $N_name_8B_psize, $N_type_8B_psize,
                    $N_psize_8B_psize, $N_dep1_8B_psize, $N_dep2_8B_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_1B_psize_dep, $N_name_1B_psize_dep, $N_type_1B_psize_dep,
                    $N_psize_1B_psize_dep, $N_dep1_1B_psize_dep, $N_dep2_1B_psize_dep, )*
                $( $N_doc_2B_psize_dep, $N_name_2B_psize_dep, $N_type_2B_psize_dep,
                    $N_psize_2B_psize_dep, $N_dep1_2B_psize_dep, $N_dep2_2B_psize_dep, )*
                $( $N_doc_4B_psize_dep, $N_name_4B_psize_dep, $N_type_4B_psize_dep,
                    $N_psize_4B_psize_dep, $N_dep1_4B_psize_dep, $N_dep2_4B_psize_dep, )*
                $( $N_doc_8B_psize_dep, $N_name_8B_psize_dep, $N_type_8B_psize_dep,
                    $N_psize_8B_psize_dep, $N_dep1_8B_psize_dep, $N_dep2_8B_psize_dep, )* ;
        }
        // 16-Byte / 128-bit
        #[cfg(feature = "_value128")]
        $crate::define_data_value_type_raw! {
            single_size: $Vname, $Tname, $Rname, size: 16, 128, feature: "_value128",
            copy_variants:
                $( $C_doc_1B, $C_name_1B, $C_type_1B, )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B, )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B, )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B, )*
                $( $C_doc_16B, $C_name_16B, $C_type_16B, )* ;
            copy_variants_dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep, )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep, )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                    $C_dep1_4B_dep, $C_dep2_4B_dep, )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                    $C_dep1_8B_dep, $C_dep2_8B_dep, )*
                $( $C_doc_16B_dep, $C_name_16B_dep, $C_type_16B_dep,
                    $C_dep1_16B_dep, $C_dep2_16B_dep, )* ;
            copy_variants_psize:
                $( $C_doc_1B_psize, $C_name_1B_psize, $C_type_1B_psize,
                    $C_psize_1B_psize, )*
                $( $C_doc_2B_psize, $C_name_2B_psize, $C_type_2B_psize,
                    $C_psize_2B_psize, )*
                $( $C_doc_4B_psize, $C_name_4B_psize, $C_type_4B_psize,
                    $C_psize_4B_psize, )*
                $( $C_doc_8B_psize, $C_name_8B_psize, $C_type_8B_psize,
                    $C_psize_8B_psize, )*
                $( $C_doc_16B_psize, $C_name_16B_psize, $C_type_16B_psize,
                    $C_psize_16B_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_1B_psize_dep, $C_name_1B_psize_dep, $C_type_1B_psize_dep,
                    $C_psize_dep_1B_psize_dep, )*
                $( $C_doc_2B_psize_dep, $C_name_2B_psize_dep, $C_type_2B_psize_dep,
                    $C_psize_dep_2B_psize_dep, )*
                $( $C_doc_4B_psize_dep, $C_name_4B_psize_dep, $C_type_4B_psize_dep,
                    $C_psize_dep_4B_psize_dep, )*
                $( $C_doc_8B_psize_dep, $C_name_8B_psize_dep, $C_type_8B_psize_dep,
                    $C_psize_dep_8B_psize_dep, )*
                $( $C_doc_16B_psize_dep, $C_name_16B_psize_dep, $C_type_16B_psize_dep,
                    $C_psize_dep_16B_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc_1B, $N_name_1B, $N_type_1B, )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B, )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B, )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B, )*
                $( $N_doc_16B, $N_name_16B, $N_type_16B, )* ;
            noncopy_variants_dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep, )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep, )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                    $N_dep1_4B_dep, $N_dep2_4B_dep, )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                    $N_dep1_8B_dep, $N_dep2_8B_dep, )*
                $( $N_doc_16B_dep, $N_name_16B_dep, $N_type_16B_dep,
                    $N_dep1_16B_dep, $N_dep2_16B_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_1B_psize, $N_name_1B_psize, $N_type_1B_psize,
                    $N_psize_1B_psize, $N_dep1_1B_psize, $N_dep2_1B_psize, )*
                $( $N_doc_2B_psize, $N_name_2B_psize, $N_type_2B_psize,
                    $N_psize_2B_psize, $N_dep1_2B_psize, $N_dep2_2B_psize, )*
                $( $N_doc_4B_psize, $N_name_4B_psize, $N_type_4B_psize,
                    $N_psize_4B_psize, $N_dep1_4B_psize, $N_dep2_4B_psize, )*
                $( $N_doc_8B_psize, $N_name_8B_psize, $N_type_8B_psize,
                    $N_psize_8B_psize, $N_dep1_8B_psize, $N_dep2_8B_psize, )*
                $( $N_doc_16B_psize, $N_name_16B_psize, $N_type_16B_psize,
                    $N_psize_16B_psize, $N_dep1_16B_psize, $N_dep2_16B_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_1B_psize_dep, $N_name_1B_psize_dep, $N_type_1B_psize_dep,
                    $N_psize_1B_psize_dep, $N_dep1_1B_psize_dep, $N_dep2_1B_psize_dep, )*
                $( $N_doc_2B_psize_dep, $N_name_2B_psize_dep, $N_type_2B_psize_dep,
                    $N_psize_2B_psize_dep, $N_dep1_2B_psize_dep, $N_dep2_2B_psize_dep, )*
                $( $N_doc_4B_psize_dep, $N_name_4B_psize_dep, $N_type_4B_psize_dep,
                    $N_psize_4B_psize_dep, $N_dep1_4B_psize_dep, $N_dep2_4B_psize_dep, )*
                $( $N_doc_8B_psize_dep, $N_name_8B_psize_dep, $N_type_8B_psize_dep,
                    $N_psize_8B_psize_dep, $N_dep1_8B_psize_dep, $N_dep2_8B_psize_dep, )*
                $( $N_doc_16B_psize_dep, $N_name_16B_psize_dep, $N_type_16B_psize_dep,
                    $N_psize_16B_psize_dep, $N_dep1_16B_psize_dep, $N_dep2_16B_psize_dep, )* ;
        }
        // 32-Byte / 256-bit
        #[cfg(feature = "_value256")]
        $crate::define_data_value_type_raw! {
            single_size: $Vname, $Tname, $Rname, size: 32, 256, feature: "_value256",
            copy_variants:
                $( $C_doc_1B, $C_name_1B, $C_type_1B, )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B, )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B, )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B, )*
                $( $C_doc_16B, $C_name_16B, $C_type_16B, )*
                $( $C_doc_32B, $C_name_32B, $C_type_32B, )* ;
            copy_variants_dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep,
                    $C_dep2_1B_dep, )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep,
                    $C_dep2_2B_dep, )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                    $C_dep1_4B_dep,
                    $C_dep2_4B_dep, )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                    $C_dep1_8B_dep,
                    $C_dep2_8B_dep, )*
                $( $C_doc_16B_dep, $C_name_16B_dep, $C_type_16B_dep,
                    $C_dep1_16B_dep,
                    $C_dep2_16B_dep, )*
                $( $C_doc_32B_dep, $C_name_32B_dep, $C_type_32B_dep,
                    $C_dep1_32B_dep,
                    $C_dep2_32B_dep, )* ;
            copy_variants_psize:
                $( $C_doc_1B_psize, $C_name_1B_psize, $C_type_1B_psize,
                    $C_psize_1B_psize, )*
                $( $C_doc_2B_psize, $C_name_2B_psize, $C_type_2B_psize,
                    $C_psize_2B_psize, )*
                $( $C_doc_4B_psize, $C_name_4B_psize, $C_type_4B_psize,
                    $C_psize_4B_psize, )*
                $( $C_doc_8B_psize, $C_name_8B_psize, $C_type_8B_psize,
                    $C_psize_8B_psize, )*
                $( $C_doc_16B_psize, $C_name_16B_psize, $C_type_16B_psize,
                    $C_psize_16B_psize, )*
                $( $C_doc_32B_psize, $C_name_32B_psize, $C_type_32B_psize,
                    $C_psize_32B_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_1B_psize_dep, $C_name_1B_psize_dep, $C_type_1B_psize_dep,
                    $C_psize_dep_1B_psize_dep, )*
                $( $C_doc_2B_psize_dep, $C_name_2B_psize_dep, $C_type_2B_psize_dep,
                    $C_psize_dep_2B_psize_dep, )*
                $( $C_doc_4B_psize_dep, $C_name_4B_psize_dep, $C_type_4B_psize_dep,
                    $C_psize_dep_4B_psize_dep, )*
                $( $C_doc_8B_psize_dep, $C_name_8B_psize_dep, $C_type_8B_psize_dep,
                    $C_psize_dep_8B_psize_dep, )*
                $( $C_doc_16B_psize_dep, $C_name_16B_psize_dep, $C_type_16B_psize_dep,
                    $C_psize_dep_16B_psize_dep, )*
                $( $C_doc_32B_psize_dep, $C_name_32B_psize_dep, $C_type_32B_psize_dep,
                    $C_psize_dep_32B_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc_1B, $N_name_1B, $N_type_1B, )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B, )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B, )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B, )*
                $( $N_doc_16B, $N_name_16B, $N_type_16B, )*
                $( $N_doc_32B, $N_name_32B, $N_type_32B, )* ;
            noncopy_variants_dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep, )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep, )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                    $N_dep1_4B_dep, $N_dep2_4B_dep, )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                    $N_dep1_8B_dep, $N_dep2_8B_dep, )*
                $( $N_doc_16B_dep, $N_name_16B_dep, $N_type_16B_dep,
                    $N_dep1_16B_dep, $N_dep2_16B_dep, )*
                $( $N_doc_32B_dep, $N_name_32B_dep, $N_type_32B_dep,
                    $N_dep1_32B_dep, $N_dep2_32B_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_1B_psize, $N_name_1B_psize, $N_type_1B_psize,
                    $N_psize_1B_psize, $N_dep1_1B_psize, $N_dep2_1B_psize, )*
                $( $N_doc_2B_psize, $N_name_2B_psize, $N_type_2B_psize,
                    $N_psize_2B_psize, $N_dep1_2B_psize, $N_dep2_2B_psize, )*
                $( $N_doc_4B_psize, $N_name_4B_psize, $N_type_4B_psize,
                    $N_psize_4B_psize, $N_dep1_4B_psize, $N_dep2_4B_psize, )*
                $( $N_doc_8B_psize, $N_name_8B_psize, $N_type_8B_psize,
                    $N_psize_8B_psize, $N_dep1_8B_psize, $N_dep2_8B_psize, )*
                $( $N_doc_16B_psize, $N_name_16B_psize, $N_type_16B_psize,
                    $N_psize_16B_psize, $N_dep1_16B_psize, $N_dep2_16B_psize, )*
                $( $N_doc_32B_psize, $N_name_32B_psize, $N_type_32B_psize,
                    $N_psize_32B_psize, $N_dep1_32B_psize, $N_dep2_32B_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_1B_psize_dep, $N_name_1B_psize_dep, $N_type_1B_psize_dep,
                    $N_psize_1B_psize_dep, $N_dep1_1B_psize_dep, $N_dep2_1B_psize_dep, )*
                $( $N_doc_2B_psize_dep, $N_name_2B_psize_dep, $N_type_2B_psize_dep,
                    $N_psize_2B_psize_dep, $N_dep1_2B_psize_dep, $N_dep2_2B_psize_dep, )*
                $( $N_doc_4B_psize_dep, $N_name_4B_psize_dep, $N_type_4B_psize_dep,
                    $N_psize_4B_psize_dep, $N_dep1_4B_psize_dep, $N_dep2_4B_psize_dep, )*
                $( $N_doc_8B_psize_dep, $N_name_8B_psize_dep, $N_type_8B_psize_dep,
                    $N_psize_8B_psize_dep, $N_dep1_8B_psize_dep, $N_dep2_8B_psize_dep, )*
                $( $N_doc_16B_psize_dep, $N_name_16B_psize_dep, $N_type_16B_psize_dep,
                    $N_psize_16B_psize_dep, $N_dep1_16B_psize_dep, $N_dep2_16B_psize_dep, )*
                $( $N_doc_32B_psize_dep, $N_name_32B_psize_dep, $N_type_32B_psize_dep,
                    $N_psize_32B_psize_dep, $N_dep1_32B_psize_dep, $N_dep2_32B_psize_dep, )* ;
        }
        // 64-Byte / 512-bit
        #[cfg(feature = "_value512")]
        $crate::define_data_value_type_raw! {
            single_size: $Vname, $Tname, $Rname, size: 64, 512, feature: "_value512",
            copy_variants:
                $( $C_doc_1B, $C_name_1B, $C_type_1B, )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B, )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B, )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B, )*
                $( $C_doc_16B, $C_name_16B, $C_type_16B, )*
                $( $C_doc_32B, $C_name_32B, $C_type_32B, )*
                $( $C_doc_64B, $C_name_64B, $C_type_64B, )* ;
            copy_variants_dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep, )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep, )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                    $C_dep1_4B_dep, $C_dep2_4B_dep, )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                    $C_dep1_8B_dep, $C_dep2_8B_dep, )*
                $( $C_doc_16B_dep, $C_name_16B_dep, $C_type_16B_dep,
                    $C_dep1_16B_dep, $C_dep2_16B_dep, )*
                $( $C_doc_32B_dep, $C_name_32B_dep, $C_type_32B_dep,
                    $C_dep1_32B_dep, $C_dep2_32B_dep, )*
                $( $C_doc_64B_dep, $C_name_64B_dep, $C_type_64B_dep,
                    $C_dep1_64B_dep, $C_dep2_64B_dep, )* ;
            copy_variants_psize:
                $( $C_doc_1B_psize, $C_name_1B_psize, $C_type_1B_psize,
                    $C_psize_1B_psize, )*
                $( $C_doc_2B_psize, $C_name_2B_psize, $C_type_2B_psize,
                    $C_psize_2B_psize, )*
                $( $C_doc_4B_psize, $C_name_4B_psize, $C_type_4B_psize,
                    $C_psize_4B_psize, )*
                $( $C_doc_8B_psize, $C_name_8B_psize, $C_type_8B_psize,
                    $C_psize_8B_psize, )*
                $( $C_doc_16B_psize, $C_name_16B_psize, $C_type_16B_psize,
                    $C_psize_16B_psize, )*
                $( $C_doc_32B_psize, $C_name_32B_psize, $C_type_32B_psize,
                    $C_psize_32B_psize, )*
                $( $C_doc_64B_psize, $C_name_64B_psize, $C_type_64B_psize,
                    $C_psize_64B_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_1B_psize_dep, $C_name_1B_psize_dep, $C_type_1B_psize_dep,
                    $C_psize_dep_1B_psize_dep, )*
                $( $C_doc_2B_psize_dep, $C_name_2B_psize_dep, $C_type_2B_psize_dep,
                    $C_psize_dep_2B_psize_dep, )*
                $( $C_doc_4B_psize_dep, $C_name_4B_psize_dep, $C_type_4B_psize_dep,
                    $C_psize_dep_4B_psize_dep, )*
                $( $C_doc_8B_psize_dep, $C_name_8B_psize_dep, $C_type_8B_psize_dep,
                    $C_psize_dep_8B_psize_dep, )*
                $( $C_doc_16B_psize_dep, $C_name_16B_psize_dep, $C_type_16B_psize_dep,
                    $C_psize_dep_16B_psize_dep, )*
                $( $C_doc_32B_psize_dep, $C_name_32B_psize_dep, $C_type_32B_psize_dep,
                    $C_psize_dep_32B_psize_dep, )*
                $( $C_doc_64B_psize_dep, $C_name_64B_psize_dep, $C_type_64B_psize_dep,
                    $C_psize_dep_64B_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc_1B, $N_name_1B, $N_type_1B, )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B, )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B, )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B, )*
                $( $N_doc_16B, $N_name_16B, $N_type_16B, )*
                $( $N_doc_32B, $N_name_32B, $N_type_32B, )*
                $( $N_doc_64B, $N_name_64B, $N_type_64B, )* ;
            noncopy_variants_dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep, )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep, )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                    $N_dep1_4B_dep, $N_dep2_4B_dep, )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                    $N_dep1_8B_dep, $N_dep2_8B_dep, )*
                $( $N_doc_16B_dep, $N_name_16B_dep, $N_type_16B_dep,
                    $N_dep1_16B_dep, $N_dep2_16B_dep, )*
                $( $N_doc_32B_dep, $N_name_32B_dep, $N_type_32B_dep,
                    $N_dep1_32B_dep, $N_dep2_32B_dep, )*
                $( $N_doc_64B_dep, $N_name_64B_dep, $N_type_64B_dep,
                    $N_dep1_64B_dep, $N_dep2_64B_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_1B_psize, $N_name_1B_psize, $N_type_1B_psize,
                    $N_psize_1B_psize, $N_dep1_1B_psize, $N_dep2_1B_psize, )*
                $( $N_doc_2B_psize, $N_name_2B_psize, $N_type_2B_psize,
                    $N_psize_2B_psize, $N_dep1_2B_psize, $N_dep2_2B_psize, )*
                $( $N_doc_4B_psize, $N_name_4B_psize, $N_type_4B_psize,
                    $N_psize_4B_psize, $N_dep1_4B_psize, $N_dep2_4B_psize, )*
                $( $N_doc_8B_psize, $N_name_8B_psize, $N_type_8B_psize,
                    $N_psize_8B_psize, $N_dep1_8B_psize, $N_dep2_8B_psize, )*
                $( $N_doc_16B_psize, $N_name_16B_psize, $N_type_16B_psize,
                    $N_psize_16B_psize, $N_dep1_16B_psize, $N_dep2_16B_psize, )*
                $( $N_doc_32B_psize, $N_name_32B_psize, $N_type_32B_psize,
                    $N_psize_32B_psize, $N_dep1_32B_psize, $N_dep2_32B_psize, )*
                $( $N_doc_64B_psize, $N_name_64B_psize, $N_type_64B_psize,
                    $N_psize_64B_psize, $N_dep1_64B_psize, $N_dep2_64B_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_1B_psize_dep, $N_name_1B_psize_dep, $N_type_1B_psize_dep,
                    $N_psize_1B_psize_dep, $N_dep1_1B_psize_dep, $N_dep2_1B_psize_dep, )*
                $( $N_doc_2B_psize_dep, $N_name_2B_psize_dep, $N_type_2B_psize_dep,
                    $N_psize_2B_psize_dep, $N_dep1_2B_psize_dep, $N_dep2_2B_psize_dep, )*
                $( $N_doc_4B_psize_dep, $N_name_4B_psize_dep, $N_type_4B_psize_dep,
                    $N_psize_4B_psize_dep, $N_dep1_4B_psize_dep, $N_dep2_4B_psize_dep, )*
                $( $N_doc_8B_psize_dep, $N_name_8B_psize_dep, $N_type_8B_psize_dep,
                    $N_psize_8B_psize_dep, $N_dep1_8B_psize_dep, $N_dep2_8B_psize_dep, )*
                $( $N_doc_16B_psize_dep, $N_name_16B_psize_dep, $N_type_16B_psize_dep,
                    $N_psize_16B_psize_dep, $N_dep1_16B_psize_dep, $N_dep2_16B_psize_dep, )*
                $( $N_doc_32B_psize_dep, $N_name_32B_psize_dep, $N_type_32B_psize_dep,
                    $N_psize_32B_psize_dep, $N_dep1_32B_psize_dep, $N_dep2_32B_psize_dep, )*
                $( $N_doc_64B_psize_dep, $N_name_64B_psize_dep, $N_type_64B_psize_dep,
                    $N_psize_64B_psize_dep, $N_dep1_64B_psize_dep, $N_dep2_64B_psize_dep, )* ;
        }
        // 128-Byte / 1024-bit
        #[cfg(feature = "_value1024")]
        $crate::define_data_value_type_raw! {
            single_size: $Vname, $Tname, $Rname, size: 128, 1024, feature: "_value1024",
            copy_variants:
                $( $C_doc_1B, $C_name_1B, $C_type_1B, )*
                $( $C_doc_2B, $C_name_2B, $C_type_2B, )*
                $( $C_doc_4B, $C_name_4B, $C_type_4B, )*
                $( $C_doc_8B, $C_name_8B, $C_type_8B, )*
                $( $C_doc_16B, $C_name_16B, $C_type_16B, )*
                $( $C_doc_32B, $C_name_32B, $C_type_32B, )*
                $( $C_doc_64B, $C_name_64B, $C_type_64B, )*
                $( $C_doc_128B, $C_name_128B, $C_type_128B, )* ;
            copy_variants_dep:
                $( $C_doc_1B_dep, $C_name_1B_dep, $C_type_1B_dep,
                    $C_dep1_1B_dep, $C_dep2_1B_dep, )*
                $( $C_doc_2B_dep, $C_name_2B_dep, $C_type_2B_dep,
                    $C_dep1_2B_dep, $C_dep2_2B_dep, )*
                $( $C_doc_4B_dep, $C_name_4B_dep, $C_type_4B_dep,
                    $C_dep1_4B_dep, $C_dep2_4B_dep, )*
                $( $C_doc_8B_dep, $C_name_8B_dep, $C_type_8B_dep,
                    $C_dep1_8B_dep, $C_dep2_8B_dep, )*
                $( $C_doc_16B_dep, $C_name_16B_dep, $C_type_16B_dep,
                    $C_dep1_16B_dep, $C_dep2_16B_dep, )*
                $( $C_doc_32B_dep, $C_name_32B_dep, $C_type_32B_dep,
                    $C_dep1_32B_dep, $C_dep2_32B_dep, )*
                $( $C_doc_64B_dep, $C_name_64B_dep, $C_type_64B_dep,
                    $C_dep1_64B_dep, $C_dep2_64B_dep, )*
                $( $C_doc_128B_dep, $C_name_128B_dep, $C_type_128B_dep,
                    $C_dep1_128B_dep, $C_dep2_128B_dep, )* ;
            copy_variants_psize:
                $( $C_doc_1B_psize, $C_name_1B_psize, $C_type_1B_psize,
                    $C_psize_1B_psize, )*
                $( $C_doc_2B_psize, $C_name_2B_psize, $C_type_2B_psize,
                    $C_psize_2B_psize, )*
                $( $C_doc_4B_psize, $C_name_4B_psize, $C_type_4B_psize,
                    $C_psize_4B_psize, )*
                $( $C_doc_8B_psize, $C_name_8B_psize, $C_type_8B_psize,
                    $C_psize_8B_psize, )*
                $( $C_doc_16B_psize, $C_name_16B_psize, $C_type_16B_psize,
                    $C_psize_16B_psize, )*
                $( $C_doc_32B_psize, $C_name_32B_psize, $C_type_32B_psize,
                    $C_psize_32B_psize, )*
                $( $C_doc_64B_psize, $C_name_64B_psize, $C_type_64B_psize,
                    $C_psize_64B_psize, )*
                $( $C_doc_128B_psize, $C_name_128B_psize, $C_type_128B_psize,
                    $C_psize_128B_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_1B_psize_dep, $C_name_1B_psize_dep, $C_type_1B_psize_dep,
                    $C_psize_dep_1B_psize_dep, )*
                $( $C_doc_2B_psize_dep, $C_name_2B_psize_dep, $C_type_2B_psize_dep,
                    $C_psize_dep_2B_psize_dep, )*
                $( $C_doc_4B_psize_dep, $C_name_4B_psize_dep, $C_type_4B_psize_dep,
                    $C_psize_dep_4B_psize_dep, )*
                $( $C_doc_8B_psize_dep, $C_name_8B_psize_dep, $C_type_8B_psize_dep,
                    $C_psize_dep_8B_psize_dep, )*
                $( $C_doc_16B_psize_dep, $C_name_16B_psize_dep, $C_type_16B_psize_dep,
                    $C_psize_dep_16B_psize_dep, )*
                $( $C_doc_32B_psize_dep, $C_name_32B_psize_dep, $C_type_32B_psize_dep,
                    $C_psize_dep_32B_psize_dep, )*
                $( $C_doc_64B_psize_dep, $C_name_64B_psize_dep, $C_type_64B_psize_dep,
                    $C_psize_dep_64B_psize_dep, )*
                $( $C_doc_128B_psize_dep, $C_name_128B_psize_dep, $C_type_128B_psize_dep,
                    $C_psize_dep_128B_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc_1B, $N_name_1B, $N_type_1B, )*
                $( $N_doc_2B, $N_name_2B, $N_type_2B, )*
                $( $N_doc_4B, $N_name_4B, $N_type_4B, )*
                $( $N_doc_8B, $N_name_8B, $N_type_8B, )*
                $( $N_doc_16B, $N_name_16B, $N_type_16B, )*
                $( $N_doc_32B, $N_name_32B, $N_type_32B, )*
                $( $N_doc_64B, $N_name_64B, $N_type_64B, )*
                $( $N_doc_128B, $N_name_128B, $N_type_128B, )* ;
            noncopy_variants_dep:
                $( $N_doc_1B_dep, $N_name_1B_dep, $N_type_1B_dep,
                    $N_dep1_1B_dep, $N_dep2_1B_dep, )*
                $( $N_doc_2B_dep, $N_name_2B_dep, $N_type_2B_dep,
                    $N_dep1_2B_dep, $N_dep2_2B_dep, )*
                $( $N_doc_4B_dep, $N_name_4B_dep, $N_type_4B_dep,
                    $N_dep1_4B_dep, $N_dep2_4B_dep, )*
                $( $N_doc_8B_dep, $N_name_8B_dep, $N_type_8B_dep,
                    $N_dep1_8B_dep, $N_dep2_8B_dep, )*
                $( $N_doc_16B_dep, $N_name_16B_dep, $N_type_16B_dep,
                    $N_dep1_16B_dep, $N_dep2_16B_dep, )*
                $( $N_doc_32B_dep, $N_name_32B_dep, $N_type_32B_dep,
                    $N_dep1_32B_dep, $N_dep2_32B_dep, )*
                $( $N_doc_64B_dep, $N_name_64B_dep, $N_type_64B_dep,
                    $N_dep1_64B_dep, $N_dep2_64B_dep, )*
                $( $N_doc_128B_dep, $N_name_128B_dep, $N_type_128B_dep,
                    $N_dep1_128B_dep, $N_dep2_128B_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_1B_psize, $N_name_1B_psize, $N_type_1B_psize,
                    $N_psize_1B_psize, $N_dep1_1B_psize, $N_dep2_1B_psize, )*
                $( $N_doc_2B_psize, $N_name_2B_psize, $N_type_2B_psize,
                    $N_psize_2B_psize, $N_dep1_2B_psize, $N_dep2_2B_psize, )*
                $( $N_doc_4B_psize, $N_name_4B_psize, $N_type_4B_psize,
                    $N_psize_4B_psize, $N_dep1_4B_psize, $N_dep2_4B_psize, )*
                $( $N_doc_8B_psize, $N_name_8B_psize, $N_type_8B_psize,
                    $N_psize_8B_psize, $N_dep1_8B_psize, $N_dep2_8B_psize, )*
                $( $N_doc_16B_psize, $N_name_16B_psize, $N_type_16B_psize,
                    $N_psize_16B_psize, $N_dep1_16B_psize, $N_dep2_16B_psize, )*
                $( $N_doc_32B_psize, $N_name_32B_psize, $N_type_32B_psize,
                    $N_psize_32B_psize, $N_dep1_32B_psize, $N_dep2_32B_psize, )*
                $( $N_doc_64B_psize, $N_name_64B_psize, $N_type_64B_psize,
                    $N_psize_64B_psize, $N_dep1_64B_psize, $N_dep2_64B_psize, )*
                $( $N_doc_128B_psize, $N_name_128B_psize, $N_type_128B_psize,
                    $N_psize_128B_psize, $N_dep1_128B_psize, $N_dep2_128B_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_1B_psize_dep, $N_name_1B_psize_dep, $N_type_1B_psize_dep,
                    $N_psize_1B_psize_dep, $N_dep1_1B_psize_dep, $N_dep2_1B_psize_dep, )*
                $( $N_doc_2B_psize_dep, $N_name_2B_psize_dep, $N_type_2B_psize_dep,
                    $N_psize_2B_psize_dep, $N_dep1_2B_psize_dep, $N_dep2_2B_psize_dep, )*
                $( $N_doc_4B_psize_dep, $N_name_4B_psize_dep, $N_type_4B_psize_dep,
                    $N_psize_4B_psize_dep, $N_dep1_4B_psize_dep, $N_dep2_4B_psize_dep, )*
                $( $N_doc_8B_psize_dep, $N_name_8B_psize_dep, $N_type_8B_psize_dep,
                    $N_psize_8B_psize_dep, $N_dep1_8B_psize_dep, $N_dep2_8B_psize_dep, )*
                $( $N_doc_16B_psize_dep, $N_name_16B_psize_dep, $N_type_16B_psize_dep,
                    $N_psize_16B_psize_dep, $N_dep1_16B_psize_dep, $N_dep2_16B_psize_dep, )*
                $( $N_doc_32B_psize_dep, $N_name_32B_psize_dep, $N_type_32B_psize_dep,
                    $N_psize_32B_psize_dep, $N_dep1_32B_psize_dep, $N_dep2_32B_psize_dep, )*
                $( $N_doc_64B_psize_dep, $N_name_64B_psize_dep, $N_type_64B_psize_dep,
                    $N_psize_64B_psize_dep, $N_dep1_64B_psize_dep, $N_dep2_64B_psize_dep, )*
                $( $N_doc_128B_psize_dep, $N_name_128B_psize_dep, $N_type_128B_psize_dep,
                    $N_psize_128B_psize_dep, $N_dep1_128B_psize_dep, $N_dep2_128B_psize_dep, )* ;
        }
    };
    (
    // -------------------------------------------------------------------------

    // This arm defines in one pass a single size `DataValue*`, `DataType*`, `DataRaw*`.
    //
    // It calls the macros: `define_data_value!`, `define_data_type!` and `define_data_raw!`.
    single_size: $Vname:ident, $Tname:ident, $Rname:ident,
    size: $B:literal, $b:literal,
    feature: $feature:literal,

    copy_variants:
        $( $C_doc:literal, $C_name:ident, $C_type:ty, )* ;
    copy_variants_dep:
        $( $C_doc_dep:literal, $C_name_dep:ident, $C_type_dep:ty,
        $C_dep1_dep:literal, $C_dep2_dep:literal, )* ;
    copy_variants_psize:
        $( $C_doc_psize:literal, $C_name_psize:ident, $C_type_psize:ty,
        $C_psize_psize:meta, )* ;
    copy_variants_psize_dep:
        $( $C_doc_psize_dep:literal, $C_name_psize_dep:ident, $C_type_psize_dep:ty,
        $C_psize_psize_dep:meta, $C_dep1_psize_dep:literal, $C_dep2_psize_dep:literal, )* ;
    noncopy_variants:
        $( $N_doc:literal, $N_name:ident, $N_type:ty, )* ;
    noncopy_variants_dep:
        $( $N_doc_dep:literal, $N_name_dep:ident, $N_type_dep:ty,
        $N_dep1_dep:literal, $N_dep2_dep:literal, )* ;
    noncopy_variants_psize:
        $( $N_doc_psize:literal, $N_name_psize:ident, $N_type_psize:ty,
        $N_psize_psize:meta, $N_dep1_psize:literal, $N_dep2_psize:literal, )* ;
    noncopy_variants_psize_dep:
        $( $N_doc_psize_dep:literal, $N_name_psize_dep:ident, $N_type_psize_dep:ty,
        $N_psize_psize_dep:meta, $N_dep1_psize_dep:literal, $N_dep2_psize_dep:literal, )* ;
    ) => {
        $crate::define_data_value! {
            v:$Vname, t:$Tname, r:$Rname, size: $B, $b, feature: $feature,
            copy_variants:
                $( $C_doc, $C_name, $C_type, )* ;
            copy_variants_dep:
                $( $C_doc_dep, $C_name_dep, $C_type_dep,
                    $C_dep1_dep, $C_dep2_dep, )* ;
            copy_variants_psize:
                $( $C_doc_psize, $C_name_psize, $C_type_psize,
                    $C_psize_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_psize_dep, $C_name_psize_dep, $C_type_psize_dep,
                    $C_psize_psize_dep, $C_dep1_psize_dep, $C_dep2_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc, $N_name, $N_type, )* ;
            noncopy_variants_dep:
                $( $N_doc_dep, $N_name_dep, $N_type_dep,
                    $N_dep1_dep, $N_dep2_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_psize, $N_name_psize, $N_type_psize,
                    $N_psize_psize, $N_dep1_psize, $N_dep2_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_psize_dep, $N_name_psize_dep, $N_type_psize_dep,
                    $N_psize_psize_dep, $N_dep1_psize_dep, $N_dep2_psize_dep, )* ;
        }
        $crate::define_data_type! {
            v:$Vname, t:$Tname, r:$Rname, size: $B, $b, feature: $feature,
            copy_variants:
                $( $C_doc, $C_name, $C_type, )* ;
            copy_variants_dep:
                $( $C_doc_dep, $C_name_dep, $C_type_dep,
                    $C_dep1_dep, $C_dep2_dep, )* ;
            copy_variants_psize:
                $( $C_doc_psize, $C_name_psize, $C_type_psize,
                    $C_psize_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_psize_dep, $C_name_psize_dep, $C_type_psize_dep,
                    $C_psize_psize_dep, $C_dep1_psize_dep, $C_dep2_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc, $N_name, $N_type, )* ;
            noncopy_variants_dep:
                $( $N_doc_dep, $N_name_dep, $N_type_dep,
                    $N_dep1_dep, $N_dep2_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_psize, $N_name_psize, $N_type_psize,
                    $N_psize_psize, $N_dep1_psize, $N_dep2_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_psize_dep, $N_name_psize_dep, $N_type_psize_dep,
                    $N_psize_psize_dep, $N_dep1_psize_dep, $N_dep2_psize_dep, )* ;
        }
        $crate::define_data_raw! {
            v:$Vname, t:$Tname, r:$Rname, size: $B, $b, feature: $feature,
            copy_variants:
                $( $C_doc, $C_name, $C_type, )* ;
            copy_variants_dep:
                $( $C_doc_dep, $C_name_dep, $C_type_dep,
                    $C_dep1_dep, $C_dep2_dep, )* ;
            copy_variants_psize:
                $( $C_doc_psize, $C_name_psize, $C_type_psize,
                    $C_psize_psize, )* ;
            copy_variants_psize_dep:
                $( $C_doc_psize_dep, $C_name_psize_dep, $C_type_psize_dep,
                    $C_psize_psize_dep, $C_dep1_psize_dep, $C_dep2_psize_dep, )* ;
            noncopy_variants:
                $( $N_doc, $N_name, $N_type, )* ;
            noncopy_variants_dep:
                $( $N_doc_dep, $N_name_dep, $N_type_dep,
                    $N_dep1_dep, $N_dep2_dep, )* ;
            noncopy_variants_psize:
                $( $N_doc_psize, $N_name_psize, $N_type_psize,
                    $N_psize_psize, $N_dep1_psize, $N_dep2_psize, )* ;
            noncopy_variants_psize_dep:
                $( $N_doc_psize_dep, $N_name_psize_dep, $N_type_psize_dep,
                    $N_psize_psize_dep, $N_dep1_psize_dep, $N_dep2_psize_dep, )* ;
        }
    };
}
pub(crate) use define_data_value_type_raw;
