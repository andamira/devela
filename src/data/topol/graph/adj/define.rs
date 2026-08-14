// devela/src/data/topol/graph/adj/define.rs
//
//! Defines [`graph_adj!`].
//

#[doc = crate::_tags!(construction data_structure topol)]
/// Defines a directed adjacency graph with static or allocating storage.
#[doc = crate::_doc_meta!{location("data/topol/graph")}]
///
/// The generated graph stores connectivity only. Vertex and edge application
/// data can be associated externally through the generated handles.
///
/// Edges are directed. Self-loops, parallel edges, and cycles are allowed.
///
/// New edges are prepended to each source vertex's adjacency chain, so outgoing
/// edges are traversed in reverse insertion order. Edge handles themselves are
/// assigned in global insertion order.
///
/// The vertex domain is fixed for the graph's lifetime: statically
/// through the vertex count, or at construction for the allocating backend.
///
/// Edge handles remain stable until `clear()`. Clearing invalidates them
/// contextually, later insertion may reuse the same handle values.
///
/// The initial representation is append-only:
/// individual edges cannot be removed, while `clear()` removes all edges.
///
/// `static` uses fixed arrays and is the default. `alloc` uses dynamic storage.
///
/// # Examples
/// ```
/// # use devela::{NonMaxU8, NonMaxU16, graph_adj};
/// graph_adj! {
///     [
///         vertex: u8 + NonMaxU8;
///         edge: u16 + NonMaxU16;
///     ]
///     pub MyGraph;
///     pub MyVertex;
///     pub MyEdge;
/// }
///
/// let mut graph = MyGraph::<4, 8>::new();
/// let a = graph.vertex(0).unwrap();
/// let b = graph.vertex(1).unwrap();
/// let edge = graph.add_edge(a, b).unwrap();
///
/// assert_eq!(graph.edge_target(edge), Some(b));
/// assert!(graph.has_edge(a, b));
/// ```
#[macro_export]
macro_rules! graph_adj {
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
        $crate::graph_adj! { %normalize_vertex
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
        $crate::graph_adj! {
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
        $crate::graph_adj! {
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
        $crate::graph_adj! {
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
        $crate::graph_adj! {
            %generate
            [kind: $($kind)?]
            [vertex: $vprim + $VertexIndex]
            [edge: $eprim + $EdgeIndex]
            $($rest)*
        }
    };

    /* generate shared handle family */

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

        $crate::graph_adj! { %backend
            [kind: $($kind)?]
            [vertex: $vprim + $VertexIndex]
            [edge: $eprim + $EdgeIndex]
            [graph: $(#[$graph_attr])* $vis $Graph]
            [vertex_handle: $vvis $Vertex]
            [edge_handle: $evis $Edge]
        }
    };

    /* backend dispatch */

    (%backend
        [kind:]
        $($rest:tt)*
    ) => {
        $crate::graph_adj! { %backend [kind: static] $($rest)* }
    };

    (%backend
        [kind: static]
        [vertex: $vprim:ident + $VertexIndex:ty]
        [edge: $eprim:ident + $EdgeIndex:ty]
        [graph: $(#[$graph_attr:meta])* $vis:vis $Graph:ident]
        [vertex_handle: $vvis:vis $Vertex:ident]
        [edge_handle: $evis:vis $Edge:ident]
    ) => {
        $crate::__graph_adj_impl_array! {
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
        $crate::__graph_adj_impl_vec! {
            [vertex: $vprim + $VertexIndex;]
            [edge: $eprim + $EdgeIndex;]
            $(#[$graph_attr])* $vis $Graph;
            $vvis $Vertex;
            $evis $Edge;
        }
    };
}
#[doc(inline)]
pub use graph_adj;

mod _all {
    #[doc(inline)]
    pub use super::graph_adj;
}
