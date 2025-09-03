// devela::build::features
//
//! Features debugging and compile flags enabling for reflexion.
//
// NOTE: this file is shared between the build scripts in:
// - devela/build/main/
// - devela_base/build/
//

#[cfg(feature = "__dbg")]
use super::Build;
use std::{collections::HashSet, env, sync::OnceLock};

/// The set of enabled cargo features.
pub(crate) static ENABLED_CARGO_FEATURES: OnceLock<HashSet<String>> = OnceLock::new();
/// The set of enabled cfg flags.
pub(crate) static ENABLED_CFG_FLAGS: OnceLock<HashSet<String>> = OnceLock::new();

pub(crate) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    Build::println_heading("Features & Configuration Flags:");

    /* Collect enabled cargo features from CARGO_FEATURE_* environment variables */

    ENABLED_CARGO_FEATURES.get_or_init(|| {
        let mut features = HashSet::new();
        for (key, _) in env::vars_os() {
            if let Some(feature) = key.to_str().and_then(|k| k.strip_prefix("CARGO_FEATURE_")) {
                features.insert(feature.to_lowercase());
            }
        }
        features
    });
    #[cfg(feature = "__dbg")]
    if let Some(f) = ENABLED_CARGO_FEATURES.get() {
        Build::println(&format!("Active cargo features ({}): {:?}", f.len(), f));
        Build::println("");
    };
    // Enable reflection flags based on cargo features
    let _enabled_flags_from_features = reflection::set_ref_flags_from_cargo_features();
    #[cfg(feature = "__dbg")]
    {
        Build::println(&format!(
            "Reflection flags auto-enabled by features ({}): {:?}",
            _enabled_flags_from_features.len(),
            _enabled_flags_from_features,
        ));
        Build::println("");
    }

    /* Collect enabled cfg flags from both RUSTFLAGS and RUSTDOCFLAGS */

    // WARNING: `RUSTDOCFLAGS` changes may not take effect immediately!
    // ------------------------------------------------------------------
    // Unlike `RUSTFLAGS`, changes to `RUSTDOCFLAGS` may not always trigger a rebuild.
    // Cargo caches documentation builds, so previously compiled flags might still apply.
    ENABLED_CFG_FLAGS.get_or_init(|| {
        let mut cfg_flags = HashSet::new();
        if let Ok(value) = env::var("RUSTDOCFLAGS") {
            cfg_flags.extend(value.split_whitespace().map(String::from));
        }
        if let Ok(value) = env::var("CARGO_ENCODED_RUSTFLAGS") {
            cfg_flags.extend(value.split('\x1f').map(String::from));
        }
        cfg_flags
    });
    #[cfg(feature = "__dbg")]
    if let Some(f) = ENABLED_CFG_FLAGS.get() {
        // IMPROVE FIXME always shows as ""
        let filtered_flags: Vec<_> = f.iter().filter(|&f| f != "--cfg" && f != "-C").collect();
        // let filtered_flags: Vec<_> = f.iter().collect(); // SAME FOR THIS (TEMP)
        Build::println(&format!(
            "Active compiler cfg flags ({}): {:?}",
            filtered_flags.len(),
            filtered_flags
        ));
        Build::println("");
    }
    // Enable reflection flags based on cfg flags (e.g., RUSTFLAGS)
    let _enabled_flags_from_cfg_flags = reflection::set_ref_flags_from_cfg_flags();
    #[cfg(feature = "__dbg")]
    {
        Build::println(&format!(
            "Flags auto-enabled by cfg flags ({}): {:?}",
            _enabled_flags_from_cfg_flags.len(),
            _enabled_flags_from_cfg_flags,
        ));
        Build::println("");
    }

    Ok(())
}

/// Sets configuration options for reflection, based on enabled features.
//
// https://doc.rust-lang.org/reference/conditional-compilation.html#set-configuration-options
#[rustfmt::skip]
mod reflection {
    use super::{ENABLED_CARGO_FEATURES, ENABLED_CFG_FLAGS};

    /* FLAGS
   -------------------------------------------------------------------------
   - Each `FlagsFlags` entry maps `cfg_flags` (required for activation)
     to `auto_flags` (which get enabled when any `cfg_flags` is present).

   - Important: Auto-enabled flags **do not propagate recursively**.
     If a flag (e.g., `nightly_stable`) has its own dependent flags,
     it must be explicitly included in the `cfg_flags` list of the parent
     (e.g., `nightly` must list `nightly_stable` if its children should be active).
    */

