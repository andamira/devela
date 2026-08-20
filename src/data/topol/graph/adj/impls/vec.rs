#[doc(hidden)]
#[macro_export]
macro_rules! __graph_adj_impl_vec {
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
            $vis struct $Graph {
                heads: $crate::Vec<Option<$Edge>>,
                edges: $crate::Vec<[<__ $Graph EdgeSlot>]>,
            }

            #[allow(dead_code)]
            impl $Graph {
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
                };

                /* construction */

                /// Returns an empty graph over `vertex_count` vertices.
                ///
                /// # Panics
                /// Panics if `vertex_count` exceeds the configured vertex
                /// representation or if allocation fails.
                #[must_use]
                $vis fn new(vertex_count: usize) -> Self { Self::with_capacity(vertex_count, 0) }

                /// Returns an empty graph with edge storage reserved for at
                /// least `edge_capacity` edges.
                ///
                /// # Panics
                /// Panics if either requested size exceeds its configured
                /// representation or if allocation fails.
                #[must_use]
                $vis fn with_capacity(vertex_count: usize, edge_capacity: usize) -> Self {
                    let () = Self::__VALID_CONFIG;
                    assert!(vertex_count <= Self::__vertex_index_capacity(),
                        "the requested graph vertex count exceeds its vertex representation");
                    assert!(edge_capacity <= Self::__edge_index_capacity(),
                        "the requested graph edge capacity exceeds its edge representation");
                    let mut heads = $crate::Vec::<Option<$Edge>>::with_capacity(vertex_count);
                    heads.resize(vertex_count, None);
                    Self {
                        heads,
                        edges: $crate::Vec::<[<__ $Graph EdgeSlot>]>::with_capacity(edge_capacity),
                    }
                }

                /* capacity */

                /// Returns the number of vertices in the graph domain.
                #[must_use]
                $vis const fn vertex_count(&self) -> usize { self.heads.len() }

                /// Returns the number of edges.
                #[must_use]
                $vis const fn edge_count(&self) -> usize { self.edges.len() }

                /// Returns the usable edge capacity available without reallocating.
                #[must_use]
                $vis const fn edge_capacity(&self) -> usize {
                    $crate::Cmp(self.edges.capacity()).min(Self::__edge_index_capacity())
                }
                /// Returns how many more edges fit without reallocating.
                ///
                /// The graph may grow when this reaches zero unless
                /// [`is_full`][Self::is_full] is true.
                #[must_use]
                $vis const fn edge_remaining(&self) -> usize {
                    self.edge_capacity() - self.edge_count()
                }

                /// Returns whether the graph contains no edges.
                #[must_use]
                $vis const fn is_empty(&self) -> bool { self.edges.is_empty() }

                /// Returns whether no further edge index can be represented.
                #[must_use]
                $vis const fn is_full(&self) -> bool {
                    $crate::MaybeNiche::<$EdgeIndex>::try_from_usize(self.edges.len()).is_err()
                }

                /* handles */

                /// Returns the vertex at `index`.
                #[must_use]
                $vvis const fn vertex(&self, index: usize) -> Option<$Vertex> {
                    if index >= self.heads.len() { return None; }
                    $crate::unwrap![ok_some $Vertex::try_from_usize(index)]
                }

                /// Returns the edge at `index` if it has been added.
                #[must_use]
                $evis const fn edge(&self, index: usize) -> Option<$Edge> {
                    if index >= self.edges.len() { return None; }
                    $crate::unwrap![ok_some $Edge::try_from_usize(index)]
                }
                /// Returns whether `vertex` resolves in this graph's domain.
                #[must_use]
                $vis const fn contains_vertex(&self, vertex: $Vertex) -> bool {
                    $crate::unwrap![ok_or vertex.get_index_usize(), i => i < self.heads.len(), false]
                }
                /// Returns whether `edge` currently resolves in this graph.
                #[must_use]
                $vis const fn contains_edge(&self, edge: $Edge) -> bool {
                    $crate::unwrap![ok_or edge.get_index_usize(), idx => idx < self.edges.len(), false]
                }

                /* mutation */

                /// Adds the directed edge `from → to`.
                ///
                /// Returns `None` if either endpoint lies outside the vertex
                /// domain or no further edge index can be represented.
                $evis fn add_edge(&mut self, from: $Vertex, to: $Vertex) -> Option<$Edge> {
                    let from_index = $crate::unwrap![some? self.__vertex_index(from)];
                    if self.__vertex_index(to).is_none() || self.is_full() { return None; }
                    let edge = $crate::unwrap![ok_some? $Edge::try_from_usize(self.edges.len())];
                    self.edges.push([<__ $Graph EdgeSlot>] {
                        target: to,
                        next: self.heads[from_index],
                    });
                    self.heads[from_index] = Some(edge);
                    Some(edge)
                }
                /// Removes every edge while preserving the vertex domain.
                ///
                /// Previously issued edge handles may resolve again after
                /// later insertion reuses their indices.
                $vis fn clear(&mut self) {
                    self.edges.clear();
                    for head in self.heads.iter_mut() {
                        *head = None;
                    }
                }

                /* adjacency */

                /// Returns the first outgoing edge of `vertex`.
                #[must_use]
                $evis fn first_out_edge(&self, vertex: $Vertex) -> Option<$Edge> {
                    let index = $crate::unwrap![some? self.__vertex_index(vertex)];
                    self.heads[index]
                }
                /// Returns the next outgoing edge in the same adjacency chain.
                #[must_use]
                $evis fn next_out_edge(&self, edge: $Edge) -> Option<$Edge> {
                    let index = $crate::unwrap![some? self.__edge_index(edge)];
                    self.edges[index].next
                }
                /// Returns the target vertex of `edge`.
                #[must_use]
                $vvis fn edge_target(&self, edge: $Edge) -> Option<$Vertex> {
                    let index = $crate::unwrap![some? self.__edge_index(edge)];
                    Some(self.edges[index].target)
                }

                /* queries */

                /// Returns whether the directed edge `from → to` exists.
                #[must_use]
                $vis fn has_edge(&self, from: $Vertex, to: $Vertex) -> bool {
                    let to_index = $crate::unwrap![some_or? self.__vertex_index(to), false];
                    let mut edge = self.first_out_edge(from);
                    while let Some(current) = edge {
                        let target = $crate::unwrap![some_or? self.edge_target(current), false];
                        let index = $crate::unwrap![some_or? self.__vertex_index(target), false];
                        if index == to_index { return true; }
                        edge = self.next_out_edge(current);
                    }
                    false
                }
                /// Returns the number of outgoing edges of `vertex`.
                ///
                /// Returns `None` if `vertex` lies outside the graph domain.
                #[must_use]
                $vis fn out_degree(&self, vertex: $Vertex) -> Option<usize> {
                    if self.__vertex_index(vertex).is_none() { return None; }
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
                $vis fn in_degree(&self, vertex: $Vertex) -> Option<usize> {
                    let target = $crate::unwrap![some? self.__vertex_index(vertex)];
                    let mut count = 0;
                    $crate::whilst! { edge in 0..self.edges.len(); {
                        let index = $crate::unwrap![some? self.__vertex_index(self.edges[edge].target)];
                        if index == target { count += 1; }
                    }}
                    Some(count)
                }
                /// Returns whether `to` is reachable from `from`, using caller-provided scratch.
                ///
                /// Reachability is reflexive. `scratch` must contain at least
                /// [`vertex_count`](#method.vertex_count) entries.
                ///
                /// Returns `None` if either endpoint is invalid or `scratch` is too small.
                #[must_use]
                $vis fn is_reachable_in(&self, from: $Vertex, to: $Vertex,
                    scratch: &mut [Option<$Vertex>]) -> Option<bool> {
                    let from_index = $crate::unwrap![some? self.__vertex_index(from)];
                    let to_index = $crate::unwrap![some? self.__vertex_index(to)];
                    if scratch.len() < self.vertex_count() { return None; }
                    if from_index == to_index { return Some(true); }
                    scratch[0] = Some(from);
                    let (mut read, mut queued) = (0, 1);
                    while read < queued {
                        let vertex = $crate::unwrap![some? scratch[read]];
                        read += 1;
                        let mut edge = self.first_out_edge(vertex);
                        while let Some(current) = edge {
                            let target = $crate::unwrap![some? self.edge_target(current)];
                            let target_index = $crate::unwrap![some? self.__vertex_index(target)];
                            if target_index == to_index { return Some(true); }
                            let mut seen = false;
                            $crate::whilst! { index in 0..queued; {
                                let seen_vertex = $crate::unwrap![some? scratch[index]];
                                let seen_index = $crate::unwrap![some? self.__vertex_index(seen_vertex)];
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
                /// Returns whether the graph is acyclic, using caller-provided scratch.
                ///
                /// `scratch` must contain at least [`vertex_count`](#method.vertex_count)
                /// entries. Its previous contents are ignored.
                ///
                /// Returns `None` if `scratch` is too small.
                #[must_use]
                $vis fn is_acyclic_in(&self, scratch: &mut [Option<usize>]) -> Option<bool> {
                    let vertices = self.vertex_count();
                    if scratch.len() < vertices { return None; }
                    $crate::whilst! { vertex in 0..vertices; {
                        scratch[vertex] = Some(0);
                    }}
                    $crate::whilst! { edge in 0..self.edges.len(); {
                        let target = $crate::unwrap![some? self.__vertex_index(self.edges[edge].target)];
                        let degree = $crate::unwrap![some? scratch[target]];
                        scratch[target] = Some(degree + 1);
                    }}
                    let mut removed = 0;
                    while removed < vertices {
                        $crate::whilst! { source in 0..vertices; {
                            if matches!(scratch[source], Some(0)) { break; }
                        }}
                        if source == vertices { return Some(false); }
                        scratch[source] = None;
                        removed += 1;
                        let source = $crate::unwrap![some? self.vertex(source)];
                        let mut edge = self.first_out_edge(source);
                        while let Some(current) = edge {
                            let target = $crate::unwrap![some? self.edge_target(current)];
                            let target = $crate::unwrap![some? self.__vertex_index(target)];
                            if let Some(degree) = scratch[target] {
                                scratch[target] = Some(degree - 1);
                            }
                            edge = self.next_out_edge(current);
                        }
                    }
                    Some(true)
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

                const fn __vertex_index(&self, vertex: $Vertex) -> Option<usize> {
                    $crate::unwrap![ok_or vertex.get_index_usize(),
                        index => $crate::is![index < self.heads.len(), Some(index), None], None]
                }
                const fn __edge_index(&self, edge: $Edge) -> Option<usize> {
                    $crate::unwrap![ok_or edge.get_index_usize(),
                        index => $crate::is![index < self.edges.len(), Some(index), None], None]
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
