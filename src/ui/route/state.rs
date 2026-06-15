// devela::ui::route::state
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
            /// No hot identity.
            pub const NONE: Self = Self(None);

            /// Constructs a hot state from an optional identity.
            pub const fn new(id: Option<UiId>) -> Self { Self(id) }
            /// Constructs a hot state with an identity.
            pub const fn some(id: UiId) -> Self { Self(Some(id)) }

            /// Returns the hot identity.
            #[must_use]
            pub const fn id(self) -> Option<UiId> { self.0 }

            /// Returns whether `id` is the hot identity.
            #[must_use]
            pub const fn is(self, id: UiId) -> bool {
                match self.0 {
                    Some(hot) => hot.raw() == id.raw(),
                    None => false,
                }
            }
            /// Returns whether there is a hot identity.
            #[must_use]
            pub const fn is_some(self) -> bool { self.0.is_some() }
            /// Returns whether there is no hot identity.
            #[must_use]
            pub const fn is_none(self) -> bool { self.0.is_none() }
        }
    };
}
define_ui_state! {
    RouteActive, "Current owner of an ongoing interaction.", "an", "active";
    RouteCapture, "Current pointer-capture owner", "a", "pointer-captured";
    RouteFocus, "Current keyboard or navigation focus owner.", "a", "focused";
    RouteHot, "Current candidate under direct pointing interaction.", "a", "hot";
}
