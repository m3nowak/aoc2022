use std::collections::HashMap;
use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};

use crate::common;

enum FileTree {
    Dir(HashMap<String, FileTree>),
    File(u64),
}

#[derive(PartialEq, Eq, Debug)]
enum InputOutput {
    CdRoot,
    CdUp,
    CdDir(String),
    Ls,
    DirInfo(String),
    FileInfo(String, u64),
    Unknown,
}

pub fn cli() -> Command {
    Command::new("day07")
        .about("Elvish inability to use du command")
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

fn solve(filepath: PathBuf) {
    let mut current_path = Vec::new();
    let mut tree_root = FileTree::Dir(HashMap::new());

    if let Ok(lines) = common::read_lines(filepath) {
        for line in lines {
            if let Ok(line_text) = line {
                match parse_input_output(&line_text) {
                    InputOutput::CdRoot => {
                        current_path.clear();
                    }
                    InputOutput::CdUp => {
                        current_path.pop();
                    }
                    InputOutput::CdDir(dirname) => {
                        current_path.push(dirname);
                    }
                    InputOutput::DirInfo(dirname) => {
                        add_at_path(
                            &mut tree_root,
                            &current_path,
                            &dirname,
                            FileTree::Dir(HashMap::new()),
                        );
                    }
                    InputOutput::FileInfo(filename, size) => {
                        add_at_path(
                            &mut tree_root,
                            &current_path,
                            &filename,
                            FileTree::File(size),
                        );
                    }
                    InputOutput::Ls => {
                        //nothing happens
                    }
                    InputOutput::Unknown => {
                        panic!("Unsupported operation")
                    }
                }
            }
        }
    }
    println!("Tree created!");
    let mut travel_vec: Vec<u64> = Vec::new();
    let total_size = measure_element(&tree_root, &mut travel_vec);
    let space_free = 70000000 - total_size;
    let space_remaining = 30000000 - space_free;
    println!("Total size: {}\nFree: {}\nTo reclaim {}", total_size, space_free, space_remaining);
    let mut acc = 0;
    let mut best_del_candidate = total_size;
    for value in travel_vec {
        if value <= 100000{
            acc += value;
        }
        if value < best_del_candidate && value >= space_remaining{
            best_del_candidate = value;
        }
    }
    println!("Size of selected dirs: {}\nBest deletion candidate {}", acc, best_del_candidate);
    //print_tree(&tree_root);
}

fn measure_element(tree_root: &FileTree, travel_vec: &mut Vec<u64>) -> u64 {
    match tree_root {
        FileTree::Dir(tree) => {
            let mut acc = 0;
            for (_, subelement) in tree {
                acc += measure_element(subelement, travel_vec);
            }
            travel_vec.push(acc);
            acc
        }
        FileTree::File(size) => *size,
    }
}

fn _print_tree(tree_root: &FileTree) {
    _print_tree_inner(tree_root, 0);
}

fn _print_tree_inner(tree_root: &FileTree, indent: usize) {
    match tree_root {
        FileTree::Dir(tree) => {
            for (name, element) in tree {
                print!("{}", "  ".repeat(indent));
                match element {
                    &FileTree::File(size) => {
                        println!("{} {}", name, size)
                    }
                    subtree => {
                        println!("<{}>", name);
                        _print_tree_inner(subtree, indent + 1);
                    }
                }
            }
        }
        _ => unreachable!(),
    }
}

fn add_at_path(
    tree_root: &mut FileTree,
    path: &Vec<String>,
    new_elem_name: &str,
    to_add: FileTree,
) {
    match tree_root {
        FileTree::Dir(children) => {
            if path.is_empty() {
                children.insert(new_elem_name.to_string(), to_add);
            } else {
                let key = &path[0];
                let new_root = children.get_mut(&key.to_string()).unwrap();
                add_at_path(new_root, &path[1..].to_vec(), new_elem_name, to_add)
            }
        }
        _ => unreachable!(),
    }
}

fn parse_input_output(line: &str) -> InputOutput {
    let file_line_re: regex::Regex = regex::Regex::new(r"^\d+ .*$").unwrap();
    match line {
        "$ cd /" => InputOutput::CdRoot,
        "$ cd .." => InputOutput::CdUp,
        "$ ls" => InputOutput::Ls,
        cmd if cmd.starts_with("$ cd ") => InputOutput::CdDir(parse_cd_command(cmd)),
        cmd if cmd.starts_with("dir") => InputOutput::DirInfo(parse_dir_output(cmd)),
        cmd if file_line_re.is_match(cmd) => {
            let (fname, fsize) = parse_file_output(cmd);
            InputOutput::FileInfo(fname, fsize)
        }
        _ => InputOutput::Unknown,
    }
}

fn parse_cd_command(line: &str) -> String {
    String::from(line.split(' ').nth(2).unwrap())
}

fn parse_dir_output(line: &str) -> String {
    String::from(line.split(' ').nth(1).unwrap())
}

fn parse_file_output(line: &str) -> (String, u64) {
    let mut iter = line.split(' ');
    let size = iter.next().unwrap().parse::<u64>().unwrap();
    (String::from(iter.next().unwrap()), size)
}

#[cfg(test)]
mod _tests {
    use super::{parse_input_output, InputOutput};

    #[test]
    fn test_parse_input_output() {
        assert_eq!(parse_input_output("$ cd /"), InputOutput::CdRoot);
        assert_eq!(parse_input_output("$ cd .."), InputOutput::CdUp);
        assert_eq!(
            parse_input_output("$ cd aaa"),
            InputOutput::CdDir(String::from("aaa"))
        );
        assert_eq!(parse_input_output("$ ls"), InputOutput::Ls);
        assert_eq!(
            parse_input_output("dir some_dir_name"),
            InputOutput::DirInfo(String::from("some_dir_name"))
        );
        assert_eq!(
            parse_input_output("90131 filename.txt"),
            InputOutput::FileInfo(String::from("filename.txt"), 90131)
        );
        assert_eq!(
            parse_input_output("kukuryku na patyku"),
            InputOutput::Unknown
        );
    }
}
