//
//! Defines [`graph_csr!`].
//

#[doc = crate::_tags!(construction data_structure topol)]
/// Defines a directed graph in compressed sparse row representation.
#[doc = crate::_doc_meta!{
    location("data/topol/graph", macro graph_csr),
}]
///
/// The graph stores connectivity only. Vertex and edge application
/// data can be associated externally through the generated handles.
///
/// Edges are directed. Self-loops, parallel edges, and cycles are allowed.
///
/// Outgoing edges of each vertex occupy one contiguous range. Edge handles
/// identify positions in this packed global edge sequence.
///
/// The graph is immutable after construction. Its canonical representation
/// consists of the row starts and target vertices accepted by `from_parts()`.
///
/// `None` in the starts array represents the one-past edge offset:
/// `EDGES` for static graphs, or `targets.len()` for allocating graphs.
///
/// `static` uses fixed arrays and is the default. `alloc` uses dynamic storage.
///
/// # Examples
/// ```
/// use devela::{NonMaxU8, NonMaxU16, graph_csr};
///
/// graph_csr! {
///     [
///         vertex: u8 + NonMaxU8;
///         edge: u16 + NonMaxU16;
///     ]
///     pub MyGraph;
///     pub MyVertex;
///     pub MyEdge;
/// }
///
/// let v0 = MyVertex::try_from_usize(0).unwrap();
/// let v1 = MyVertex::try_from_usize(1).unwrap();
/// let e0 = MyEdge::try_from_usize(0).unwrap();
///
/// let graph = MyGraph::<2, 1>::from_parts(
///     [Some(e0), None],
///     [v1],
/// ).unwrap();
///
/// assert!(graph.has_edge(v0, v1));
/// ```
/// See also:
/// - [`GraphCsrExample`], [`GraphCsrVertexExample`], [`GraphCsrEdgeExample`],
/// - [`GraphCsrAllocExample`], [`GraphCsrAllocVertexExample`], [`GraphCsrAllocEdgeExample`].
///
/// [`GraphCsrExample`]: crate::GraphCsrExample
/// [`GraphCsrVertexExample`]: crate::GraphCsrVertexExample
/// [`GraphCsrEdgeExample`]: crate::GraphCsrEdgeExample
/// [`GraphCsrAllocExample`]: crate::GraphCsrAllocExample
/// [`GraphCsrAllocVertexExample`]: crate::GraphCsrAllocVertexExample
/// [`GraphCsrAllocEdgeExample`]: crate::GraphCsrAllocEdgeExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! graph_csr· {
    (
        [
            vertex: $vprim:ident $(+ $VertexIndex:ty)?;
            edge: $eprim:ident $(+ $EdgeIndex:ty)?;
        ]

        $(#[$graph_attr:meta])*
        $vis:vis $Graph:ident $( : $kind:ident)?;

        $(#[$vertex_attr:meta])*
        $vvis:vis $Vertex:ident;

        $(#[$edge_attr:meta])*
        $evis:vis $Edge:ident $(;)?
    ) => {
        $crate::__graph_csr! { %normalize_vertex
            [kind: $($kind)?]
            [vertex: $vprim $(+ $VertexIndex)?]
            [edge: $eprim $(+ $EdgeIndex)?]
            [graph: $(#[$graph_attr])* $vis $Graph]
            [vertex_handle: $(#[$vertex_attr])* $vvis $Vertex]
            [edge_handle: $(#[$edge_attr])* $evis $Edge]
        }
    };
}
#[doc(inline)]
pub use graph_csr· as graph_csr;
