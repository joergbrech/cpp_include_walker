use cpp_include_walker::DependencyForest;

fn main() {
    let dir = "../tigl/src/";

    let mut forest: DependencyForest = Default::default();
    forest.fill_from_directory(dir, true);
    let topo_sort = forest.include_order(true).unwrap();

    println!("The first couple of nodes of the dependency forest:");
    for i in 0..10 {
        println!("   {:?}", topo_sort[i]);
    }

    println!("\nWho uses std::vector? \n   {:?}", forest.node_map["vector_hdr"].used_by);
}
