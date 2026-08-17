// devela/src/data/topol/graph/csr/impls/array.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __graph_csr_impl_array {
    (
        [vertex: $vprim:ident + $VertexIndex:ty;]
        [edge: $eprim:ident + $EdgeIndex:ty;]
        $(#[$graph_attr:meta])* $vis:vis $Graph:ident;
        $vvis:vis $Vertex:ident;
        $evis:vis $Edge:ident;
    ) => {
        $(#[$graph_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Graph<const VERTICES: usize, const EDGES: usize> {
            starts: [Option<$Edge>; VERTICES],
            targets: [$Vertex; EDGES],
        }

        #[allow(dead_code)]
        impl<const VERTICES: usize, const EDGES: usize> $Graph<VERTICES, EDGES> {
            /* configuration */

            const _VALID_CONFIG: () = {
                const fn _index_primitive<P: $crate::PrimIndex>() {}
                _index_primitive::<$vprim>();
                _index_primitive::<$eprim>();
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
                assert!(VERTICES <= Self::_vertex_index_capacity(),
                    "the graph vertex count exceeds its vertex representation");
                assert!(EDGES <= Self::_edge_index_capacity(),
                    "the graph edge count exceeds its edge representation");
            };

            /* construction */

            /// Constructs a graph from its CSR row starts and edge targets.
            ///
            /// `None` in `starts` encodes the one-past edge offset `EDGES`.
            ///
            /// Returns `None` if:
            /// - the first row does not start at edge offset 0;
            /// - row starts are not monotonically non-decreasing;
            /// - a row start is outside the edge domain;
            /// - a target is outside the vertex domain;
            /// - edges are supplied for an empty vertex domain.
            #[must_use]
            $vis const fn from_parts(starts: [Option<$Edge>; VERTICES], targets: [$Vertex; EDGES])
                -> Option<Self> {
                let () = Self::_VALID_CONFIG;
                if VERTICES == 0 {
                    if EDGES != 0 { return None; }
                    return Some(Self { starts, targets });
                }
                let mut previous = 0;
                $crate::whilst! { vertex in 0..VERTICES; {
                    let start = $crate::unwrap![some? Self::_start_index(starts[vertex])];
                    if vertex == 0 && start != 0 { return None; }
                    if start < previous { return None; }
                    previous = start;
                }}
                $crate::whilst! { edge in 0..EDGES; {
                    if Self::_vertex_index(targets[edge]).is_none() { return None; }
                }}
                Some(Self { starts, targets })
            }
            /// Borrows the canonical CSR representation.
            #[must_use]
            $vis const fn as_parts(&self) -> (&[Option<$Edge>; VERTICES], &[$Vertex; EDGES]) {
                (&self.starts, &self.targets)
            }
            /// Decomposes the graph into its canonical CSR representation.
            ///
            /// The returned arrays can be passed to [`from_parts`](Self::from_parts)
            /// to reconstruct an equivalent graph.
            #[must_use]
            $vis const fn into_parts(self) -> ([Option<$Edge>; VERTICES], [$Vertex; EDGES]) {
                (self.starts, self.targets)
            }

            /* size */

            /// Returns the number of vertices.
            #[must_use]
            $vis const fn vertex_count(&self) -> usize { VERTICES }

            /// Returns the number of edges.
            #[must_use]
            $vis const fn edge_count(&self) -> usize { EDGES }

            /// Returns whether the graph contains no edges.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { EDGES == 0 }

            /* handles */

            /// Returns the vertex at `index`.
            #[must_use]
            $vvis const fn vertex(&self, index: usize) -> Option<$Vertex> {
                if index >= VERTICES { return None; }
                $crate::unwrap![ok_some $Vertex::try_from_usize(index)]
            }
            /// Returns the edge at `index`.
            #[must_use]
            $evis const fn edge(&self, index: usize) -> Option<$Edge> {
                if index >= EDGES { return None; }
                $crate::unwrap![ok_some $Edge::try_from_usize(index)]
            }
            /// Returns whether `vertex` belongs to this graph's vertex domain.
            #[must_use]
            $vis const fn contains_vertex(&self, vertex: $Vertex) -> bool {
                Self::_vertex_index(vertex).is_some()
            }
            /// Returns whether `edge` belongs to this graph.
            #[must_use]
            $vis const fn contains_edge(&self, edge: $Edge) -> bool {
                Self::_edge_index(edge).is_some()
            }

            /* adjacency */

            /// Returns the half-open global edge bounds `(start, end)`
            /// for the outgoing edges of `vertex`.
            ///
            /// Empty rows return equal bounds. Returns `None` if `vertex`
            /// lies outside the graph domain.
            #[must_use]
            $vis const fn out_edge_bounds(&self, vertex: $Vertex) -> Option<(usize, usize)> {
                let vertex = $crate::unwrap![some? Self::_vertex_index(vertex)];
                let start = $crate::unwrap![some? Self::_start_index(self.starts[vertex])];
                let end = if vertex + 1 < VERTICES {
                    $crate::unwrap![some? Self::_start_index(self.starts[vertex + 1])]
                } else {
                    EDGES
                };
                Some((start, end))
            }
            /// Returns the target vertex of `edge`.
            #[must_use]
            $vvis const fn edge_target(&self, edge: $Edge) -> Option<$Vertex> {
                let edge = $crate::unwrap![some? Self::_edge_index(edge)];
                Some(self.targets[edge])
            }

            /* queries */

            /// Returns whether the directed edge `from → to` exists.
            #[must_use]
            $vis const fn has_edge(&self, from: $Vertex, to: $Vertex) -> bool {
                let to = $crate::unwrap![some_or? Self::_vertex_index(to), false];
                let (start, end) = $crate::unwrap![some_or? self.out_edge_bounds(from), false];
                $crate::whilst! { edge in start,..end; {
                    let target =
                        $crate::unwrap![some_or? Self::_vertex_index(self.targets[edge]), false];
                    if target == to { return true; }
                }}
                false
            }
            /// Returns the number of outgoing edges of `vertex`.
            ///
            /// Returns `None` if `vertex` lies outside the graph domain.
            #[must_use]
            $vis const fn out_degree(&self, vertex: $Vertex) -> Option<usize> {
                let (start, end) = $crate::unwrap![some? self.out_edge_bounds(vertex)];
                Some(end - start)
            }

            /* runtime iteration */

            /// Iterates over `(edge, target)` for outgoing edges of `vertex`.
            $vis fn out_edges(&self, vertex: $Vertex)
                -> impl Iterator<Item = ($Edge, $Vertex)> + '_ {
                let (mut edge, end) = self.out_edge_bounds(vertex).unwrap_or((0, 0));
                ::core::iter::from_fn(move || {
                    if edge >= end { return None; }
                    let handle = $Edge::try_from_usize(edge).ok()?;
                    let target = self.targets[edge];
                    edge += 1;
                    Some((handle, target))
                })
            }
            /// Iterates over target vertices of outgoing edges of `vertex`.
            $vis fn neighbors(&self, vertex: $Vertex) -> impl Iterator<Item = $Vertex> + '_ {
                self.out_edges(vertex).map(|(_, target)| target)
            }

            /* private */

            const fn _start_index(start: Option<$Edge>) -> Option<usize> {
                match start {
                    None => Some(EDGES),
                    Some(edge) => $crate::unwrap![ok_or edge.index_usize(),
                        index => $crate::is![index < EDGES, Some(index), None], None],
                }
            }
            const fn _vertex_index(vertex: $Vertex) -> Option<usize> {
                $crate::unwrap![ok_or vertex.index_usize(),
                    index => $crate::is![index < VERTICES, Some(index), None], None]
            }
            const fn _edge_index(edge: $Edge) -> Option<usize> {
                $crate::unwrap![ok_or edge.index_usize(),
                    index => $crate::is![index < EDGES, Some(index), None], None]
            }
            const fn _vertex_index_capacity() -> usize {
                $crate::unwrap![ok_or $crate::MaybeNiche::<$VertexIndex>::MAX.try_to_usize(),
                    max => max.saturating_add(1), usize::MAX]
            }
            const fn _edge_index_capacity() -> usize {
                $crate::unwrap![ok_or $crate::MaybeNiche::<$EdgeIndex>::MAX.try_to_usize(),
                    max => max.saturating_add(1), usize::MAX]
            }
        }
    };
}
