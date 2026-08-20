// devela/src/data/topol/graph/_test.rs

use crate::{
    GraphAdjExample as Adj, GraphCsrEdgeExample as CsrEdge, GraphCsrExample as Csr,
    GraphCsrVertexExample as CsrVertex,
};

fn csr_parts_from_adj<const VERTICES: usize, const EDGE_CAP: usize, const EDGES: usize>(
    graph: &Adj<VERTICES, EDGE_CAP>,
) -> Option<([Option<CsrEdge>; VERTICES], [CsrVertex; EDGES])> {
    if graph.edge_count() != EDGES {
        return None;
    }
    let mut starts = [None; VERTICES];
    let mut targets = [None; EDGES];
    let mut cursor = 0;
    let mut vertex_index = 0;
    while vertex_index < VERTICES {
        let vertex = graph.vertex(vertex_index)?;
        starts[vertex_index] =
            if cursor == EDGES { None } else { Some(CsrEdge::try_from_usize(cursor).ok()?) };
        for (_, target) in graph.out_edges(vertex) {
            if cursor >= EDGES {
                return None;
            }
            let target_index = target.get_index_usize().ok()?;
            targets[cursor] = Some(CsrVertex::try_from_usize(target_index).ok()?);
            cursor += 1;
        }
        vertex_index += 1;
    }
    if cursor != EDGES {
        return None;
    }
    let targets = core::array::from_fn(|i| targets[i].unwrap());
    Some((starts, targets))
}

#[test]
fn adjacency_can_freeze_through_csr_parts() {
    let mut adj = Adj::<4, 6>::new();
    let a = adj.vertex(0).unwrap();
    let b = adj.vertex(1).unwrap();
    let c = adj.vertex(2).unwrap();
    let d = adj.vertex(3).unwrap();
    // Global adjacency edge order:
    // e0 = a → b
    // e1 = c → d
    // e2 = a → c
    // e3 = c → c
    adj.add_edge(a, b).unwrap();
    adj.add_edge(c, d).unwrap();
    adj.add_edge(a, c).unwrap();
    adj.add_edge(c, c).unwrap();
    let (starts, targets) = csr_parts_from_adj::<4, 6, 4>(&adj).unwrap();
    let csr = Csr::<4, 4>::from_parts(starts, targets).unwrap();
    let a = csr.vertex(0).unwrap();
    let c = csr.vertex(2).unwrap();
    let mut a_neighbors = csr.neighbors(a);
    assert_eq!(a_neighbors.next(), csr.vertex(2));
    assert_eq!(a_neighbors.next(), csr.vertex(1));
    assert_eq!(a_neighbors.next(), None);
    let mut c_neighbors = csr.neighbors(c);
    assert_eq!(c_neighbors.next(), csr.vertex(2));
    assert_eq!(c_neighbors.next(), csr.vertex(3));
    assert_eq!(c_neighbors.next(), None);
}
