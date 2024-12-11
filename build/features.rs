// devela build::features
//
//! Features debugging and compile flags enabling for reflexion.
//

pub(crate) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    super::utils::println_heading("Features & Flags:");

    reflection::set_flags();

    Ok(())
}

/// Sets configuration options for reflection, based on enabled features.
//
// https://doc.rust-lang.org/reference/conditional-compilation.html#set-configuration-options
#[rustfmt::skip]
mod reflection {
    #[cfg(feature = "__dbg")]
    use super::super::utils::println;
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

    pub const SAFETY: FlagsFeatures = FlagsFeatures {
        flags: &[],
        features: &[
            "safe",
            "safe_code",
            "safe_data",
            "safe_lang",
            "safe_media",
                "safe_audio", "safe_color", "safe_draw", "safe_font", "safe_image",
            "safe_num",
            "safe_phys",
                "safe_time",
            "safe_sys",
                "safe_io", "safe_mem",
            "safe_text",
            "safe_ui", "safe_layout",
            "safe_work",

            "unsafe",
            "unsafe_array",
            "unsafe_async",
            "unsafe_hint",
            "unsafe_layout",
            "unsafe_niche",
            "unsafe_ptr",
            "unsafe_slice",
            "unsafe_str",
            "unsafe_sync",
            "unsafe_syscall",
            "unsafe_thread",
        ]
    };

    // In sync with Cargo.toml::nightly & src/lib.rs
    pub const NIGHTLY: FlagsFeatures = FlagsFeatures {
        flags: &["_nightly_·"],
        features: &[
            "nightly_autodiff",
            "nightly_bigint",
            "nightly_coro",
            "nightly_doc",
            "nightly_float",
            "nightly_simd",
            "nightly_stable",
        ]
    };

    // In sync with Cargo.toml::dep_all & ./utils/check.rs::DEP_ALL
    pub const DEPENDENCY: FlagsFeatures = FlagsFeatures {
        flags: &["_dep_·"],
        features: &[ // 28
            "dep_atomic", "dep_bytemuck", "dep_const_str", "dep_crossterm", "dep_hashbrown",
            "dep_jiff", "dep_js_sys", "dep_libm", "dep_log", "dep_memchr", "dep_miniquad",
            "dep_nc", "dep_portable_atomic", "dep_pyo3", "dep_rand_core", "dep_rayon",
            "dep_raw_cpuid", "dep_regex_lite", "dep_rodio", "dep_rustix", "dep_stringzilla",
            "dep_sysinfo", "dep_tinyaudio", "dep_unicode_segmentation", "dep_unicode_width",
            "dep_wasm_bindgen", "dep_web_sys", "dep_wide",
        ]
    };

    /* # modules */

    pub const DATA: FlagsFeatures = FlagsFeatures {
        flags: &["_data_·"],
        features: &["data", "hash"]
    };
    pub const MEDIA: FlagsFeatures = FlagsFeatures {
        flags: &["_media_·"],
        features: &["media", "audio", "color", "draw", "font", "image"]
    };
    pub const NUM: FlagsFeatures = FlagsFeatures {
        flags: &["_num_·"],
        features: &["num", "alg", "geom", "prim", "rand", "unit", "wave"]
    };
        pub const PRIM: FlagsFeatures = FlagsFeatures {
            flags: &["prim···"],
            features: &["prim", "cast", "join", "split"]
        };
    pub const SYS: FlagsFeatures = FlagsFeatures {
        flags: &["_sys_·"],
        features: &["sys", "io", "mem", "time",
            /* os: */ "linux", "windows"]
    };
        // RETHINK:
        pub const MEM: FlagsFeatures = FlagsFeatures {
            flags: &["_mem_·"],
            features: &["mem", "bit"]
        };
    pub const TEXT: FlagsFeatures = FlagsFeatures {
        flags: &["_text_·"],
        features: &["text", "ascii", "fmt", "str"]
    };
    pub const UI: FlagsFeatures = FlagsFeatures {
        flags: &["_ui_·"],
        features: &["ui", "layout"]
    };

    /* # capabilities */

    /* ## data */

    pub const BIT: FlagsFeatures = FlagsFeatures {
        flags: &["_bit_·"],
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
        flags: &["_destaque_·"],
        features: &["_destaque_u8", "_destaque_u16", "_destaque_u32", "_destaque_usize"]
    };
    pub const GRAPH: FlagsFeatures = FlagsFeatures {
        flags: &["_graph_·"],
        features: &["_graph_u8", "_graph_u16", "_graph_u32", "_graph_usize"]
    };
    pub const NODE: FlagsFeatures = FlagsFeatures {
        flags: &["_node_·"],
        features: &["_node_u8", "_node_u16", "_node_u32", "_node_usize"]
    };
    pub const STACK: FlagsFeatures = FlagsFeatures {
        flags: &["_stack_·"],
        features: &["_stack_u8", "_stack_u16", "_stack_u32", "_stack_usize"] };

    pub const SORT_INT: FlagsFeatures = FlagsFeatures {
        flags: &["_sort_·", "_sort_int_·"],
        features: &[
            "_sort_i8", "_sort_i16", "_sort_i32", "_sort_i64", "_sort_i128", "_sort_isize",
            "_sort_u8", "_sort_u16", "_sort_u32", "_sort_u64", "_sort_u128", "_sort_usize",
        ]
    };
    pub const SORT_FLOAT: FlagsFeatures = FlagsFeatures {
        flags: &["_sort_·", "_sort_float_·"],
        features: &["_sort_f32", "_sort_f64"]
    };

    /* ## num */

    pub const CMP: FlagsFeatures = FlagsFeatures {
        flags: &["_cmp_·"],
        features: &[
            "_cmp_i8", "_cmp_i16", "_cmp_i32", "_cmp_i64", "_cmp_i128", "_cmp_isize",
            "_cmp_u8", "_cmp_u16", "_cmp_u32", "_cmp_u64", "_cmp_u128", "_cmp_usize",
            "_cmp_f32", "_cmp_f64",
        ]
    };

    // ### numbers
    pub const FLOAT: FlagsFeatures = FlagsFeatures {
        flags: &["_float_·", "_nums_·"],
        features: &["_float_f32", "_float_f64"] };
    pub const INT: FlagsFeatures = FlagsFeatures {
        flags: &["_int_i·", "_int_·", "_nums_·"],
        features: &["_int_i8", "_int_i16", "_int_i32", "_int_i64", "_int_i128", "_int_isize"] };
    pub const UINT: FlagsFeatures = FlagsFeatures {
        flags: &["_int_u·", "_int_·", "_nums_·"],
        features: &["_int_u8", "_int_u16", "_int_u32", "_int_u64", "_int_u128", "_int_usize"] };

    /* ## text */

    pub const STRING_U: FlagsFeatures = FlagsFeatures {
        flags: &["_string_·", "_string_u·"],
        features: &["_string_u8", "_string_u16", "_string_u32", "_string_usize"] };
    pub const STRING: FlagsFeatures = FlagsFeatures {
        flags: &["_string_·"],
        features: &["_string_nonul"] };
    pub const CHAR: FlagsFeatures = FlagsFeatures {
        flags: &["_char·"],
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
            SAFETY,
            NIGHTLY,
            DEPENDENCY,

            /* modules */

            DATA,
            MEDIA,
            NUM, PRIM,
            SYS, MEM,
            TEXT,
            UI,

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
