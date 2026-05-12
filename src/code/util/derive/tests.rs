// devela::code::util::derive::tests

use crate::{attr_alias, derive_alias, format_buf, macro_apply, macro_derive, macro_derive_with};

/* macro_apply: item passthrough */

macro_rules! keep_item {
    ($item:item) => {
        $item
    };
}
#[macro_apply(keep_item)]
struct ApplyKeep;
#[macro_apply(keep_item!)]
struct ApplyKeepBang;

#[test]
fn apply_keeps_item() {
    let _ = ApplyKeep;
    let _ = ApplyKeepBang;
}

/* macro_apply: item replacement */

macro_rules! replace_item {
    ($($input:tt)*) => {
        struct ApplyReplace;
        impl ApplyReplace {
            const OK: u8 = 1;
        }
    };
}
#[macro_apply(replace_item)]
struct ThisItemIsConsumed;

#[test]
fn apply_can_replace_item() {
    assert_eq!(ApplyReplace::OK, 1);
}

/* macro_apply: passes arguments to the applied macro */

macro_rules! make_value {
    (
        ($value:expr)
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        $(#[$meta])*
        $vis struct $Name;
        impl $Name {
            const VALUE: usize = $value;
        }
    };
}
#[macro_apply(make_value(7 * 6))]
struct ApplyWithArgs;
#[macro_apply(make_value!(43))]
struct ApplyWithBangArgs;

#[test]
fn apply_passes_arguments_to_macro() {
    assert_eq!(ApplyWithArgs::VALUE, 42);
    assert_eq!(ApplyWithBangArgs::VALUE, 43);
}

/* macro_derive: classic-only passthrough */

#[macro_derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ClassicOnly(u8);

#[test]
fn derive_accepts_classic_derives_only() {
    let a = ClassicOnly(3);
    let b = a;
    assert_eq!(a, b);

    let mut buf = [0; 16];
    assert_eq!(format_buf!(&mut buf, "{a:?}"), Ok("ClassicOnly(3)"));
}

/* macro_derive: mixed classic + macro derives */

macro_rules! derive_name {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        impl $Name {
            const NAME: &'static str = stringify!($Name);
        }
    };
}
#[macro_derive(Debug, derive_name!)]
struct MixedDerive;

#[test]
fn derive_accepts_classic_and_macro_derives() {
    let _ = MixedDerive;
    assert_eq!(MixedDerive::NAME, "MixedDerive");

    let mut buf = [0; 16];
    assert_eq!(format_buf!(&mut buf, "{MixedDerive:?}"), Ok("MixedDerive"));
}

/* macro_derive: passes arguments to banged declarative derives */

macro_rules! derive_label_direct {
    (
        ($label:literal)
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        impl $Name {
            const DIRECT_LABEL: &'static str = $label;
        }
    };
}
#[macro_derive(Debug, Clone, derive_label_direct!("mixed-direct"))]
struct MixedDeriveWithDirectArgs;

#[test]
fn derive_passes_arguments_to_banged_macro_derives() {
    let value = MixedDeriveWithDirectArgs;
    let clone = value.clone();

    assert_eq!(MixedDeriveWithDirectArgs::DIRECT_LABEL, "mixed-direct");

    let mut buf = [0; 32];
    assert_eq!(format_buf!(&mut buf, "{clone:?}"), Ok("MixedDeriveWithDirectArgs"));
}

/* macro_derive_with: item is preserved, macro adds impl */

#[macro_derive_with(derive_name)]
struct DeriveWithName;

#[test]
fn derive_with_preserves_item_and_adds_impl() {
    let _ = DeriveWithName;
    assert_eq!(DeriveWithName::NAME, "DeriveWithName");
}

/* macro_derive_with: multiple macro derives */

