// devela build::features
//
//! Features debugging and compile flags enabling for reflexion.
//

#[cfg(feature = "__dbg")]
use crate::utils::println;
use std::{collections::HashSet, env, sync::OnceLock};

/// The set of enabled features.
pub(crate) static ENABLED_FEATURES: OnceLock<HashSet<String>> = OnceLock::new();

pub(crate) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    super::utils::println_heading("Features & Flags:");

    ENABLED_FEATURES.get_or_init(|| {
        env::vars()
            .filter_map(|(key, _)| key.strip_prefix("CARGO_FEATURE_").map(|f| f.to_lowercase()))
            .collect::<HashSet<_>>()
    });

    #[cfg(feature = "__dbg")]
    if let Some(f) = ENABLED_FEATURES.get() {
        println(&format!("Enabled features ({}): {:?}", f.len(), f));
        println("");
    };

    let _enabled_flags = reflection::set_flags();

    #[cfg(feature = "__dbg")]
    {
        println(&format!("Enabled flags ({}): {:?}", _enabled_flags.len(), _enabled_flags));
    }

    Ok(())
}

/// Sets configuration options for reflection, based on enabled features.
//
// https://doc.rust-lang.org/reference/conditional-compilation.html#set-configuration-options
#[rustfmt::skip]
mod reflection {
    use super::ENABLED_FEATURES;

