/// Simple graph functionality
///
/// # Example
///
/// ```
/// // There is a special order to getting dressed in the morning
/// use std::collections::HashMap;
/// let mut before = HashMap::new();
/// before.insert("jacket",  vec!["shirt"]);
/// before.insert("whities", vec![]);
/// before.insert("shirt",   vec![]);
/// before.insert("socks",   vec![]);
/// before.insert("hat",     vec!["shirt"]);
/// before.insert("pants",   vec!["socks", "whities"]);
/// /*
/// // j in adj[i] <=> we need to put on nodes[i] before we put on nodes[j]
/// let adj = vec![ vec![2], vec![], vec![], vec![], vec![2], vec![2], vec![1, 3]];
///
/// // Now lets make is a graph
/// struct Clothes(Vec::<&'static str>, Vec::<Vec::<i32>>);
///
/// impl SimpleGraph for Clothes {
///
///    type N = &'static str;
///
///    fn children(&self, node: &Self::N) -> Vec::<&Self::N> {
///        let mut v = Vec::<&Self::N>::new();
///        v.reserve(node.uses.len());
///        for i in 0..node.uses.len() {
///            v.push(&self.node_map[&node.uses[i]]);
///        } 
///        v
///    }
///
///    fn get_nodes(&self) -> Vec::<&Self::N> {
///        self[0]
///    }
/// }
/// 
/// let clothes = Clothes(nodes, adj);
/// */
/// ````

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
