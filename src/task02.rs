use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};

use crate::common;

pub fn cli() -> Command {
    Command::new("day02").about("Elvish hand games").arg(
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
    let mut score_acc: u32 = 0;
    let mut score2_acc: u32 = 0;
    let mut op_hand: char;
    let mut my_hand: char;
    if let Ok(lines) = common::read_lines(filepath) {
        for line in lines {
            if let Ok(line_text) = line {
                let mut line_str = line_text.chars();
                op_hand = line_str.nth(0).unwrap();
                my_hand = normalize(line_str.nth(1).unwrap());
                score_acc += score(op_hand, my_hand);
                line_str = line_text.chars();
                op_hand = line_str.nth(0).unwrap();
                my_hand = normalize2(line_str.nth(1).unwrap(), op_hand);
                score2_acc += score(op_hand, my_hand);
            }

        }
    }
    println!("Final score: {}/{}", score_acc, score2_acc)
}

fn score(op_hand: char, my_hand: char) -> u32 {
    let mut acc: u32 = 0;
    let play_score: u32 = match my_hand {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => unreachable!("No"),
    };
    acc += play_score;
    if op_hand == my_hand {
        acc += 3;
    } else {
        let win_score: u32 = match (op_hand, my_hand) {
            ('A', 'B') => 6,
            ('B', 'C') => 6,
            ('C', 'A') => 6,
            _ => 0,
        };
        acc += win_score;
    }
    return acc;
}

fn normalize(my_hand: char) -> char {
    match my_hand {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => unreachable!("No"),
    }
}

fn normalize2(my_task: char, op_hand: char) -> char {
    match (my_task, op_hand) { //got lazy
        ('Y',_) => op_hand, //draw 
        ('X', 'A') => 'C', //lose
        ('X', 'B') => 'A', //lose
        ('X', 'C') => 'B', //lose
        ('Z', 'A') => 'B', //win
        ('Z', 'B') => 'C', //win
        ('Z', 'C') => 'A', //win
        _ => unreachable!("No"),
    }
}
