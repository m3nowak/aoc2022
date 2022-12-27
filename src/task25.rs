use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};

use crate::common;

pub fn cli() -> Command {
    Command::new("day25").about("Elvish numbers").arg(
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
        let mut acc = 0;
        for line in lines{
            if let Ok(line_read) = line {
                acc += parse_snafu(&line_read);
            }
        }
        println!("Result (1): {}", serialize_snafu(&acc));
    }
}

fn parse_snafu(input: &str) -> isize {
    let len = input.len();
    let mut acc = 0;
    for (pos, chr) in input.chars().enumerate() {
        acc += 5_isize.pow((len - pos - 1) as u32)
            * match chr {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            };
    }
    acc
}

fn serialize_snafu(input: &isize) -> String {
    let mut rem_val = *input;
    let mut acc = Vec::new();
    while rem_val != 0 {
        let chr = match rem_val.rem_euclid(5) {
            4 => {
                rem_val += 1;
                '-'
            },
            3 => {
                rem_val += 2;
                '='
            },
            2 => {
                rem_val -= 2;
                '2'
            },
            1 => {
                rem_val -= 1;
                '1'
            },
            0 => '0',
            _ => unreachable!()
        };
        acc.push(chr);
        rem_val /= 5;
    }
    acc.reverse();

    acc.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_snafu("1121-1110-1=0"), 314159265);
        assert_eq!(parse_snafu("1-0---0"), 12345);
    }
    #[test]
    fn test_serialize() {
        assert_eq!(serialize_snafu(&314159265), "1121-1110-1=0");
        assert_eq!(serialize_snafu(&12345), "1-0---0");
    }
}
