use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};

use crate::common;

const KEY: isize = 811589153;

#[derive(Debug, Clone, Copy)]
struct Cell {
    val: isize,
    og_pos: isize,
}

pub fn cli() -> Command {
    Command::new("day20").about("Elvish code cracking").arg(
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
    if let Ok(lines) = common::read_lines(&filepath) {
        let input_parsed = parse_input(lines.map(|l| l.unwrap()), 1);
        let mixed = mix(&input_parsed);
        println!("Result (1): {}", calc_coors(&mixed));
    }
    if let Ok(lines) = common::read_lines(&filepath) {
        let input_parsed = parse_input(lines.map(|l| l.unwrap()), KEY);
        let mut mixed = input_parsed;
        for _ in 0..10 {
            mixed = mix(&mixed);
        }
        println!("Result (2): {}", calc_coors(&mixed));
    }
}

fn mix(original: &Vec<Cell>) -> Vec<Cell> {
    let mut acc = original.clone();
    let length = original.len();
    for i in 0..length {
        let mut current_index = 0;
        //let mut cell;
        for j in 0..length {
            if acc[j].og_pos == i.try_into().unwrap() {
                current_index = j;
                break;
            }
        }
        let bak = acc.split_off(current_index + 1);
        let to_move = acc.pop().unwrap();
        acc.extend(bak);
        let insert_point_s = (current_index as isize + to_move.val) % (length - 1) as isize;
        let insert_point = if insert_point_s < 0 {
            (length as isize - 1 + insert_point_s) as usize
        } else {
            insert_point_s as usize
        };
        let bak = acc.split_off(insert_point);
        if acc.is_empty(){
            acc.extend(bak);
            acc.push(to_move);
        }
        else{
            acc.push(to_move);
            acc.extend(bak);
        }
    }
    acc
}

fn calc_coors(mixed: &Vec<Cell>) -> isize{
    let mut index0 = 0;
    for j in 0..mixed.len() {
        if mixed[j].val == 0 {
            index0 = j;
            break;
        }
    };
    let (i1,i2,i3) = ((index0 + 1000) % mixed.len(), (index0 + 2000) % mixed.len(), (index0 + 3000) % mixed.len());
    mixed[i1].val + mixed[i2].val + mixed[i3].val
    //todo!()
}

fn parse_input(lines: impl Iterator<Item = String>, multiply_by: isize) -> Vec<Cell> {
    lines
        .enumerate()
        .map(|(i, l)| Cell {
            val: l.parse::<isize>().unwrap() * multiply_by,
            og_pos: i as isize,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        let lines = vec!["1", "2", "-3", "3", "-2", "0", "4"];

        let input_parsed = parse_input(lines.into_iter().map(|l| String::from(l)), 1);
        let mixed = mix(&input_parsed);
        let simplified: Vec<isize> = mixed.iter().map(|c| c.val).collect();
        assert_eq!(simplified, vec![1, 2, -3, 4, 0, 3, -2]);
        assert_eq!(calc_coors(&mixed), 3);
    }
    #[test]
    fn test_pt2() {
        let lines = vec!["1", "2", "-3", "3", "-2", "0", "4"];

        let input_parsed = parse_input(lines.into_iter().map(|l| String::from(l)), KEY);
        let mut mixed = input_parsed;
        for _ in 0..10 {
            mixed = mix(&mixed);
        }
        assert_eq!(calc_coors(&mixed), 1623178306);
    }
}
