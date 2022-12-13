use std::{
    path::PathBuf,
};

use clap::{value_parser, ArgMatches, Command};

use crate::common;

#[derive(PartialEq, Eq, Debug, Clone)]
enum PacketTree {
    Int(i32),
    List(Vec<PacketTree>),
    ListBeginsPlaceholder,
}

impl PacketTree {
    fn compare_tree_lists(left: &Vec<PacketTree>, right: &Vec<PacketTree>) -> Option<bool> {
        let mut i: usize = 0;
        loop{
            if left.len() <= i && right.len() <= i{
                //ran out of items, lets leave this recursion
                return None;
            }
            else if left.len() <= i {
                return Some(true);
            }
            else if right.len() <= i {
                return Some(false);
            }
            else{
                let (litem, ritem) = (&left[i],&right[i]);
                match (litem, ritem) {
                    (PacketTree::Int(lval), PacketTree::Int(rval)) if *lval < *rval =>{
                        //println!("Compare {} vs {}, true", lval, rval);
                        return Some(true);
                    }
                    (PacketTree::Int(lval), PacketTree::Int(rval)) if *lval > *rval =>{
                        //println!("Compare {} vs {}, false", lval, rval);
                        return Some(false);
                    }
                    (PacketTree::Int(_), PacketTree::Int(_)) => {
                        //println!("Compare {} vs {}, continue", lval, rval);
                        //values are same, do nothing, continue iteration
                    }
                    (PacketTree::List(llist), PacketTree::List(rlist)) => {
                        match Self::compare_tree_lists(llist, rlist) {
                            Some(result) => {
                                return Some(result);
                            }
                            None => {
                                //do nothing, continue iteration
                            }
                        }
                    }
                    (PacketTree::Int(lval), PacketTree::List(rlist)) => {
                        match Self::compare_tree_lists(&vec![PacketTree::Int(*lval)], rlist) {
                            Some(result) => {
                                return Some(result);
                            }
                            None => {
                                //do nothing, continue iteration
                            }
                        }
                    }
                    (PacketTree::List(llist), PacketTree::Int(rval)) => {
                        match Self::compare_tree_lists(llist, &vec![PacketTree::Int(*rval)]) {
                            Some(result) => {
                                return Some(result);
                            }
                            None => {
                                //do nothing, continue iteration
                            }
                        }
                    }
                    _ => unreachable!()
                }
                i += 1;
            }
        }
    }
}

impl PartialOrd for PacketTree{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self,other) {
            (Self::List(llist), Self::List(rlist)) => {
                match Self::compare_tree_lists(llist, rlist) {
                    None => Some(std::cmp::Ordering::Equal),
                    Some(false) => Some(std::cmp::Ordering::Greater),
                    Some(true) => Some(std::cmp::Ordering::Less),
                }
            }
            _ => Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Ord for PacketTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}


pub fn cli() -> Command {
    Command::new("day13").about("Elvish distress signal").arg(
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
        let mut counter: usize = 0;
        let mut packet_tree_vec = parse_lines(lines.map(|l| l.unwrap()));
        while !packet_tree_vec.is_empty() {
            let presumed_index = packet_tree_vec.len() / 2;
            let right = packet_tree_vec.pop().unwrap();
            let left = packet_tree_vec.pop().unwrap();
            if let (PacketTree::List(llist), PacketTree::List(rlist)) = (left,right){
                if PacketTree::compare_tree_lists(&llist, &rlist).unwrap(){
                    counter += presumed_index;
                }
            }

        }
        println!("Counter value: {}", counter);
    }
    if let Ok(lines) = common::read_lines(&filepath) {
        let mut bak: usize = 1;
        let divpacket_2 = PacketTree::List(vec![PacketTree::List(vec![PacketTree::Int(2)])]);
        let divpacket_6 = PacketTree::List(vec![PacketTree::List(vec![PacketTree::Int(6)])]);
        let mut packet_tree_vec = parse_lines(lines.map(|l| l.unwrap()));
        packet_tree_vec.push(divpacket_2.clone());
        packet_tree_vec.push(divpacket_6.clone());
        packet_tree_vec.sort();
        for (i, tree) in packet_tree_vec.into_iter().enumerate(){
            //println!("{}: {:?}", i, tree);
            if tree == divpacket_2 || tree == divpacket_6{
                println!("index of divider {}", i);
                bak *= i+1;
            }
        }
        println!("multiplied together {}", bak);
    }
}

fn parse_lines(lines: impl Iterator<Item = String>) -> Vec<PacketTree> {
    let mut trees_acc: Vec<PacketTree> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            trees_acc.push(parse_line(&line))
        }
    }
    trees_acc
}
fn parse_line(line: &str) -> PacketTree {
    let mut stack: Vec<PacketTree> = Vec::new();
    let mut parser_acc: String = String::new();
    for ch in line.chars() {
        match ch {
            '[' => {
                stack.push(PacketTree::ListBeginsPlaceholder);
            }
            dig if dig.is_numeric() => {
                parser_acc.push(ch);
            }
            ',' => {
                if !parser_acc.is_empty() {
                    stack.push(PacketTree::Int(parser_acc.parse().unwrap()));
                    parser_acc.clear();
                }
            }
            ']' => {
                if !parser_acc.is_empty() {
                    stack.push(PacketTree::Int(parser_acc.parse().unwrap()));
                    parser_acc.clear();
                }
                let mut new_vec: Vec<PacketTree> = Vec::new();
                loop {
                    match stack.pop().unwrap() {
                        PacketTree::ListBeginsPlaceholder => {
                            new_vec.reverse();
                            stack.push(PacketTree::List(new_vec));
                            break;
                        }
                        list_or_int => {
                            new_vec.push(list_or_int);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    assert!(parser_acc.is_empty() && stack.len() == 1);
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{parse_line, PacketTree};

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("[1,1,3,1,1]"),
            PacketTree::List(vec![
                PacketTree::Int(1),
                PacketTree::Int(1),
                PacketTree::Int(3),
                PacketTree::Int(1),
                PacketTree::Int(1)
            ])
        );
        assert_eq!(
            parse_line("[[1],[2,3,4]]"),
            PacketTree::List(vec![
                PacketTree::List(vec![PacketTree::Int(1)]),
                PacketTree::List(vec![
                    PacketTree::Int(2),
                    PacketTree::Int(3),
                    PacketTree::Int(4)
                ])
            ])
        )
    }
}
