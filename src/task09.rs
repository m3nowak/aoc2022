use std::{collections::HashSet, path::PathBuf};

use clap::{value_parser, ArgMatches, Command};

use crate::common;

enum Movement {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

pub fn cli() -> Command {
    Command::new("day09")
        .about("Elvish rope thought experiments")
        .arg(
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
        let movements = lines.map(|line| line_to_movent(&line.unwrap())).collect();
        println!(
            "{} unique positions with 2 len",
            populate_movemt(&movements, 2).len()
        );
        println!(
            "{} unique positions with 10 len",
            populate_movemt(&movements, 10).len()
        );
    }
}

fn populate_movemt(movements: &Vec<Movement>, rope_len: usize) -> HashSet<(i32, i32)> {
    let mut positions: Vec<(i32, i32)> = vec![(0, 0); rope_len];
    let mut tail_pos_set: HashSet<(i32, i32)> = HashSet::new();
    tail_pos_set.insert(*positions.last().unwrap());

    // let mut delta = Box::new(|x: (i32, i32)| x);
    let mut repeats: usize;
    for movement in movements {
        match movement {
            Movement::Up(moves) => {
                repeats = *moves;
            }
            Movement::Down(moves) => {
                repeats = *moves;
            }
            Movement::Left(moves) => {
                repeats = *moves;
            }
            Movement::Right(moves) => {
                repeats = *moves;
            }
        }
        for _ in 0..repeats {
            positions[0] = direction_single_shift(&positions[0], &movement);
            for i in 0..rope_len - 1 {
                // let new_head_pos = direction_single_shift(&positions[i], &movement);
                if check_if_needs_move(&positions[i], &positions[i + 1]) {
                    positions[i + 1] = new_tail_pos(&positions[i], &positions[i+1]);
                }
            }
            tail_pos_set.insert(*positions.last().unwrap());
        }
    }
    tail_pos_set
}

fn direction_single_shift(pos: &(i32, i32), mvmnt: &Movement) -> (i32, i32) {
    match mvmnt {
        Movement::Up(_) => (pos.0 + 1, pos.1),
        Movement::Down(_) => (pos.0 - 1, pos.1),
        Movement::Left(_) => (pos.0, pos.1 - 1),
        Movement::Right(_) => (pos.0, pos.1 + 1),
    }
}

fn check_if_needs_move(head_new: &(i32, i32), tail: &(i32, i32)) -> bool {
    (head_new.0 - tail.0).abs() > 1 || (head_new.1 - tail.1).abs() > 1
}

fn new_tail_pos(head: &(i32,i32), tail: &(i32,i32)) -> (i32,i32) {
    if check_if_needs_move(&head, &tail){
        if head.0 == tail.0 {
            return (tail.0, tail.1+(head.1 - tail.1).signum());
        }
        else if head.1 == tail.1 {
            return (tail.0+(head.0 - tail.0).signum(), tail.1);
        }
        else {
            return (tail.0+(head.0 - tail.0).signum(), tail.1+(head.1 - tail.1).signum());
        }
    }
    else {
        return *tail;
    }
}

fn line_to_movent(line: &str) -> Movement {
    match line {
        lmatched if lmatched.starts_with("U") => Movement::Up(parse_line_number(lmatched)),
        lmatched if lmatched.starts_with("D") => Movement::Down(parse_line_number(lmatched)),
        lmatched if lmatched.starts_with("L") => Movement::Left(parse_line_number(lmatched)),
        lmatched if lmatched.starts_with("R") => Movement::Right(parse_line_number(lmatched)),
        _ => panic!(),
    }
}

fn parse_line_number(line: &str) -> usize {
    line.split(' ').nth(1).unwrap().parse().unwrap()
}
