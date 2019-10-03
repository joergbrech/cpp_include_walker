#![feature(test)]

use cpp_include_walker::DependencyForest;
use test::Bencher;

extern crate test;

#[bench]
fn bench_tigl_include_order(b: &mut Bencher) {
    let mut forest: DependencyForest = Default::default();
    forest.fill_from_directory("../tigl/src/", true);
    b.iter(|| forest.include_order(true));
}