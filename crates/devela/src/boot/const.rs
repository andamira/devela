//
//! Defines the [CONST!] macro.
//
// IMPROVE: the examples showing the last two arms [hidden|inline] macro_export
// IMPROVE: support constants with macro metavariables
//
// NOTE: docs in const.md are included from ../code/util/synth/const.rs

// Related links
// - https://doc.rust-lang.org/reference/items/external-blocks.html#functions
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! CONST· {
    (
    // Either multiple `const fn`
    $(
        $(#[$CONST_ATTRS:meta])*
        $item_vis:vis $CONST_NAME:ident =
            $(#[$fn_attrs:meta])*
            $fn_vis:vis const
            $(async$($_a:block)?)? $(safe$($_s:block)?)? $(unsafe$($_u:block)?)?
            fn $fn:ident($($param:ident: $param_ty:ty),* $(,)?)
        $(-> $fn_return:ty)?
        $fn_body:block
    );* $(;)?) => {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macros)]
            macro_rules! $CONST_NAME {
                () => {
                    $(#[$fn_attrs])*
                    $fn_vis const $(async$($_a)?)? $(safe$($_s)?)? $(unsafe$($_u)?)?
                    fn $fn($($param: $param_ty),*) $(-> $fn_return)? $fn_body
                }
            }
            #[allow(unused_imports)]
            $item_vis use $CONST_NAME;
        )*
    };
    (
    $shared_vis:vis, // (shared visibility alternative)
    $(
        $(#[$CONST_ATTRS:meta])*
        $CONST_NAME:ident =
            $(#[$fn_attrs:meta])*
            $fn_vis:vis const
            $(async$($_a:block)?)? $(safe$($_s:block)?)? $(unsafe$($_u:block)?)?
            fn $fn:ident($($param:ident: $param_ty:ty),* $(,)?)
        $(-> $fn_return:ty)?
        $fn_body:block
    );* $(;)?) => {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macros)]
            macro_rules! $CONST_NAME {
                () => {
                    $(#[$fn_attrs])*
                    $shared_vis const $(async$($_a)?)? $(safe$($_s)?)? $(unsafe$($_u)?)?
                    fn $fn($($param: $param_ty),*) $(-> $fn_return)? $fn_body
                }
            }
            #[allow(unused_imports)]
            $shared_vis use $CONST_NAME;
        )*
    };
    (

    // Either multiple expressions
    $(
        $(#[$CONST_ATTRS:meta])*
        $item_vis:vis $CONST_NAME:ident = $expr:expr
     );* $(;)?) => {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macro)]
            macro_rules! $CONST_NAME { () => { $expr } }
            #[allow(unused_imports)]
            $item_vis use $CONST_NAME;
        )*
    };
    (
    $shared_vis:vis, // (shared visibility alternative)
    $(
        $(#[$CONST_ATTRS:meta])*
        $CONST_NAME:ident = $expr:expr
     );* $(;)?) => {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macro)]
            macro_rules! $CONST_NAME { () => { $expr } }
            #[allow(unused_imports)]
            $shared_vis use $CONST_NAME;
        )*
    };
    (

    // Either multiple hidden macros exported
    hidden macro_export,
    $(
        $(#[$CONST_ATTRS:meta])*
        $CONST_NAME:ident = $expr:expr
     );* $(;)?) => { $crate::paste! {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macro)]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! [<$CONST_NAME _·>] { () => { $expr } }

            #[doc(hidden)]
            #[allow(unused_imports)]
            pub use [<$CONST_NAME _·>] as $CONST_NAME;
        )*
    }};
    (

    // Either multiple visible macros exported
    inline macro_export,
    $(
        $(#[$CONST_ATTRS:meta])*
        $CONST_NAME:ident = $expr:expr
     );* $(;)?) => { $crate::paste! {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macro)]
            #[macro_export]
            #[cfg_attr(cargo_primary_package, doc(hidden))]
            macro_rules! [<$CONST_NAME _·>] { () => { $expr } }

            #[doc(inline)]
            #[allow(unused_imports)]
            pub use [<$CONST_NAME _·>] as $CONST_NAME;
        )*
    }};
}
#[doc(inline)]
pub use CONST· as CONST;
