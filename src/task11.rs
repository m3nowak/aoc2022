use std::{collections::HashMap, path::PathBuf};

use clap::{value_parser, ArgMatches, Command};

// use rust_decimal::prelude::*;
// use rust_decimal_macros::dec;

use crate::common;

#[derive(Eq, PartialEq, Debug, Clone)]
enum ApeOperation {
    Multiply(u128),
    Add(u128),
    Square,
}
#[derive(Eq, PartialEq, Debug, Clone)]
struct Ape {
    items: Vec<u128>,
    operation: ApeOperation,
    div_test: u128,
    tgt_succ: usize,
    tgt_fail: usize,
    inspection_count: u64,
}

impl Ape {
    pub fn new(
        items: Vec<u128>,
        operation: ApeOperation,
        div_test: u128,
        tgt_succ: usize,
        tgt_fail: usize,
    ) -> Self {
        Self {
            items,
            operation,
            div_test,
            tgt_succ,
            tgt_fail,
            inspection_count: 0,
        }
    }
    pub fn run_ape_logic(&mut self, universal_divisor: Option<u128>) -> Vec<(u128, usize)> {
        let mut thrown_items: Vec<(u128, usize)> = Vec::new();
        for og_item in &self.items {
            let mut item = *og_item;
            self.inspection_count += 1;
            //increase item level
            match self.operation {
                ApeOperation::Multiply(val) => {
                    item *= val;
                }
                ApeOperation::Add(val) => {
                    item += val;
                }
                ApeOperation::Square => {
                    item *= item;
                }
            }
            
            match universal_divisor{
                None => {//monke gets bored
                    item /= 3;
                },
                Some(val) => {//monke isn't bored, but we need to keep item val managable
                    item = item % val;
                }
            }

            //monke decides where to throw
            if (item % self.div_test) == 0 {
                thrown_items.push((item, self.tgt_succ))
            } else {
                thrown_items.push((item, self.tgt_fail))
            }
        }
        self.items.clear();
        thrown_items
    }
    pub fn add_item(&mut self, item: u128) {
        self.items.push(item);
    }
}

