// devela::os::macros
//
//! OS related macros.
//

use crate::codegen::paste;

// Generates the macros:
// - os_print
// - os_println
// - os_eprint
// - os_eprintln
macro_rules! generate_os_print_macros {
    () => {
        generate_os_print_macros![$ print,
            "Prints to the standard output.",
            "nothing"
        ];
        generate_os_print_macros![$ println,
            "Prints to the standard output, with a newline.",
            "a newline"
        ];
        generate_os_print_macros![$ eprint,
            "Prints to the standard error.",
            "nothing"
        ];
        generate_os_print_macros![$ eprintln,
            "Prints to the standard error, with a newline.",
            "a newline"
        ];
    };

    // # arguments:
    //
    // $d:      the dollar sign passed as a token. This is a trick to be able to nest repetitions.
    //          WAITING: https://github.com/rust-lang/rust/issues/83527 to be able to use $$
    //          SOURCE: https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
    //
    // $name:   the suffix of the generated macro (os_ + $name).
    //          It is also the name of the called function
    //          and the name of the similar std macro.
    //
    // $doc:    the first line of documentation
    // $doc_ln: documentation dependent on the `ln` variant.
    //
    ($d:tt $name:ident, $doc:literal, $doc_ln:literal $(,)?) => { paste! {
        #[doc = $doc]
        #[doc = "\n\nLeverages [`" [<linux_ $name>] "`][super::linux::" [<linux_ $name>] "]"]
        #[doc = ", [`format_buf`][crate::fmt::format_buf]"]
        #[doc = "and [`str_concat`][crate::str::str_concat]."]
        ///
        #[doc = "Usage is similar but not equal to `std::`[`" $name "!`]."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::os::" [<os_ $name>] ";"]
        ///
        #[doc = [<os_ $name>]"!(); // prints " $doc_ln " (1st arm)"]
        #[doc = [<os_ $name>]r#"!("hello world! 2nd arm"); // a literal"#]
        #[doc = [<os_ $name>]r#"!("hello", " world! ", 3_i32, 'r', "d arm"); // concatenated literals"#]
        #[doc = [<os_ $name>]r#"!(buf_a=30); // create a buffer with a byte lenght (4th arm)"#]
        #[doc = [<os_ $name>]r#"!(buf_a, "hello world! {}th arm", 5); // formatted print using the buffer"#]
        #[doc = [<os_ $name>]r#"!(buf_b=20, "hello world! {}th arm", 6); // create the buffer and print"#]
        #[doc = [<os_ $name>]r#"!(&format!["{} {}! {}th arm", "hello", "world", 7]);"#
            " // an expresion returning a &str"]
        /// ```
        ///
        /// # Error Handling
        /// If the write fails, it prints an error message and exits with status code 10.
        #[macro_export]
        #[cfg(all(
            any(
                target_arch = "x86_64", target_arch = "x86", target_arch = "arm",
                target_arch = "aarch64", target_arch = "riscv32", target_arch = "riscv64"
            ),
            feature = "unsafe_os",
            not(miri),
        ))]
        #[cfg_attr(
            feature = "nightly",
            doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
        )]
        macro_rules! [<os_ $name>] {
            // 1) print a newline
            () => {
                $crate::os::linux::[<linux_ $name>]("");
            };

            // 2) print a literal
            ($str:literal) => {
                $crate::os::linux::[<linux_ $name>]($str);
            };

            // 3) print concatenated literals
            ($d($d str:literal),+ $d(,)?) => {
                $crate::os::linux::[<linux_ $name>](
                    $crate::str::str_concat!($d($d str,)+)
                );
            };

            // 4) create a buffer of the given length
            ($buf:ident = $len:literal) => {
                let mut $buf = [0u8; $len];
            };

            // 5) print formatted args using an existing buffer
            ($buf:ident, $d($d args:tt)*) => {
                $crate::os::linux::[<linux_ $name>](
                    $crate::fmt::format_buf![&mut $buf, $d($d args)*].unwrap()
                );
            };

            // 6) create a buffer of the given lenght and print formatted args
            ($buf:ident = $len:literal, $d($d args:tt)*) => {
                [<os_ $name>]![$buf = $len]; // call 4th arm
                [<os_ $name>]![$buf, $d($d args)*]; // call 5th arm
            };

            // 7) print an expression that returns a string slice
            // (this must be the last arm to avoid conflicts)
            ($expr:expr) => {
                $crate::os::linux::[<linux_ $name>]($expr);
            };
        }

        #[doc(inline)]
        #[cfg(all(
            any(
                target_arch = "x86_64", target_arch = "x86", target_arch = "arm",
                target_arch = "aarch64", target_arch = "riscv32", target_arch = "riscv64"
            ),
            feature = "unsafe_os",
            not(miri),
        ))]
        #[cfg_attr(
            feature = "nightly",
            doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
        )]
        pub use [<os_ $name>];
    }};
}
generate_os_print_macros![];
