// devela::os::macros
//
//! OS related macros.
//

use crate::codegen::paste;

// Generates the very similar macros: os_print, os_println, os_eprint, os_eprintln.
macro_rules! generate_os_print_macros {
    () => {
        generate_os_print_macros![$ print,
            "Prints to the standard output" ];
        generate_os_print_macros![$ println,
            "Prints to the standard output, with a newline," ];
        generate_os_print_macros![$ eprint,
            "Prints to the standard error" ];
        generate_os_print_macros![$ eprintln,
            "Prints to the standard error, with a newline," ];
    };
    // # Arguments
    // $d:     the dollar sign passed as a token. This is a trick to be able to nest repetitions.
    //         WAITING: https://github.com/rust-lang/rust/issues/83527 to be able to use $$
    //         SOURCE: https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
    //
    // $name:  the suffix of the generated macro (os_ + $name).
    //         It is also the name of the called function
    //         and the name of the similar std macro.
    //
    // $doc:   the first line of documentation
    //
    ($d:tt $name:ident, $doc:literal) => { paste! {
        #[doc = $doc]
        #[doc = " using `os::linux::io::`[`" $name "`][super::linux::" $name "]."]
        #[doc = ""]
        #[doc = "Usage is similar to `std::`[`" $name "!`], with the addition of a byte buffer."]
        #[doc = ""]
        /// # Examples
        /// ```
        #[doc = "use devela::os::" [<os_ $name>] ";"]
        ///
        #[doc = [<os_ $name>]"!(); // print a blank line"]
        #[doc = [<os_ $name>]r#"!("hello world 1!"); // print a literal"#]
        #[doc = [<os_ $name>]r#"!(buf2=16); // create a buffer of 16 bytes"#]
        #[doc = [<os_ $name>]r#"!(buf2, "hello world {}!", 2); // formatted print using the buffer"#]
        #[doc = [<os_ $name>]r#"!(buf3=16, "hello world {}!", 3); // create a buffer and print"#]
        #[doc = [<os_ $name>]r#"!(buf2, "hello world {}!", 4); // reuse any buffer in scope"#]
        /// for n in 5..8 {
        #[doc = "    "[<os_ $name>]r#"!(buf3, "hello world {n}!");"#]
        /// }
        /// ```
        ///
        /// Error Handling
        /// If the write fails, it prints an error message and exits with status code 10.
        #[macro_export]
        #[cfg(all(
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "arm",
                target_arch = "aarch64",
                target_arch = "riscv32",
                target_arch = "riscv64"
            ),
            feature = "unsafe_os",
            not(miri),
        ))]
        #[cfg_attr(
            feature = "nightly",
            doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
        )]
        macro_rules! [<os_ $name>] {
            // print a newline
            () => {
                $crate::os::linux::$name("");
            };

            // print a literal
            ($str:literal) => {
                $crate::os::linux::$name($str);
            };

            // create a buffer of the given size
            ($buf:ident = $len:literal) => {
                let mut $buf = [0u8; $len];
            };

            // create a buffer and print
            ($buf:ident = $len:literal, $d($d args:tt)*) => {
                [<os_ $name>]![$buf = $len];

                [<os_ $name>]![$buf, $d($d args)*];
            };

            // print using an existing buffer
            ($buf:ident, $d($d args:tt)*) => {
                $crate::os::linux::println(
                    $crate::fmt::format_buf![&mut $buf, $d($d args)*].unwrap()
                );
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
