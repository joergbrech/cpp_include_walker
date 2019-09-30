#![crate_name = "cpp_include_walker"]

use cpp_include_walker::dependency_forest::DependencyForest;
use cpp_include_walker::simple_graph::SimpleGraph;

fn main() {
    // let dir = "/home/jan/winhome/Tools/tigl/src/geometry";
    let dir = "../tigl/src/geometry";

    let mut forest: DependencyForest = Default::default();
    forest.fill_from_directory(dir, true);
    let topo_sort = forest.get_topological_order();

    println!("The first root of the dependency forest:");
    println!("{:?}", topo_sort[0]);

    println!("\n Who uses std::vector? \n {:?}", forest.node_map["vector_hdr"]);
}
