//#[doc(include = "../README.md")]  // <-- experimental feature
//! A library to check the include-dependencies of a C++ project

pub mod simple_graph;
pub mod file_io;


use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;

use crate::file_io::ls_apply;
use crate::simple_graph::SimpleGraph;

/// keyify turns a path or a string to a *somewhat* unique key to keep track of dependencies.
///
/// It takes a path as input, reduces it to the file_stem and 
///
///  - appends `"_hdr"` if the re is no extension or the extension is any of `.h`, `.hpp`, `.hxx`; or 
///  - appends `"_src"` if the extension is any of `.c`, `.cpp`, `.cxx`
///  - appends nothing if there is no extension,
///
/// returns `Err` if the file_stem cannot be determined.
///
/// # Example
///
/// ```
/// use cpp_include_walker::keyify;
///
/// assert_eq!(keyify("/path/to/file.h").unwrap(), "file_hdr");
/// assert_eq!(keyify("a.hxx").unwrap(), "a_hdr");
/// assert_eq!(keyify("a").unwrap(), "a_hdr");
/// assert_eq!(keyify("a.cxx").unwrap(), "a_src");
/// assert_eq!(keyify("a.whoevencares").unwrap(), "a");
/// ```
pub fn keyify<P>(path: P) -> Result<String, &'static str>
where P: AsRef<Path>
{
    let mut t = "";
    let stem = path.as_ref().file_stem();
    let ext = path.as_ref().extension();
    match ext {
        Some(e) => {
            if e == "h" || e == "hpp" || e == "hxx" {
                t = "_hdr";
            }
            else if e == "c" || e == "cpp" || e == "cxx" {
                t = "_src";
            }
        },
        None => t = "_hdr",
    }
    match stem {
        Some(s) => return Ok(s.to_str().unwrap().to_owned() + &t),
        None => return Err("Cannot determine filestem of path."),
    }
}

/// A node in the dependency forest
///
/// The nodes are collected from all the `#include`s found in the source directory
#[derive(Default, Debug)]
pub struct DependencyNode {
    /// path to file, if it exists in the searched directory
    pub path: Option<String>,
    /// adjacency list (who depends on me)
    pub used_by: Vec<String>,
    /// adjacency list (who do I depend on)
    pub uses: Vec<String>,        
}

/// A class that implements the dependency forest, i.e. a set of trees
///
/// Basically, this is just a [`HashMap`](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html) of nodes with a 
/// [`SimpleGraph`](../simple_graph/trait.SimpleGraph.html) implementation for the node type
/// [`DependencyNode`](struct.DependencyNode.html).
#[derive(Default, Debug)]
pub struct DependencyForest {
    /// The source directory
    pub directory: String,   
    /// a hash map of dependencies                       
    pub node_map: HashMap<String, DependencyNode>,
}

impl DependencyForest {

    /// fill the dependency forest from all header and source files in a directory
    pub fn fill_from_directory<P>(&mut self, dir: P, recursive: bool)
    where P: AsRef<Path> + Copy
    {
        self.directory = dir.as_ref().to_str().unwrap().to_string();

        // apply self.add_includes_from_file to all files found in dir
        ls_apply(dir, &mut |p| self.add_includes_from_file(p), recursive);
    }

    /// add all `#include`s of a file as a node to the `node_map`, if it doesn't exist
    /// as a node already.
    fn add_includes_from_file<P>(&mut self, path: P)
    where P: AsRef<Path>
    {
        // only work on source or header files
        let ext = path.as_ref().extension();
        match ext {
            Some(e) => {
                if !(e == "h" || e == "hpp" || e == "hxx" ||  e == "c" || e == "cpp" || e == "cxx") {
                    return;
                }
            },
            None => {},
        }

        let key = keyify(&path);
        match &key {
            Ok(kf) => {
                // register file if not registered
                let entry = self.node_map.entry(kf.to_string()).or_insert(Default::default());

                let deps = &crate::file_io::get_deps(&path).unwrap();

                // remember path and `uses` dependencies
                entry.path = Some(path.as_ref().strip_prefix(&self.directory)
                    .unwrap().to_str().unwrap().to_string());
                entry.uses = deps.to_vec().iter().map(|x| keyify(&x).unwrap()).collect();
                
                deps.to_vec();

                // now append the `used_by` vector of all dependencies
                for i in 0..deps.len() {
                    let key = keyify(&deps[i]);

                    match &key {
                        Ok(kd) => {
                            // register dependency if not registered
                            let entry = self.node_map.entry(kd.to_string()).or_insert(Default::default());

                            // rememember that dependency is used by file
                            entry.used_by.push(kf.as_str().to_string());
                        },
                        Err(why) => println!("{:?}", why)
                    }
                }
            },
            Err(why) => println!("{:?}", why)
        }
    }
}


impl SimpleGraph for DependencyForest {

    type N = DependencyNode;

    fn children(&self, node: &Self::N) -> Vec::<&Self::N> {
        let mut v = Vec::<&Self::N>::new();
        v.reserve(node.uses.len());
        for i in 0..node.uses.len() {
            v.push(&self.node_map[&node.uses[i]]);
        } 
        v
    }

    fn get_nodes(&self) -> Vec::<&Self::N> {
        let mut v = Vec::<&Self::N>::new();
        for (_, val) in self.node_map.iter() {
            v.push(val);
        } 
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyify_err() {
        assert!(keyify(".").is_err());
    }

    #[test]
    fn keyify_val() {
        assert_eq!(keyify("/who/even/cares.h").unwrap(), "cares_hdr");
        assert_eq!(keyify("a.hxx").unwrap(), "a_hdr");
        assert_eq!(keyify("a.hpp").unwrap(), "a_hdr");
        assert_eq!(keyify("a.h").unwrap(), "a_hdr");
        assert_eq!(keyify("a").unwrap(), "a_hdr");
        assert_eq!(keyify("a.cxx").unwrap(), "a_src");
        assert_eq!(keyify("a.cpp").unwrap(), "a_src");
        assert_eq!(keyify("a.c").unwrap(), "a_src");
        assert_eq!(keyify("a.whoevencares").unwrap(), "a");
    }
}