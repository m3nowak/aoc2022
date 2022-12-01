use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use clap::{value_parser, Arg, ArgAction, Command};

pub fn cli() -> Command {
    Command::new("day01")
        .about("Elvish calorie counter")
        .arg(clap::arg!(path: <PATH>).required(true).value_parser(value_parser!(std::path::PathBuf)))
}

pub fn solve(filepath: PathBuf) {
    if let Ok(lines) = read_lines(filepath) {
        let mut max_kcal: [u32; 3] = [0, 0, 0];
        let mut acc_kcal: u32 = 0;
        let mut total: u32 = 0;
        for line in lines {
            if let Ok(line_text) = line {
                let line_parsed = line_text.parse::<u32>();
                match line_parsed {
                    Ok(kcal) => {
                        acc_kcal += kcal;
                    }
                    Err(_) => {
                        for index in 0..max_kcal.len() {
                            if max_kcal[index] < acc_kcal {
                                let temp_kcal = max_kcal[index];
                                max_kcal[index] = acc_kcal;
                                acc_kcal = temp_kcal;
                            }
                        }
                        acc_kcal = 0;
                        total += 1;
                    }
                }
            }
        }
        println!("Total {}", total);
        let mut max_sum: u32 = 0;
        for index in 0..max_kcal.len() {
            println!("Max no {} = {}", index, max_kcal[index]);
            max_sum += max_kcal[index];
        }
        println!("Max sum {}", max_sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
