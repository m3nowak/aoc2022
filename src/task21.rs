use std::{path::PathBuf, collections::HashMap, ops::{Add, Sub, Mul, Div}};

use clap::{value_parser, ArgMatches, Command};
use regex::Regex;

use crate::common;

const NUM_RE: &str = r"^[a-z]*: \d+$";
const ADD_RE: &str = r"^[a-z]*: [a-z]+ \+ [a-z]+$";
const MUL_RE: &str = r"^[a-z]*: [a-z]+ \* [a-z]+$";
const DIV_RE: &str = r"^[a-z]*: [a-z]+ / [a-z]+$";
const SUB_RE: &str = r"^[a-z]*: [a-z]+ \- [a-z]+$";

const NAME_DIG_RE: &str = r"[a-z0-9]+";


#[derive(PartialEq, Eq, Debug)]
enum MonkeOp {
    Number(isize),
    Add(String, String),
    Multiply(String, String),
    Divide(String, String),
    Subtract(String, String),
    Equal(String, String),
    Human,
}

impl MonkeOp {
    fn from_line(line: &str, human: bool) -> Option<(Self, String)> {
        let nn_vec: Vec<String> = Regex::new(NAME_DIG_RE)
            .unwrap()
            .find_iter(line)
            .map(|m| String::from(m.as_str()))
            .collect();
        if human{
            match &nn_vec[0] {
                s if s.starts_with("root") => {
                    return Some((Self::Equal(nn_vec[1].clone(), nn_vec[2].clone()), s.clone()));
                }
                s if s.starts_with("humn") => {
                    return Some((Self::Human, s.clone()));
                }
                _ => {
                    //do nothing
                }
            }
        }
        match line {
            lm if Regex::new(NUM_RE).unwrap().is_match(lm) => {
                Some((Self::Number(nn_vec[1].parse().unwrap()), nn_vec[0].clone()))
            }
            lm if Regex::new(ADD_RE).unwrap().is_match(lm) => Some((
                Self::Add(nn_vec[1].clone(), nn_vec[2].clone()),
                nn_vec[0].clone(),
            )),
            lm if Regex::new(MUL_RE).unwrap().is_match(lm) => Some((
                Self::Multiply(nn_vec[1].clone(), nn_vec[2].clone()),
                nn_vec[0].clone(),
            )),
            lm if Regex::new(SUB_RE).unwrap().is_match(lm) => Some((
                Self::Subtract(nn_vec[1].clone(), nn_vec[2].clone()),
                nn_vec[0].clone(),
            )),
            lm if Regex::new(DIV_RE).unwrap().is_match(lm) => Some((
                Self::Divide(nn_vec[1].clone(), nn_vec[2].clone()),
                nn_vec[0].clone(),
            )),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum StackOp {
    Root,
    Add(isize, Box<StackOp>),
    Mul(isize, Box<StackOp>),
    SubRhs(isize, Box<StackOp>), // 5 - humn
    SubLhs(isize, Box<StackOp>), // humn - 5
    DivRhs(isize, Box<StackOp>), // 5 / humn
    DivLhs(isize, Box<StackOp>), // humn / 5
}

#[derive(PartialEq, Eq, Debug)]
enum MonkeRes{
    Response(isize),
    HumanStack(StackOp)
}

impl Add for MonkeRes {
    type Output = MonkeRes;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MonkeRes::Response(v1), MonkeRes::Response(v2)) => Self::Response(v1+v2),
            (MonkeRes::HumanStack(sta), MonkeRes::Response(val)) => Self::HumanStack(StackOp::Add(val, Box::new(sta))),
            (MonkeRes::Response(val), MonkeRes::HumanStack(sta)) => Self::HumanStack(StackOp::Add(val, Box::new(sta))),
            _ => unreachable!()
        }
    }
}

impl Sub for MonkeRes {
    type Output = MonkeRes;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MonkeRes::Response(v1), MonkeRes::Response(v2)) => Self::Response(v1-v2),
            (MonkeRes::HumanStack(sta), MonkeRes::Response(val)) => Self::HumanStack(StackOp::SubLhs(val, Box::new(sta))),
            (MonkeRes::Response(val), MonkeRes::HumanStack(sta)) => Self::HumanStack(StackOp::SubRhs(val, Box::new(sta))),
            _ => unreachable!()
        }
    }
}

impl Mul for MonkeRes {
    type Output  = MonkeRes;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MonkeRes::Response(v1), MonkeRes::Response(v2)) => Self::Response(v1*v2),
            (MonkeRes::HumanStack(sta), MonkeRes::Response(val)) => Self::HumanStack(StackOp::Mul(val, Box::new(sta))),
            (MonkeRes::Response(val), MonkeRes::HumanStack(sta)) => Self::HumanStack(StackOp::Mul(val, Box::new(sta))),
            _ => unreachable!()
        }
    }
}

