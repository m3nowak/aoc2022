use std::{cmp, collections::HashSet, path::PathBuf};

use clap::{value_parser, ArgMatches, Command};

use crate::common;

const WIDTH: usize = 7;

const MAX_CYCLES: usize = 1000000000000;

enum Direction {
    Left,
    Right,
}

struct DirectionFeed {
    source_str: String,
    index: usize,
}

impl DirectionFeed {
    fn new(source: &str) -> Self {
        Self {
            source_str: String::from(source),
            index: 0,
        }
    }
    fn reset(&mut self) {
        self.index = 0;
    }

    fn cycle_len(&self) -> usize {
        self.source_str.len()
    }
}

impl Iterator for DirectionFeed {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let to_ret = match self.source_str.chars().nth(self.index) {
            Some('<') => Direction::Left,
            Some('>') => Direction::Right,
            _ => unreachable!(),
        };
        self.index = (self.index + 1) % self.source_str.len();

        Some(to_ret)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RockShape {
    LineH,
    Plus,
    LMirrored,
    LiveV,
    Square,
}
// 4|   .x. | Shape pos (*) = (3,2)
// 3|   xxx |
// 2|   *x. |
// 1|       |
// 0+-------+
//   0123456
// Rock position is determined by bottom-left space on boundary square

impl RockShape {
    fn spaces(&self, pos: &(usize, usize)) -> HashSet<(usize, usize)> {
        match self {
            Self::LineH => HashSet::from([
                pos.clone(),
                (pos.0 + 1, pos.1),
                (pos.0 + 2, pos.1),
                (pos.0 + 3, pos.1),
            ]),
            Self::Plus => HashSet::from([
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 + 1),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 2, pos.1 + 1),
                (pos.0 + 1, pos.1 + 2),
            ]),
            Self::LMirrored => HashSet::from([
                pos.clone(),
                (pos.0 + 1, pos.1),
                (pos.0 + 2, pos.1),
                (pos.0 + 2, pos.1 + 1),
                (pos.0 + 2, pos.1 + 2),
            ]),
            Self::LiveV => HashSet::from([
                pos.clone(),
                (pos.0, pos.1 + 1),
                (pos.0, pos.1 + 2),
                (pos.0, pos.1 + 3),
            ]),
            Self::Square => HashSet::from([
                pos.clone(),
                (pos.0, pos.1 + 1),
                (pos.0 + 1, pos.1),
                (pos.0 + 1, pos.1 + 1),
            ]),
        }
    }
    fn can_move_left(&self, pos: &(usize, usize), taken_spaces: &HashSet<(usize, usize)>) -> bool {
        self.spaces(pos).iter().fold(true, |acc, x| {
            acc && x.0 > 0 && !taken_spaces.contains(&(x.0 - 1, x.1))
        })
    }
    fn can_move_right(&self, pos: &(usize, usize), taken_spaces: &HashSet<(usize, usize)>) -> bool {
        self.spaces(pos).iter().fold(true, |acc, x| {
            acc && x.0 < WIDTH - 1 && !taken_spaces.contains(&(x.0 + 1, x.1))
        })
    }
    fn can_move_down(&self, pos: &(usize, usize), taken_spaces: &HashSet<(usize, usize)>) -> bool {
        self.spaces(pos).iter().fold(true, |acc, x| {
            acc && !taken_spaces.contains(&(x.0, x.1 - 1)) && x.1 != 1
        })
    }
}

fn simulate_rocks(
    dir_feed: &mut DirectionFeed,
    iter_count: usize,
    accelerate: bool,
) -> (usize, HashSet<(usize, usize)>) {
    let rock_order = [
        RockShape::LineH,
        RockShape::Plus,
        RockShape::LMirrored,
        RockShape::LiveV,
        RockShape::Square,
    ];
    let mut highest_point: usize = 0;
    let mut highest_growth_history = Vec::new();

    let mut taken_spaces: HashSet<(usize, usize)> = HashSet::new();
    let mut journal: Vec<(RockShape, usize, isize, usize)> = Vec::new(); // shape, xpos, ypos in relation to highest point so far, height growth
    let cycle_interop = &rock_order.len() * dir_feed.cycle_len();
    for iteration in 0..iter_count {
        let rock_shape = &rock_order[iteration % rock_order.len()];
        let mut rock_pos = (2_usize, highest_point + 4);
        let dir = dir_feed.next().unwrap();
        move_rock_if_possible(dir, &mut rock_pos, &taken_spaces, &rock_shape);
        while rock_shape.can_move_down(&rock_pos, &taken_spaces) {
            rock_pos.1 -= 1;
            let dir = dir_feed.next().unwrap();
            move_rock_if_possible(dir, &mut rock_pos, &taken_spaces, &rock_shape)
        }
        
        let new_pos_set = rock_shape.spaces(&rock_pos);
        let mut levels_changed = HashSet::new();
        let old_highest_point = highest_point;
        for new_pos in new_pos_set {
            highest_point = cmp::max(highest_point, new_pos.1);
            levels_changed.insert(new_pos.1);
            taken_spaces.insert(new_pos);
        }
        highest_growth_history.push(highest_point - old_highest_point);
        if accelerate {
            journal.push((
                rock_shape.clone(),
                rock_pos.0,
                highest_point as isize - rock_pos.1 as isize,
                highest_point - old_highest_point
            ));
            if let Some(cycle_size) = check_for_cycles_e(&journal, cycle_interop) {
                let cycle_growth = (1..=cycle_size).into_iter().fold(0, |acc, x| {
                    acc + highest_growth_history[highest_growth_history.len() - x]
                });
                let mut simulated_size = highest_point;
                let mut simulated_iteration = iteration;
                let cycles_to_skip = (iter_count-iteration)/cycle_size;
                simulated_iteration += cycles_to_skip * cycle_size;
                simulated_size += cycles_to_skip * cycle_growth;
                for i in 0..iter_count-simulated_iteration-1{
                    simulated_size += highest_growth_history[highest_growth_history.len() - cycle_size + i];
                    simulated_iteration += 1;
                }
                return (simulated_size, taken_spaces);
            }
        }
    }
    (highest_point, taken_spaces)
}

