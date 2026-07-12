// devela/ui/route/state.rs
//
//! Defines [`RouteActive`], [`RouteCapture`], [`RouteFocus`], [`RouteHot`].
//

use crate::UiId;

macro_rules! define_ui_state {
    ($($name:ident, $doc:literal, $det:literal, $adj:literal);+ $(;)?) => {
        $( define_ui_state!(% $name, $doc, $det, $adj); )+
    };
    (% $name:ident, $doc:literal, $det:literal, $adj:literal) => {
        #[doc = crate::_tags!(ui)]
        #[doc = $doc]
        #[doc = crate::_doc_meta! {
            location("ui/route"),
            test_size_of($name = 8|64),
        }]
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(Option<UiId>);
        impl $name {
            #[doc = concat!("No ", $adj, " identity.")]
            pub const NONE: Self = Self(None);

            #[doc = concat!("Constructs ", $det, " ", $adj, " state from an optional identity.")]
            pub const fn new(id: Option<UiId>) -> Self { Self(id) }
            #[doc = concat!("Constructs ", $det, " ", $adj, " state with an identity.")]
            pub const fn some(id: UiId) -> Self { Self(Some(id)) }

            #[must_use]
            #[doc = concat!("Returns the ", $adj, " identity.")]
            pub const fn id(self) -> Option<UiId> { self.0 }

            #[must_use]
            #[doc = concat!("Returns whether `id` is the ", $adj, " identity.")]
            pub const fn is(self, id: UiId) -> bool {
                match self.0 {
                    Some(hot) => hot.raw() == id.raw(),
                    None => false,
                }
            }
            #[must_use]
            #[doc = concat!("Returns whether there is ", $det, " ", $adj, " identity.")]
            pub const fn is_some(self) -> bool { self.0.is_some() }
            #[doc = concat!("Returns whether there is no ", $adj, " identity.")]
            #[must_use]
            pub const fn is_none(self) -> bool { self.0.is_none() }
        }
    };
}
define_ui_state! {
    RouteActive, "Current owner of an ongoing interaction.", "an", "active";
    RouteCapture, "Current pointer-capture owner", "a", "captured";
    RouteFocus, "Current keyboard or navigation focus owner.", "a", "focused";
    RouteHot, "Current candidate under direct pointing interaction.", "a", "hot";
}
