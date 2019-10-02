//! simple graph functionality specially purposed for topo sort

/// A trait for simple graphs
pub trait SimpleGraph {

    /// Node type
    type N;

    /// get all nodes
    fn get_nodes(&self) -> Vec::<&Self::N>;

    /// get all children of a node
    fn children(&self, node: &Self::N) -> Vec::<&Self::N>;

    /// returns the number of nodes
    fn len(&self) -> usize {
        self.get_nodes().len()
    }

    /// get topologically sorted vector of nodes for a graph
    fn get_topological_order(&self) -> Result<Vec::<&Self::N>, &'static str> {

        // Kahn's method
        let nodes = self.get_nodes();
        let n = nodes.len();

        // calculate in-degree
        let mut in_degree = vec!(0; n);
        // TODO!!

        // initialize candidate list and return list
        let mut candidates = Vec::<&Self::N>::new();
        let mut list = Vec::<&Self::N>::new();

        // candidates are all nodes with zero in-degree
        for i in 0..n {
            if in_degree[i] == 0 {
                candidates.push(nodes[i]);
            }
        }

        while candidates.len() > 0 {
            let node = candidates.pop().unwrap();
            list.push(node);

            let children = self.children(node);
            for i in 0..children.len() {
                in_degree[i] -= 1;
                if in_degree[i] == 0 {
                    candidates.push(nodes[i]);
                }
            }
        }
        if list.len() == n {
            return Ok(list);
        }
        else {
            return Err("Circular dependency detected!")
        }
    }
}
