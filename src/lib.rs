
/// Module used for file IO
pub mod file_io {

    use std::io::{self, BufRead};
    use std::fs;
    use std::fs::{File, metadata};
    use std::path::Path;

    // use nom for file parsing
    extern crate nom;
    use nom::sequence::delimited;
    use nom::character::complete::{char, multispace0};
    use nom::branch::alt;
    use nom::bytes::complete::{is_not, tag};

    /// helper function for file parsing
    ///
    /// # Arguments
    /// path of a file
    ///
    /// # Output
    /// Returns an iterator to the lines
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    /// parser for #include statements that can be used by nom
    ///
    /// matches any `#include "[...]"` and `#include <...>` statements and
    /// returns the matched result
    fn include_parser(input: &str) -> nom::IResult<&str, &str> {

        let alt_match = alt((
            delimited(char('"'), is_not("\""), char('\"')),     // #include "..."
            delimited(char('<'), is_not(">"), char('>')),       // #include <...>
        ));

        let (input, _) = tag("#include")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, header) = alt_match(input)?;

        Ok((input, header))
    }

    /// parse all dependencies of a cpp source or header file
    ///
    /// # Argument
    /// path to file
    ///
    /// # Output
    /// Returns a vector of `#include`d dependencies
    pub fn get_deps<P>(filename: P) -> Vec::<String>
    where P: AsRef<Path>, {
        let mut deps = Vec::<String>::new();

        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for res in lines {
                if res.is_ok() {
                    let line = res.unwrap();
                    let output = include_parser(&line);
                    if output.is_ok() { 
                        let (_, header) = output.unwrap();
                        deps.push(header.to_string());
                    }
                }
            }
        }
        else {
            println!("Could not open file.");
        }
        return deps;
    }

    /// recursively find all files and apply a function to the path of the file
    ///
    /// # Examples
    ///
    /// Lets print all files in the current directory, but not in the subdirectories
    //
    /// ```
    /// let mut v = Vec::<String>::new();
    /// ls_apply('.', &mut |p| res.push(p), false);
    /// println!("{:?}", v);
    /// ```
    ///
    /// # Input
    ///  - `dir`: path to a directory
    ///  - `func`: A function that can be converted to `FnMut(String)`
    ///  - `recursive`: bool wether to recurse into subdirectories of `dir`. 
    /// 
    pub fn ls_apply<P,F>(dir: P, func: &mut F, recursive: bool)
    where P: AsRef<Path> + Copy, 
          F: FnMut(String)
    {
        match fs::read_dir(dir) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => for path in paths {
                let p = path.unwrap().path();
                let md = metadata(&p).unwrap();

                if md.is_dir() {
                    if recursive {
                        // recurse into subdir
                        ls_apply(p.to_str().unwrap(), func, recursive);
                    }
                }
                else {
                    // apply func to path string of file 
                    func(p.to_str().unwrap().to_string());
                }
            },
        }    
    }
} // mod file_io


/// Simple graph functionality
pub mod simple_graph {
    /// A trait for simple graphs
    pub trait SimpleGraph {
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
}


/// module for all dependency tracking
pub mod dependency_forest {
    use std::path::Path;
    use std::vec::Vec;
    use std::collections::HashMap;

    use crate::file_io::ls_apply;
    use crate::simple_graph::SimpleGraph;

    /// keyify turns a path or a string to a *somewhat* unique key to keep track of dependencies
    ///
    /// It takes a path as input, reduces it to the file_stem and 
    ///  - appends `"_hdr"` if the re is no extension or the extension is any of `.h`, `.hpp`, `.hxx`; or 
    ///  - appends `"_src"` if the extension is any of `.c`, `.cpp`, `.cxx`
    ///  - appends nothing if there is no extension.
    /// returns `Err` if the file_stem cannot be determined
    ///
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
        path: Option<String>,
        /// adjacency list (who depends on me)
        used_by: Vec<String>,
        /// adjacency list (who do I depend on)
        uses: Vec<String>,        
    }

    /// A class that implements the dependency forest, i.e. a set of trees
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

                    let deps = &crate::file_io::get_deps(&path);

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

} // mod dependency_forest