// devela::sys::os::print::linux

/// Generates os_linux_* print macros for `no_std` linux using syscalls.
macro_rules! generate_os_linux_print_macros {
    () => {
        generate_os_linux_print_macros![
            $print + print,
            "Prints to the standard output.",
            "nothing",
            ""
        ];
        generate_os_linux_print_macros![
            $println + print,
            "Prints to the standard output, with a newline.",
            "a newline",
            "\n\n"
        ];
        generate_os_linux_print_macros![
            $eprint + eprint,
            "Prints to the standard error.",
            "nothing",
            ""
        ];
        generate_os_linux_print_macros![
            $eprintln + eprint,
            "Prints to the standard error, with a newline.",
            "a newline",
            "\n\n"
        ];
    };

    // # arguments:
    //
    // $d:       the dollar sign passed as a token. This is a trick to be able to nest repetitions.
    //           WAIT: https://github.com/rust-lang/rust/issues/83527 to be able to use $$
    //           IDEA: https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
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
    ($d:tt $name:ident + $name2:ident, $doc:literal, $doc_ln:literal, $newline:literal $(,)?
    ) => {
        $crate::paste! {
            generate_os_linux_print_macros!(@[<os_ $name>], [<_os_ $name>],
                $d $name + $name2, $doc, $doc_ln, $newline);
        }
    };
    (@$os_name:ident, $_os_name:ident,
    $d:tt $name:ident + $name2:ident, $doc:literal, $doc_ln:literal, $newline:literal $(,)?
    ) => {
        $crate::paste! {
            #[doc = $doc]
            #[doc = "\n\nLeverages [`Linux::" $name "`][crate::Linux::" $name "]"]
            #[doc = ", [`format_buf`][crate::format_buf] and [`join`][crate::join]."]
            ///
            #[doc = "Usage is similar but not equal to `std::`[`" $name "!`]."]
            ///
            /// # Examples
            /// ```
            #[doc = "use devela::{" [<os_ $name>] "};"] // TODO:ansi
            ///
            #[doc = [<os_ $name>]"!(); // prints " $doc_ln " (1st arm)"]
            #[doc = [<os_ $name>]r#"!("hello world! 2nd arm"); // one literal"#]
            #[doc = [<os_ $name>]r#"!("hello", " world! ", "3", "rd arm"); // many literals"#]
            ///
            #[doc = [<os_ $name>]r#"!(buf_a=30); // create a buffer of some byte lenght (4th arm)"#]
            #[doc = [<os_ $name>]r#"!(buf_a, "hello world! {}th arm", 5);"#
                " // formatted print using the buffer"]
            #[doc = [<os_ $name>]r#"!(buf_b=20, "hello world! {}th arm", 6);"#
                "// create a buffer and use it to print"]
            ///
            #[doc = [<os_ $name>]r#"!(&format!["{} {}! {}th arm", "hello", "world", 7]);"#
                " // one &str expresion"]
            // #[doc = [<os_ $name>]r#"!(ansi![red], "hello", &format!["{}! {}th arm", "world", 8]);"#
            //     " // many &str expressions"] // TODO:ansi
            /// ```
            ///
            /// Output:
            /// <pre>
            #[doc = $newline "hello world! 2nd arm"]
            ///hello world! 3rd arm
            ///hello world! 5th arm
            ///hello world! 6th arm
            ///hello world! 7th arm
            // ///<span style="color: red;">hello world! 8th arm</span></pre> // TODO:ansi
            ///</pre>
            #[macro_export]
            #[doc(hidden)]
            macro_rules! $_os_name {
                // 1) print a newline (or nothing)
                () => {
                    $crate::Linux::$name("");
                };

                // 2) print a literal
                ($str:literal) => {
                    $crate::Linux::$name($str);
                };

                // 3) print concatenated literals
                ($d($d str:literal),+ $d(,)?) => {
                    $crate::Linux::$name($crate::join!(str: $d($d str,)+));
                };

                // 4) create a buffer of the given length
                ($buf:ident = $len:literal) => {
                    let mut $buf = [0u8; $len];
                };

                // 5) print formatted args using an existing buffer
                ($buf:ident, $d($d args:tt)*) => {
                    // NOTE: truncates without failing in case of buffer overflow
                    $crate::Linux::$name($crate::format_buf![? &mut $buf, $d($d args)*]);
                };

                // 6) create a buffer of the given lenght and print formatted args
                ($buf:ident = $len:literal, $d($d args:tt)*) => {
                    $os_name![$buf = $len]; // call 4th arm
                    $os_name![$buf, $d($d args)*]; // call 5th arm
                };

                /* these two must be the last ones to avoid conflicts */

                // 7) print an expression that returns a string slice
                ($expr:expr) => {
                    $crate::Linux::$name($expr);
                };

                // 8) print concatenated expressions that returns string slices
                ($d($d expr:expr),+ $d(,)?) => {
                    // $d( $name2!["{}", $d expr]; )+
                    // $name![""];
                    $d( $crate::Linux::$name2($d expr); )+
                    $crate::Linux::$name("");
                };
            }
            #[doc(inline)]
            #[cfg_attr(nightly_doc, doc(cfg(any(
                feature = "std",
                all(feature = "linux", feature = "unsafe_syscall"),
            ))))]
            pub use $_os_name as $os_name;
        }
    };
}
pub(crate) use generate_os_linux_print_macros;
