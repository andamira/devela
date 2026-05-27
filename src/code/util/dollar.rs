// devela::code::util::dollar
//
//! Defines [`macro_dollar!`].
//

#[doc = crate::_tags!(code)]
/// Provides a literal `$` token to a macro body.
///
/// This is useful when a macro generates another `macro_rules!` macro
/// whose matcher or transcriber needs nested repetitions.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// The body is written as a temporary macro arm that receives the dollar token,
/// conventionally named `$d`.
///
/// Inside the body,
/// use `$d` wherever the generated macro must receive a literal `$`:
///
/// - `$d($item:tt)*` instead of `$($item:tt)*`
/// - `$d($item),+` instead of `$($item),+`
/// - `$d(,)?` instead of `$(,)?`
/// - `$d crate::path` instead of `$crate::path`,
///   when the generated macro must keep its own `$crate`.
///
/// This macro only supplies the dollar token.
/// Generated macro names still follow normal `macro_rules!` scope rules.
///
/// # Examples
///
/// Defines a macro that captures any token sequence:
/// ```
/// # use devela::macro_dollar;
/// macro_dollar! { ($d:tt) => {
///     macro_rules! capture_tokens {
///         ($d($item:tt)*) => {
///             stringify!($d($item)*)
///         };
///     }
/// }}
/// assert_eq!(capture_tokens!(a + b), "a + b");
/// ```
///
/// Defines a macro with a repeated matcher and optional trailing comma:
/// ```
/// # use devela::macro_dollar;
/// macro_dollar! { ($d:tt) => {
///     macro_rules! capture_idents {
///         ($d($item:ident),+ $d(,)?) => {
///             stringify!($d($item),+)
///         };
///     }
/// }}
/// assert_eq!(capture_idents!(a, b, c,), "a, b, c");
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! macro_dollar {
    ($($body:tt)*) => {
        macro_rules! __macro_dollar_289B9D1C9f3a7c2b { $($body)* }
        __macro_dollar_289B9D1C9f3a7c2b! { $ }
    };
}
#[doc(inline)]
pub use macro_dollar;

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    const MACRO_DOLLAR_SENTINEL: usize = 123;

    #[test]
    fn defines_nested_repetition_macro() {
        macro_dollar! { ($d:tt) => {
            macro_rules! count_tts {
                () => { 0usize };
                ($d head:tt $d($tail:tt)*) => {
                    1usize + count_tts!($d($tail)*)
                };
            }
        }}
        assert_eq!(count_tts!(a b c), 3);
    }
    #[test]
    fn supports_use_or_shim_shape() {
        macro_rules! make_shim {
            ($($name:ident),+ $(,)?) => {
                macro_dollar! { ($d:tt) => {
                    $(
                        make_shim![%($d) $name];
                    )+
                }}
            };
            (%($_d:tt) $name:ident) => {
                macro_rules! $name {
                    ($_d($tt:tt)*) => {
                        stringify!($_d($tt)*)
                    };
                }
            };
        }
        make_shim!(shim_a, shim_b);
        assert_eq!(shim_a!(one two), "one two");
        assert_eq!(shim_b!(three), "three");
    }
    #[test]
    fn block_scope_distinct_names_work() {
        macro_dollar! { ($d:tt) => {
            macro_rules! probe_outer {
                ($d($t:tt)*) => {
                    (1usize, stringify!($d($t)*))
                };
            }
        }}
        assert_eq!(probe_outer!(a b), (1, "a b"));
        {
            macro_dollar! { ($d:tt) => {
                macro_rules! probe_inner {
                    ($d($t:tt)*) => {
                        (2usize, stringify!($d($t)*))
                    };
                }
            }}
            assert_eq!(probe_inner!(c d), (2, "c d"));
            assert_eq!(probe_outer!(e f), (1, "e f"));
        }
        assert_eq!(probe_outer!(g h), (1, "g h"));
    }
    #[test]
    fn works_inside_outer_macro_repetition() {
        macro_rules! make_many {
            ($($name:ident => $value:expr),+ $(,)?) => {
                macro_dollar! { ($d:tt) => {
                    $(
                        macro_rules! $name {
                            () => { $value };
                            ($d($t:tt)*) => {
                                ($value, stringify!($d($t)*))
                            };
                        }
                    )+
                }}
            };
        }
        make_many! {
            many_a => 10usize,
            many_b => 20usize,
            many_c => 30usize,
        }
        assert_eq!(many_a!(), 10);
        assert_eq!(many_b!(x y), (20, "x y"));
        assert_eq!(many_c!([z]), (30, "[z]"));
    }
    #[test]
    fn can_generate_macro_that_generates_macro() {
        macro_dollar! { ($d:tt) => {
            macro_rules! make_inner {
                ($name:ident) => {
                    macro_dollar! { ($d2:tt) => {
                        macro_rules! $name {
                            ($d2($t:tt)*) => {
                                stringify!($d2($t)*)
                            };
                        }
                    }}
                };
            }
        }}
        make_inner!(generated_inner);
        assert_eq!(generated_inner!(a b c), "a b c");
    }
    #[test]
    fn supports_item_scope_visibility_reexport() {
        mod local_mod {
            macro_dollar! { ($d:tt) => {
                macro_rules! exported_probe {
                    ($d($t:tt)*) => {
                        stringify!($d($t)*)
                    };
                }
                pub(super) use exported_probe;
            }}
        }
        assert_eq!(local_mod::exported_probe!(hello world), "hello world");
    }
    #[test]
    fn supports_macro_defined_inside_function_scope() {
        fn make_value() -> usize {
            macro_dollar! { ($d:tt) => {
                macro_rules! local_value {
                    () => { 42usize };
                    ($d($t:tt)*) => { 7usize };
                }
            }}
            local_value!() + local_value!(ignored tokens)
        }
        assert_eq!(make_value(), 49);
    }
    #[test]
    fn preserves_outer_macro_crate_context() {
        macro_rules! make_crate_context_probe {
            ($name:ident) => {
                macro_dollar! { ($d:tt) => {
                    macro_rules! $name {
                        () => {
                            $crate::code::util::dollar::tests::MACRO_DOLLAR_SENTINEL
                        };
                        ($d($t:tt)*) => {
                            stringify!($d($t)*)
                        };
                    }
                }}
            };
        }
        make_crate_context_probe!(crate_context_probe);
        assert_eq!(crate_context_probe!(), 123);
        assert_eq!(crate_context_probe!(a b), "a b");
    }
    #[test]
    fn generated_macro_can_call_another_generated_macro() {
        macro_dollar! { ($d:tt) => {
            macro_rules! base_probe {
                ($d($t:tt)*) => {
                    stringify!($d($t)*)
                };
            }
            macro_rules! wrapper_probe {
                ($d($t:tt)*) => {
                    base_probe!($d($t)*)
                };
            }
        }}
        assert_eq!(wrapper_probe!(a b c), "a b c");
    }
    #[test]
    fn supports_match_arm_expansion_context() {
        macro_dollar! { ($d:tt) => {
            macro_rules! make_match {
                ($value:expr; $d($pat:pat => $out:expr),+ $d(,)?) => {
                    match $value {
                        $d($pat => $out,)+
                    }
                };
            }
        }}
        let value = make_match! {
            2u8;
            0 => "zero",
            1 => "one",
            2 => "two",
            _ => "many",
        };
        assert_eq!(value, "two");
    }
    #[test]
    fn supports_type_and_expr_positions_from_same_generated_macro() {
        macro_dollar! { ($d:tt) => {
            macro_rules! choose {
                (type u8  $d($rest:tt)*) => { u8 };
                (type u16 $d($rest:tt)*) => { u16 };
                (expr $value:expr; $d($rest:tt)*) => { $value };
            }
        }}
        type A = choose!(type u8 trailing);
        type B = choose!(type u16 trailing);
        let a: A = choose!(expr 3u8; ignored);
        let b: B = choose!(expr 5u16; ignored);
        assert_eq!(a, 3);
        assert_eq!(b, 5);
    }
    #[test]
    fn can_be_used_from_an_exported_style_macro() {
        macro_rules! public_style_generator {
            ($name:ident) => {
                macro_dollar! { ($d:tt) => {
                    macro_rules! $name {
                        ($d($item:tt)*) => {
                            stringify!($d($item)*)
                        };
                    }
                }}
            };
        }
        public_style_generator!(public_style_probe);
        assert_eq!(
            public_style_probe!(
                struct Thing;
            ),
            "struct Thing;"
        );
    }
    #[test]
    fn two_generators_in_same_scope_do_not_conflict() {
        macro_rules! gen_a {
            () => {
                macro_dollar! { ($d:tt) => {
                    macro_rules! generated_a {
                        ($d($t:tt)*) => {
                            (1usize, stringify!($d($t)*))
                        };
                    }
                }}
            };
        }
        macro_rules! gen_b {
            () => {
                macro_dollar! { ($d:tt) => {
                    macro_rules! generated_b {
                        ($d($t:tt)*) => {
                            (2usize, stringify!($d($t)*))
                        };
                    }
                }}
            };
        }
        gen_a!();
        gen_b!();
        assert_eq!(generated_a!(a b c), (1, "a b c"));
        assert_eq!(generated_b!(x y z), (2, "x y z"));
    }
    #[test]
    fn repeated_direct_invocations_in_same_scope_do_not_conflict() {
        macro_dollar! { ($d:tt) => {
            macro_rules! first_generated {
                ($d($t:tt)*) => {
                    stringify!($d($t)*)
                };
            }
        }}
        macro_dollar! { ($d:tt) => {
            macro_rules! second_generated {
                ($d($t:tt)*) => {
                    stringify!($d($t)*)
                };
            }
        }}
        macro_dollar! { ($d:tt) => {
            macro_rules! third_generated {
                ($d($t:tt)*) => {
                    stringify!($d($t)*)
                };
            }
        }}
        assert_eq!(first_generated!(one), "one");
        assert_eq!(second_generated!(two), "two");
        assert_eq!(third_generated!(three), "three");
    }
}
