// devela build script
//
//! Shows debugging information if the `__debug` feature is enabled.
//

#![allow(dead_code, unused)]

#[cfg(feature = "__debug")]
fn main() {
    /* information about the enabled features */

    printfeat![msg: "Enabled miscellaneous features:", features:
        "default", "__debug", "__excluded",
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
        "num", "_-num-_", "num_float", "num_geom", "num_int",
        "rend", "_-rend-_", "rend_audio", "rend_color", "rend_image",
        "sys",
        "time",
        "ui", "_-ui-_", "ui_term",
    ];

    printfeat![msg: "Enabled *nightly* features:", features:
        "nightly", "_-nightly-_", "nightly_coro", "nightly_doc", "nigthly_simd",
    ];

    printfeat![msg: "Enabled capability features:", features:
        "_default", "_max",
        "_docsrs", "_docsrs_max", "_docsrs_stable",
        "_i8", "_i16", "_i32", "_i64", "_i128", "_isize",
        "_u8", "_u16", "_u32", "_u64", "_u128", "_usize",
        "_f32", "_f64",
        "_-nums-_", "_-ints-_", "_-sints-_", "_-uints-_", "_-floats-_",
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
#[cfg(not(feature = "__debug"))]
fn main() {}

// private helpers
#[rustfmt::skip] fn println(msg: &str) { println!("cargo:warning={}", msg); }
macro_rules! printfeat {
    // if any of the features are enabled, prints the msg and the features
    (msg: $msg:literal, features: $($feature:literal),+ $(,)?) => {
        #[cfg(feature = "__debug")]
        {
            if cfg!(any($(feature = $feature),+)) { println($msg); }
            $( if cfg!(feature = $feature) { println(&format!["  {}", $feature]); } )+
        }
    };
}
use printfeat;
