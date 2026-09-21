//
//! Defines [`__graph_csr!`].
//

/// Private API of [`graph_csr`][crate::graph_csr].
#[doc(hidden)]
#[macro_export]
macro_rules! __graph_csr· {
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

    /* normalize representations */

    (%normalize_vertex
        [kind: $($kind:ident)?]
        [vertex: $vprim:ident]
        $($rest:tt)*
    ) => {
        $crate::__graph_csr! {
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
        $crate::__graph_csr! {
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
        $crate::__graph_csr! {
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
        $crate::__graph_csr! {
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
        $crate::__graph_csr! { %backend
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
        $crate::__graph_csr! { %backend [kind: static] $($rest)* }
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
#[doc(hidden)]
pub use __graph_csr· as __graph_csr;
