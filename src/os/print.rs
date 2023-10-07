// devela::os::print
//
//! OS print related macros.
//!
//! Generates the macros: `os_print`, `os_println`, `os_eprint`, `os_eprintln`.
//
// NOTE: It's necessary to duplicate the macros because the `cfg` attribute
// checks the features of the current crate, not the current library.

#[cfg(all(feature = "std", feature = "depend"))]
macro_rules! generate_os_print_std_macros {
    () => {
        generate_os_print_std_macros![
            $print + print,
            "Prints to the standard output.",
            "nothing",
            ""
        ];
        generate_os_print_std_macros![
            $println + print,
            "Prints to the standard output, with a newline.",
            "a newline",
            "\n\n"
        ];
        generate_os_print_std_macros![
            $eprint + eprint,
            "Prints to the standard error.",
            "nothing",
            ""
        ];
        generate_os_print_std_macros![
            $eprintln + eprint,
            "Prints to the standard error, with a newline.",
            "a newline",
            "\n\n"
        ];
    };

    // # arguments:
    //
    // $d:       the dollar sign passed as a token. This is a trick to be able to nest repetitions.
    //           WAITING: https://github.com/rust-lang/rust/issues/83527 to be able to use $$
    //           SOURCE: https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
    //
    // $name:    the suffix of the generated macro (os_ $name).
    //           It is also the name of the called function
    //           and the name of the similar std macro.
    //
    // +$name2:  the name of the called function without an `ln` suffix (used for the 8th arm)
    //
    // $doc:     the first line of documentation
    // $doc_ln:  documentation dependent on the `ln` variant.
    // $newline: a newline, or nothing, depending on the `ln` variant.
    //
    (
        $d:tt $name:ident + $name2:ident, $doc:literal, $doc_ln:literal, $newline:literal $(,)?
    ) => {
        $crate::codegen::paste! {
            #[doc = $doc]
            #[doc = "\n\nLeverages [`" [<linux_ $name>] "`][super::linux::" [<linux_ $name>] "]"]
            #[doc = ", [`format_buf`][crate::fmt::format_buf]"]
            #[doc = "and [`str_concat`][crate::str::str_concat]."]
            ///
            #[doc = "Usage is similar but not equal to `std::`[`" $name "!`]."]
            ///
            /// # Examples
            /// ```
            #[doc = "use devela::os::{ansi, " [<os_ $name>] "};"]
            ///
            #[doc = [<os_ $name>]"!(); // prints " $doc_ln " (1st arm)"]
            #[doc = [<os_ $name>]r#"!("hello world! 2nd arm"); // one literal"#]
            #[doc = [<os_ $name>]r#"!("hello", " world! ", 3_i32, 'r', "d arm"); // many literals"#]
            ///
            #[doc = [<os_ $name>]r#"!(buf_a=30); // create a buffer of some byte lenght (4th arm)"#]
            #[doc = [<os_ $name>]r#"!(buf_a, "hello world! {}th arm", 5);"#
                " // formatted print using the buffer"]
            #[doc = [<os_ $name>]r#"!(buf_b=20, "hello world! {}th arm", 6);"#
                "// create a buffer and print"]
            ///
            #[doc = [<os_ $name>]r#"!(&format!["{} {}! {}th arm", "hello", "world", 7]);"#
                " // one &str expresion"]
            #[doc = [<os_ $name>]r#"!(ansi![red], "hello", &format!["{}! {}th arm", "world", 8]);"#
                " // many &str expressions"]
            /// ```
            ///
            /// Output:
            /// <pre>
            #[doc = $newline "hello world! 2nd arm"]
            ///hello world! 3rd arm
            ///hello world! 5th arm
            ///hello world! 6th arm
            ///hello world! 7th arm
            ///<span style="color: red;">hello world! 8th arm</span></pre>
            ///
            /// # Error Handling
            /// If the write fails, it prints an error message and exits with status code 10.
            #[macro_export]
            #[cfg(all(feature = "std", feature = "depend"))]
            #[cfg_attr(
                feature = "nightly",
                doc(cfg(any(feature = "std", feature = "depend", feature = "linux_unsafe")))
            )]
            macro_rules! [<os_ $name>] {
                // 1) print a newline (or nothing)
                () => {
                    $name![""];
                };

                // 2) print a literal
                ($str:literal) => {
                    $name![$str];
                };

                // FIXME
                // // 3) print concatenated literals
                // ($d($d str:literal),+ $d(,)?) => {
                //     $name!["{}", $crate::str::str_concat!($d($d str,)+) ];
                // };

                // 4) create a buffer of the given length
                ($buf:ident = $len:literal) => {
                    let mut $buf = [0u8; $len];
                };

                // 5) print formatted args using an existing buffer
                ($buf:ident, $d($d args:tt)*) => {
                    $name!["{}", $crate::fmt::format_buf![&mut $buf, $d($d args)*].unwrap()];
                };

                // 6) create a buffer of the given lenght and print formatted args
                ($buf:ident = $len:literal, $d($d args:tt)*) => {
                    [<os_ $name>]![$buf = $len]; // call 4th arm
                    [<os_ $name>]![$buf, $d($d args)*]; // call 5th arm
                };

                /* these two must be the last ones to avoid conflicts */

                // 7) print an expression that returns a string slice
                ($expr:expr) => {
                    $name!["{}", $expr];
                };

                // 8) print concatenated expressions that returns string slices
                ($d($d expr:expr),+ $d(,)?) => {
                    $d( $name2!["{}", $d expr]; )+
                    $name![""];
                };
            }

            #[doc(inline)]
            #[cfg(feature = "std")]
            pub use [<os_ $name>];
        }
    };
}
#[cfg(all(feature = "std", feature = "depend"))]
generate_os_print_std_macros![];