fn check_for_cycles_e<T: Eq>(vector: &Vec<T>, minimal_cycle_size: usize) -> Option<usize> {
    let max_possible_cycle_len = (vector.len() - 1) / 3;
    for tested_cycle_len in minimal_cycle_size..=max_possible_cycle_len {
        let mut is_cycle = true;
        for offset in 1..=tested_cycle_len {
            let offset_ok = vector[vector.len() - offset]
                == vector[vector.len() - offset - tested_cycle_len]
                && vector[vector.len() - offset]
                    == vector[vector.len() - offset - 2 * tested_cycle_len];
            if !offset_ok {
                is_cycle = false;
                break;
            }
        }
        if is_cycle {
            return Some(tested_cycle_len);
        }
    }
    None
}

fn move_rock_if_possible(
    direction: Direction,
    rock_pos: &mut (usize, usize),
    taken_spaces: &HashSet<(usize, usize)>,
    rock_shape: &RockShape,
) {
    match direction {
        Direction::Left => {
            if rock_shape.can_move_left(&rock_pos, &taken_spaces) {
                rock_pos.0 -= 1
            }
        }
        Direction::Right => {
            if rock_shape.can_move_right(&rock_pos, &taken_spaces) {
                rock_pos.0 += 1
            }
        }
    }
}

pub fn cli() -> Command {
    Command::new("day17").about("Elephant tertis games").arg(
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
    if let Ok(mut lines) = common::read_lines(filepath) {
        let the_line = lines.nth(0).unwrap().unwrap();
        let mut dir_feed = DirectionFeed::new(&the_line);
        let (result, _) = simulate_rocks(&mut dir_feed, 2022, false);
        println!("result {}", result);
        dir_feed.reset();
        let (result2, _) = simulate_rocks(&mut dir_feed, MAX_CYCLES, true);
        println!("result (1): {}", result);
        println!("result (2): {}", result2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_mini() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut dir_feed = DirectionFeed::new(&input);
        let (result, hset) = simulate_rocks(&mut dir_feed, 1, false);
        assert_eq!(result, 1);
        assert_eq!(hset, HashSet::from([(2, 1), (3, 1), (4, 1), (5, 1),]));

        dir_feed.reset();
        let (result, hset) = simulate_rocks(&mut dir_feed, 2, false);
        assert_eq!(result, 4);
        assert_eq!(
            hset,
            HashSet::from([
                (2, 1),
                (3, 1),
                (4, 1),
                (5, 1),
                (3, 2),
                (2, 3),
                (3, 3),
                (4, 3),
                (3, 4),
            ])
        );

        dir_feed.reset();
        let (result, _) = simulate_rocks(&mut dir_feed, 3, false);
        assert_eq!(result, 6);

        dir_feed.reset();
        let (result, _) = simulate_rocks(&mut dir_feed, 4, false);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_pt1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut dir_feed = DirectionFeed::new(&input);
        let (result, _) = simulate_rocks(&mut dir_feed, 2022, false);
        assert_eq!(result, 3068)
    }
    #[test]
    fn test_pt2() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut dir_feed = DirectionFeed::new(&input);

        let (result, _) = simulate_rocks(&mut dir_feed, MAX_CYCLES, true);
        assert_eq!(result, 1514285714288);

        dir_feed.reset();
        let (result_og, _) = simulate_rocks(&mut dir_feed, 40, false);
        dir_feed.reset();
        let (result_acc, _) = simulate_rocks(&mut dir_feed, 40, true);
        assert_eq!(result_og, result_acc);

        dir_feed.reset();
        let (result_og, _) = simulate_rocks(&mut dir_feed, 645, false);
        dir_feed.reset();
        let (result_acc, _) = simulate_rocks(&mut dir_feed, 645, true);
        assert_eq!(result_og, result_acc);

        dir_feed.reset();
        let (result_og, _) = simulate_rocks(&mut dir_feed, 4600, false);
        dir_feed.reset();
        let (result_acc, _) = simulate_rocks(&mut dir_feed, 4600, true);
        assert_eq!(result_og, result_acc);

        dir_feed.reset();
        let (result_og, _) = simulate_rocks(&mut dir_feed, 8000, false);
        dir_feed.reset();
        let (result_acc, _) = simulate_rocks(&mut dir_feed, 8000, true);
        assert_eq!(result_og, result_acc);
    }

    #[test]
    fn test_cycle_check() {
        let a: Vec<usize> = vec![0, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        assert_eq!(check_for_cycles_e(&a, 1), Some(4));
        let a: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(check_for_cycles_e(&a, 1), None);
        let a: Vec<usize> = vec![0, 1, 2, 3, 4, 1, 2, 3, 4];
        assert_eq!(check_for_cycles_e(&a, 1), None);
        let a: Vec<usize> = vec![
            1, 2, 3, 4, 9, 8, 6, 6, 0, 9, 8, 9, 8, 6, 6, 0, 9, 8, 9, 8, 6, 6, 0, 9, 8,
        ];
        assert_eq!(check_for_cycles_e(&a, 1), Some(7));
    }
}
