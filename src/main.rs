use std::fs;
use std::fs::{File, metadata};
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;

extern crate nom;
use nom::sequence::delimited;
use nom::character::complete::{char, multispace0};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};

// parse all dependencies of a cpp source or header file
fn get_deps<P>(filename: P) -> Vec::<String>
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
fn ls_src<P,F>(dir: P, func: &mut F, recursive: bool)
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
                // apply func to path of file
                func(p.to_str().unwrap().to_string());
            }
        },
    }    
}

/*
struct dependency_node {
    key: String,                     // a uid of the dependency
    path: Option<String>,     // path to file, if it exists in the searched directory
    used_by: Vec<String>,            // adjacency list (who depends on me)
}

struct dependency_tree {
    node_map: HashMap<String, dependency_node>,
}

impl dependency_tree {
    fn fill_from_directory<P>(&self, dir: P) {
            //TODO
    }
}
*/

fn main() {
    // parse all #include statements of one file
    let filename = "/home/jan/winhome/Tools/tigl/src/engine_nacelle/CCPACSNacelleCenterCowl.cpp";
    println!("{:?}", get_deps(filename));

    // recursive ls *.h, *.hpp, *.hxx, *.c, *.cpp, *.cxx
    let src_dir = "/home/jan/winhome/Tools/tigl/src";

    let mut res = Vec::<String>::new();
    ls_src(src_dir, &mut |p| res.push(p), false);

    // strip the src_dir from the found files
    for idx in 0..res.len() {
        let stripped_path_res = Path::new(&res[idx]).strip_prefix(src_dir);
        match stripped_path_res {
            Err(why) => println!("! {:?}", why), 
            Ok(stripped_path) => res[idx] = stripped_path.to_str().unwrap().to_string(),
        }
    }
    println!("{:?}", res);
    println!("{:?}", res.len())
    
}