    /// Associates a set of automatically enabled flags with a set of `cfg` flags.
    pub struct FlagsFlags<'a> {
        /// Flags that are enabled when any of the `cfg_flags` are present.
        auto_flags: &'a [&'a str],
        /// `cfg` flags that trigger the corresponding `auto_flags`.
        cfg_flags: &'a [&'a str],
    }

    // In sync with ./Cargo.toml, ./docs/nightly.md && ./src/lib.rs
    pub const FLAGS_NIGHTLY: FlagsFlags = FlagsFlags {
        auto_flags: &[
            "nightly_unstable",
                // "nightly_autodiff", // FLAG_DISABLED
                "nightly_allocator", "nightly_become", "nightly_bigint", "nightly_coro",
                "nightly_doc", "nightly_float", "nightly_simd",
            //
            "nightly_stable",
                "nightly_stable_1_90", "nightly_stable_1_91", "nightly_stable_later",
        ],
        cfg_flags: &["nightly"],
    };
        pub const FLAGS_NIGHTLY_UNSTABLE: FlagsFlags = FlagsFlags {
            auto_flags: &[
                // "nightly_autodiff", // FLAG_DISABLED
                "nightly_allocator", "nightly_become", "nightly_bigint", "nightly_coro",
                "nightly_doc", "nightly_float", "nightly_simd",
            ],
            cfg_flags: &["nightly_stable"],
        };
        pub const FLAGS_NIGHTLY_STABLE: FlagsFlags = FlagsFlags {
            auto_flags: &["nightly_stable_1_90", "nightly_stable_1_91", "nightly_stable_later"],
            cfg_flags: &["nightly_stable"],
        };
        pub const FLAGS_NIGHTLY_REFLECT: FlagsFlags = FlagsFlags {
            auto_flags: &["nightly··"],
            cfg_flags: &[ "nightly",
                "nightly_unstable",
                    // "nightly_autodiff", // FLAG_DISABLED
                    "nightly_allocator", "nightly_bigint", "nightly_coro",
                    "nightly_doc", "nightly_float", "nightly_simd",
                //
                "nightly_stable",
                    "nightly_stable_1_90", "nightly_stable_1_91", "nightly_stable_later",
            ],
        };

    /* FEATURES
   -------------------------------------------------------------------------
   - Each `FlagsFeatures` entry maps `features` (required for activation)
     to `ref_flags` (which get enabled when any `features` are present).

   - Features propagate **transitively**—enabling a feature activates all its
     direct and indirect flags. (e.g., enabling `"sys"` also enables `"mem"` and,
     through it, `"mem··"`, even though `"mem··"` isn't directly enabled in `"sys"`).
    */

    /// Associates a set of reflection flags with a set of `cfg` features.
    pub struct FlagsFeatures<'a> {
        /// Reflection flags enabled when any of the `features` are present.
        ref_flags: &'a [&'a str],
        /// Cargo features that trigger the corresponding `ref_flags`.
        features: &'a [&'a str],
    }

    /* # miscellaneous */

    pub const DEVELOPMENT: FlagsFeatures = FlagsFeatures {
        ref_flags: &[],
        features: &[
            "__build",
            "__dbg",
            "__no_test",
            "__force_miri_dst",
            "__force_test_no_mangle",
            // "default",
            // "_default",
            "_docs", "_docs_nodeps", "_docs_min",
            "_max", "_maxest",
        ]
    };

    pub const ENVIRONMENT: FlagsFeatures = FlagsFeatures {
        ref_flags: &[],
        features: &["std", "alloc", "no_std"]
    };

    // In sync with ./Cargo.toml::[un][safe][st] & ./src/lib.rs::safety
    pub const SAFE: FlagsFeatures = FlagsFeatures {
        ref_flags: &["safe··"],
        features: &[
            "safest",
            "safe",
            "safe_base",
            "safe_build",
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
            "safe_ui",
                "safe_layout",
            "safe_work",
        ]
    };
    pub const UNSAFE: FlagsFeatures = FlagsFeatures {
        ref_flags: &["unsafe··"],
        features: &[
            "unsafe", // [11]
            "unsafe_array", "unsafe_ffi", "unsafe_hint", "unsafe_layout",
            "unsafe_niche", "unsafe_ptr", "unsafe_slice", "unsafe_str",
            "unsafe_sync", "unsafe_syscall", "unsafe_thread",
        ]
    };

    // In sync with ../Cargo.toml::dep_all & ../src/_dep.rs
    pub const DEPENDENCY: FlagsFeatures = FlagsFeatures {
        ref_flags: &["dep··"],
        features: &include!{concat!(env!("CARGO_WORKSPACE_DIR"), "/config/dep_all.rs")},
    };

    /* # modules */

    pub const CODE: FlagsFeatures = FlagsFeatures {
        ref_flags: &["code··"],
        features: &["code"]
    };
    pub const DATA: FlagsFeatures = FlagsFeatures {
        ref_flags: &["data··"],
        features: &["data", "hash"]
    };
    pub const GAME: FlagsFeatures = FlagsFeatures {
        ref_flags: &["game··"],
        features: &["game"]
    };
    pub const LANG: FlagsFeatures = FlagsFeatures {
        ref_flags: &["lang··"],
        features: &["lang", "glsl", "js"]
    };
        pub const FFI: FlagsFeatures = FlagsFeatures {
            ref_flags: &["ffi··"],
            features: &["glsl", "js"]
        };
    pub const MEDIA: FlagsFeatures = FlagsFeatures {
        ref_flags: &["media··"],
        features: &["media", "audio", "color", "draw", "font", "image"]
    };
    pub const NUM: FlagsFeatures = FlagsFeatures {
        ref_flags: &["num··"],
        features: &["num",
            "geom",
                "linear", "metric", "shape",
            "rand",
            "unit",
        ]
    };
        pub const GEOM: FlagsFeatures = FlagsFeatures {
            ref_flags: &["geom··"],
            features: &["geom", "linear", "metric", "shape"]
        };
    pub const PHYS: FlagsFeatures = FlagsFeatures {
        ref_flags: &["phys··"],
        features: &["phys", "time", "wave"]
    };
    pub const SYS: FlagsFeatures = FlagsFeatures {
        ref_flags: &["sys··"],
        features: &["sys", "io",
            "mem", "bit",
            /* os: */ "linux", "windows"]
    };
        // RETHINK:
        pub const MEM: FlagsFeatures = FlagsFeatures {
            ref_flags: &["mem··"],
            features: &["mem", "bit"]
        };
    pub const TEXT: FlagsFeatures = FlagsFeatures {
        ref_flags: &["text··"],
        features: &["text", "ascii", "fmt", "str"]
    };
    pub const UI: FlagsFeatures = FlagsFeatures {
        ref_flags: &["ui··"],
        features: &[
            "ui", "layout",
            /* front: */ "desk", "term", "web",
            "dep_crossterm", "dep_fltk", "dep_girls", "dep_miniquad", "dep_sdl2", "dep_sdl3",
        ]
    };
    pub const WORK: FlagsFeatures = FlagsFeatures {
        ref_flags: &["work··"],
        features: &["work", "process", "sync", "thread"]
    };

    /* # capabilities */

    /* ## code */

    pub const UNROLL: FlagsFeatures = FlagsFeatures {
        ref_flags: &[],
        features: &[
            "_unroll", "_unroll_128", "_unroll_256", "_unroll_512", "_unroll_1024", "_unroll_2048",
        ]
    };

    /* ## data */

    pub const TUPLE: FlagsFeatures = FlagsFeatures {
        ref_flags: &[],
        features: &["_tuple", "_tuple_24", "_tuple_36", "_tuple_48", "_tuple_72"]
    };

    // ### collections
    pub const DESTAQUE: FlagsFeatures = FlagsFeatures {
        ref_flags: &["_destaque··"],
        features: &["_destaque_u8", "_destaque_u16", "_destaque_u32", "_destaque_usize"]
    };
    pub const GRAPH: FlagsFeatures = FlagsFeatures {
        ref_flags: &["_graph··"],
        features: &["_graph_u8", "_graph_u16", "_graph_u32", "_graph_usize"]
    };
    pub const NODE: FlagsFeatures = FlagsFeatures {
        ref_flags: &["_node··"],
        features: &["_node_u8", "_node_u16", "_node_u32", "_node_usize"]
    };
    pub const STACK: FlagsFeatures = FlagsFeatures {
        ref_flags: &["_stack··"],
        features: &["_stack_u8", "_stack_u16", "_stack_u32", "_stack_usize"] };

    /* ## num */

    // ### numbers
    pub const FLOAT: FlagsFeatures = FlagsFeatures {
        ref_flags: &["_float··", "_nums··"],
        features: &["_float_f32", "_float_f64"] };
    pub const INT: FlagsFeatures = FlagsFeatures {
        ref_flags: &["_int_i··", "_int··", "_nums··"],
        features: &["_int_i8", "_int_i16", "_int_i32", "_int_i64", "_int_i128", "_int_isize"] };
    pub const UINT: FlagsFeatures = FlagsFeatures {
        ref_flags: &["_int_u··", "_int··", "_nums··"],
        features: &["_int_u8", "_int_u16", "_int_u32", "_int_u64", "_int_u128", "_int_usize"] };

    /* ## text */

    pub const STRING: FlagsFeatures = FlagsFeatures {
        ref_flags: &["_str··"],
        features: &["_str_nonul"] };
    pub const CHAR: FlagsFeatures = FlagsFeatures {
        ref_flags: &["_char··"],
        features: &["_char7", "_char8", "_char16", "_char24", "_char32"] };

    // function helpers
    // -------------------------------------------------------------------------

    /* cargo features */

    /// Sets the reflection flags for all the corresponding enabled cargo features from the list.
    ///
    /// This is the list of the constants defined above.
    pub(super) fn set_ref_flags_from_cargo_features() -> Vec<String> {
        let mut enabled_ref_flags = Vec::new();

        for ff in [
            /* development */

            DEVELOPMENT,
            ENVIRONMENT,
            SAFE, UNSAFE,
            DEPENDENCY,

            /* modules */

            CODE,
            DATA,
            GAME,
            LANG, FFI,
            MEDIA,
            NUM, GEOM,
            PHYS,
            SYS, MEM,
            TEXT,
            UI,
            WORK,

            /* capabilities */

            // code
            UNROLL,
            // data
            TUPLE,
            DESTAQUE, GRAPH, NODE, STACK, // collections
            // text
            CHAR, STRING,
            // num
            FLOAT, INT, UINT, // numbers

        ] { set_flags_dbg_features(ff.ref_flags, ff.features, &mut enabled_ref_flags); }

        enabled_ref_flags
    }
    /// Sets reflection flags if some **FEATURES** are enabled.
    ///
    /// - flag_names: The name of the reflection flag to set if any feature is enabled.
    /// - features:   The cargo features names to check.
    fn set_flags_dbg_features(ref_flags: &[&str], features: &[&str], enabled: &mut Vec<String>) {
        let is_enabled = features.iter().any(|&f| ENABLED_CARGO_FEATURES.get().unwrap().contains(f));
        if is_enabled {
            for flag in ref_flags {
                println!("cargo:rustc-cfg={flag}");
                enabled.push(flag.to_string());
            }
        }
    }

    /* cfg flags */

    /// Sets automatic flags, based on enabled cfg flags.
    pub(super) fn set_ref_flags_from_cfg_flags() -> Vec<String> {
        let mut enabled_ref_flags = Vec::new();
        for ff in [
            FLAGS_NIGHTLY, FLAGS_NIGHTLY_UNSTABLE, FLAGS_NIGHTLY_STABLE, FLAGS_NIGHTLY_REFLECT,
        ] {
            set_flags_dbg_flags(ff.auto_flags, ff.cfg_flags, &mut enabled_ref_flags);
        }
        enabled_ref_flags
    }
    /// Sets automatic flags if some **FLAGS** are enabled.
    ///
    /// - flag_names: The name of the reflection flag to set if any cfg flag is enabled.
    /// - cfg_flags:   The cfg flags names to check.
    fn set_flags_dbg_flags(ref_flags: &[&str], cfg_flags: &[&str], enabled: &mut Vec<String>) {
        let is_enabled = cfg_flags.iter().any(|&f| ENABLED_CFG_FLAGS.get().unwrap().contains(f));
        if is_enabled {
            for flag in ref_flags {
                println!("cargo:rustc-cfg={flag}");
                enabled.push(flag.to_string());
            }
        }
    }
}
