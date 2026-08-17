// devela/src/data/topol/graph/csr/define.rs
//
//! Defines [`graph_csr!`].
//

#[doc = crate::_tags!(construction data_structure topol)]
/// Defines a directed graph in compressed sparse row representation.
#[doc = crate::_doc_meta!{location("data/topol/graph")}]
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
/// # use devela::{NonMaxU8, NonMaxU16, graph_csr};
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
macro_rules! graph_csr {
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
        $crate::graph_csr! { %normalize_vertex
            [kind: $($kind)?]
            [vertex: $vprim $(+ $VertexIndex)?]
            [edge: $eprim $(+ $EdgeIndex)?]
            [graph: $(#[$graph_attr])* $vis $Graph]
            [vertex_handle: $(#[$vertex_attr])* $vvis $Vertex]
            [edge_handle: $(#[$edge_attr])* $evis $Edge]
        }
    };

    /* normalize representations */

    (%normalize_vertex
        [kind: $($kind:ident)?]
        [vertex: $vprim:ident]
        $($rest:tt)*
    ) => {
        $crate::graph_csr! {
            %normalize_edge
            [kind: $($kind)?]
            [vertex: $vprim + $vprim]
            $($rest)*
        }
    };
    (%normalize_vertex
        [kind: $($kind:ident)?]
        [vertex: $vprim:ident + $VertexIndex:ty]
        $($rest:tt)*
    ) => {
        $crate::graph_csr! {
            %normalize_edge
            [kind: $($kind)?]
            [vertex: $vprim + $VertexIndex]
            $($rest)*
        }
    };

    (%normalize_edge
        [kind: $($kind:ident)?]
        [vertex: $vprim:ident + $VertexIndex:ty]
        [edge: $eprim:ident]
        $($rest:tt)*
    ) => {
        $crate::graph_csr! {
            %generate
            [kind: $($kind)?]
            [vertex: $vprim + $VertexIndex]
            [edge: $eprim + $eprim]
            $($rest)*
        }
    };
    (%normalize_edge
        [kind: $($kind:ident)?]
        [vertex: $vprim:ident + $VertexIndex:ty]
        [edge: $eprim:ident + $EdgeIndex:ty]
        $($rest:tt)*
    ) => {
        $crate::graph_csr! {
            %generate
            [kind: $($kind)?]
            [vertex: $vprim + $VertexIndex]
            [edge: $eprim + $EdgeIndex]
            $($rest)*
        }
    };

    /* generate handle family */

    (%generate
        [kind: $($kind:ident)?]
        [vertex: $vprim:ident + $VertexIndex:ty]
        [edge: $eprim:ident + $EdgeIndex:ty]
        [graph: $(#[$graph_attr:meta])* $vis:vis $Graph:ident]
        [vertex_handle: $(#[$vertex_attr:meta])* $vvis:vis $Vertex:ident]
        [edge_handle: $(#[$edge_attr:meta])* $evis:vis $Edge:ident]
    ) => {
        $crate::handle! {
            [index: $vprim + $VertexIndex;]
            $(#[$vertex_attr])*
            $vvis $Vertex;
        }
        $crate::handle! {
            [index: $eprim + $EdgeIndex;]
            $(#[$edge_attr])*
            $evis $Edge;
        }
        $crate::graph_csr! { %backend
            [kind: $($kind)?]
            [vertex: $vprim + $VertexIndex]
            [edge: $eprim + $EdgeIndex]
            [graph: $(#[$graph_attr])* $vis $Graph]
            [vertex_handle: $vvis $Vertex]
            [edge_handle: $evis $Edge]
        }
    };

    /* backend dispatch */

    (%backend [kind:] $($rest:tt)*) => {
        $crate::graph_csr! { %backend [kind: static] $($rest)* }
    };
    (%backend
        [kind: static]
        [vertex: $vprim:ident + $VertexIndex:ty]
        [edge: $eprim:ident + $EdgeIndex:ty]
        [graph: $(#[$graph_attr:meta])* $vis:vis $Graph:ident]
        [vertex_handle: $vvis:vis $Vertex:ident]
        [edge_handle: $evis:vis $Edge:ident]
    ) => {
        $crate::__graph_csr_impl_array! {
            [vertex: $vprim + $VertexIndex;]
            [edge: $eprim + $EdgeIndex;]
            $(#[$graph_attr])* $vis $Graph;
            $vvis $Vertex;
            $evis $Edge;
        }
    };
    (%backend
        [kind: alloc]
        [vertex: $vprim:ident + $VertexIndex:ty]
        [edge: $eprim:ident + $EdgeIndex:ty]
        [graph: $(#[$graph_attr:meta])* $vis:vis $Graph:ident]
        [vertex_handle: $vvis:vis $Vertex:ident]
        [edge_handle: $evis:vis $Edge:ident]
    ) => {
        $crate::__graph_csr_impl_vec! {
            [vertex: $vprim + $VertexIndex;]
            [edge: $eprim + $EdgeIndex;]
            $(#[$graph_attr])* $vis $Graph;
            $vvis $Vertex;
            $evis $Edge;
        }
    };
}

#[doc(inline)]
pub use graph_csr;
