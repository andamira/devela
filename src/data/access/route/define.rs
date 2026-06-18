// devela/src/data/access/route/define.rs
//
//! Defines [`Route`], [`RouteAnchor`], [`RouteName`], and [`RouteSeg`].
//

/// A borrowed sequence of route segments.
#[doc = crate::_tags!(data)]
#[doc = crate::_doc_meta! {
    location("data/access/route"),
}]
///
/// A route is structural reachability. It does not resolve, canonicalize,
/// access storage, perform I/O, or encode platform-specific path rules.
///
/// It is suitable as a shared substrate for filesystem path adapters, URL path
/// segments, command routes, UI routes, asset routes, and symbolic addresses.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Route<'a> {
    anchor: RouteAnchor,
    segments: &'a [RouteSeg<'a>],
}
#[rustfmt::skip]
impl<'a> Route<'a> {
    /* constructors */

    /// An empty relative route.
    pub const EMPTY: Self = Self { anchor: RouteAnchor::Relative, segments: &[] };
    /// Constructs a route from an anchor and borrowed segments.
    pub const fn new(anchor: RouteAnchor, segments: &'a [RouteSeg<'a>]) -> Self {
        Self { anchor, segments }
    }
    /// Constructs a relative route.
    pub const fn relative(segments: &'a [RouteSeg<'a>]) -> Self {
        Self::new(RouteAnchor::Relative, segments)
    }
    /// Constructs a root-anchored route.
    pub const fn root(segments: &'a [RouteSeg<'a>]) -> Self {
        Self::new(RouteAnchor::Root, segments)
    }
    /// Constructs a route explicitly anchored at the current context.
    pub const fn current(segments: &'a [RouteSeg<'a>]) -> Self {
        Self::new(RouteAnchor::Current, segments)
    }

    /* queries */

    /// Returns the route anchor.
    pub const fn anchor(self) -> RouteAnchor { self.anchor }
    /// Returns the route segments.
    pub const fn segments(self) -> &'a [RouteSeg<'a>] { self.segments }
    /// Returns the number of segments.
    pub const fn len(self) -> usize { self.segments.len() }

    /// Returns whether the route has no segments.
    pub const fn is_empty(self) -> bool { self.segments.is_empty() }
    /// Returns whether the route is root-anchored.
    pub const fn is_rooted(self) -> bool { matches!(self.anchor, RouteAnchor::Root) }
    /// Returns whether the route has no explicit anchor.
    pub const fn is_relative(self) -> bool { matches!(self.anchor, RouteAnchor::Relative) }
}

/// The starting relation of a [`Route`].
#[doc = crate::_tags!(data)]
#[doc = crate::_doc_meta! { location("data/access/route") }]
///
/// This is not a filesystem root, URL origin, process directory, or storage
/// location. It only describes how the first segment relates to a resolver
/// context.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RouteAnchor {
    /// No explicit anchor.
    #[default]
    Relative,

    /// Anchored at the resolver's root context.
    Root,

    /// Explicitly anchored at the resolver's current context.
    Current,
}

/// A borrowed route segment name.
#[doc = crate::_tags!(data)]
#[doc = crate::_doc_meta! { location("data/access/route") }]
///
/// UTF-8 names are the ergonomic path for commands, URLs, assets, and most
/// user-facing routes. Byte names preserve non-UTF-8 segment data for adapters
/// that need it.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RouteName<'a> {
    /// A UTF-8 segment name.
    Utf8(&'a str),
    /// A raw byte segment name.
    Bytes(&'a [u8]),
}
#[rustfmt::skip]
impl<'a> RouteName<'a> {
    /* constructors */

    /// Constructs a UTF-8 route name.
    pub const fn utf8(name: &'a str) -> Self { Self::Utf8(name) }
    /// Constructs a byte route name.
    pub const fn bytes(name: &'a [u8]) -> Self { Self::Bytes(name) }

    /* queries */

    /// Returns the UTF-8 string, if this name is UTF-8.
    pub const fn as_utf8(self) -> Option<&'a str> {
        match self { Self::Utf8(name) => Some(name), Self::Bytes(_) => None }
    }
    /// Returns the underlying bytes.
    pub const fn as_bytes(self) -> &'a [u8] {
        match self { Self::Utf8(name) => name.as_bytes(), Self::Bytes(name) => name }
    }
    /// Returns whether this name is empty.
    pub const fn is_empty(self) -> bool { self.as_bytes().is_empty() }
}

/// One segment in a [`Route`].
#[doc = crate::_tags!(data)]
#[doc = crate::_doc_meta! { location("data/access/route") }]
///
/// Special segments are preserved structurally. Normalization policy belongs
/// above this type, because filesystems, URLs, commands, and virtual routes do
/// not all treat `.`, `..`, empty segments, or trailing separators identically.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RouteSeg<'a> {
    /// A named segment.
    Name(RouteName<'a>),

    /// The current context segment.
    Current,

    /// The parent context segment.
    Parent,

    /// An empty segment.
    Empty,
}
#[rustfmt::skip]
impl<'a> RouteSeg<'a> {
    /* constructors */

    /// Constructs a UTF-8 named segment.
    pub const fn utf8(name: &'a str) -> Self { Self::Name(RouteName::Utf8(name)) }
    /// Constructs a byte named segment.
    pub const fn bytes(name: &'a [u8]) -> Self { Self::Name(RouteName::Bytes(name)) }

    /* queries*/

    /// Returns the segment name, if this is a named segment.
    pub const fn name(self) -> Option<RouteName<'a>> {
        match self { Self::Name(name) => Some(name), _ => None }
    }
    /// Returns whether this is a named segment.
    pub const fn is_name(self) -> bool { matches!(self, Self::Name(_)) }

    /// Returns whether this is [`RouteSeg::Current`].
    pub const fn is_current(self) -> bool { matches!(self, Self::Current) }

    /// Returns whether this is [`RouteSeg::Parent`].
    pub const fn is_parent(self) -> bool { matches!(self, Self::Parent) }

    /// Returns whether this is [`RouteSeg::Empty`].
    pub const fn is_empty(self) -> bool { matches!(self, Self::Empty) }
}
