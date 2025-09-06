// devela_base_core::code::util::cfg_if
//
//! Defines the [`cfg_if!`] macro.
//

/// A macro for defining `#[cfg]` if-else statements.
///
/// Allows definition of a cascade of `#[cfg]` cases,
/// emitting the implementation which matches first.
///
/// # Example
/// ```
/// # use devela_base_core::cfg_if;
/// cfg_if! {
///     if #[cfg(unix)] {
///         fn foo() { /* unix specific functionality */ }
///     } else if #[cfg(target_pointer_width = "32")] {
///         fn foo() { /* non-unix, 32-bit functionality */ }
///     } else {
///         fn foo() { /* fallback implementation */ }
///     }
///
///     // there can be multiple conditions
///     if #[cfg(feature = "bar")] {
///         fn bar() {}
///     }
/// }
/// ```
#[doc = crate::_doc!(vendor: "cfg-if")] // FIXME
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! cfg_if {
    // match if/else chains with a final `else`
    ( $( if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* } ) else+
      else { $( $e_tokens:tt )* }
    ) => {
        $crate::cfg_if! {
            @__items () ;
            $( (( $i_meta ) ( $( $i_tokens )* )) , )+
            (() ( $( $e_tokens )* )) ,
        }
    };
    // allow multiple conditions
    ( $( if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* } ) else+
      else { $( $e_tokens:tt )* }
      if $($extra_conditions:tt)+
    ) => {
        $crate::cfg_if! {
            @__items () ;
            $( (( $i_meta ) ( $( $i_tokens )* )) , )+
            (() ( $( $e_tokens )* )) ,
        }
        $crate::cfg_if! { if $($extra_conditions)+ }
    };

    // match if/else chains lacking a final `else`
    ( if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
      $( else if #[cfg( $e_meta:meta )] { $( $e_tokens:tt )* } )*
    ) => {
        $crate::cfg_if! {
            @__items () ;
            (( $i_meta ) ( $( $i_tokens )* )) ,
            $( (( $e_meta ) ( $( $e_tokens )* )) , )*
        }
    };
    // allow multiple conditions
    ( if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
      $( else if #[cfg( $e_meta:meta )] { $( $e_tokens:tt )* } )*
      if $($extra_conditions:tt)+
    ) => {
        $crate::cfg_if! {
            @__items () ;
            (( $i_meta ) ( $( $i_tokens )* )) ,
            $( (( $e_meta ) ( $( $e_tokens )* )) , )*
        }
        $crate::cfg_if! { if $($extra_conditions)+ }
    };

    // Internal and recursive macro to emit all the items
    //
    // Collects all the previous cfgs in a list at the beginning, so they can be
    // negated. After the semicolon are all the remaining items.
    (@__items ( $( $_:meta , )* ) ; ) => {};
    ( @__items ( $( $no:meta , )* ) ;
      (( $( $yes:meta )? ) ( $( $tokens:tt )* )) ,
      $( $rest:tt , )*
    ) => {
        // Emit all items within one block, applying an appropriate #[cfg]. The
        // #[cfg] will require all `$yes` matchers specified and must also negate
        // all previous matchers.
        #[cfg(all(
            $( $yes , )?
            not(any( $( $no ),* ))
        ))]
        $crate::cfg_if! { @__identity $( $tokens )* }

        // Recurse to emit all other items in `$rest`, and when we do so add all
        // our `$yes` matchers to the list of `$no` matchers as future emissions
        // will have to negate everything we just matched as well.
        $crate::cfg_if! {
            @__items ( $( $no , )* $( $yes , )? ) ;
            $( $rest , )*
        }
    };

    // Internal macro to make __apply work out right for different match types,
    // because of how macros match/expand stuff.
    (@__identity $( $tokens:tt )* ) => {
        $( $tokens )*
    };
}
#[doc(inline)]
pub use cfg_if;

#[cfg(test)]
mod test_cfg_if {
    #![allow(dead_code, unexpected_cfgs)]

    use crate::cfg_if;

    cfg_if! {
        if #[cfg(test)] {
            use core::option::Option as Option2;
            fn works1() -> Option2<u32> { Some(1) }
        } else {
            fn works1() -> Option<u32> { None }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works2() -> bool { false }
        } else if #[cfg(test)] {
            fn works2() -> bool { true }
        } else {
            fn works2() -> bool { false }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works3() -> bool { false }
        } else {
            fn works3() -> bool { true }
        }
    }

    cfg_if! {
        if #[cfg(test)] {
            use core::option::Option as Option3;
            fn works4() -> Option3<u32> { Some(1) }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works5() -> bool { false }
        } else if #[cfg(test)] {
            fn works5() -> bool { true }
        }
    }

    // multiple conditions
    cfg_if! {
        if #[cfg(foo)] {
            fn works6() -> bool { false }
        } else if #[cfg(test)] {
            fn works6() -> bool { true }
        }
        if #[cfg(test)] {
            fn works7() -> bool { true }
        } else {
            fn works7() -> bool { false }
        }
    }
    cfg_if! {
        if #[cfg(test)] {
            fn works8() -> bool { true }
        } else if #[cfg(foo)] {
            fn works8() -> bool { false }
        }
        if #[cfg(foo)] {
            fn works9() -> bool { false }
        } else if #[cfg(test)] {
            fn works9() -> bool { true }
        }
    }

    #[test]
    fn it_works() {
        assert!(works1().is_some());
        assert!(works2());
        assert!(works3());
        assert!(works4().is_some());
        assert!(works5());
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_usage_within_a_function() {
        cfg_if! {if #[cfg(debug_assertions)] {
            // we want to put more than one thing here to make sure that they
            // all get configured properly.
            assert!(cfg!(debug_assertions));
            assert_eq!(4, 2+2);
        } else {
            assert!(works1().is_some());
            assert_eq!(10, 5+5);
        }}
    }

    #[allow(dead_code)]
    trait Trait {
        fn blah(&self);
    }

    #[allow(dead_code)]
    struct Struct;

    impl Trait for Struct {
        cfg_if! {
            if #[cfg(feature = "blah")] {
                fn blah(&self) {
                    unimplemented!();
                }
            } else {
                fn blah(&self) {
                    unimplemented!();
                }
            }
        }
    }
}
