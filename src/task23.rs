use std::{
    cmp,
    collections::{HashMap, HashSet},
    hash::Hash,
    path::PathBuf,
};

use clap::{value_parser, ArgMatches, Command};

use crate::common;

pub fn cli() -> Command {
    Command::new("day23")
        .about("Elvish uncontrolled spread")
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

pub fn solve(filepath: PathBuf) {
    if let Ok(lines) = common::read_lines(&filepath) {
        let (mut elf_pos, dim) = parse_input(lines.map(|l| l.unwrap()));
        for _ in 0..10 {
            elf_pos = calc_round(&elf_pos, &dim);
        }
        print_map(&elf_pos, &dim);
        println!("Solution (1): {}", calc_score(&elf_pos));
    }

    if let Ok(lines) = common::read_lines(&filepath) {
        let (mut elf_pos, dim) = parse_input(lines.map(|l| l.unwrap()));
        let mut count = 0;
        loop {
            let new_elf_pos = calc_round(&elf_pos, &dim);
            count += 1;
            if new_elf_pos == elf_pos {
                break;
            } else {
                elf_pos = new_elf_pos;
            }
        }

        println!("Solution (2): {}", count);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Heading {
    N,
    E,
    S,
    W,
}

impl Heading {
    pub fn as_num(&self) -> isize {
        match self {
            Self::N => 0,
            Self::S => 1,
            Self::W => 2,
            Self::E => 3,
        }
    }
    pub fn from_num(num: isize) -> Self {
        match num.rem_euclid(4) {
            0 => Self::N,
            1 => Self::S,
            2 => Self::W,
            3 => Self::E,
            _ => unreachable!(),
        }
    }
    pub fn shift_pos(&self, pos: &(isize, isize)) -> (isize, isize) {
        match self {
            Self::N => (pos.0, pos.1 - 1),
            Self::S => (pos.0, pos.1 + 1),
            Self::W => (pos.0 - 1, pos.1),
            Self::E => (pos.0 + 1, pos.1),
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct Elf {
    pos: (isize, isize),
    decision_count: usize,
}

impl Elf {
    fn new(pos: &(isize, isize)) -> Self {
        Self {
            pos: pos.clone(),
            decision_count: 0,
        }
    }
    fn inc_dec_count(&mut self) {
        self.decision_count = (self.decision_count + 1) % 4;
    }
    fn consideration(&self) -> Vec<Heading> {
        (self.decision_count..self.decision_count + 4)
            .map(|v| Heading::from_num(v as isize))
            .collect()
    }
}

impl Hash for Elf {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

fn all_neighbours(pos: &(isize, isize), dim: &(isize, isize)) -> Vec<(isize, isize)> {
    let pos_i = (pos.0 as isize, pos.1 as isize);
    //let dim_i = (dim.0 as isize, dim.1 as isize);

    let candidates = vec![
        (pos_i.0 + 1, pos_i.1),
        (pos_i.0 + 1, pos_i.1 + 1),
        (pos_i.0, pos_i.1 + 1),
        (pos_i.0 - 1, pos_i.1 + 1),
        (pos_i.0 - 1, pos_i.1),
        (pos_i.0 - 1, pos_i.1 - 1),
        (pos_i.0, pos_i.1 - 1),
        (pos_i.0 + 1, pos_i.1 - 1),
    ];

    candidates
        .into_iter()
        //.filter(|c| c.0 >= 0 && c.0 < dim_i.0 && c.1 >= 0 && c.1 < dim_i.1)
        .map(|t| (t.0, t.1))
        .collect()
}

fn heading_neighbours(
    pos: &(isize, isize),
    heading: &Heading,
    dim: &(isize, isize),
) -> Option<Vec<(isize, isize)>> {
    let mut ret = vec![];
    match heading {
        Heading::E => {
            ret.push((pos.0 + 1, pos.1));
            //if pos.1 > 0 {
            ret.push((pos.0 + 1, pos.1 - 1));
            //}
            //if pos.1 < dim.1 - 1 {
            ret.push((pos.0 + 1, pos.1 + 1));
            //}
        }
        Heading::S => {
            ret.push((pos.0, pos.1 + 1));
            //if pos.0 > 0 {
            ret.push((pos.0 - 1, pos.1 + 1));
            //}
            //if pos.0 < dim.0 - 1 {
            ret.push((pos.0 + 1, pos.1 + 1));
            //}
        }
        Heading::W => {
            ret.push((pos.0 - 1, pos.1));
            //if pos.1 > 0 {
            ret.push((pos.0 - 1, pos.1 - 1));
            //}
            //if pos.1 < dim.1 - 1 {
            ret.push((pos.0 - 1, pos.1 + 1));
            //}
        }
        Heading::N => {
            ret.push((pos.0, pos.1 - 1));
            //if pos.0 > 0 {
            ret.push((pos.0 - 1, pos.1 - 1));
            //}
            //if pos.0 < dim.0 - 1 {
            ret.push((pos.0 + 1, pos.1 - 1));
            //}
        }
    };
    return Some(ret);
}

fn calc_decision(
    elf: &Elf,
    elf_pos: &HashSet<(isize, isize)>,
    dim: &(isize, isize),
) -> (Option<(isize, isize)>, bool) {
    let should_move = all_neighbours(&elf.pos, dim)
        .into_iter()
        .map(|t| elf_pos.contains(&t))
        .fold(false, |acc, c| acc || c);
    if should_move {
        for heading in elf.consideration() {
            match heading_neighbours(&elf.pos, &heading, dim) {
                Some(hvec) => {
                    let move_in_heading = hvec
                        .into_iter()
                        .map(|t| !elf_pos.contains(&t))
                        .fold(true, |acc, c| acc && c);
                    if move_in_heading {
                        return (Some(heading.shift_pos(&elf.pos)), should_move);
                    }
                }
                None => {
                    //we cannot move this way, but consideration occured
                }
            }
        }
    }

    return (None, should_move);
}

fn calc_round(elf_set: &HashSet<Elf>, dim: &(isize, isize)) -> HashSet<Elf> {
    let mut decisions: HashMap<(isize, isize), Elf> = HashMap::new();
    let mut burned_spots: HashSet<(isize, isize)> = HashSet::new();
    let mut new_elf_set: HashSet<Elf> = HashSet::new();

    let elf_pos_set: HashSet<(isize, isize)> = elf_set.iter().map(|e| e.pos.clone()).collect();
    for elf in elf_set.iter() {
        let (decision, consideration_occured) = calc_decision(&elf, &elf_pos_set, dim);
        let mut elf_clone = elf.clone();
        match decision {
            Some(new_pos) => {
                if !burned_spots.contains(&new_pos) {
                    if decisions.contains_key(&new_pos) {
                        decisions.remove(&new_pos);
                        burned_spots.insert(new_pos);
                    } else {
                        decisions.insert(new_pos, elf.clone());
                    }
                }
            }
            None => {
                //do nothing
            }
        }
        //if consideration_occured {
        elf_clone.inc_dec_count();
        //}
        new_elf_set.insert(elf_clone);
    }

    for (new_pos, elf) in decisions {
        let mut new_elf = new_elf_set.take(&elf).unwrap();
        new_elf.pos = new_pos;
        new_elf_set.insert(new_elf);
    }
    new_elf_set
}

fn calc_score(elf_pos: &HashSet<Elf>) -> isize {
    let pos_set: HashSet<(isize, isize)> = elf_pos.iter().map(|e| e.pos).collect();
    let mut elf_pos_iter = pos_set.iter();
    let mut min_anchor = elf_pos_iter.next().unwrap().clone();
    let mut max_anchor = min_anchor.clone();
    for pos in elf_pos_iter {
        min_anchor.0 = cmp::min(pos.0, min_anchor.0);
        min_anchor.1 = cmp::min(pos.1, min_anchor.1);
        max_anchor.0 = cmp::max(pos.0, max_anchor.0);
        max_anchor.1 = cmp::max(pos.1, max_anchor.1);
    }
    let mut acc = 0;
    for x in min_anchor.0..=max_anchor.0 {
        for y in min_anchor.1..=max_anchor.1 {
            if !pos_set.contains(&(x, y)) {
                acc += 1;
            }
        }
    }
    acc
}

fn print_map(elf_pos: &HashSet<Elf>, dim: &(isize, isize)) {
    let pos_set: HashSet<(isize, isize)> = elf_pos.iter().map(|e| e.pos).collect();
    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if pos_set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_input(lines: impl Iterator<Item = String>) -> (HashSet<Elf>, (isize, isize)) {
    let mut ret: HashSet<Elf> = HashSet::new();
    let mut dim = (0, 0);

    for (y, line) in lines.enumerate() {
        dim.1 = cmp::max(dim.1, y as isize);
        for (x, chr) in line.chars().enumerate() {
            dim.0 = cmp::max(dim.0, x as isize);
            match chr {
                '#' => {
                    ret.insert(Elf::new(&(x as isize, y as isize)));
                }
                _ => {
                    //do nothing
                }
            }
        }
    }
    dim.0 += 1;
    dim.1 += 1;
    (ret, dim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_neighbours_none() {
        assert_eq!(heading_neighbours(&(2, 0), &Heading::N, &(10, 10)), None);
        assert_eq!(heading_neighbours(&(0, 4), &Heading::W, &(10, 10)), None);
        assert_eq!(heading_neighbours(&(9, 3), &Heading::E, &(10, 10)), None);
        assert_eq!(heading_neighbours(&(6, 9), &Heading::S, &(10, 10)), None);
    }

    #[test]
    fn test_heading_neighbours_some() {
        let ngbh = heading_neighbours(&(0, 2), &Heading::N, &(10, 10)).unwrap();
        assert_eq!(ngbh.len(), 2);
        assert!(ngbh.contains(&(0, 1)));
        assert!(ngbh.contains(&(1, 1)));

        let ngbh = heading_neighbours(&(4, 9), &Heading::W, &(10, 10)).unwrap();
        assert_eq!(ngbh.len(), 2);
        assert!(ngbh.contains(&(3, 9)));
        assert!(ngbh.contains(&(3, 8)));

        let ngbh = heading_neighbours(&(9, 2), &Heading::S, &(10, 10)).unwrap();
        assert_eq!(ngbh.len(), 2);
        assert!(ngbh.contains(&(9, 3)));
        assert!(ngbh.contains(&(8, 3)));

        let ngbh = heading_neighbours(&(5, 5), &Heading::E, &(10, 10)).unwrap();
        assert_eq!(ngbh.len(), 3);
        assert!(ngbh.contains(&(6, 4)));
        assert!(ngbh.contains(&(6, 5)));
        assert!(ngbh.contains(&(6, 6)));
    }

    fn get_mock_input() -> Vec<String> {
        vec![
            "..............",
            "..............",
            ".......#......",
            ".....###.#....",
            "...#...#.#....",
            "....#...##....",
            "...#.###......",
            "...##.#.##....",
            "....#..#......",
            "..............",
            "..............",
            "..............",
        ]
        .into_iter()
        .map(|l| String::from(l))
        .collect()
    }

    fn get_mock_result() -> Vec<String> {
        vec![
            ".......#......",
            "...........#..",
            "..#.#..#......",
            "......#.......",
            "...#.....#..#.",
            ".#......##....",
            ".....##.......",
            "..#........#..",
            "....#.#..#....",
            "..............",
            "....#..#..#...",
            "..............",
        ]
        .into_iter()
        .map(|l| String::from(l))
        .collect()
    }

    fn get_mock_input_small() -> Vec<String> {
        vec![".....", "..##.", "..#..", ".....", "..##.", "....."]
            .into_iter()
            .map(|l| String::from(l))
            .collect()
    }

    #[test]
    fn test_pt1_mini() {
        let lines = get_mock_input_small();
        let (mut elf_pos, dim) = parse_input(lines.into_iter());
        for _ in 0..3 {
            elf_pos = calc_round(&elf_pos, &dim);
        }
        assert_eq!(elf_pos.len(), 5);
        assert!(elf_pos.contains(&Elf::new(&(2, 0))));
        assert!(elf_pos.contains(&Elf::new(&(4, 1))));
        assert!(elf_pos.contains(&Elf::new(&(0, 2))));
        assert!(elf_pos.contains(&Elf::new(&(4, 3))));
        assert!(elf_pos.contains(&Elf::new(&(2, 5))));
        print_map(&elf_pos, &dim);
    }

    #[test]
    fn test_pt1() {
        let lines = get_mock_input();
        let (mut elf_pos, dim) = parse_input(lines.into_iter());
        let len_start = elf_pos.len();
        for _ in 0..10 {
            elf_pos = calc_round(&elf_pos, &dim);
        }
        assert_eq!(elf_pos.len(), len_start);
        assert_eq!(calc_score(&elf_pos), 110);
        let (elf_pos_desired, _) = parse_input(get_mock_result().into_iter());
        assert_eq!(elf_pos_desired, elf_pos);
    }
    #[test]
    fn test_pt2() {
        let lines = get_mock_input();
        let (mut elf_pos, dim) = parse_input(lines.into_iter());
        let mut count = 0;
        loop {
            let new_elf_pos = calc_round(&elf_pos, &dim);
            count += 1;
            if new_elf_pos == elf_pos {
                break;
            } else {
                elf_pos = new_elf_pos;
            }
        }

        assert_eq!(count, 20);
    }
}
