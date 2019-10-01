/// Module used for file IO


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
    let (input, _) = multispace0(input)?;
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

/// recursively find all files and apply a function to the path of the file.
/// The function is not applied to directories.
///
/// # Examples
///
/// Lets print all files in the current directory, but not in the subdirectories
//
/// ```
/// # use cpp_include_walker::file_io::ls_apply;
/// let mut v = Vec::<String>::new();
/// ls_apply(".", &mut |p| v.push(p), false);
/// println!("{:?}", v);
///
/// assert_eq!(v.len(), 5);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn include_parser_err() {
        assert!(include_parser("noinclude").is_err());
        assert!(include_parser("#include <>").is_err());
        assert!(include_parser("#include \"\"").is_err());
    }

    #[test]
    fn include_parser_ok() {
        assert!(include_parser("#include < >").is_ok());
        assert!(include_parser("#include \" \"").is_ok());
        assert!(include_parser("#include <b.cpp>").is_ok());
        assert!(include_parser("#include <a.hpp>").is_ok());
        assert!(include_parser("#include<b.cpp>").is_ok());
        assert!(include_parser(" #include <a.hpp>").is_ok());
    }

    #[test]
    fn include_parser_val() {
        assert_eq!(include_parser("#include <a0.h>").unwrap(), ("", "a0.h"));
        assert_eq!(include_parser("#include \"b1.h\"").unwrap(), ("", "b1.h"));
    }

    #[test]
    fn read_lines_err() {
        assert!(read_lines("not/a/valid/path").is_err());
    }
 
    #[test]
    fn read_lines_val() {
        let mut lines = read_lines("./Cargo.toml").unwrap();
        assert_eq!(lines.next().unwrap().unwrap(), "[package]");
    }

    
}
