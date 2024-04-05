// devela build script
//
//! Shows debugging information if the `__dbg` feature is enabled.
//

#![allow(dead_code, unused)]

#[cfg(feature = "__dbg")]
fn main() {
    /* information about the enabled features */

    printfeat![msg: "Enabled miscellaneous features (other than `__dbg`:", features:
        "default", "__excluded",
        // "__dbg",
    ];

    printfeat![msg: "Enabled environment features:", features:
        "std", "alloc", "no_std",
    ];

    printfeat![msg: "Enabled safety features:", features:
        "safe",
        "safe_code",
        "safe_data", "safe_data_collections",
        "safe_exec",
        "safe_lex",
        "safe_mem",
        "safe_num",
        "safe_rend", "safe_rend_audio", "safe_rend_image",
        "safe_sys",
        "safe_time",
        "safe_ui", "safe_ui_term",
        "unsafe",
        "unsafe_array", "unsafe_async", "unsafe_const", "unsafe_dyn",
        "unsafe_niche", "unsafe_ptr", "unsafe_slice", "unsafe_str",
        "unsafe_thread",
    ];

    printfeat![msg: "Enabled module features:", features:
        "all",
        "code",
        "data", "_-data-_", "data_bit", "data_collections",
        "exec",
        "lex",
        "mem",
        "num", "_-num-_", "num_float", "num_geom", "num_int",
        "rend", "_-rend-_", "rend_audio", "rend_color", "rend_image",
        "sys",
        "time",
        "ui", "_-ui-_", "ui_term", "ui_service",
    ];

    printfeat![msg: "Enabled *nightly* features:", features:
        "nightly", "_-nightly-_", "nightly_coro", "nightly_doc", "nigthly_simd",
    ];

    printfeat![msg: "Enabled capability features:", features:
        "_default", "_max",

        "_docsrs", "_docsrs_max", "_docsrs_stable",

        "_-num_any-_",
        "_-float_any-_",
        "_float_f32", "_float_f64",
        "_-int_any-_", "_-int_iany-_",
        "_int_i8", "_int_i16", "_int_i32", "_int_i64", "_int_i128", "_int_isize",
        "_-int_uany-_",
        "_int_u8", "_int_u16", "_int_u32", "_int_u64", "_int_u128", "_int_usize",

        "_-bit_any-_",
        "_bit_i8", "_bit_i16", "_bit_i32", "_bit_i64", "_bit_i128", "_bit_isize",
        "_bit_u8", "_bit_u16", "_bit_u32", "_bit_u64", "_bit_u128", "_bit_usize",

        "_-niche_any-_", "_-non_specific_any-_",
        "_non_specific_i8", "_non_specific_i16", "_non_specific_i32",
        "_non_specific_i64", "_non_specific_i128", "_non_specific_isize",
        "_non_specific_u8", "_non_specific_u16", "_non_specific_u32",
        "_non_specific_u64", "_non_specific_u128", "_non_specific_usize",
        "_-non_range_any-_",
        "_non_range_i8", "_non_range_i16", "_non_range_i32",
        "_non_range_i64", "_non_range_i128", "_non_range_isize",
        "_non_range_u8", "_non_range_u16", "_non_range_u32",
        "_non_range_u64", "_non_range_u128", "_non_range_usize",
        "_-range_any-_",
        "_range_i8", "_range_i16", "_range_i32", "_range_i64", "_range_i128", "_range_isize",
        "_range_u8", "_range_u16", "_range_u32", "_range_u64", "_range_u128", "_range_usize",

        "_tuple_arity_31", "_tuple_arity_63", "_tuple_arity_95", "_tuple_arity_127",
    ];

    printfeat![msg: "Enabled dependency features:", features:
        "dep_all",
            "dep_exec", "dep_lex", "dep_ui_term",
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
    ];
}
#[cfg(not(feature = "__dbg"))]
fn main() {}

// private helpers
#[rustfmt::skip] fn println(msg: &str) { println!("cargo:warning={}", msg); }
macro_rules! printfeat {
    // if any of the features are enabled, prints the msg and the features
    (msg: $msg:literal, features: $($feature:literal),+ $(,)?) => {
        // #[cfg(feature = "__dbg")]
        {
            if cfg!(any($(feature = $feature),+)) { println($msg); }
            $( if cfg!(feature = $feature) { println(&format!["  {}", $feature]); } )+
        }
    };
}
use printfeat;
