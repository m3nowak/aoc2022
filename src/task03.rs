use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};

use crate::common;

pub fn cli() -> Command {
    Command::new("day03").about("Elvish backpack mistakes").arg(
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
    let mut acc_split: u32 = 0;
    let mut acc_badge: u32 = 0;

    let mut index: u8 = 0;

    let mut bak_line1: String = String::new();
    let mut bak_line2: String = String::new();

    if let Ok(lines) = common::read_lines(filepath) {
        for line in lines {
            if let Ok(line_text) = line {

                let (sub1, sub2) = split_str(&line_text);
                match find_common_char(&sub1, &sub2){
                    Some(cc) => {acc_split += score(cc)},
                    None => {}
                }
                index = (index + 1) % 3;
                if index == 0 {
                    acc_badge += score(find_common_char3(&bak_line1, &bak_line2, &line_text).unwrap())
                }
                else if index == 1 {
                    bak_line1 = line_text;
                }
                else {
                    bak_line2 = line_text;
                }
            }

        }
    }
    println!("Final score {}/{}", acc_split, acc_badge)
}

fn find_common_char(input1:&String, input2:&String) -> Option<char>{
    for i1c in input1.chars(){
        if input2.contains(i1c) {
            return Some(i1c);
        }
    }
    return None;
}

fn find_common_char3(input1:&String, input2:&String, input3:&String) -> Option<char>{
    for i1c in input1.chars(){
        if input2.contains(i1c) &&  input3.contains(i1c){
            return Some(i1c);
        }
    }
    return None;
}

fn split_str(input: &String) -> (String, String){
    let half_length = input.len()/2;
    return (String::from(&input[0..half_length]) , String::from(&input[half_length..2*half_length])) ;
}

fn score(item: char) -> u32 {
    if item.is_ascii_uppercase() {
       return item as u32 - 38;
    }
    else {
        return item as u32 - 96;
    }
}
