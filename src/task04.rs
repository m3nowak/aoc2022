use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};

use crate::common;

pub fn cli() -> Command {
    Command::new("day04").about("Elvish camp cleanup").arg(
        clap::arg!(path: <PATH>)
            .required(true)
            .value_parser(value_parser!(std::path::PathBuf)),
    )
}

pub fn handle(matches: &ArgMatches) {
    let path = matches.get_one::<std::path::PathBuf>("path");
    solve(path.unwrap().to_path_buf());
}

fn solve(filepath: PathBuf) {
    let mut acc_coll: u32 = 0;
    let mut acc_any: u32 = 0;

    if let Ok(lines) = common::read_lines(filepath) {
        for line in lines {
            if let Ok(line_text) = line {
                let (pair1, pair2) = line_to_range_pair(&line_text);
                if one_contains_other(pair1, pair2){
                    acc_coll += 1;
                }
                if any_overlap(pair1, pair2){
                    acc_any += 1;
                }
            }
        }
    }
    println!("{} where one contasins other\n{} with any overlap", acc_coll, acc_any)
}

fn one_contains_other(pair1: (u32, u32), pair2: (u32, u32)) -> bool {
    (pair1.0 <= pair2.0 && pair1.1 >= pair2.1) || (pair1.0 >= pair2.0 && pair1.1 <= pair2.1)
}

fn is_between(tested: u32, pair: (u32, u32)) -> bool {
    tested >= pair.0 && tested <= pair.1
}

fn any_overlap(pair1: (u32, u32), pair2: (u32, u32)) -> bool {
    is_between(pair1.0, pair2) || is_between(pair1.1, pair2) || one_contains_other(pair1, pair2)
}

fn line_to_range_pair(line: &str) -> ((u32, u32), (u32, u32)) {
    let mut sub_split: Vec<u32> = Vec::new();
    for slice in line.split(',') {
        for subslice in slice.split('-') {
            sub_split.push(subslice.parse().unwrap());
        }
    }
    return ((sub_split[0], sub_split[1]), (sub_split[2], sub_split[3]));
}

#[cfg(test)]
mod _tests {
    use super::any_overlap;
    #[test]
    fn any_overlap1(){
        assert!(any_overlap((0,10), (2,8)))
    }
    #[test]
    fn any_overlap2(){
        assert!(any_overlap((2,8), (0,10)))
    }
    #[test]
    fn any_overlap3(){
        assert!(any_overlap((0,10), (0,10)))
    }
    #[test]
    fn any_overlap4(){
        assert!(any_overlap((0,10), (5,15)))
    }
    #[test]
    fn any_overlap5(){
        assert!(any_overlap((100,200), (5,150)))
    }
    #[test]
    fn any_overlap_not1(){
        assert!(!any_overlap((0,10), (100,150)))
    }
    #[test]
    fn any_overlap_not2(){
        assert!(!any_overlap((78,100), (3,5)))
    }
}

