/// Simple graph functionality


/// A trait for simple graphs
pub trait SimpleGraph {

    /// Node type
    type N;

    /// get all nodes
    fn get_nodes(&self) -> Vec::<&Self::N>;

    /// get all children of a node
    fn children(&self, node: &Self::N) -> Vec::<&Self::N>;
}

/// get topologically sorted vector of nodes for a graph
pub fn get_topological_order<G: SimpleGraph>(graph: &G) -> Vec::<&G::N> {
    let mut res = graph.get_nodes();
    
    res.sort_by(|a, b| graph.children(a).len().cmp(&graph.children(b).len()) );
    return res;

    //TODO: Implement Kahn or smth. First, find circular dependencies
}
