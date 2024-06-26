// devela construct::features
//
//! Features debugging and compile flags enabling for reflexion.
//

pub(crate) fn main() -> Result<(), std::io::Error> {
    reflection::set_flags();

    Ok(())
}

/// Sets configuration options for reflection, based on enabled features.
//
// https://doc.rust-lang.org/reference/conditional-compilation.html#set-configuration-options
#[rustfmt::skip]
mod reflection {
    use std::env::var;

    /// (flags, features)
    type FlagsFeatures<'a> = (&'a [&'a str], &'a [&'a str]);

    //* misc. *//

    pub const MISC: FlagsFeatures = (&[], &[
        "_default", "_max",
        "_min_docs", "_max_docs",
        "_docsrs", "_docsrs_stable",
    ]);

    pub const ENVIRONMENT: FlagsFeatures = (&[], &[
        "std", "alloc", "no_std",
    ]);

    pub const SAFETY: FlagsFeatures = (&[], &[
        "safe",
        "safe_code",
        "safe_data", "safe_data_collections",
        "safe_exec",
        "safe_mem",
        "safe_num",
        "safe_rend", "safe_rend_audio", "safe_rend_image",
        "safe_sys",
        "safe_text",
        "safe_time",
        "safe_ui", "safe_ui_term",

        "unsafe",
        "unsafe_array", "unsafe_async", "unsafe_const", "unsafe_dyn",
        "unsafe_niche", "unsafe_ptr", "unsafe_slice", "unsafe_str",
        "unsafe_thread",
    ]);

    pub const NIGHTLY: FlagsFeatures = (&["_some_nightly"], &[
        "nightly_coro", "nightly_doc", "nightly_simd", "nightly_stabilized",
    ]);

    pub const DEPENDENCY: FlagsFeatures = (&["_some_dep"], &[
        "atomic",
        "const-str",
        "crossterm",
        "hashbrown",
        "libm",
        "log",
        "memchr",
        "miniquad",
        "portable-atomic",
        "rand_core",
        "unicode-segmentation",
        "unicode-width",
        "wide",
    ]);

    //* modules *//

    pub const MEM: FlagsFeatures = (&["_some_mem"], &[
        "mem_bit",
    ]);
    pub const NUM: FlagsFeatures = (&["_some_num"], &[
        "num_geom", "num_rand",
    ]);
    pub const REND: FlagsFeatures = (&["_some_rend"], &[
        "rend_audio", "rend_color", "rend_font", "rend_image", "rend_video",
    ]);
    pub const UI: FlagsFeatures = (&["_some_ui"], &[
        "ui_service",
    ]);

    //* capabilities *//

    /* data */

    pub const BIT: FlagsFeatures = (&["_some_bit"], &[
        "_bit_i8", "_bit_i16", "_bit_i32", "_bit_i64", "_bit_i128", "_bit_isize",
        "_bit_u8", "_bit_u16", "_bit_u32", "_bit_u64", "_bit_u128", "_bit_usize",
    ]);

    // collections
    pub const DESTAQUE: FlagsFeatures = (&["_some_destaque"], &[
        "_destaque_u8", "_destaque_u16", "_destaque_u32", "_destaque_usize",
    ]);
    pub const GRAPH: FlagsFeatures = (&["_some_graph"], &[
        "_graph_u8", "_graph_u16", "_graph_u32", "_graph_usize",
    ]);
    pub const NODE: FlagsFeatures = (&["_some_node"], &[
        "_node_u8", "_node_u16", "_node_u32", "_node_usize",
    ]);
    pub const STACK: FlagsFeatures = (&["_some_stack"], &[
        "_stack_u8", "_stack_u16", "_stack_u32", "_stack_usize",
    ]);

    pub const SORT_INT: FlagsFeatures = (&["_some_sort", "_some_sort_int"], &[
        "_sort_i8", "_sort_i16", "_sort_i32", "_sort_i64", "_sort_i128", "_sort_isize",
        "_sort_u8", "_sort_u16", "_sort_u32", "_sort_u64", "_sort_u128", "_sort_usize",
    ]);
    pub const SORT_FLOAT: FlagsFeatures = (&["_some_sort", "_some_sort_float"], &[
        "_sort_f32", "_sort_f64"
    ]);

    /* num */

    pub const CMP: FlagsFeatures = (&["_some_cmp"], &[
        "_cmp_i8", "_cmp_i16", "_cmp_i32", "_cmp_i64", "_cmp_i128", "_cmp_isize",
        "_cmp_u8", "_cmp_u16", "_cmp_u32", "_cmp_u64", "_cmp_u128", "_cmp_usize",
        "_cmp_f32", "_cmp_f64"
    ]);

    // numbers
    pub const FLOAT: FlagsFeatures = (&["_some_float", "_some_nums"], &[
        "_float_f32", "_float_f64"
    ]);
    pub const INT: FlagsFeatures = (&["_some_int_i", "_some_int", "_some_nums"], &[
        "_int_i8", "_int_i16", "_int_i32", "_int_i64", "_int_i128", "_int_isize",
    ]);
    pub const UINT: FlagsFeatures = (&["_some_int_u", "_some_int", "_some_nums"], &[
        "_int_u8", "_int_u16", "_int_u32", "_int_u64", "_int_u128", "_int_usize",
    ]);

    // niche
    pub const NON_VALUE_I: FlagsFeatures = (
        &["_some_non_value", "_some_non_value_i", "_some_niche"],
        &["_non_value_i8", "_non_value_i16", "_non_value_i32",
          "_non_value_i64", "_non_value_i128", "_non_value_isize" ]);
    pub const NON_VALUE_U: FlagsFeatures = (
        &["_some_non_value", "_some_non_value_u", "_some_niche"],
        &["_non_value_u8", "_non_value_u16", "_non_value_u32",
          "_non_value_u64", "_non_value_u128", "_non_value_usize"]);
    pub const NON_RANGE: FlagsFeatures = (
        &["_some_non_range", "_some_niche"],
        &["_non_range_i8", "_non_range_i16", "_non_range_i32",
        "_non_range_i64", "_non_range_i128", "_non_range_isize",
        "_non_range_u8", "_non_range_u16", "_non_range_u32",
        "_non_range_u64", "_non_range_u128", "_non_range_usize",
    ]);
    pub const IN_RANGE: FlagsFeatures = (
        &["_some_in_range", "_some_niche"],
        &["_in_range_i8", "_in_range_i16", "_in_range_i32",
        "_in_range_i64", "_in_range_i128", "_in_range_isize",
        "_in_range_u8", "_in_range_u16", "_in_range_u32",
        "_in_range_u64", "_in_range_u128", "_in_range_usize",
    ]);

    /* text */

    pub const STRING_U: FlagsFeatures = (&["_some_string", "_some_string_u"], &[
        "_string_u8", "_string_u16", "_string_u32", "_string_usize",
    ]);
    pub const STRING: FlagsFeatures = (&["_some_string"], &[
        "_string_nonul",
    ]);


    // funtion helpers
    // -------------------------------------------------------------------------

    /// Set the flags for all the corresponding enabled features from the list.
    pub(super) fn set_flags() {
        for ff in [
            /* misc. */

            MISC,
            ENVIRONMENT,
            SAFETY,
            NIGHTLY,
            DEPENDENCY,

            /* modules */

            MEM, NUM, REND, UI,

            /* capabilities */

            // data
            BIT,
            DESTAQUE, GRAPH, NODE, STACK, // collections
            SORT_INT, SORT_FLOAT,
            // text
            STRING, STRING_U,
            // num
            CMP,
            FLOAT, INT, UINT, // numbers
            NON_VALUE_I, NON_VALUE_U, NON_RANGE, IN_RANGE, // niche

        ] { set_flags_dbg_features(ff.0, ff.1); }
    }

    /// Sets configuration flags for reflection if some features are enabled.
    ///
    /// - flag_names: The name of the cfg flag to set if any feature is enabled.
    /// - features:  The feature names to check.
    #[cfg(not(feature = "__dbg"))]
    fn set_flags_dbg_features(flags: &[&str], features: &[&str]) {
        let is_enabled = features
            .iter()
            .any(|&f| var(format!("CARGO_FEATURE_{}", f.to_uppercase())).is_ok());
        if is_enabled {
            for flag in flags {
                println!("cargo:rustc-cfg={}", flag);
            }
        }
    }
    #[cfg(feature = "__dbg")]
    fn set_flags_dbg_features(flags: &[&str], features: &[&str]) {
        let mut is_enabled = false;
        features.iter().for_each(|&f| {
            if var(format!("CARGO_FEATURE_{}", f.to_uppercase())).is_ok() {
                println(&format!["feature = \"{f}\""]);
                is_enabled = true;
            }
        });
        if is_enabled {
            for flag in flags {
                println!("cargo:rustc-cfg={}", flag);
                println(flag);
            }
            println("---------------------------");
        }
    }

    #[cfg(feature = "__dbg")]
    pub(crate) fn println(msg: &str) { println!("cargo:warning={}", msg); }
}
