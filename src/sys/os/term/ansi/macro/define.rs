// devela/src/sys/os/term/ansi/macro/define.rs
//
//! Defines the [`ansi!`] macro.
//

#[rustfmt::skip] // rustfmt gets confused
macro_rules! _ansi_define {
    (print $print_method:ident) => {
        $crate::macro_dollar! { ($d:tt) => { _ansi_define!(%emit ($d) print $print_method); }}
    };
    (fallback) => {
        $crate::macro_dollar! { ($d:tt) => { _ansi_define!(%emit ($d) fallback); }}
    };
    /* linux and std versions */
    (%emit ($d:tt) print $print_method:ident) => {
        #[doc = crate::_tags!(term)]
        /// Concatenates or prints [`Ansi`][crate::Ansi] escape codes.
        #[doc = crate::_doc_meta!{location("sys/os/term")}]
        #[doc = _docs::_DOC_ANSI!()]
        #[macro_export]
        #[cfg_attr(cargo_primary_package, doc(hidden))]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = "linux"))))]
        macro_rules! ansi· {
            // outputs a static byte slice
            (b: $d($command:ident $d(($d($arg:expr),*))? $d(,)?)+) => {{
                const BYTES: &'static [u8] = $d crate::paste! {
                    $d crate::const_join!(bytes:
                        $d( $d crate::Ansi::[<$d command:upper _B>] $d(($d($arg),*))? ,)+
                    )
                };
                BYTES
            }};
            // outputs a static string slice
            (s: $d($arg:tt)*) => {
                $d crate::Str::__utf8_bytes_to_str($d crate::ansi![b: $d($arg)*])
            };

            /* printing */

            // prints static commands (prints all commands concatenated)
            (p: $d($arg:tt)*) => {
                $d crate::$print_method($d crate::ansi![b: $d($arg)*])
            };
            // + unwrap()
            (p! $d($arg:tt)*) => {
                $d crate::ansi![p: $d($arg)*].unwrap()
            };
            // prints dynamic commands (prints each command immediately)
            (@p: $d($command:ident $d(($d($arg:expr),*))? $d(,)?)+) => {{
                let mut some_error = None;
                $d(
                    if some_error.is_none() {
                        match $d crate::$print_method($d crate::paste! {
                            &$d crate::Ansi::[<$d command:upper _B>] $d(($d($arg),*))?
                        }) {
                            Ok(_) => (),
                            Err(e) => some_error = Some(e),
                        }
                    }
                )+
                if let Some(e) = some_error { Err(e) } else { Ok(()) }
            }};
            // + unwrap()
            (@p! $d($arg:tt)*) => {
                $d crate::ansi![@p: $d($arg)*].unwrap()
            };

            /* rendering */

            // writes static commands into a renderer
            ($d dst:expr => $d($arg:tt)*) => {{
                ($d dst).try_push_bytes($d crate::ansi![b: $d($arg)*]).map(|_| ())
            }};
            // + unwrap()
            ($d dst:expr =>! $d($arg:tt)*) => {
                $d crate::ansi![$d dst => $d($arg)*].unwrap()
            };
            // + unwrap()
            (! $d dst:expr => $d($arg:tt)*) => {
                $d crate::ansi![$d dst => $d($arg)*].unwrap()
            };
            // writes dynamic commands into a renderer
            (@$d dst:expr => $d($command:ident $d(($d($arg:expr),*))? $d(,)?)+) => {{
                #[allow(unused_mut)]
                let mut __result = Ok(());
                $d(
                    if __result.is_ok() {
                        __result = $d crate::paste! {
                            ($d dst).try_push_bytes(
                                &$d crate::Ansi::[<$d command:upper _B>] $d(($d($arg),*))?
                            ).map(|_| ())
                        };
                    }
                )+
                __result
            }};
            // + unwrap()
            (@! $d dst:expr => $d($arg:tt)*) => {
                $d crate::ansi![@$d dst => $d($arg)*].unwrap()
            };


        }
        #[doc(inline)]
        pub use ansi· as ansi;
    };
    (%emit ($d:tt) fallback) => {
        #[doc = crate::_tags!(term)]
        /// Concatenates [`Ansi`][crate::Ansi] escape codes.
        #[doc = crate::_doc_meta!{location("sys/os/term")}]
        #[doc = _docs::_DOC_ANSI!()]
        #[macro_export]
        #[cfg_attr(cargo_primary_package, doc(hidden))]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = "linux"))))]
        macro_rules! ansi· {
            (b: $d($command:ident $d(($d($arg:expr),*))? $d(,)?)+) => {{
                const BYTES: &'static [u8] = $d crate::paste! {
                    $d crate::const_join!(bytes:
                        $d( $d crate::Ansi::[<$d command:upper _B>] $d(($d($arg),*))? ,)+
                    )
                };
                BYTES
            }};
            (s: $d($arg:tt)*) => {
                $d crate::Str::__utf8_bytes_to_str($d crate::ansi![b: $d($arg)*])
            };

            /* printing fallbacks */

            (p: $d($arg:tt)*) => {{
                let _ = $d crate::ansi![b: $d ($arg)*];
                Ok::<(), core::convert::Infallible>(())
            }};
            (p! $d($arg:tt)*) => {{
                $d crate::ansi![p: $d ($arg)*].unwrap()
            }};
            (@p: $d($command:ident $d(($d($arg:expr),*))? $d(,)?)+) => {{
                if false {
                    $d(
                        let _ = $crate::paste! {
                            &$d crate::Ansi::[<$command:upper _B>] $d (($d($arg),*))?
                        };
                    )+
                }
                Ok::<(), core::convert::Infallible>(())
            }};
            (@p! $d($arg:tt)*) => {{
                $d crate::ansi![@p: $d($arg)*].unwrap()
            }};

            /* rendering */

            // writes static commands into a renderer
            ($d dst:expr => $d($arg:tt)*) => {{
                ($d dst).try_push_bytes($d crate::ansi![b: $d($arg)*]).map(|_| ())
            }};
            // + unwrap()
            ($d dst:expr =>! $d($arg:tt)*) => {
                $d crate::ansi![$d dst => $d($arg)*].unwrap()
            };
            // + unwrap()
            (! $d dst:expr => $d($arg:tt)*) => {
                $d crate::ansi![$d dst => $d($arg)*].unwrap()
            };
            // writes dynamic commands into a renderer
            (@$d dst:expr => $d($command:ident $d(($d($arg:expr),*))? $d(,)?)+) => {{
                #[allow(unused_mut)]
                let mut __result = Ok(());
                $d(
                    if __result.is_ok() {
                        __result = $d crate::paste! {
                            ($d dst).try_push_bytes(
                                &$d crate::Ansi::[<$d command:upper _B>] $d(($d($arg),*))?
                            ).map(|_| ())
                        };
                    }
                )+
                __result
            }};
            // + unwrap()
            (@! $d dst:expr => $d($arg:tt)*) => {
                $d crate::ansi![@$d dst => $d($arg)*].unwrap()
            };
        }
        #[doc(inline)]
        pub use ansi· as ansi;
    };
}
pub(crate) use _ansi_define;