macro_rules! derive_a {
    ( $(#[$meta:meta])* $vis:vis struct $Name:ident; ) => {
        impl $Name {
            const A: u8 = 1;
        }
    };
}
macro_rules! derive_b {
    ( $(#[$meta:meta])* $vis:vis struct $Name:ident; ) => {
        impl $Name {
            const B: u8 = 2;
        }
    };
}
#[macro_derive_with(derive_a, derive_b)]
struct DeriveWithTwo;

#[test]
fn derive_with_accepts_multiple_macros() {
    let _ = DeriveWithTwo;
    assert_eq!(DeriveWithTwo::A, 1);
    assert_eq!(DeriveWithTwo::B, 2);
}

/* macro_derive_with: passes arguments to declarative derives */

#[macro_derive_with(derive_label_direct("direct"))]
struct DeriveWithDirectArgs;

#[test]
fn derive_with_passes_arguments_to_macro() {
    let _ = DeriveWithDirectArgs;
    assert_eq!(DeriveWithDirectArgs::DIRECT_LABEL, "direct");
}

/* macro_derive_with: direct macro args and item helper args coexist */

macro_rules! derive_two_labels {
    (
        ($direct:literal)
        #[macro_derive_args($helper:literal)]
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        impl $Name {
            const DIRECT: &'static str = $direct;
            const HELPER: &'static str = $helper;
        }
    };
}
#[macro_derive_with(derive_two_labels("direct"))]
#[macro_derive_args("helper")]
struct DeriveWithTwoArgLanes;

#[test]
fn derive_with_direct_args_and_helper_args_coexist() {
    assert_eq!(DeriveWithTwoArgLanes::DIRECT, "direct");
    assert_eq!(DeriveWithTwoArgLanes::HELPER, "helper");
}

/* __derive_helpers: */

macro_rules! derive_value {
    (
        #[macro_derive_args($value:expr)]
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        impl $Name {
            const VALUE: usize = $value;
        }
    };
}
#[macro_derive_with(derive_value)]
#[macro_derive_args(42)]
struct WithArgs;
#[test]
fn derive_with_accepts_derive_args() {
    assert_eq!(WithArgs::VALUE, 42);
}

/* __derive_helpers: permits helper attrs consumed by a macro derive */

macro_rules! derive_custom_label {
    (
        #[macro_derive_args($label:literal)]
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        impl $Name {
            const LABEL: &'static str = $label;
        }
    };
}
#[macro_derive_with(derive_custom_label)]
#[macro_derive_args("ok")]
struct HelperCustom;

#[test]
fn derive_with_permits_custom_helper_attr() {
    let _ = HelperCustom;
    assert_eq!(HelperCustom::LABEL, "ok");
}

/* __derive_helpers: same check through mixed derive */

#[macro_derive(Debug, derive_custom_label!)]
#[macro_derive_args("mixed")]
struct HelperCustomMixed;

#[test]
fn mixed_derive_permits_custom_helper_attr() {
    let _ = HelperCustomMixed;
    assert_eq!(HelperCustomMixed::LABEL, "mixed");

    let mut buf = [0; 20];
    assert_eq!(format_buf!(&mut buf, "{HelperCustomMixed:?}"), Ok("HelperCustomMixed"));
}

/* attr_alias */

attr_alias! {
    #[macro_apply(inline_answer)] =
        #[inline]
        #[must_use]
    ;
}
#[macro_apply(inline_answer)]
fn alias_answer() -> u8 {
    42
}

#[test]
fn attr_alias_defines_attribute_bundle() {
    assert_eq!(alias_answer(), 42);
}

/* attr_alias: accepts arguments */

attr_alias! {
    #[macro_apply(repr_alias($repr:ident))] =
        #[repr($repr)]
    ;
}

#[macro_apply(repr_alias(u8))]
enum AliasRepr {
    A = 1,
    B = 2,
}

#[test]
fn attr_alias_accepts_arguments() {
    assert_eq!(size_of::<AliasRepr>(), 1);
    assert_eq!(AliasRepr::A as u8, 1);
    assert_eq!(AliasRepr::B as u8, 2);
}

/* attr_alias: accepts literal arguments and may hardcode other attributes */

attr_alias! {
    #[macro_apply(doc_repr_u8($doc:literal))] =
        #[doc = $doc]
        #[repr(u8)]
    ;
}

#[macro_apply(doc_repr_u8("A documented repr-u8 enum."))]
enum AliasDocRepr {
    A = 3,
}

#[test]
fn attr_alias_accepts_literal_arguments_and_hardcoded_attrs() {
    assert_eq!(size_of::<AliasDocRepr>(), 1);
    assert_eq!(AliasDocRepr::A as u8, 3);
}

/* derive_alias */

derive_alias! {
    #[derive(Value!)] = #[derive(Clone, Copy, Debug, PartialEq, Eq)];
}
#[macro_derive(Value!)]
struct AliasValue(u8);

#[test]
fn derive_alias_defines_derive_bundle() {
    let a = AliasValue(3);
    let b = a; // Copy
    assert_eq!(a, b);

    let mut buf = [0; 16];
    assert_eq!(format_buf!(&mut buf, "{a:?}"), Ok("AliasValue(3)"));
}

/* derive_alias: accepts arguments */

derive_alias! {
    #[derive(Labeled!($label:literal))] =
        #[derive(Debug, derive_label_direct!($label))];
}

#[macro_derive(Labeled!("alias-direct"))]
struct AliasDeriveWithArgs;

#[test]
fn derive_alias_accepts_arguments() {
    let _ = AliasDeriveWithArgs;

    assert_eq!(AliasDeriveWithArgs::DIRECT_LABEL, "alias-direct");

    let mut buf = [0; 24];
    assert_eq!(format_buf!(&mut buf, "{AliasDeriveWithArgs:?}"), Ok("AliasDeriveWithArgs"));
}
