use std::collections::VecDeque;
use std::path::PathBuf;

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

use crate::common;

#[derive(PartialEq, Eq, Debug)]
struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

pub fn cli() -> Command {
    Command::new("day05")
        .about("Elvish crate troubles")
        .arg(
            Arg::new("modern")
                .short('m')
                .long("modern")
                .action(ArgAction::SetTrue),
        )
        .arg(
            clap::arg!(path: <PATH>)
                .required(true)
                .value_parser(value_parser!(std::path::PathBuf)),
        )
}

pub fn handle(matches: &ArgMatches) {
    let path = matches.get_one::<std::path::PathBuf>("path");
    let modern = matches.get_flag("modern");
    solve(path.unwrap().to_path_buf(), modern);
}

fn solve(filepath: PathBuf, modern: bool) {
    let mut crate_stacks: Vec<VecDeque<char>> = Vec::new();
    let mut column_count = 0;
    let mut analyzing_state = true;
    let mut crate_buffer: VecDeque<char> = VecDeque::new();

    if let Ok(lines) = common::read_lines(filepath) {
        for line in lines {
            if let Ok(line_text) = line {
                if analyzing_state {
                    if column_count == 0 {
                        column_count = (line_text.len() + 1) / 4;
                        for _ in 0..column_count {
                            crate_stacks.push(VecDeque::new());
                        }
                    }
                    if line_text.is_empty() {
                        analyzing_state = false;
                    } else {
                        if let Some(crate_vec) = crate_line_split(&line_text) {
                            for (i, co) in crate_vec.iter().enumerate() {
                                if let Some(c) = co {
                                    crate_stacks[i].push_front(*c);
                                }
                            }
                        }
                    }
                } else {
                    let op = parse_operation(&line_text);
                    if modern {
                        for _ in 0..op.count {
                            crate_buffer.push_front(crate_stacks[op.from].pop_back().unwrap());
                        }
                        for _ in 0..op.count {
                            crate_stacks[op.to].push_back(crate_buffer.pop_front().unwrap());
                        }
                    } else {
                        for _ in 0..op.count {
                            let buffer = crate_stacks[op.from].pop_back().unwrap();
                            crate_stacks[op.to].push_back(buffer);
                        }
                    }
                }
            }
        }
    }

    println!("Final result:");
    for (i, vc) in crate_stacks.iter().enumerate() {
        print!("{} |", i);
        for element in vc {
            print!("{}", *element);
        }
        println!("|")
    }
}

fn parse_operation(line: &str) -> Operation {
    let vec: Vec<&str> = line.split(' ').collect();
    Operation {
        count: vec[1].parse::<usize>().unwrap(),
        from: vec[3].parse::<usize>().unwrap() - 1,
        to: vec[5].parse::<usize>().unwrap() - 1,
    }
}

fn crate_line_split(line: &str) -> Option<Vec<Option<char>>> {
    let crate_line_re: regex::Regex = regex::Regex::new(r"^((   |\[[A-Z]\]) ?)+$").unwrap();
    if crate_line_re.is_match(line) {
        let column_count = (line.len() + 1) / 4;
        let chars: Vec<char> = line.chars().collect();
        let mut ret: Vec<Option<char>> = Vec::new();
        for i in 0..column_count {
            let cho = match chars[1 + 4 * i] {
                ' ' => None,
                ch => Some(ch),
            };
            ret.push(cho);
        }
        return Some(ret);
    } else {
        return None;
    }
}

#[cfg(test)]
mod _tests {
    use super::{crate_line_split, parse_operation, Operation};
    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation(&String::from("move 10 from 9 to 1")),
            Operation {
                count: 10,
                from: 8,
                to: 0
            }
        )
    }
    #[test]
    fn test_crate_line_split() {
        assert_eq!(
            crate_line_split(&String::from(" 1   2   3   4   5   6   7   8   9 ")),
            None
        );
        assert_eq!(crate_line_split(&String::from("")), None);

        let a = crate_line_split(&String::from("    [C]             [L]         [T]"));
        let b = Some(vec![
            None,
            Some('C'),
            None,
            None,
            None,
            Some('L'),
            None,
            None,
            Some('T'),
        ]);
        assert_eq!(a, b);

        let a = crate_line_split(&String::from("    [W] [L] [P] [V] [M] [V]     [F]"));
        let b = Some(vec![
            None,
            Some('W'),
            Some('L'),
            Some('P'),
            Some('V'),
            Some('M'),
            Some('V'),
            None,
            Some('F'),
        ]);
        assert_eq!(a, b);

        let a = crate_line_split(&String::from("[Z] [Q] [F] [L] [G] [W] [H] [F] [M]"));
        let b = Some(vec![
            Some('Z'),
            Some('Q'),
            Some('F'),
            Some('L'),
            Some('G'),
            Some('W'),
            Some('H'),
            Some('F'),
            Some('M'),
        ]);
        assert_eq!(a, b);
    }
}
