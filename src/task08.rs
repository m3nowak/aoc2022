use std::{path::PathBuf, ops::Range};

use clap::{value_parser,Command, ArgMatches};

use crate::common;

pub fn cli() -> Command {
    Command::new("day08")
        .about("Elvish tree survey")
        .arg(clap::arg!(path: <PATH>).required(true).value_parser(value_parser!(std::path::PathBuf)))
}

pub fn handle(matches: &ArgMatches) {
    let path = matches.get_one::<std::path::PathBuf>("path");
    solve(path.unwrap().to_path_buf());
}

pub fn solve(filepath: PathBuf) {
    let mut rows:Vec<Vec<u8>> = Vec::new();
    let mut width: usize = 0;
    let mut length: usize = 0;
    let a = 0..10;
    
    if let Ok(lines) = common::read_lines(filepath) {
        for line in lines {
            if let Ok(line_text) = line {
                if width == 0 {
                    width = line_text.len();
                }
                rows.push(line_to_u8_vec(&line_text));
                length += 1;
            }
        }
    }
}

fn validate_pos_x(map: &Vec<Vec<u8>>, height: u8, y: u8, ymax: u8) -> bool{
    
}

fn validate_pos(map: &Vec<Vec<u8>>, x: usize, y:usize, length: u8, width: u8) -> bool {
    let height = map[x][y]+1; //+1 bc 0 is no tree
    for x_pos in 0..x {
        if map[x_pos][y] >= height {
            return false;
        }
    }
    true
}

fn line_to_u8_vec(line: &str) -> Vec<u8>{
    Vec::from_iter(line.chars().into_iter().map(|n| String::from(n).parse().unwrap()))
}
