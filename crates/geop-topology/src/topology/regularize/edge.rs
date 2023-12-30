use crate::topology::edge::Edge;


pub type RegularizedEdge = Edge; // Edges with exactly one interval that is not a point

pub fn edge_regularize(edge: &Edge) -> Vec<RegularizedEdge> {
    let mut edge = edge.clone();
    let mut regularized_edges = Vec::<RegularizedEdge>::new();
    for (s, e) in edge.boundaries.iter() {
        if s == e {
            continue;
        }
        regularized_edges.push(Edge::new(vec![(s.clone(), e.clone())], edge.curve.clone()));
    }
    regularized_edges
}

pub fn edge_regularize_all(edges: Vec<Edge>) -> Vec<RegularizedEdge> {
    let mut regularized_edges = Vec::<RegularizedEdge>::new();
    for edge in edges.iter() {
        regularized_edges.extend(edge_regularize(edge));
    }
    regularized_edges
}

pub fn edge_is_regular(edge: &Edge) -> bool {
    edge.boundaries.len() == 1
}
