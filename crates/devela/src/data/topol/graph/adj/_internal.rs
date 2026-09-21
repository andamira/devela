//
//! Defines [`__graph_adj!`].
//

/// Private API of [`graph_adj`][crate::graph_adj].
#[doc(hidden)]
#[macro_export]
macro_rules! __graph_adj· {
    /* normalize representations */

    (%normalize_vertex
        [kind: $($kind:ident)?]
        [vertex: $vprim:ident]
        $($rest:tt)*
    ) => {
        $crate::__graph_adj! {
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
        $crate::__graph_adj! {
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
        $crate::__graph_adj! {
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
        $crate::__graph_adj! {
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

        $crate::__graph_adj! { %backend
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
        $crate::__graph_adj! { %backend [kind: static] $($rest)* }
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
#[doc(hidden)]
pub use __graph_adj· as __graph_adj;
