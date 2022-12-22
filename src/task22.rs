use std::{path::PathBuf, collections::HashMap};

use clap::{value_parser,Command, ArgMatches};
use regex::{Regex, Matches, Match};

use crate::common;

const MOVE_RE: &str = r"[0-9]+|L|R";


struct Map{
    hmap: HashMap<(usize,usize), bool>
}

enum Move{
    RotCC,
    RotCW,
    Forward(usize)
}

fn gen_moves(source: &str) -> Vec<Move>{
    let regex = Regex::new(MOVE_RE).unwrap();
    regex.find_iter(source).map(|val| match val.as_str() {
        "L" => Move::RotCC,
        "R" => Move::RotCW,
        mvmnt => Move::Forward(mvmnt.parse().unwrap())
    }).collect()
}


fn parse_input(lines: impl Iterator<Item = String>) -> (Map,Vec<Move>) {
    todo!()
}

pub fn cli() -> Command {
    Command::new("day22")
        .about("Jungle traversal")
        .arg(clap::arg!(path: <PATH>).required(true).value_parser(value_parser!(std::path::PathBuf)))
}

pub fn handle(matches: &ArgMatches) {
    let path = matches.get_one::<std::path::PathBuf>("path");
    solve(path.unwrap().to_path_buf());
}

pub fn solve(filepath: PathBuf) {
    
    if let Ok(lines) = common::read_lines(filepath) {
        
    }
}


