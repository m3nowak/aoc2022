use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};
use regex::Regex;

use crate::common;

struct Blueprint {
    no: usize,
    ore_bot_cost: usize,
    clay_bot_cost: usize,
    obsidian_bot_cost: (usize, usize), //ore,clay
    geode_bot_cost: (usize, usize),    //ore,obsidian
}

impl Blueprint {
    fn from_line(line: &str) -> Self {
        let dreg: Regex = Regex::new(r"\d+").unwrap();
        let numbers_collected: Vec<usize> = dreg
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        Self {
            no: numbers_collected[0],
            ore_bot_cost: numbers_collected[1],
            clay_bot_cost: numbers_collected[2],
            obsidian_bot_cost: (numbers_collected[3], numbers_collected[4]), //ore,clay
            geode_bot_cost: (numbers_collected[5], numbers_collected[6]),    //ore,obsidian
        }
    }
}

pub fn cli() -> Command {
    Command::new("day19").about("Elvish geode collecting").arg(
        clap::arg!(path: <PATH>)
            .required(true)
            .value_parser(value_parser!(std::path::PathBuf)),
    )
}

pub fn handle(matches: &ArgMatches) {
    let path = matches.get_one::<std::path::PathBuf>("path");
    solve(path.unwrap().to_path_buf());
}

pub fn solve(filepath: PathBuf) {
    if let Ok(lines) = common::read_lines(filepath) {
        let lines2 = lines.map(|l| l.unwrap());
        let blueprints = parse_lines(lines2);
    } else {
        println!("Could not open file!")
    }
}

fn parse_lines<'a>(
    lines: impl Iterator<Item = String> + 'a,
) -> impl Iterator<Item = Blueprint> + 'a {
    lines.map(|l| Blueprint::from_line(&l))
}