impl Div for MonkeRes {
    type Output  = MonkeRes;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MonkeRes::Response(v1), MonkeRes::Response(v2)) => Self::Response(v1/v2),
            (MonkeRes::HumanStack(sta), MonkeRes::Response(val)) => Self::HumanStack(StackOp::DivLhs(val, Box::new(sta))),
            (MonkeRes::Response(val), MonkeRes::HumanStack(sta)) => Self::HumanStack(StackOp::DivRhs(val, Box::new(sta))),
            _ => unreachable!()
        }
    }
}


pub fn cli() -> Command {
    Command::new("day21").about("Monke srikes back").arg(
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
        let monkes = parse_input(lines.map(|l| l.unwrap()), false);
        println!("Result(1): {:?}", find_val("root", &monkes));
    }
    if let Ok(lines) = common::read_lines(&filepath) {
        let monkes = parse_input(lines.map(|l| l.unwrap()), true);
        println!("Result(2): {:?}", find_val("root", &monkes));
    }
}

fn find_val_equality(stack: StackOp, val: isize) -> isize{
    match stack {
        StackOp::Root => {val},
        StackOp::Add(opval, inner_sta) => {
            find_val_equality(*inner_sta, val-opval)
        }
        StackOp::Mul(opval, inner_sta) => {
            find_val_equality(*inner_sta, val/opval)
        }
        StackOp::SubRhs(opval, inner_sta) => { // 5 - stack = x -> 5 - x = stack
            find_val_equality(*inner_sta, opval-val)
        }
        StackOp::SubLhs(opval, inner_sta) => { // stack - 5 = x -> x + 5 = stack
            find_val_equality(*inner_sta, val+opval)
        }
        StackOp::DivRhs(opval, inner_sta) => { //5 / stack = x -> 5 / x = stack
            find_val_equality(*inner_sta, val/opval)
        }
        StackOp::DivLhs(opval, inner_sta) => { //stack / 5
            find_val_equality(*inner_sta, val*opval)
        }
    }
}

fn find_val(key: &str, monkes: &HashMap<String,MonkeOp>) -> MonkeRes{
    match monkes.get(key) {
        Some(MonkeOp::Human) => MonkeRes::HumanStack(StackOp::Root),
        Some(MonkeOp::Equal(a, b)) => {
            match (find_val(a, monkes), find_val(b, monkes)) {
                (MonkeRes::Response(val), MonkeRes::HumanStack(sta)) => MonkeRes::Response(find_val_equality(sta, val)),
                (MonkeRes::HumanStack(sta), MonkeRes::Response(val)) => MonkeRes::Response(find_val_equality(sta, val)),
                _ => unreachable!()
            }
        },
        Some(MonkeOp::Number(num)) => MonkeRes::Response(*num),
        Some(MonkeOp::Add(a,b)) => find_val(a, monkes) + find_val(b, monkes),
        Some(MonkeOp::Multiply(a,b)) => find_val(a, monkes) * find_val(b, monkes),
        Some(MonkeOp::Divide(a,b)) => find_val(a, monkes) / find_val(b, monkes),
        Some(MonkeOp::Subtract(a,b)) => find_val(a, monkes) - find_val(b, monkes),
        _ => unreachable!()
    }
}

fn parse_input(lines: impl Iterator<Item = String>, human: bool) -> HashMap<String,MonkeOp> {
    lines
        .filter_map(|l| match MonkeOp::from_line(&l, human){
            Some((v,k)) => Some((k,v)),
            None => None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            MonkeOp::from_line("hmdt: 32", false),
            Some((MonkeOp::Number(32), String::from("hmdt")))
        );
        assert_eq!(
            MonkeOp::from_line("root: pppw + sjmn", false),
            Some((MonkeOp::Add(String::from("pppw"), String::from("sjmn")), String::from("root")))
        );
        assert_eq!(
            MonkeOp::from_line("root: pppw - sjmn", false),
            Some((MonkeOp::Subtract(String::from("pppw"), String::from("sjmn")), String::from("root")))
        );
        assert_eq!(
            MonkeOp::from_line("root: pppw / sjmn", false),
            Some((MonkeOp::Divide(String::from("pppw"), String::from("sjmn")), String::from("root")))
        );
        assert_eq!(
            MonkeOp::from_line("root: pppw * sjmn", false),
            Some((MonkeOp::Multiply(String::from("pppw"), String::from("sjmn")), String::from("root")))
        );
        assert_eq!(
            MonkeOp::from_line("haha i dont know", false),
            None
        );
    }
    #[test]
    fn test_part1() {
        let lines = vec![
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ];
        let monkes = parse_input(lines.into_iter().map(|l| String::from(l)), false);
        assert_eq!(find_val("root", &monkes), MonkeRes::Response(152));
    }
    #[test]
    fn test_part2() {
        let lines = vec![
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ];
        let monkes = parse_input(lines.into_iter().map(|l| String::from(l)), true);
        assert_eq!(find_val("root", &monkes), MonkeRes::Response(301));
    }
}
