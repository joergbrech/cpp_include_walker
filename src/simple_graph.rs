/// Simple graph functionality


/// A trait for simple graphs
pub trait SimpleGraph {

    /// Node type
    type N;

    /// get all nodes
    fn get_nodes(&self) -> Vec::<&Self::N>;

    /// get all children of a node
    fn children(&self, node: &Self::N) -> Vec::<&Self::N>;

    /// get topologically sorted vector of nodes
    fn get_topological_order(&self) -> Vec::<&Self::N> {
        let mut res = self.get_nodes();
        
        res.sort_by(|a, b| self.children(a).len().cmp(&self.children(b).len()) );
        return res;

        //TODO: Implement Kahn or smth. First, find circular dependencies
    }
}
