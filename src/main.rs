use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

extern crate nom;
use nom::sequence::delimited;
use nom::character::complete::{char, multispace0};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};

// get all dependencies of a cpp file
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

// helper function for file io
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

fn main() {
    let filename = "/home/jan/winhome/Tools/tigl/src/engine_nacelle/CCPACSNacelleCenterCowl.cpp";

    let deps = get_deps(filename);
    println!("{:?}", deps);

}