#[cfg(all(
    any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv32",
        target_arch = "riscv64"
    ),
    feature = "depend",
    feature = "linux",
    feature = "unsafe_linux",
    not(feature = "std"),
    not(miri),
))]
macro_rules! generate_os_print_linux_macros {
    () => {
        generate_os_print_linux_macros![
            $print + print,
            "Prints to the standard output.",
            "nothing",
            ""
        ];
        generate_os_print_linux_macros![
            $println + print,
            "Prints to the standard output, with a newline.",
            "a newline",
            "\n\n"
        ];
        generate_os_print_linux_macros![
            $eprint + eprint,
            "Prints to the standard error.",
            "nothing",
            ""
        ];
        generate_os_print_linux_macros![
            $eprintln + eprint,
            "Prints to the standard error, with a newline.",
            "a newline",
            "\n\n"
        ];
    };

    // # arguments:
    //
    // $d:       the dollar sign passed as a token. This is a trick to be able to nest repetitions.
    //           WAITING: https://github.com/rust-lang/rust/issues/83527 to be able to use $$
    //           SOURCE: https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
    //
    // $name:    the suffix of the generated macro (os_ $name).
    //           It is also the name of the called function
    //           and the name of the similar std macro.
    //
    // +$name2:  the name of the called function without an `ln` suffix (used for the 8th arm)
    //
    // $doc:     the first line of documentation
    // $doc_ln:  documentation dependent on the `ln` variant.
    // $newline: a newline, or nothing, depending on the `ln` variant.
    //
    (
        $d:tt $name:ident + $name2:ident, $doc:literal, $doc_ln:literal, $newline:literal $(,)?
    ) => {
        $crate::codegen::paste! {
            #[doc = $doc]
            #[doc = "\n\nLeverages [`" [<linux_ $name>] "`][super::linux::" [<linux_ $name>] "]"]
            #[doc = ", [`format_buf`][crate::fmt::format_buf]"]
            // #[doc = "and [`str_concat`][crate::str::str_concat]."] // FIXME
            ///
            #[doc = "Usage is similar but not equal to `std::`[`" $name "!`]."]
            ///
            /// # Examples
            /// ```
            #[doc = "use devela::os::{ansi, " [<os_ $name>] "};"]
            ///
            #[doc = [<os_ $name>]"!(); // prints " $doc_ln " (1st arm)"]
            #[doc = [<os_ $name>]r#"!("hello world! 2nd arm"); // one literal"#]
            #[doc = [<os_ $name>]r#"!("hello", " world! ", 3_i32, 'r', "d arm"); // many literals"#]
            ///
            #[doc = [<os_ $name>]r#"!(buf_a=30); // create a buffer of some byte lenght (4th arm)"#]
            #[doc = [<os_ $name>]r#"!(buf_a, "hello world! {}th arm", 5);"#
                " // formatted print using the buffer"]
            #[doc = [<os_ $name>]r#"!(buf_b=20, "hello world! {}th arm", 6);"#
                "// create a buffer and print"]
            ///
            #[doc = [<os_ $name>]r#"!(&format!["{} {}! {}th arm", "hello", "world", 7]);"#
                " // one &str expresion"]
            #[doc = [<os_ $name>]r#"!(ansi![red], "hello", &format!["{}! {}th arm", "world", 8]);"#
                " // many &str expressions"]
            /// ```
            ///
            /// Output:
            /// <pre>
            #[doc = $newline "hello world! 2nd arm"]
            ///hello world! 3rd arm
            ///hello world! 5th arm
            ///hello world! 6th arm
            ///hello world! 7th arm
            ///<span style="color: red;">hello world! 8th arm</span></pre>
            ///
            /// # Error Handling
            /// If the write fails, it prints an error message and exits with status code 10.
            #[macro_export]
            #[cfg(all(
                any(
                    target_arch = "x86_64", target_arch = "x86", target_arch = "arm",
                    target_arch = "aarch64", target_arch = "riscv32", target_arch = "riscv64"
                ),
                feature = "depend",
                feature = "linux",
                feature = "unsafe_linux",
                not(miri),
            ))]
            #[cfg_attr(
                feature = "nightly",
                doc(cfg(any(feature = "std", all(feature = "depend", feature = "linux", feature = "unsafe_linux"))))
            )]
            macro_rules! [<os_ $name>] {
                // 1) print a newline (or nothing)
                () => {
                    $crate::os::linux::[<linux_ $name>]("");
                };

                // 2) print a literal
                ($str:literal) => {
                    $crate::os::linux::[<linux_ $name>]($str);
                };

                // FIXME
                // // 3) print concatenated literals
                // ($d($d str:literal),+ $d(,)?) => {
                //     $crate::os::linux::[<linux_ $name>](
                //         $crate::str::str_concat!($d($d str,)+)
                //     );
                // };

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

                // 8) print concatenated expressions that returns string slices
                ($d($d expr:expr),+ $d(,)?) => {
                    $d( $crate::os::linux::[<linux_ $name2>]($d expr); )+
                    $crate::os::linux::[<linux_ $name>]("") // newline or nothing
                };
            }

            #[doc(inline)]
            #[cfg(all(
                any(
                    target_arch = "x86_64", target_arch = "x86", target_arch = "arm",
                    target_arch = "aarch64", target_arch = "riscv32", target_arch = "riscv64"
                ),
                feature = "linux",
                feature = "unsafe_linux",
                not(miri),
            ))]
            pub use [<os_ $name>];
        }
    };
}
#[cfg(all(
    any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv32",
        target_arch = "riscv64"
    ),
    feature = "depend",
    feature = "linux",
    feature = "unsafe_linux",
    not(feature = "std"),
    not(miri),
))]
generate_os_print_linux_macros![];
