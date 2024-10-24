// devela build::features
//
//! Features debugging and compile flags enabling for reflexion.
//

pub(crate) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    crate::utils::println_heading("Features & Flags:");

    reflection::set_flags();

    Ok(())
}

/// Sets configuration options for reflection, based on enabled features.
//
// https://doc.rust-lang.org/reference/conditional-compilation.html#set-configuration-options
#[rustfmt::skip]
mod reflection {
    #[cfg(feature = "__dbg")]
    use crate::utils::println;
    use std::env::var;

    /// A type that associates a list of flags with a list of features.
    pub struct FlagsFeatures<'a> {
        /// The list of flags to be enabled if any of the features are enabled.
        flags: &'a [&'a str],
        /// The list of features that enables the associated flags.
        features: &'a [&'a str],
    }

    /* # miscellaneous */

    pub const MISCELLANEOUS: FlagsFeatures = FlagsFeatures {
        flags: &[],
        features: &[
            "__dbg",
            "__lints",
            "__no_test",
            "__force_miri_dst",
            // "default",
            // "_default",
            "_docsrs", "_docsrs_min",
            "_docsrs_stable", "_docsrs_stable_min",
            "_docs_max", "_docs_min",
        ]
    };

    pub const ENVIRONMENT: FlagsFeatures = FlagsFeatures {
        flags: &[],
        features: &["std", "alloc", "no_std"]
    };

    pub const PLATFORM: FlagsFeatures = FlagsFeatures {
        flags: &[],
        features: &["linux"]
    };

    pub const SAFETY: FlagsFeatures = FlagsFeatures {
        flags: &[],
        features: &[
            "safe",
            "safe_code",
            "safe_data",
            "safe_error",
            "safe_mem",
            "safe_num",
            "safe_rend",
            "safe_sys", "safe_time",
            "safe_text",
            "safe_work",

            "unsafe",
            "unsafe_array",
            "unsafe_async",
            "unsafe_const",
            "unsafe_layout",
            "unsafe_hint",
            "unsafe_niche",
            "unsafe_ptr",
            "unsafe_slice",
            "unsafe_str",
            "unsafe_sync",
            "unsafe_thread",
        ]
    };

    pub const NIGHTLY: FlagsFeatures = FlagsFeatures {
        flags: &["_some_nightly"],
        features: &[
            "nightly_coro",
            "nightly_doc",
            "nightly_simd",
            "nightly_stabilized",
        ]
    };

    pub const DEPENDENCY: FlagsFeatures = FlagsFeatures {
        flags: &["_some_dep"],
        features: &[
            "dep_atomic", "dep_bytemuck", "dep_const_str", "dep_hashbrown", "dep_libm",
            "dep_log", "dep_macroquad", "dep_miniquad", "dep_memchr", "dep_miniquad",
            "dep_portable_atomic", "dep_rand_core", "dep_rodio", "dep_tinyaudio",
            "dep_unicode_segmentation", "dep_unicode_width", "dep_wide",
        ]
    };

    /* # modules */

    pub const MEM: FlagsFeatures = FlagsFeatures {
        flags: &["_some_mem"],
        features: &["mem_bit"]
    };
    pub const NUM: FlagsFeatures = FlagsFeatures {
        flags: &["_some_num"],
        features: &["num_geom", "num_rand"]
    };
    pub const REND: FlagsFeatures = FlagsFeatures {
        flags: &["_some_rend"],
        features: &["rend_audio", "rend_color", "rend_draw", "rend_font", "rend_image", "rend_layout"]
    };

    /* # capabilities */

    /* ## data */

    pub const BIT: FlagsFeatures = FlagsFeatures {
        flags: &["_some_bit"],
        features: &[
            "_bit_i8", "_bit_i16", "_bit_i32", "_bit_i64", "_bit_i128", "_bit_isize",
            "_bit_u8", "_bit_u16", "_bit_u32", "_bit_u64", "_bit_u128", "_bit_usize",
        ]
    };

    pub const TUPLE: FlagsFeatures = FlagsFeatures {
        flags: &[],
        features: &["_tuple", "_tuple_24", "_tuple_36", "_tuple_48", "_tuple_72"]
    };

    // ### collections
    pub const DESTAQUE: FlagsFeatures = FlagsFeatures {
        flags: &["_some_destaque"],
        features: &["_destaque_u8", "_destaque_u16", "_destaque_u32", "_destaque_usize"]
    };
    pub const GRAPH: FlagsFeatures = FlagsFeatures {
        flags: &["_some_graph"],
        features: &["_graph_u8", "_graph_u16", "_graph_u32", "_graph_usize"]
    };
    pub const NODE: FlagsFeatures = FlagsFeatures {
        flags: &["_some_node"],
        features: &["_node_u8", "_node_u16", "_node_u32", "_node_usize"]
    };
    pub const STACK: FlagsFeatures = FlagsFeatures {
        flags: &["_some_stack"],
        features: &["_stack_u8", "_stack_u16", "_stack_u32", "_stack_usize"] };

    pub const SORT_INT: FlagsFeatures = FlagsFeatures {
        flags: &["_some_sort", "_some_sort_int"],
        features: &[
            "_sort_i8", "_sort_i16", "_sort_i32", "_sort_i64", "_sort_i128", "_sort_isize",
            "_sort_u8", "_sort_u16", "_sort_u32", "_sort_u64", "_sort_u128", "_sort_usize",
        ]
    };
    pub const SORT_FLOAT: FlagsFeatures = FlagsFeatures {
        flags: &["_some_sort", "_some_sort_float"],
        features: &["_sort_f32", "_sort_f64"]
    };

    /* ## num */

    pub const CMP: FlagsFeatures = FlagsFeatures {
        flags: &["_some_cmp"],
        features: &[
            "_cmp_i8", "_cmp_i16", "_cmp_i32", "_cmp_i64", "_cmp_i128", "_cmp_isize",
            "_cmp_u8", "_cmp_u16", "_cmp_u32", "_cmp_u64", "_cmp_u128", "_cmp_usize",
            "_cmp_f32", "_cmp_f64",
        ]
    };

    // ### numbers
    pub const FLOAT: FlagsFeatures = FlagsFeatures {
        flags: &["_some_float", "_some_nums"],
        features: &["_float_f32", "_float_f64"] };
    pub const INT: FlagsFeatures = FlagsFeatures {
        flags: &["_some_int_i", "_some_int", "_some_nums"],
        features: &["_int_i8", "_int_i16", "_int_i32", "_int_i64", "_int_i128", "_int_isize"] };
    pub const UINT: FlagsFeatures = FlagsFeatures {
        flags: &["_some_int_u", "_some_int", "_some_nums"],
        features: &["_int_u8", "_int_u16", "_int_u32", "_int_u64", "_int_u128", "_int_usize"] };

    /* ## text */

    pub const STRING_U: FlagsFeatures = FlagsFeatures {
        flags: &["_some_string", "_some_string_u"],
        features: &["_string_u8", "_string_u16", "_string_u32", "_string_usize"] };
    pub const STRING: FlagsFeatures = FlagsFeatures {
        flags: &["_some_string"],
        features: &["_string_nonul"] };
    pub const CHAR: FlagsFeatures = FlagsFeatures {
        flags: &["_some_char"],
        features: &["_char7", "_char8", "_char16", "_char24", "_char32"] };


    // function helpers
    // -------------------------------------------------------------------------

    /// Set the flags for all the corresponding enabled features from the list.
    ///
    /// This is the list of the constants defined above.
    pub(super) fn set_flags() {
        for ff in [
            /* miscellaneous */

            MISCELLANEOUS,
            ENVIRONMENT,
            PLATFORM,
            SAFETY,
            NIGHTLY,
            DEPENDENCY,

            /* modules */

            MEM, NUM, REND,

            /* capabilities */

            // data
            BIT, TUPLE,
            DESTAQUE, GRAPH, NODE, STACK, // collections
            SORT_INT, SORT_FLOAT,
            // text
            CHAR, STRING, STRING_U,
            // num
            CMP,
            FLOAT, INT, UINT, // numbers

        ] { set_flags_dbg_features(ff.flags, ff.features); }
    }

    /// Sets configuration flags for reflection if some features are enabled.
    ///
    /// - flag_names: The name of the cfg flag to set if any feature is enabled.
    /// - features:   The feature names to check.
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
                println(&format!["  flag = \"{flag}\""]);
            }
            println("");
        }
    }
}
