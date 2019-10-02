//! simple graph functionality specially purposed for topological sorting

/// A simple graph implementation on unordered borrowed data
pub trait SimpleGraph {

    /// Node type
    type N : PartialEq;

    /// get all nodes
    fn nodes(&self) -> Vec::<&Self::N>;

    /// get all children of a node
    fn children(&self, node: &Self::N) -> Vec::<&Self::N>;

    /// get all ancestors of a node
    /// 
    /// This function is slow without any additional assumptions on the generic type and memory management. 
    /// Usually, this function can be implemented more efficiently and it is recommended
    /// to explicitly implement it.
    fn ancestors(&self, node: &Self::N) -> Vec::<&Self::N> {
        let mut res = Vec::<&Self::N>::new();
        let nodes = self.nodes();
        for other in nodes {
            let children = self.children(other);
            for child in children {
                if child == node {
                    res.push(other);
                }
            }
        }
        return res;
    }

    /// get topologically sorted vector of nodes for a graph
    fn get_topological_order(&self) -> Result<Vec::<&Self::N>, &'static str> {

        // Kahn's method
        let nodes = self.nodes();
        let n = nodes.len();

        // calculate in-degree
        let mut in_degree = vec![0; n];
        for i in 0..n {
            in_degree[i] = self.ancestors(nodes[i]).len();
        }

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
            for child in children {
                //TODO is there a faster way to get the index of the child?
                let idx = nodes.iter().position(|&x| x==child).unwrap();
                in_degree[idx] -= 1;
                if in_degree[idx] == 0 {
                    candidates.push(nodes[idx]);
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
