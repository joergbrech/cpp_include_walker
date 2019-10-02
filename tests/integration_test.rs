use cpp_include_walker::DependencyForest;
use cpp_include_walker::simple_graph::SimpleGraph;

#[test]
fn integration_test() {
    let mut forest: DependencyForest = Default::default();
    forest.fill_from_directory("tests/test_data/simple", true);
    let topo_sort = forest.get_include_order().unwrap();

    assert_eq!(forest.len(), 4);

    println!("{:?}", topo_sort);
    assert_eq!(topo_sort[0].uses.len(), 0);
    assert_eq!(topo_sort[0].used_by, ["a_hdr", "b_hdr"]);


    assert_eq!(forest.node_map["vector_hdr"].used_by, ["a_hdr", "b_hdr"]);
}