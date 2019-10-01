use cpp_include_walker::dependency_forest::DependencyForest;
use cpp_include_walker::simple_graph::get_topological_order;

#[test]
fn integration_test() {
    let mut forest: DependencyForest = Default::default();
    forest.fill_from_directory("tests/test_data/circular_dep", true);
    let topo_sort = get_topological_order(&forest);

    assert_eq!(topo_sort[0].uses.len(), 0);
    assert_eq!(topo_sort[0].used_by, ["a_hdr", "b_hdr"]);


    assert_eq!(forest.node_map["vector_hdr"].used_by, ["a_hdr", "b_hdr"]);
}