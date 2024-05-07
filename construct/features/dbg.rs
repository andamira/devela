// devela construct::dbg
//
//! Shows debugging information if the `__dbg` feature is enabled.
//

use super::super::println;
macro_rules! printfeat {
    // if any of the features are enabled, prints the msg and the features
    (msg: $msg:literal, features: $($feature:literal),+ $(,)?) => {
        {
            if cfg!(any($(feature = $feature),+)) { println($msg); }
            $( if cfg!(feature = $feature) { println(&format!["  {}", $feature]); } )+
        }
    };
}

/// Prints debugging information about the enabled features.
pub(super) fn print_features() {
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
        "data",
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
        "_default", "_min_docs", "_max_docs", "_max",

        "_docsrs", "_docsrs_stable",

        /* data capabilities */

        // "_data_all",

        // "_bit_all",
        "_-bit_any-_",
        "_bit_i8", "_bit_i16", "_bit_i32", "_bit_i64", "_bit_i128", "_bit_isize",
        "_bit_u8", "_bit_u16", "_bit_u32", "_bit_u64", "_bit_u128", "_bit_usize",

        // "_collections_all",
        "_-collections_any-_",
            // "_destaque_all",
            "_-destaque_any-_",
            "_destaque_u8", "_destaque_u16", "_destaque_u32", "_destaque_usize",
            // "_graph_all",
            "_-graph_any-_",
            "_graph_u8", "_graph_u16", "_graph_u32", "_graph_usize",
            // "_node_all",
            "_-node_any-_",
            "_node_u8", "_node_u16", "_node_u32", "_node_usize",
            // "_stack_all",
            "_-stack_any-_",
            "_stack_u8", "_stack_u16", "_stack_u32", "_stack_usize",

        // "_sort_all",
        "_-sort_any-_",
        "_sort_i8", "_sort_i16", "_sort_i32", "_sort_i64", "_sort_i128", "_sort_isize",
        "_sort_u8", "_sort_u16", "_sort_u32", "_sort_u64", "_sort_u128", "_sort_usize",
        "_sort_f32", "_sort_f64",

        "_tuple", "_tuple_24", "_tuple_36", "_tuple_48", "_tuple_72",

        /* lex capabilities */

        // "_lex_all",

        "_-string_any-_",
        "_-string_uany-_",
        // "_string_all",
        "_string_u8", "_string_u16", "_string_u32", "_string_usize",
        "_string_nonul",

        /* num capabilities */

        // "_cmp_all",
        "_-cmp_any-_",
        "_cmp_i8", "_cmp_i16", "_cmp_i32", "_cmp_i64", "_cmp_i128", "_cmp_isize",
        "_cmp_u8", "_cmp_u16", "_cmp_u32", "_cmp_u64", "_cmp_u128", "_cmp_usize",
        "_cmp_f32", "_cmp_f64",

        // "_num_all",
        "_-num_any-_",
        // "_float_all"
        "_-float_any-_",
        "_float_f32", "_float_f64",
        // "_int_all",
        "_-int_any-_",
        // "_int_iall",
        "_-int_iany-_",
        "_int_i8", "_int_i16", "_int_i32", "_int_i64", "_int_i128", "_int_isize",
        // "_int_uall",
        "_-int_uany-_",
        "_int_u8", "_int_u16", "_int_u32", "_int_u64", "_int_u128", "_int_usize",

        // "_niche_all",
        "_-niche_any-_",
        // "_non_value_all",
        "_-non_value_any-_",
        "_non_value_i8", "_non_value_i16", "_non_value_i32",
        "_non_value_i64", "_non_value_i128", "_non_value_isize",
        "_non_value_u8", "_non_value_u16", "_non_value_u32",
        "_non_value_u64", "_non_value_u128", "_non_value_usize",
        // "_non_range_all",
        "_-non_range_any-_",
        "_non_range_i8", "_non_range_i16", "_non_range_i32",
        "_non_range_i64", "_non_range_i128", "_non_range_isize",
        "_non_range_u8", "_non_range_u16", "_non_range_u32",
        "_non_range_u64", "_non_range_u128", "_non_range_usize",
        // "_range_all",
        "_-in_range_any-_",
        "_in_range_i8", "_in_range_i16", "_in_range_i32",
        "_in_range_i64", "_in_range_i128", "_in_range_isize",
        "_in_range_u8", "_in_range_u16", "_in_range_u32",
        "_in_range_u64", "_in_range_u128", "_in_range_usize",

    ];

    printfeat![msg: "Enabled dependency features:", features:
        // "dep_all",
        //     "dep_exec", "dep_lex", "dep_ui_term",
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