    /// A type that associates a list of flags with a list of features.
    pub struct FlagsFeatures<'a> {
        /// The list of flags to be enabled if any of the features are enabled.
        flags: &'a [&'a str],
        /// The list of features that enables the associated flags.
        features: &'a [&'a str],
    }

    /* # miscellaneous */

    pub const DEVELOPMENT: FlagsFeatures = FlagsFeatures {
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

    // In sync with ./Cargo.toml::[un][safe][st] & ./src/lib.rs::safety
    pub const SAFE: FlagsFeatures = FlagsFeatures {
        flags: &["safe··"],
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
        ]
    };
    pub const SAFEST: FlagsFeatures = FlagsFeatures {
        flags: &[],
        features: &["safest"],
    };
    pub const UNSAFE: FlagsFeatures = FlagsFeatures {
        flags: &["unsafe··"],
        features: &[
            "unsafe", // [11]
            "unsafe_array", "unsafe_ffi", "unsafe_hint", "unsafe_layout",
            "unsafe_niche", "unsafe_ptr", "unsafe_slice", "unsafe_str",
            "unsafe_sync", "unsafe_syscall", "unsafe_thread",
        ]
    };

    // In sync with ./Cargo.toml::nightly & ./src/lib.rs
    pub const NIGHTLY: FlagsFeatures = FlagsFeatures {
        flags: &["nightly··"],
        features: &[
            "nightly_allocator",
            "nightly_autodiff",
            "nightly_bigint",
            "nightly_coro",
            "nightly_doc",
            "nightly_float",
            "nightly_simd",
            "nightly_stable",
        ]
    };

    pub const DEPENDENCY: FlagsFeatures = FlagsFeatures {
        flags: &["dep··"],
        features: &include!{"../config/dep_all.rs"},
    };

    /* # modules */

    pub const CODE: FlagsFeatures = FlagsFeatures {
        flags: &["code··"],
        features: &["code", "error"]
    };
    pub const DATA: FlagsFeatures = FlagsFeatures {
        flags: &["data··"],
        features: &["data", "hash"]
    };
    pub const LANG: FlagsFeatures = FlagsFeatures {
        flags: &["lang··"],
        features: &["lang", "glsl", "js"]
    };
    pub const MEDIA: FlagsFeatures = FlagsFeatures {
        flags: &["media··"],
        features: &["media", "audio", "color", "draw", "font", "image"]
    };
    pub const NUM: FlagsFeatures = FlagsFeatures {
        flags: &["num··"],
        features: &["num", "alg", "geom", "prim", "rand", "unit"]
    };
        pub const PRIM: FlagsFeatures = FlagsFeatures {
            flags: &["prim··"],
            features: &["prim", "cast", "join", "split"]
        };
    pub const PHYS: FlagsFeatures = FlagsFeatures {
        flags: &["phys··"],
        features: &["phys", "time", "wave"]
    };
    pub const SYS: FlagsFeatures = FlagsFeatures {
        flags: &["sys··"],
        features: &["sys", "io", "mem",
            /* os: */ "linux", "windows"]
    };
        // RETHINK:
        pub const MEM: FlagsFeatures = FlagsFeatures {
            flags: &["mem··"],
            features: &["mem", "bit"]
        };
    pub const TEXT: FlagsFeatures = FlagsFeatures {
        flags: &["text··"],
        features: &["text", "ascii", "fmt", "str"]
    };
    pub const UI: FlagsFeatures = FlagsFeatures {
        flags: &["ui··"],
        features: &[
            "ui", "layout",
            "dep_crossterm", "dep_fltk", "dep_girls", "dep_miniquad", "dep_sdl2", "dep_sdl3",
        ]
    };
    pub const WORK: FlagsFeatures = FlagsFeatures {
        flags: &["work··"],
        features: &["work", "process", "sync", "thread"]
    };

    /* # capabilities */

    /* ## code */

    pub const UNROLL: FlagsFeatures = FlagsFeatures {
        flags: &[],
        features: &[
            "_unroll", "_unroll_128", "_unroll_256", "_unroll_512", "_unroll_1024", "_unroll_2048",
        ]
    };

    /* ## data */

    pub const BIT: FlagsFeatures = FlagsFeatures {
        flags: &["_bit··"],
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
        flags: &["_destaque··"],
        features: &["_destaque_u8", "_destaque_u16", "_destaque_u32", "_destaque_usize"]
    };
    pub const GRAPH: FlagsFeatures = FlagsFeatures {
        flags: &["_graph··"],
        features: &["_graph_u8", "_graph_u16", "_graph_u32", "_graph_usize"]
    };
    pub const NODE: FlagsFeatures = FlagsFeatures {
        flags: &["_node··"],
        features: &["_node_u8", "_node_u16", "_node_u32", "_node_usize"]
    };
    pub const STACK: FlagsFeatures = FlagsFeatures {
        flags: &["_stack··"],
        features: &["_stack_u8", "_stack_u16", "_stack_u32", "_stack_usize"] };

    pub const SORT_INT: FlagsFeatures = FlagsFeatures {
        flags: &["_sort··", "_sort_int··"],
        features: &[
            "_sort_i8", "_sort_i16", "_sort_i32", "_sort_i64", "_sort_i128", "_sort_isize",
            "_sort_u8", "_sort_u16", "_sort_u32", "_sort_u64", "_sort_u128", "_sort_usize",
        ]
    };
    pub const SORT_FLOAT: FlagsFeatures = FlagsFeatures {
        flags: &["_sort··", "_sort_float··"],
        features: &["_sort_f32", "_sort_f64"]
    };

    /* ## num */

    pub const CMP: FlagsFeatures = FlagsFeatures {
        flags: &["_cmp··"],
        features: &[
            "_cmp_i8", "_cmp_i16", "_cmp_i32", "_cmp_i64", "_cmp_i128", "_cmp_isize",
            "_cmp_u8", "_cmp_u16", "_cmp_u32", "_cmp_u64", "_cmp_u128",
            "_cmp_f32", "_cmp_f64",
        ]
    };

    // ### numbers
    pub const FLOAT: FlagsFeatures = FlagsFeatures {
        flags: &["_float··", "_nums··"],
        features: &["_float_f32", "_float_f64"] };
    pub const INT: FlagsFeatures = FlagsFeatures {
        flags: &["_int_i··", "_int··", "_nums··"],
        features: &["_int_i8", "_int_i16", "_int_i32", "_int_i64", "_int_i128", "_int_isize"] };
    pub const UINT: FlagsFeatures = FlagsFeatures {
        flags: &["_int_u··", "_int··", "_nums··"],
        features: &["_int_u8", "_int_u16", "_int_u32", "_int_u64", "_int_u128", "_int_usize"] };

    /* ## text */

    pub const STRING_U: FlagsFeatures = FlagsFeatures {
        flags: &["_str··", "_str_u··"],
        features: &["_str_u8", "_str_u16", "_str_u32", "_str_usize"] };
    pub const STRING: FlagsFeatures = FlagsFeatures {
        flags: &["_str··"],
        features: &["_str_nonul"] };
    pub const CHAR: FlagsFeatures = FlagsFeatures {
        flags: &["_char··"],
        features: &["_char7", "_char8", "_char16", "_char24", "_char32"] };


    // function helpers
    // -------------------------------------------------------------------------

    /// Set the flags for all the corresponding enabled features from the list.
    ///
    /// This is the list of the constants defined above.
    pub(super) fn set_flags() -> Vec<String> {
        let mut enabled_flags = Vec::new();

        for ff in [
            /* development */

            DEVELOPMENT,
            ENVIRONMENT,
            SAFE, SAFEST, UNSAFE,
            NIGHTLY,
            DEPENDENCY,

            /* modules */

            CODE,
            DATA,
            LANG,
            MEDIA,
            NUM, PRIM,
            PHYS,
            SYS, MEM,
            TEXT,
            UI,
            WORK,

            /* capabilities */

            // code
            UNROLL,
            // data
            BIT, TUPLE,
            DESTAQUE, GRAPH, NODE, STACK, // collections
            SORT_INT, SORT_FLOAT,
            // text
            CHAR, STRING, STRING_U,
            // num
            CMP,
            FLOAT, INT, UINT, // numbers

        ] { set_flags_dbg_features(ff.flags, ff.features, &mut enabled_flags); }

        enabled_flags
    }

    /// Sets configuration flags for reflection if some features are enabled.
    ///
    /// - flag_names: The name of the cfg flag to set if any feature is enabled.
    /// - features:   The feature names to check.
    fn set_flags_dbg_features(flags: &[&str], features: &[&str], enabled: &mut Vec<String>) {
        let is_enabled = features.iter().any(|&f| ENABLED_FEATURES.get().unwrap().contains(f));
        if is_enabled {
            for flag in flags {
                println!("cargo:rustc-cfg={}", flag);
                enabled.push(flag.to_string());
            }
        }
    }
}