pub fn cli() -> Command {
    Command::new("day11").about("Sudden ape attack").arg(
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
    let mut ape_map: HashMap<usize, Ape> = HashMap::new();
    let mut str_acc: Vec<String> = Vec::new();
    let mut ape_indicies: Vec<usize> = Vec::new();
    if let Ok(lines) = common::read_lines(filepath) {
        for line in lines {
            if let Ok(line_text) = line {
                if !line_text.is_empty() {
                    str_acc.push(line_text);
                }
            }
            if str_acc.len() >= 6 {
                let (ape, ape_no) = lines_to_ape(&str_acc);
                ape_map.insert(ape_no, ape);
                ape_indicies.push(ape_no);
                str_acc.clear();
            }
        }
    }
    ape_indicies.sort();
    let ape_map_clone = ape_map.clone();
    for _ in 0..20 {
        for ape_index in &ape_indicies {
            let thrown_items = ape_map.get_mut(ape_index).unwrap().run_ape_logic(None);
            for (item_value, item_target) in &thrown_items {
                ape_map.get_mut(item_target).unwrap().add_item(*item_value)
            }
        }
    }
    let mut ape_levels: Vec<u64> = ape_indicies
        .iter()
        .map(|i| ape_map[i].inspection_count)
        .collect();
    ape_levels.sort_by(|a, b| b.cmp(a));
    print!("Ape levels:");
    for ape_level in &ape_levels {
        print!(" {}", ape_level);
    }
    println!(
        "\nTwo higest multiplied together: {}",
        ape_levels[0] * ape_levels[1]
    );
    ape_map = ape_map_clone;

    let mut universal_divisor = 1; //shamelessly stolen from https://github.com/LinAGKar/advent-of-code-2022-rust/blob/main/day11b/src/main.rs
    for (_, ape) in &ape_map{
        if universal_divisor % ape.div_test != 0 {
            universal_divisor *= ape.div_test;
        }
    }

    for _ in 0..10000 {
        for ape_index in &ape_indicies {
            let thrown_items = ape_map.get_mut(ape_index).unwrap().run_ape_logic(Some(universal_divisor));
            for (item_value, item_target) in &thrown_items {
                ape_map.get_mut(item_target).unwrap().add_item(*item_value)
            }
        }
    }
    let mut ape_levels: Vec<u64> = ape_indicies
        .iter()
        .map(|i| ape_map[i].inspection_count)
        .collect();
    ape_levels.sort_by(|a, b| b.cmp(a));
    print!("Ape levels (run 2):");
    for ape_level in &ape_levels {
        print!(" {}", ape_level);
    }
    println!(
        "\nTwo higest multiplied together: {}",
        ape_levels[0] * ape_levels[1]
    );
}

fn lines_to_ape(lines: &Vec<String>) -> (Ape, usize) {
    let fst_line_no = lines[0].split(' ').nth(1).unwrap();
    let ape_no: usize = fst_line_no[0..fst_line_no.len() - 1].parse().unwrap();
    let snd_line_cut = lines[1].split(": ").nth(1).unwrap();
    let items: Vec<u128> = snd_line_cut
        .split(", ")
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let operation: ApeOperation = match &lines[2] {
        trdline
            if regex::Regex::new(r"^  Operation: new = old \* \d+$")
                .unwrap()
                .is_match(trdline) =>
        {
            ApeOperation::Multiply(trdline.split(' ').last().unwrap().parse().unwrap())
        }
        trdline
            if regex::Regex::new(r"^  Operation: new = old \+ \d+$")
                .unwrap()
                .is_match(trdline) =>
        {
            ApeOperation::Add(trdline.split(' ').last().unwrap().parse().unwrap())
        }
        trdline
            if regex::Regex::new(r"^  Operation: new = old \* old$")
                .unwrap()
                .is_match(trdline) =>
        {
            ApeOperation::Square
        }
        _ => panic!(),
    };
    let div_test: u128 = lines[3].split(' ').last().unwrap().parse().unwrap();
    let tgt_succ: usize = lines[4].split(' ').last().unwrap().parse().unwrap();
    let tgt_fail: usize = lines[5].split(' ').last().unwrap().parse().unwrap();
    (
        Ape::new(items, operation, div_test, tgt_succ, tgt_fail),
        ape_no,
    )
}

#[cfg(test)]
mod tests {

    use crate::task11::{lines_to_ape, Ape, ApeOperation};

    #[test]
    fn test_parse() {
        let (ape, ape_no) = lines_to_ape(&vec![
            String::from("Monkey 0:"),
            String::from("  Starting items: 79, 98"),
            String::from("  Operation: new = old * 19"),
            String::from("  Test: divisible by 23"),
            String::from("      If true: throw to monkey 2"),
            String::from("      If false: throw to monkey 3"),
        ]);
        assert_eq!(ape_no, 0);
        assert_eq!(
            ape,
            Ape::new(vec![79, 98], ApeOperation::Multiply(19), 23, 2, 3)
        );

        let (ape, ape_no) = lines_to_ape(&vec![
            String::from("Monkey 1:"),
            String::from("  Starting items: 54, 65, 75, 74"),
            String::from("  Operation: new = old + 6"),
            String::from("  Test: divisible by 19"),
            String::from("      If true: throw to monkey 2"),
            String::from("      If false: throw to monkey 0"),
        ]);
        assert_eq!(ape_no, 1);
        assert_eq!(
            ape,
            Ape::new(vec![54, 65, 75, 74], ApeOperation::Add(6), 19, 2, 0)
        );

        let (ape, ape_no) = lines_to_ape(&vec![
            String::from("Monkey 2:"),
            String::from("  Starting items: 79, 60, 97"),
            String::from("  Operation: new = old * old"),
            String::from("  Test: divisible by 13"),
            String::from("      If true: throw to monkey 1"),
            String::from("      If false: throw to monkey 3"),
        ]);
        assert_eq!(ape_no, 2);
        assert_eq!(
            ape,
            Ape::new(vec![79, 60, 97], ApeOperation::Square, 13, 1, 3)
        );

        let (ape, ape_no) = lines_to_ape(&vec![
            String::from("Monkey 3:"),
            String::from("  Starting items: 74"),
            String::from("  Operation: new = old + 3"),
            String::from("  Test: divisible by 17"),
            String::from("      If true: throw to monkey 0"),
            String::from("      If false: throw to monkey 1"),
        ]);
        assert_eq!(ape_no, 3);
        assert_eq!(ape, Ape::new(vec![74], ApeOperation::Add(3), 17, 0, 1));
    }
    #[test]
    fn test_round() {
        let mut ape_vec = vec![
            Ape::new(vec![79, 98], ApeOperation::Multiply(19), 23, 2, 3),
            Ape::new(vec![54, 65, 75, 74], ApeOperation::Add(6), 19, 2, 0),
            Ape::new(vec![79, 60, 97], ApeOperation::Square, 13, 1, 3),
            Ape::new(vec![74], ApeOperation::Add(3), 17, 0, 1),
        ];
        for ape_index in 0..ape_vec.len(){
            let mus = ape_vec[ape_index].run_ape_logic(None);
            for (item, item_tgt) in mus{
                ape_vec[item_tgt].add_item(item);
            }
        }
        assert_eq!(ape_vec[0].items, vec![20, 23, 27, 26]);
        assert_eq!(ape_vec[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert!(ape_vec[2].items.is_empty());
        assert!(ape_vec[3].items.is_empty());
    }
}
