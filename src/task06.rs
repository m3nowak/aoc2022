use std::{path::PathBuf, collections::VecDeque};

use clap::{value_parser, ArgMatches, Command};

use crate::common;

pub fn cli() -> Command {
    Command::new("day06").about("Elvish signal standards").arg(
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

    if let Ok(lines) = common::read_lines(filepath) {
        for line in lines {
            if let Ok(line_text) = line {
                println!("{} begins sequence", find_diff_sequence(&line_text, 4));
                println!("{} begins message", find_diff_sequence(&line_text, 14));
            }
        }
    }
    
}

fn find_diff_sequence(signal: &str, buf_size: usize) -> usize {
    let mut buffer = VecDeque::new();
    let mut chars = signal.chars();
    for pos in 0..signal.len() {
        if buffer.len() < buf_size {
            buffer.push_back(chars.next().unwrap())
        }
        else {
            if is_unique(&buffer) {
                return pos;
            }
            else {
                buffer.pop_front();
                buffer.push_back(chars.next().unwrap());
            }
        }
    }
    return 0;
}

fn is_unique(coll: &VecDeque<char>) -> bool {
    for pos in 1..coll.len(){
        for pos2 in 0..pos{
            if coll[pos] == coll[pos2] {
                return false;
            }
        }
    }
    return true;
}


#[cfg(test)]
mod _tests {
    use std::collections::VecDeque;

    use super::{find_diff_sequence, is_unique};
    #[test]
    fn test_find_diff_sequence(){
        assert_eq!(find_diff_sequence(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 4), 5);
        assert_eq!(find_diff_sequence(&String::from("nppdvjthqldpwncqszvftbrmjlhg"), 4), 6);
        assert_eq!(find_diff_sequence(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 4), 10);
        assert_eq!(find_diff_sequence(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 4), 11);
        assert_eq!(find_diff_sequence(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 14), 19);
        assert_eq!(find_diff_sequence(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 14), 23);
        assert_eq!(find_diff_sequence(&String::from("nppdvjthqldpwncqszvftbrmjlhg"), 14), 23);
        assert_eq!(find_diff_sequence(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 14), 29);
        assert_eq!(find_diff_sequence(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 14), 26);
    }
    #[test]
    fn test_is_unique(){
        assert!(is_unique(&VecDeque::from(vec!['a','b','c','d'])));
        assert!(!is_unique(&VecDeque::from(vec!['a','b','c','a'])));
    }
}

