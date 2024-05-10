// devela::data::collections::graph
//
//! Graphs are node-based, dynamically sized, heterogeneous data structures.
//!
//! They enable efficient representation of relationships between pairs of elements,
//! with nodes that may contain different types and sets of edges that connect these nodes.
//!
//! Edges are potentially ordered, with optional data.
//!
//

mod edge;
use edge::impl_graph_edge;

use crate::{
    code::{paste, unwrap, ConstDefault},
    data::{
        Array,
        DataError::{KeyAlreadyExists, NodeEmpty, NotEnoughSpace, OutOfBounds},
        DataResult as Result,
    },
    mem::{Bare, Storage},
    num::niche::*,
};

macro_rules! impl_graph {
    () => {
        impl_graph!(u8:"_graph_u8", u16:"_graph_u16", u32:"_graph_u32", usize:"_graph_usize");
    };

    // $IDX:  the index primitive type. E.g. u8.
    // $cap:  the capability feature that enables the given implementation. E.g "_graph_u8".
    //
    // $Graph: the graph type name. E.g. GraphU8.
    // $Node: the node type name. E.g. NodeU8.
    // $Edge:  the edge type name. E.g. EdgeU8.
    // $Index: the index real type name. E.g. NonExtremeU8.
    ($( $IDX:ty : $cap:literal ),+) => { paste! {
        $(
            #[cfg(feature = $cap )]
            impl_graph!(@call
                [<Graph $IDX:camel>],      // $Graph
                [<Node $IDX:camel>],       // $Node
                [<Edge $IDX:camel>],       // $Edge
                [<NonExtreme $IDX:camel>], // $Index
                $IDX);
        )+
    }};
    (@call $($Graph:ty, $Node:ty, $Edge:ty, $Index:ty, $IDX:ty)+) => {
        $(
            impl_graph_edge!(@$Graph, $Node, $Edge, $Index, $IDX);
            impl_graph!(@$Graph, $Node, $Edge, $Index, $IDX);
        )+
    };

    (@$Graph:ty, $Node:ty, $Edge:ty, $Index:ty, $IDX:ty) => { paste! {
        use super::$Node;

        /* definition */

        #[doc = "A static graph with [`" $Index "`] indices,"]
        /// and two [`Array`]s for vertices and edges.
        ///
        /// It is generic in respect to the
        /// - vertices data type (`V`),
        /// - vertices capacity (`VCAP`),
        /// - edges data type (`E`),
        /// - edges capacity (`ECAP`)
        /// - storage (`S`).
        ///
        #[doc = "Edges are represented with the [`" $Edge "`]s type."]
        ///
        /// ## Features
        /// It supports multiple index sizes by enabling the corresponding features:
        /// `_graph_u[8|16|32|size]`.
        pub struct $Graph<const VCAP: usize, const ECAP: usize, V = (), E = (), S: Storage = Bare> {
            pub(super) verts: Array<Option<V>, VCAP, S>,
            pub(super) edges: Array<Option<$Edge<E>>, ECAP, S>,
        }

        /* constructors */

        // S: Bare, E: Copy
        impl<V, E: Copy, const VCAP: usize, const ECAP: usize> $Graph<VCAP, ECAP, V, E, Bare> {
            /// Returns a disconnected graph, allocated in the stack,
            /// with the given array of `vertices`,
            /// where `E: Copy`.
            #[inline]
            pub fn with_vertices(vertices: [Option<V>; VCAP]) -> Result<Self> {
                let _ = unwrap![ok? Self::check_capacity_bounds()];
                Ok(Self {
                    verts: Array::<Option<V>, VCAP, Bare>::new(vertices),
                    edges: Array::<Option<$Edge<E>>, ECAP, Bare>::with_copied(None),
                })
            }
        }

        // S: Bare, E: Clone
        impl<V, E: Clone, const VCAP: usize, const ECAP: usize> $Graph<VCAP, ECAP, V, E, Bare> {
            /// Returns a disconnected graph, allocated in the stack,
            /// with the given array of `vertices`,
            /// where `E: Clone`.
            #[inline]
            pub fn with_vertices_clone(vertices: [Option<V>; VCAP]) -> Result<Self> {
                let _ = unwrap![ok? Self::check_capacity_bounds()];
                Ok(Self {
                    verts: Array::<Option<V>, VCAP, Bare>::new(vertices),
                    edges: Array::<Option<$Edge<E>>, ECAP, Bare>::with_cloned(None),
                })
            }
        }

        // S: Bare, V: Copy, E: Copy
        impl<V: Copy, E: Copy, const VCAP: usize, const ECAP: usize> $Graph<VCAP, ECAP, V, E, Bare> {
            /// Returns an empty graph, allocated in the stack, in *compile-time*,
            /// where `V: Copy, E: Copy`.
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `(ECAP|VCAP) >= [`" $IDX "::MAX`]."]
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::data::" $Graph ";"]
            #[doc = "let g = " $Graph "::<(), 10, 5>::new_copy().unwrap();"]
            /// ```
            #[inline]
            pub const fn new_copy() -> Result<Self> {
                let _ = unwrap![ok? Self::check_capacity_bounds()];
                Ok(Self {
                    verts: Array::<Option<V>, VCAP, Bare>::new_bare([None; VCAP]),
                    edges: Array::<Option<$Edge<E>>, ECAP, Bare>::new_bare([None; ECAP]),
                })
            }
        }

        // S: Bare, V: Clone, E: Clone.
        impl<V: Clone, E: Clone, const VCAP: usize, const ECAP: usize> $Graph<VCAP, ECAP, V, E, Bare> {
            /// Returns an empty graph, allocated in the stack,
            /// where `V: Clone, E: Clone`.
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `(ECAP|VCAP) >= [`" $IDX "::MAX`]."]
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::data::" $Graph ";"]
            #[doc = "let g = " $Graph "::<(), 10, 5>::new_clone().unwrap();"]
            /// ```
            #[inline]
            pub fn new_clone() -> Result<Self> {
                Self::check_capacity_bounds()?;
                Ok(Self {
                    verts: Array::<Option<V>, VCAP, Bare>::with_cloned(None),
                    edges: Array::<Option<$Edge<E>>, ECAP, Bare>::with_cloned(None),
                })
            }
        }

        /* methods */

        // S: Bare
        impl<const VCAP: usize, const ECAP: usize> $Graph<VCAP, ECAP, Bare> {
            /// Returns `true` if there's no space left for edges.
            #[inline]
            pub const fn is_edges_full(&self) -> bool {
                self.remaining_edges_capacity() == 0
            }
            /// Returns the remaining capacity for additional edges.
            #[inline]
            pub const fn remaining_edges_capacity(&self) -> $IDX {
                ECAP as $IDX - self.edges_count()
            }
            /// Returns the number of existing edges.
            #[inline]
            pub const fn edges_count(&self) -> $IDX {
                let mut i = 0;
                let mut count = 0;
                while i < ECAP {
                    count += self.edges.as_bare_slice()[i].is_some() as $IDX;
                    i += 1;
                }
                count
            }
            /// Returns `true` if a given `edge` id exists.
            #[inline]
            pub const fn edge_exists(&self, edge: $IDX) -> bool {
                self.edges.as_bare_slice()[edge as usize].is_some()
            }

            /// Returns `true` if there's no space left for vertices.
            #[inline]
            pub const fn is_vertices_full(&self) -> bool {
                self.remaining_vertices_capacity() == 0
            }
            /// Returns the remaining capacity for additional vertices.
            #[inline]
            pub const fn remaining_vertices_capacity(&self) -> $IDX {
                VCAP as $IDX - self.vertices_count()
            }
            /// Returns the number of existing vertices.
            #[inline]
            pub const fn vertices_count(&self) -> $IDX {
                let mut i = 0;
                let mut count = 0;
                while i < VCAP {
                    count += self.verts.as_bare_slice()[i].is_some() as $IDX;
                    i += 1;
                }
                count
            }
            /// Returns `true` if a given `vertex` id exists.
            #[inline]
            pub const fn vertex_exists(&self, vertex: $IDX) -> bool {
                self.verts.as_bare_slice()[vertex as usize].is_some()
            }

            /// Adds an edge to the graph, connecting the given `orig` and `dest` vertices.
            ///
            /// Returns the new edge index, if there was space for it.
            ///
            /// Errors
            #[doc = "Returns [`OutOfBounds`] if `(ECAP|VCAP) >= `[`" $IDX "::MAX`]."]
            /// or [`NodeEmpty`] if any of the given vertices is empty,
            /// or [`NotEnoughSpace`] if the array of edges is full.
            pub fn add_edge(&mut self, orig: $IDX, dest: $IDX) -> Result<$IDX> {
                Self::check_vertex_bounds(orig)?;
                Self::check_vertex_bounds(dest)?;

                // make sure the referred vertices exist
                if self.verts[orig as usize].is_none() {
                    Err(NodeEmpty(Some(orig as usize)))
                } else if self.verts[dest as usize].is_none() {
                    Err(NodeEmpty(Some(dest as usize)))

                } else {
                    // These can't fail since we've already checked their bounds
                    let orig = <$Index>::new(orig).unwrap();
                    let dest = <$Index>::new(dest).unwrap();

                    for (index, edge) in self.edges.iter_mut().enumerate() {
                        if edge.is_none() {
                            *edge = Some($Edge::new_some_valid(orig, dest));
                            return Ok(index as $IDX);
                        }
                    }
                    Err(NotEnoughSpace(Some(1)))
                }
            }

            /// Adds a vertex to the graph with the given `data`.
            ///
            /// Errors
            /// Returns [`NotEnoughSpace`] if there's no space left for the new vertex.
            #[inline]
            pub fn add_vertex(&mut self) -> Result<$IDX> {
                for (idx, vertex) in self.verts.iter_mut().enumerate() {
                    if vertex.is_none() {
                        *vertex = Some(());
                        return Ok(idx as $IDX);
                    }
                }
                Err(NotEnoughSpace(Some(1))) // MAYBE Option
            }

            /// Removes a vertex from the graph, at the given `index`.
            ///
            /// Errors
            /// Returns [`NodeEmpty`] if the vertex didn't exist.
            #[inline]
            pub fn remove_vertex(&mut self, index: $IDX) -> Result<()> {
                Self::check_vertex_bounds(index)?;
                let index = index as usize;
                if self.verts[index].is_none() {
                    Err(NodeEmpty(Some(index)))
                } else {
                    self.verts[index] = None;
                    Ok(())
                }
            }
        }

        impl<V, const VCAP: usize, const ECAP: usize> $Graph<VCAP, ECAP, V, Bare> {
            /// Adds a vertex to the graph with the given `data`.
            ///
            /// Errors
            /// Returns [`NotEnoughSpace`] if there's no space left for the new vertex.
            #[inline]
            pub fn add_vertex_with(&mut self, data: V) -> Result<$IDX> {
                for (idx, vertex) in self.verts.iter_mut().enumerate() {
                    if vertex.is_none() {
                        *vertex= Some(data);
                        return Ok(idx as $IDX);
                    }
                }
                Err(NotEnoughSpace(Some(1))) // MAYBE Option
            }
        }

        impl<V: PartialEq, const VCAP: usize, const ECAP: usize> $Graph<VCAP, ECAP, V> {
            /// Adds a vertex to the graph with the given `data`, but only if the data is unique.
            ///
            /// Errors
            /// Returns [`NotEnoughSpace`] if there's no space left for the new vertex,
            /// or returns [`KeyAlreadyExists`] if there's already a vertex with the same data.
            #[inline]
            pub fn add_vertex_unique(&mut self, data: V) -> Result<$IDX> {
                let (found_data, free_index) = self.find_vertex_and_first_free(&data);

                if free_index.is_none() {
                    return Err(NotEnoughSpace(Some(1)));
                } else if found_data.is_none() {
                    if let Some(index) = free_index {
                        self.verts[index] = Some(data);
                        return Ok(index as $IDX);
                    }
                }
                Err(KeyAlreadyExists)
            }

            /// Checks for the presence of the given vertex `data` and returns its index.
            #[inline] #[must_use]
            pub fn find_vertex(&self, data: &V) -> Option<$IDX> {
                for (index, vertex) in self.verts.iter().enumerate() {
                    if let Some(v) = vertex {
                        if v == data {
                            return Some(index as $IDX);
                        }
                    }
                }
                None
            }
        }

        /* graph helpers */

        impl<V: PartialEq, const VCAP: usize, const ECAP: usize> $Graph<VCAP, ECAP, V> {
            // Checks for the presence of the given vertex `data` and returns its index,
            // as well as the first free vertex index.
            //
            // returns usizes
            fn find_vertex_and_first_free(&self, data: &V) -> (Option<usize>, Option<usize>) {
                let mut found_data = None;
                let mut free_index = None;

                for (index, vertex) in self.verts.iter().enumerate() {
                    if found_data.is_none() {
                        if vertex.as_ref() == Some(data) {
                            found_data = Some(index);
                        }
                    } else if free_index.is_some() { break; }

                    if free_index.is_none() {
                        if vertex.is_none() {
                            free_index = Some(index);
                        }
                    } else if found_data.is_some() { break; }
                }
                (found_data, free_index)
            }
        }

        #[allow(dead_code)]
        impl<V, E, const VCAP: usize, const ECAP: usize> $Graph<VCAP, ECAP, V, E, Bare> {
            // Makes sure the given `vertex` is in bounds,
            #[inline]
            const fn check_vertex_bounds(vertex: $IDX) -> Result<()> {
                if vertex == $IDX::MAX || vertex >= VCAP as $IDX {
                    Err(OutOfBounds(Some(vertex as usize)))
                } else {
                    Ok(())
                }
            }
            // Makes sure the given `edge` is in bounds,
            #[inline]
            const fn check_edge_bounds(edge: $IDX) -> Result<()> {
                if edge == $IDX::MAX || edge >= ECAP as $IDX {
                    Err(OutOfBounds(Some(edge as usize)))
                } else {
                    Ok(())
                }
            }

            // Makes sure the capacity const-generic arguments are in bounds.
            #[inline]
            const fn check_capacity_bounds() -> Result<()> {
                if ECAP >= $IDX::MAX as usize {
                    Err(OutOfBounds(Some(ECAP)))
                } else if VCAP >= $IDX::MAX as usize {
                    Err(OutOfBounds(Some(VCAP)))
                } else {
                    Ok(())
                }
            }
        }
    }};
}
impl_graph!();
