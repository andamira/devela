// devela::data::collections::graph::edge

macro_rules! impl_graph_edge {
    // $IDX:  the index primitive type. E.g. u8.
    //
    // $Graph: the graph type name. E.g. GraphU8.
    // $Edge:  the edge type name. E.g. EdgeU8.
    // $Index: the index real type name. E.g. NonExtremeU8.
    (@$Graph:ty, $Node:ty, $Edge:ty, $Index:ty, $IDX:ty) => { paste! {
        /* definition */

        #[doc = "A static [`" $Graph "`] edge."]
        ///
        /// It can contain data `E` that defaults to the unit type without cost.
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $Edge<E = ()> (pub $Node<E, 2>);

        /* constructors */

        /// Constructors without data.
        impl $Edge {
            /// Returns a new edge without data, and two optional indices,
            /// in compile-time.
            ///
            /// # Errors
            #[doc = "Returns [`IndexOutOfBounds`] if `(orig|dest) >= `[`" $IDX "::MAX`]."]
            pub const fn new(orig: Option<$IDX>, dest: Option<$IDX>) -> Result<Self> {
                let orig = match orig {
                    None => None,
                    Some(idx) => {
                        if let Some(n) = <$Index>::new(idx) { Some(n) }
                        else { return Err(IndexOutOfBounds(Some(idx as usize))); }
                    }
                };
                let dest = match dest {
                    None => None,
                    Some(idx) => {
                        if let Some(n) = <$Index>::new(idx) { Some(n) }
                        else { return Err(IndexOutOfBounds(Some(idx as usize))); }
                    }
                };
                Ok(Self($Node { data: (), links: [orig, dest] }))
            }

            /// Returns a new edge without data, and two indices,
            /// in compile-time.
            ///
            /// # Errors
            #[doc = "Returns [`IndexOutOfBounds`] if `(orig|dest) >= `[`" $IDX "::MAX`]."]
            pub const fn new_some(orig: $IDX, dest: $IDX) -> Result<Self> {
                let Some(orig) = <$Index>::new(orig) else {
                    return Err(IndexOutOfBounds(Some(orig as usize)));
                };
                let Some(dest) = <$Index>::new(dest) else {
                    return Err(IndexOutOfBounds(Some(dest as usize)));
                };
                Ok(Self($Node { data: (), links: [Some(orig), Some(dest)] }))
            }

            /// Returns a new edge without data, and two optional validated indices,
            /// in compile time.
            pub const fn new_valid(orig: Option<$Index>, dest: Option<$Index>) -> Self {
                Self($Node { data: (), links: [orig, dest] })
            }

            /// Returns a new edge without data, and two validated indices,
            /// in compile time.
            pub const fn new_some_valid(orig: $Index, dest: $Index) -> Self {
                Self($Node { data: (), links: [Some(orig), Some(dest)] })
            }
        }

        /// Constructors with data.
        impl<E> $Edge<E> {
            /// Returns a new graph edge, with `data`, and two optional indices.
            ///
            /// # Errors
            #[doc = "Returns [`IndexOutOfBounds`] if `(orig|dest) >= `[`" $IDX "::MAX`]."]
            ///
            /// This method can't be *const* because in case of error,
            /// the destructor can't be evaluated.
            pub fn with(data: E, orig: Option<$IDX>, dest: Option<$IDX>) -> Result<Self> {
                let orig = match orig {
                    None => None,
                    Some(idx) => {
                        if let Some(n) = <$Index>::new(idx) { Some(n) }
                        else { return Err(IndexOutOfBounds(Some(idx as usize))); }
                    }
                };
                let dest = match dest {
                    None => None,
                    Some(idx) => {
                        if let Some(n) = <$Index>::new(idx) { Some(n) }
                        else { return Err(IndexOutOfBounds(Some(idx as usize))); }
                    }
                };
                Ok(Self($Node { data, links: [orig, dest] }))
            }

            /// Returns a new edge, with `data` and two indices.
            ///
            /// # Errors
            #[doc = "Returns [`IndexOutOfBounds`] if `(orig|dest) >= `[`" $IDX "::MAX`]."]
            ///
            /// This method can't be *const* because in case of error,
            /// the destructor can't be evaluated.
            pub fn with_some(data: E, orig: $IDX, dest: $IDX) -> Result<Self> {
                let orig = if let Some(n) = <$Index>::new(orig) { Some(n) } else {
                    return Err(IndexOutOfBounds(Some(orig as usize)));
                };
                let dest = if let Some(n) = <$Index>::new(dest) { Some(n) } else {
                    return Err(IndexOutOfBounds(Some(dest as usize)));
                };
                Ok(Self($Node { data, links: [orig, dest] }))
            }

            /// Returns a new edge with the given `data` and two optional validated indices,
            /// in compile-time.
            pub const fn with_valid(data: E, orig: Option<$Index>, dest: Option<$Index>) -> Self {
                Self($Node { data, links: [orig, dest] })
            }
            /// Returns a new edge with the given `data` and two validated indices,
            /// in compile-time.
            pub const fn with_some_valid(data: E, orig: $Index, dest: $Index) -> Self {
                Self($Node { data, links: [Some(orig), Some(dest)] })
            }
        }

        /// Constructors with copiable data.
        impl<E: Copy> $Edge<E> {
            /// Returns a new edge, with the given `data`.
            ///
            /// # Errors
            #[doc = "Returns [`IndexOutOfBounds`] if `(orig|dest) >= `[`" $IDX "::MAX`]."]
            pub fn with_copy(data: E, orig: Option<$IDX>, dest: Option<$IDX>) -> Result<Self> {
                let orig = match orig {
                    None => None,
                    Some(idx) => {
                        if let Some(n) = <$Index>::new(idx) { Some(n) }
                        else { return Err(IndexOutOfBounds(Some(idx as usize))); }
                    }
                };
                let dest = match dest {
                    None => None,
                    Some(idx) => {
                        if let Some(n) = <$Index>::new(idx) { Some(n) }
                        else { return Err(IndexOutOfBounds(Some(idx as usize))); }
                    }
                };
                Ok(Self($Node { data, links: [orig, dest] }))
            }

            /// Returns a new edge, with the given `data`.
            ///
            /// # Errors
            #[doc = "Returns [`IndexOutOfBounds`] if `(orig|dest) >= `[`" $IDX "::MAX`]."]
            // This can't be const because the destructor can't be evaluated in case of Err.
            pub fn with_some_copy(data: E, orig: $IDX, dest: $IDX) -> Result<Self> {
                let orig = if let Some(n) = <$Index>::new(orig) { Some(n) } else {
                    return Err(IndexOutOfBounds(Some(orig as usize)));
                };
                let dest = if let Some(n) = <$Index>::new(dest) { Some(n) } else {
                    return Err(IndexOutOfBounds(Some(dest as usize)));
                };
                Ok(Self($Node { data, links: [orig, dest] }))
            }
        }

        /* methods */

        /// Methods.
        impl<E> $Edge<E> {
            ///
            #[must_use]
            pub const fn orig(&self) -> Option<$IDX> { self.0.get_link_unchecked(0) }
            ///
            #[must_use]
            pub const fn dest(&self) -> Option<$IDX> { self.0.get_link_unchecked(1) }

            ///
            #[must_use]
            pub const fn is_orig_set(&self) -> bool { self.0.is_link_set_unchecked(0) }
            ///
            #[must_use]
            pub const fn is_dest_set(&self) -> bool { self.0.is_link_set_unchecked(1) }

        }

        /* trait impls */

        // T: ConstInit
        impl<E: ConstInit> ConstInit for $Edge<E> {
            /// Returns an edge with default data and unset links.
            const INIT: Self = Self($Node::INIT);
        }
    }};
}
pub(super) use impl_graph_edge;
