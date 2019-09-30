use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;

mod file_io {

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

    // parse all dependencies of a cpp source or header file
    pub fn get_deps<P>(filename: P) -> Vec::<String>
    where P: AsRef<Path>, {
        let mut deps = Vec::<String>::new();

        // File hosts must exist in current path before this produces output
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

    // helper function for file parsing
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    // parser for #include statements
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

    // recursively find all files and apply function func to the path of the file
    pub fn ls_src<P,F>(dir: P, func: &mut F, recursive: bool)
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
                        ls_src(p.to_str().unwrap(), func, recursive);
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

/* keyify turns a path or a string to a somewhat unique key to keep track of dependencies
 *
 * takes a path, reduce it to the file_stem and 
 *   - append "_hdr" if there is no extension or the extension is any of .h, .hpp, .hxx; or 
 *   - append "_src" if the extension is any of .c, .cpp, .cxx
 *   - append nothing if there is no extension
 * return None if the stem cannot be determined
 */
fn keyify<P>(path: P) -> Result<String, &'static str>
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

#[derive(Default, Debug)]
struct DependencyNode {
    path: Option<String>,     // path to file, if it exists in the searched directory
    used_by: Vec<String>,     // adjacency list (who depends on me)
}

#[derive(Default, Debug)]
struct DependencyForest {
    node_map: HashMap<String, DependencyNode>,  // a hash map of dependencies
}

impl DependencyForest {

    // fill the dependency forest from all header and source files in a directory
    fn fill_from_directory<P>(&mut self, dir: P, recursive: bool)
    where P: AsRef<Path> + Copy
    {
        // apply self.add_includes_from_file to all files found in dir
        crate::file_io::ls_src(dir, &mut |p| self.add_includes_from_file(p), recursive);
    }

    // add #include finds in a file as a node to the node_map, if it doesn't exist already
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

                // remember path 
                // TODO: strip source directory!!
                entry.path = Some(path.as_ref().to_str().unwrap().to_string());

                let deps = crate::file_io::get_deps(&path);
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

    // get topologically sorted vector of dependencies
    fn topologically_sorted_vec(&self) -> Vec::<&DependencyNode> {
        let mut res = Vec::<&DependencyNode>::new();
        res.reserve(self.node_map.len());
        for (_, entry) in &self.node_map {
            res.push(entry);
        }
        res.sort_by(|a, b| a.used_by.len().cmp(&b.used_by.len()) );
        return res;

        //TODO: Implement Kahn or smth. Currently this is only sorted by how often a dependency is used
    }
}


fn main() {
    let mut forest: DependencyForest = Default::default();
    forest.fill_from_directory("/home/jan/winhome/Tools/tigl/src/geometry", true);
    let topo_sort = forest.topologically_sorted_vec();

    println!("Get the first couple of roots");
    for i in 0..20 { //topo_sort.len() {
        println!("{:?}", topo_sort[i]);
    }

    println!("\n Who uses std::vector? \n {:?}", forest.node_map["vector_hdr"]);
}
