#![feature(test)]

use cpp_include_walker::DependencyForest;
use test::Bencher;

extern crate test;

#[bench]
fn tigl_parse_includes(b: &mut Bencher) {
    let mut forest: DependencyForest = Default::default();
    b.iter(|| forest.fill_from_directory("../tigl/src/", true));
}

#[bench]
fn tigl_include_order(b: &mut Bencher) {
    let mut forest: DependencyForest = Default::default();
    forest.fill_from_directory("../tigl/src/", true);
    b.iter(|| forest.include_order(true));
}
