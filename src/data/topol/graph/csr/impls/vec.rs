// devela/src/data/topol/graph/csr/impls/vec.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __graph_csr_impl_vec {
    (
        [vertex: $vprim:ident + $VertexIndex:ty;]
        [edge: $eprim:ident + $EdgeIndex:ty;]
        $(#[$graph_attr:meta])* $vis:vis $Graph:ident;
        $vvis:vis $Vertex:ident;
        $evis:vis $Edge:ident;
    ) => {
        $(#[$graph_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Graph {
            starts: $crate::Vec<Option<$Edge>>,
            targets: $crate::Vec<$Vertex>,
        }

        #[allow(dead_code)]
        impl $Graph {
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
            };

            /* construction */

            /// Constructs a graph from its CSR row starts and edge targets.
            ///
            /// `None` in `starts` encodes the one-past edge offset
            /// `targets.len()`.
            ///
            /// Returns `None` if:
            /// - the vertex or edge domain exceeds its configured representation;
            /// - the first row does not start at edge offset 0;
            /// - row starts are not monotonically non-decreasing;
            /// - a row start is outside the edge domain;
            /// - a target is outside the vertex domain;
            /// - edges are supplied for an empty vertex domain.
            #[must_use]
            $vis fn from_parts(starts: $crate::Vec<Option<$Edge>>, targets: $crate::Vec<$Vertex>)
                -> Option<Self> {
                let () = Self::_VALID_CONFIG;
                let vertices = starts.len();
                let edges = targets.len();
                if vertices > Self::_vertex_index_capacity()
                    || edges > Self::_edge_index_capacity()
                {
                    return None;
                }
                if vertices == 0 {
                    if edges != 0 { return None; }
                    return Some(Self { starts, targets });
                }
                let mut previous = 0;
                for (vertex, &raw) in starts.iter().enumerate() {
                    let start = $crate::unwrap![some? Self::_start_index(edges, raw)];
                    if vertex == 0 && start != 0 { return None; }
                    if start < previous { return None; }
                    previous = start;
                }
                for &target in targets.iter() {
                    if Self::_vertex_index_in(vertices, target).is_none() { return None; }
                }
                Some(Self { starts, targets })
            }

            /// Borrows the canonical CSR representation.
            #[must_use]
            $vis fn as_parts(&self) -> (&[Option<$Edge>], &[$Vertex]) {
                (&self.starts, &self.targets)
            }
            /// Decomposes the graph into its canonical CSR representation.
            #[must_use]
            $vis fn into_parts(self) -> ($crate::Vec<Option<$Edge>>, $crate::Vec<$Vertex>) {
                (self.starts, self.targets)
            }

            /* size */

            /// Returns the number of vertices.
            #[must_use]
            $vis const fn vertex_count(&self) -> usize { self.starts.len() }

            /// Returns the number of edges.
            #[must_use]
            $vis const fn edge_count(&self) -> usize { self.targets.len() }

            /// Returns whether the graph contains no edges.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.targets.is_empty() }

            /* handles */

            /// Returns the vertex at `index`.
            #[must_use]
            $vvis const fn vertex(&self, index: usize) -> Option<$Vertex> {
                if index >= self.starts.len() { return None; }
                $crate::unwrap![ok_some $Vertex::try_from_usize(index)]
            }
            /// Returns the edge at `index`.
            #[must_use]
            $evis const fn edge(&self, index: usize) -> Option<$Edge> {
                if index >= self.targets.len() { return None; }
                $crate::unwrap![ok_some $Edge::try_from_usize(index)]
            }
            /// Returns whether `vertex` resolves in this graph's vertex domain.
            #[must_use]
            $vis const fn contains_vertex(&self, vertex: $Vertex) -> bool {
                $crate::unwrap![ok_or vertex.get_index_usize(),
                    index => index < self.starts.len(), false]
            }
            /// Returns whether `edge` resolves in this graph's edge domain.
            #[must_use]
            $vis const fn contains_edge(&self, edge: $Edge) -> bool {
                $crate::unwrap![ok_or edge.get_index_usize(),
                    index => index < self.targets.len(), false]
            }

            /* adjacency */

            /// Returns the half-open global edge bounds `(start, end)`
            /// for the outgoing edges of `vertex`.
            #[must_use]
            $vis fn out_edge_bounds(&self, vertex: $Vertex) -> Option<(usize, usize)> {
                let vertex = $crate::unwrap![some? self._vertex_index(vertex)];
                let start = $crate::unwrap![some?
                    Self::_start_index(self.targets.len(), self.starts[vertex])];
                let end = if vertex + 1 < self.starts.len() {
                    $crate::unwrap![some?
                        Self::_start_index(self.targets.len(), self.starts[vertex + 1])]
                } else {
                    self.targets.len()
                };
                Some((start, end))
            }
            /// Returns the target vertex of `edge`.
            #[must_use]
            $vvis fn edge_target(&self, edge: $Edge) -> Option<$Vertex> {
                let edge = $crate::unwrap![some? self._edge_index(edge)];
                Some(self.targets[edge])
            }

            /* queries */

            /// Returns whether the directed edge `from → to` exists.
            #[must_use]
            $vis fn has_edge(&self, from: $Vertex, to: $Vertex) -> bool {
                let to = $crate::unwrap![some_or? self._vertex_index(to), false];
                let (start, end) = $crate::unwrap![some_or? self.out_edge_bounds(from), false];
                $crate::whilst! { edge in start,..end; {
                    let target =
                        $crate::unwrap![some_or? self._vertex_index(self.targets[edge]), false];
                    if target == to { return true; }
                }}
                false
            }

            /// Returns the number of outgoing edges of `vertex`.
            #[must_use]
            $vis fn out_degree(&self, vertex: $Vertex) -> Option<usize> {
                let (start, end) = $crate::unwrap![some? self.out_edge_bounds(vertex)];
                Some(end - start)
            }
            /// Returns the number of incoming edges of `vertex`.
            #[must_use]
            $vis fn in_degree(&self, vertex: $Vertex) -> Option<usize> {
                let target = $crate::unwrap![some? self._vertex_index(vertex)];
                let mut count = 0;
                $crate::whilst! { edge in 0..self.targets.len(); {
                    let index = $crate::unwrap![some? self._vertex_index(self.targets[edge])];
                    if index == target { count += 1; }
                }}
                Some(count)
            }
            /// Returns whether `to` is reachable from `from`, using caller-provided scratch.
            #[must_use]
            $vis fn is_reachable_in(&self, from: $Vertex, to: $Vertex,
                scratch: &mut [Option<$Vertex>]) -> Option<bool> {
                let from_index = $crate::unwrap![some? self._vertex_index(from)];
                let to_index = $crate::unwrap![some? self._vertex_index(to)];
                let vertices = self.vertex_count();
                if scratch.len() < vertices { return None; }
                if from_index == to_index { return Some(true); }
                scratch[0] = Some(from);
                let (mut read, mut queued) = (0, 1);
                while read < queued {
                    let vertex = $crate::unwrap![some? scratch[read]];
                    read += 1;
                    let (start, end) = $crate::unwrap![some? self.out_edge_bounds(vertex)];
                    $crate::whilst! { edge in start,..end; {
                        let target = self.targets[edge];
                        let target_index = $crate::unwrap![some? self._vertex_index(target)];
                        if target_index == to_index { return Some(true); }
                        let mut seen = false;
                        $crate::whilst! { index in 0..queued; {
                            let seen_vertex = $crate::unwrap![some? scratch[index]];
                            let seen_index = $crate::unwrap![some? self._vertex_index(seen_vertex)];
                            if seen_index == target_index { seen = true; break; }
                        }}
                        if !seen {
                            scratch[queued] = Some(target);
                            queued += 1;
                        }
                    }}
                }
                Some(false)
            }
            /// Returns whether the graph is acyclic, using caller-provided scratch.
            #[must_use]
            $vis fn is_acyclic_in(&self, scratch: &mut [Option<usize>]) -> Option<bool> {
                let vertices = self.vertex_count();
                if scratch.len() < vertices { return None; }
                $crate::whilst! { vertex in 0..vertices; {
                    scratch[vertex] = Some(0);
                }}
                $crate::whilst! { edge in 0..self.targets.len(); {
                    let target = $crate::unwrap![some? self._vertex_index(self.targets[edge])];
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
                    let source_vertex = $crate::unwrap![some? self.vertex(source)];
                    let (start, end) = $crate::unwrap![some? self.out_edge_bounds(source_vertex)];
                    $crate::whilst! { edge in start,..end; {
                        let target = $crate::unwrap![some? self._vertex_index(self.targets[edge])];
                        if let Some(degree) = scratch[target] {
                            scratch[target] = Some(degree - 1);
                        }
                    }}
                }
                Some(true)
            }

            /* iteration */

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

            const fn _start_index(edge_count: usize, start: Option<$Edge>) -> Option<usize> {
                match start {
                    None => Some(edge_count),
                    Some(edge) => $crate::unwrap![ok_or edge.get_index_usize(),
                        index => $crate::is![index < edge_count, Some(index), None], None],
                }
            }
            const fn _vertex_index_in(vertex_count: usize, vertex: $Vertex) -> Option<usize> {
                $crate::unwrap![ok_or vertex.get_index_usize(),
                    index => $crate::is![index < vertex_count, Some(index), None], None]
            }
            const fn _vertex_index(&self, vertex: $Vertex) -> Option<usize> {
                Self::_vertex_index_in(self.starts.len(), vertex)
            }
            const fn _edge_index(&self, edge: $Edge) -> Option<usize> {
                $crate::unwrap![ok_or edge.get_index_usize(),
                    index => $crate::is![index < self.targets.len(), Some(index), None], None]
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
