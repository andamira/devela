// devela/src/data/topol/graph/adj/impls/array.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __graph_adj_impl_array {
    (
        [vertex: $vprim:ident + $VertexIndex:ty;]
        [edge: $eprim:ident + $EdgeIndex:ty;]
        $(#[$graph_attr:meta])* $vis:vis $Graph:ident;
        $vvis:vis $Vertex:ident;
        $evis:vis $Edge:ident;
    ) => {
        $crate::paste! {
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            struct [<__ $Graph EdgeSlot>] {
                target: $Vertex,
                next: Option<$Edge>,
            }

            $(#[$graph_attr])*
            #[derive(Clone, Debug)]
            $vis struct $Graph<const VERTICES: usize, const EDGE_CAP: usize> {
                heads: [Option<$Edge>; VERTICES],
                edges: [Option<[<__ $Graph EdgeSlot>]>; EDGE_CAP],
                len: usize,
            }

            impl<const VERTICES: usize, const EDGE_CAP: usize>
                $crate::ConstInit for $Graph<VERTICES, EDGE_CAP> {
                const INIT: Self = Self::new();
            }

            impl<const VERTICES: usize, const EDGE_CAP: usize>
                Default for $Graph<VERTICES, EDGE_CAP> {
                fn default() -> Self { Self::new() }
            }

            #[allow(dead_code)]
            impl<const VERTICES: usize, const EDGE_CAP: usize> $Graph<VERTICES, EDGE_CAP> {
                /* configuration */

                const __VALID_CONFIG: () = {
                    const fn __index_primitive<P: $crate::PrimIndex>() {}
                    __index_primitive::<$vprim>();
                    __index_primitive::<$eprim>();
                    assert!(!$crate::MaybeNiche::<$VertexIndex>::HAS_NEGATIVE,
                        "the graph vertex representation must be unsigned");
                    assert!($crate::MaybeNiche::<$VertexIndex>::IS_CONTIGUOUS,
                        "the graph vertex representation must be contiguous");
                    assert!($crate::MaybeNiche::<$VertexIndex>::ZERO.is_some(),
                        "the graph vertex representation must contain zero");
                    assert!(!$crate::MaybeNiche::<$EdgeIndex>::HAS_NEGATIVE,
                        "the graph edge representation must be unsigned");
                    assert!($crate::MaybeNiche::<$EdgeIndex>::IS_CONTIGUOUS,
                        "the graph edge representation must be contiguous");
                    assert!($crate::MaybeNiche::<$EdgeIndex>::ZERO.is_some(),
                        "the graph edge representation must contain zero");
                    assert!(VERTICES <= Self::__vertex_index_capacity(),
                        "the graph vertex count exceeds its vertex representation");
                    assert!(EDGE_CAP <= Self::__edge_index_capacity(),
                        "the graph edge capacity exceeds its edge representation");
                };

                /* construction */

                /// Returns a graph without edges.
                #[must_use]
                $vis const fn new() -> Self {
                    let () = Self::__VALID_CONFIG;
                    Self {
                        heads: [const { None }; VERTICES],
                        edges: [const { None }; EDGE_CAP],
                        len: 0,
                    }
                }

                /* capacity */

                /// Returns the number of vertices in the graph domain.
                #[must_use]
                $vis const fn vertex_count(&self) -> usize { VERTICES }

                /// Returns the number of edges.
                #[must_use]
                $vis const fn edge_count(&self) -> usize { self.len }

                /// Returns the maximum number of edges.
                #[must_use]
                $vis const fn edge_capacity(&self) -> usize { EDGE_CAP }

                /// Returns how many additional edges can be added.
                #[must_use]
                $vis const fn edge_remaining(&self) -> usize { EDGE_CAP - self.len }

                /// Returns whether the graph contains no edges.
                #[must_use]
                $vis const fn is_empty(&self) -> bool { self.len == 0 }

                /// Returns whether no additional edge can be added.
                #[must_use]
                $vis const fn is_full(&self) -> bool { self.len == EDGE_CAP }

                /* handles */

                /// Returns the vertex at `index`.
                #[must_use]
                $vvis const fn vertex(&self, index: usize) -> Option<$Vertex> {
                    if index >= VERTICES { return None; }
                    $crate::unwrap![ok_some $Vertex::try_from_usize(index)]
                }
                /// Returns the edge at `index` if it has been added.
                #[must_use]
                $evis const fn edge(&self, index: usize) -> Option<$Edge> {
                    if index >= self.len { return None; }
                    $crate::unwrap![ok_some $Edge::try_from_usize(index)]
                }

                /// Returns whether `vertex` belongs to this graph's vertex domain.
                #[must_use]
                $vis const fn contains_vertex(&self, vertex: $Vertex) -> bool {
                    $crate::unwrap![ok_or vertex.get_index_usize(), idx => idx < VERTICES, false]
                }
                /// Returns whether `edge` currently belongs to this graph.
                #[must_use]
                $vis const fn contains_edge(&self, edge: $Edge) -> bool {
                    $crate::unwrap![ok_or edge.get_index_usize(), idx => idx < self.len, false]
                }

                /* mutation */

                /// Adds the directed edge `from → to`.
                ///
                /// Returns its edge handle, or `None` if either vertex does not
                /// belong to this graph or no further edge can be stored.
                $evis const fn add_edge(&mut self, from: $Vertex, to: $Vertex) -> Option<$Edge> {
                    let from_index = match Self::__vertex_index(from) {
                        Some(index) => index,
                        None => return None,
                    };
                    if Self::__vertex_index(to).is_none() || self.is_full() { return None; }
                    let edge = match $Edge::try_from_usize(self.len) {
                        Ok(edge) => edge,
                        Err(_) => return None,
                    };
                    self.edges[self.len] = Some([<__ $Graph EdgeSlot>] {
                        target: to,
                        next: self.heads[from_index],
                    });
                    self.heads[from_index] = Some(edge);
                    self.len += 1;
                    Some(edge)
                }
                /// Removes all edges while preserving the vertex domain.
                ///
                /// Previously issued edge handles may resolve again after later
                /// insertion reuses their indices.
                $vis const fn clear(&mut self) {
                    $crate::whilst! { edge in 0..self.len; {
                        self.edges[edge] = None;
                    }}
                    $crate::whilst! { vertex in 0..VERTICES; {
                        self.heads[vertex] = None;
                    }}
                    self.len = 0;
                }

                /* adjacency */

                /// Returns the first outgoing edge of `vertex`.
                #[must_use]
                $evis const fn first_out_edge(&self, vertex: $Vertex) -> Option<$Edge> {
                    let index = $crate::unwrap![some? Self::__vertex_index(vertex)];
                    self.heads[index]
                }
                /// Returns the next outgoing edge in the same adjacency chain.
                #[must_use]
                $evis const fn next_out_edge(&self, edge: $Edge) -> Option<$Edge> {
                    let index = $crate::unwrap![some? self.__edge_index(edge)];
                    $crate::unwrap![some_map_into self.edges[index], |slot| slot.next]
                }
                /// Returns the target vertex of `edge`.
                #[must_use]
                $vvis const fn edge_target(&self, edge: $Edge) -> Option<$Vertex> {
                    let index = $crate::unwrap![some? self.__edge_index(edge)];
                    $crate::unwrap![some_map self.edges[index], |slot| slot.target]
                }

                /* queries */

                /// Returns whether the directed edge `from → to` exists.
                #[must_use]
                $vis const fn has_edge(&self, from: $Vertex, to: $Vertex) -> bool {
                    let to_index = $crate::unwrap![some_or? Self::__vertex_index(to), false];
                    let mut edge = self.first_out_edge(from);
                    while let Some(current) = edge {
                        let target = $crate::unwrap![some_or? self.edge_target(current), false];
                        let index = $crate::unwrap![some_or? Self::__vertex_index(target), false];
                        if index == to_index { return true; }
                        edge = self.next_out_edge(current);
                    }
                    false
                }
                /// Returns the number of outgoing edges of `vertex`.
                ///
                /// Returns `None` if `vertex` is outside the graph domain.
                #[must_use]
                $vis const fn out_degree(&self, vertex: $Vertex) -> Option<usize> {
                    if Self::__vertex_index(vertex).is_none() { return None; }
                    let mut count = 0;
                    let mut edge = self.first_out_edge(vertex);
                    while let Some(current) = edge {
                        count += 1;
                        edge = self.next_out_edge(current);
                    }
                    Some(count)
                }
                /// Returns the number of incoming edges of `vertex`.
                ///
                /// Parallel edges are counted independently.
                /// Returns `None` if `vertex` lies outside the graph domain.
                #[must_use]
                $vis const fn in_degree(&self, vertex: $Vertex) -> Option<usize> {
                    let target = $crate::unwrap![some? Self::__vertex_index(vertex)];
                    let mut count = 0;
                    $crate::whilst!{ edge in 0..self.len; {
                        if let Some(slot) = self.edges[edge] {
                            let index = $crate::unwrap![some? Self::__vertex_index(slot.target)];
                            if index == target { count += 1; }
                        }
                    }}
                    Some(count)
                }
                /// Returns whether `to` is reachable from `from`, using caller-provided scratch.
                ///
                /// Reachability is reflexive: every valid vertex is reachable from itself
                /// through a path of zero edges.
                ///
                /// `scratch` is used as a discovered-vertex queue and must contain at least
                /// [`vertex_count`](#method.vertex_count) entries.
                /// Its previous contents are ignored.
                ///
                /// Returns `None` if either endpoint lies outside the graph domain or if
                /// `scratch` is too small.
                #[must_use]
                $vis const fn is_reachable_in(&self, from: $Vertex, to: $Vertex,
                    scratch: &mut [Option<$Vertex>]) -> Option<bool> {
                    let from_index = $crate::unwrap![some? Self::__vertex_index(from)];
                    let to_index = $crate::unwrap![some? Self::__vertex_index(to)];
                    if scratch.len() < VERTICES { return None; }
                    if from_index == to_index { return Some(true); }
                    scratch[0] = Some(from);
                    let (mut read, mut queued) = (0, 1);
                    while read < queued {
                        let vertex = $crate::unwrap![some? scratch[read]];
                        read += 1;
                        let mut edge = self.first_out_edge(vertex);
                        while let Some(current) = edge {
                            let target = $crate::unwrap![some? self.edge_target(current)];
                            let target_index = $crate::unwrap![some? Self::__vertex_index(target)];
                            if target_index == to_index { return Some(true); }
                            let mut seen = false;
                            $crate::whilst! { index in 0..queued; {
                                let seen_vertex = $crate::unwrap![some? scratch[index]];
                                let seen_index = $crate::unwrap![some?
                                    Self::__vertex_index(seen_vertex)];
                                if seen_index == target_index { seen = true; break; }
                            }}
                            if !seen {
                                scratch[queued] = Some(target);
                                queued += 1;
                            }
                            edge = self.next_out_edge(current);
                        }
                    }
                    Some(false)
                }
                /// Returns whether `to` is reachable from `from`.
                ///
                /// Reachability is reflexive: every valid vertex is reachable from itself
                /// through a path of zero edges.
                #[must_use]
                $vis const fn is_reachable(&self, from: $Vertex, to: $Vertex) -> bool {
                    let mut scratch = [None; VERTICES];
                    $crate::unwrap![some_or self.is_reachable_in(from, to, &mut scratch), false]
                }
                /// Returns whether the graph is acyclic, using caller-provided scratch.
                ///
                /// `scratch` stores the remaining incoming degree of each vertex while
                /// performing Kahn elimination. It must contain at least
                /// [`vertex_count`](#method.vertex_count) entries.
                /// Its previous contents are ignored.
                ///
                /// Returns `None` if `scratch` is too small.
                #[must_use]
                $vis const fn is_acyclic_in(&self, scratch: &mut [Option<usize>]) -> Option<bool> {
                    if scratch.len() < VERTICES { return None; }
                    $crate::whilst! { vertex in 0..VERTICES; {
                        scratch[vertex] = Some(0);
                    }}
                    // Compute incoming degrees.
                    $crate::whilst! { edge in 0..self.len; {
                        let slot = $crate::unwrap![some? self.edges[edge]];
                        let target = $crate::unwrap![some? Self::__vertex_index(slot.target)];
                        let degree = $crate::unwrap![some? scratch[target]];
                        scratch[target] = Some(degree + 1);
                    }}
                    // Repeatedly remove the lowest-index zero-indegree vertex.
                    let mut removed = 0;
                    while removed < VERTICES {
                        $crate::whilst! { source in 0..VERTICES; {
                            if matches!(scratch[source], Some(0)) { break; }
                        }}
                        if source == VERTICES { return Some(false); }
                        scratch[source] = None;
                        removed += 1;
                        let source = $crate::unwrap![some? self.vertex(source)];
                        let mut edge = self.first_out_edge(source);
                        while let Some(current) = edge {
                            let target = $crate::unwrap![some? self.edge_target(current)];
                            let target = $crate::unwrap![some? Self::__vertex_index(target)];
                            if let Some(degree) = scratch[target] {
                                scratch[target] = Some(degree - 1);
                            }
                            edge = self.next_out_edge(current);
                        }
                    }
                    Some(true)
                }
                /// Returns whether the graph contains no directed cycle.
                #[must_use]
                $vis const fn is_acyclic(&self) -> bool {
                    let mut scratch = [Some(0); VERTICES];
                    $crate::unwrap![some_or self.is_acyclic_in(&mut scratch), false]
                }

                /* runtime iteration */

                /// Iterates over `(edge, target)` for outgoing edges of `vertex`.
                $vis fn out_edges(&self, vertex: $Vertex)
                    -> impl Iterator<Item = ($Edge, $Vertex)> + '_ {
                    let mut next = self.first_out_edge(vertex);
                    ::core::iter::from_fn(move || {
                        let edge = next?;
                        let target = self.edge_target(edge)?;
                        next = self.next_out_edge(edge);
                        Some((edge, target))
                    })
                }
                /// Iterates over target vertices of outgoing edges of `vertex`.
                $vis fn neighbors(&self, vertex: $Vertex) -> impl Iterator<Item = $Vertex> + '_ {
                    self.out_edges(vertex).map(|(_, target)| target)
                }

                /* private */

                const fn __vertex_index(vertex: $Vertex) -> Option<usize> {
                    $crate::unwrap![ok_or vertex.get_index_usize(),
                        index => $crate::is![index < VERTICES, Some(index), None], None]
                }
                const fn __edge_index(&self, edge: $Edge) -> Option<usize> {
                    $crate::unwrap![ok_or edge.get_index_usize(),
                        index => $crate::is![index < self.len, Some(index), None], None]
                }
                const fn __vertex_index_capacity() -> usize {
                    $crate::unwrap![ok_or $crate::MaybeNiche::<$VertexIndex>::MAX.try_to_usize(),
                        max => max.saturating_add(1), usize::MAX]
                }
                const fn __edge_index_capacity() -> usize {
                    $crate::unwrap![ok_or $crate::MaybeNiche::<$EdgeIndex>::MAX.try_to_usize(),
                        max => max.saturating_add(1), usize::MAX]
                }
            }
        }
    };
}
