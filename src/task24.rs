use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use rayon::prelude::*;

use clap::{value_parser, ArgMatches, Command};

use crate::common;

pub fn cli() -> Command {
    Command::new("day24").about("Elvish blizzard").arg(
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
        let mut map = BasinMap::parse_input(lines.map(|l| l.unwrap()));
        let result = calculate_path(&map, false).unwrap();
        let len = result.len() - 1;
        println!("Result (1): {}", len);
        for _ in 0..len {
            map.blizzards = map.next_blizzards()
        }
        let result = calculate_path(&map, true);
        let len2 = result.unwrap().len() - 1;
        for _ in 0..len2 {
            map.blizzards = map.next_blizzards()
        }
        let result = calculate_path(&map, false);
        let len3 = result.unwrap().len() - 1;
        println!("Result (2): {}", len+len2+len3);

    }
}

fn mh_dist(from: &(usize, usize), to: &(usize, usize)) -> usize {
    let i_from = (from.0 as isize, from.1 as isize);
    let i_to = (to.0 as isize, to.1 as isize);
    (i_from.0 - i_to.0).abs() as usize + (i_from.1 - i_to.1).abs() as usize
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn all_iter() -> impl Iterator<Item = Self> {
        [Self::N, Self::E, Self::S, Self::W].into_iter()
    }

    fn neg(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct BasinMap {
    dim: (usize, usize),
    pos: (usize, usize),
    blizzards: HashMap<(usize, usize), HashSet<Direction>>,
}

impl BasinMap {
    fn possible_moves(
        &self,
        pos: &(usize, usize),
        next_blizzards: &HashMap<(usize, usize), HashSet<Direction>>,
    ) -> Vec<(usize, usize)> {
        let mut ret = match next_blizzards.contains_key(pos) {
            true => vec![],
            false => vec![*pos],
        };
        for dir in Direction::all_iter() {
            let new_pos = self.shift_coor(&dir, &pos);
            if let Some(n_pos) = new_pos {
                if !next_blizzards.contains_key(&n_pos) {
                    ret.push(n_pos);
                }
            }
        }
        ret
    }
    fn next_blizzards(&self) -> HashMap<(usize, usize), HashSet<Direction>> {
        let mut new_blizzards: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
        for (pos, dir_hs) in &self.blizzards {
            for dir in dir_hs {
                let new_pos = match self.shift_coor(dir, pos) {
                    Some(new_pos) => new_pos,
                    None => {
                        let mut acc = pos.clone();
                        while let Some(alt_pos) = self.shift_coor(&dir.neg(), &acc) {
                            acc = alt_pos;
                        }
                        acc
                    }
                };
                if new_blizzards.contains_key(&new_pos) {
                    let hset = new_blizzards.get_mut(&new_pos).unwrap();
                    hset.insert(dir.clone());
                } else {
                    new_blizzards.insert(new_pos, HashSet::from([dir.clone()]));
                }
            }
        }

        new_blizzards
    }

    fn shift_coor(&self, direction: &Direction, coor: &(usize, usize)) -> Option<(usize, usize)> {
        if (*direction == Direction::N && coor.1 == 0)
            || (*direction == Direction::W && coor.0 == 0)
        {
            return None;
        }
        let candidate = match direction {
            Direction::N => (coor.0, coor.1 - 1),
            Direction::E => (coor.0 + 1, coor.1),
            Direction::S => (coor.0, coor.1 + 1),
            Direction::W => (coor.0 - 1, coor.1),
        };
        if self.is_pos_legal(&candidate) {
            Some(candidate)
        } else {
            None
        }
    }

    fn is_pos_legal(&self, pos: &(usize, usize)) -> bool {
        pos == &self.start_pos()
            || pos == &self.exit_pos()
            || (pos.0 > 0 && pos.1 > 0 && pos.0 < self.dim.0 - 1 && pos.1 < self.dim.1 - 1)
    }

    fn start_pos(&self) -> (usize, usize) {
        (1, 0)
    }
    fn exit_pos(&self) -> (usize, usize) {
        (self.dim.0 - 2, self.dim.1 - 1)
    }

    fn parse_input(lines: impl Iterator<Item = String>) -> Self {
        let mut blizzards: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
        let mut ymax = 0;
        let mut xmax = 0;
        for (y_pos, line) in lines.enumerate() {
            xmax = line.len();
            ymax += 1;
            for (x_pos, chr) in line.chars().enumerate() {
                match chr {
                    '>' => {
                        blizzards.insert((x_pos, y_pos), HashSet::from([Direction::E]));
                    }
                    'v' => {
                        blizzards.insert((x_pos, y_pos), HashSet::from([Direction::S]));
                    }
                    '<' => {
                        blizzards.insert((x_pos, y_pos), HashSet::from([Direction::W]));
                    }
                    '^' => {
                        blizzards.insert((x_pos, y_pos), HashSet::from([Direction::N]));
                    }
                    _ => {
                        //do nothing
                    }
                }
            }
        }
        Self {
            dim: (xmax, ymax),
            pos: (1, 0),
            blizzards,
        }
    }
}

fn calculate_path(map: &BasinMap, reverse: bool) -> Option<Vec<((usize, usize), usize)>> {
    let mut blizzards_map = HashMap::from([(0, map.next_blizzards().clone())]);
    let start = if reverse {
        (map.exit_pos(), 0_usize)
    } else {
        (map.start_pos(), 0_usize)
    };
    let end = if reverse {
        map.start_pos()
    } else {
        map.exit_pos()
    };
    let mut open_set = HashSet::from([start.clone()]);
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start.clone(), 0);

    let mut f_score = HashMap::new();
    f_score.insert(start.clone(), mh_dist(&map.pos, &map.exit_pos()));

    while !open_set.is_empty() {
        let current = open_set
            .par_iter()
            .min_by_key(|k| f_score.get(k).unwrap_or(&usize::MAX))
            .unwrap()
            .clone();
        if current.0 == end {
            return Some(reconstruct_path(&came_from, &current));
        }
        open_set.remove(&current);
        let ngbhs = match blizzards_map.get(&current.1) {
            Some(bmap) => map.possible_moves(&current.0, bmap),
            None => {
                let mut bas_map = map.clone();
                bas_map.blizzards = blizzards_map.get(&(current.1 - 1)).unwrap().clone();
                let next_blizzards = bas_map.next_blizzards();
                let ret = map.possible_moves(&current.0, &next_blizzards);
                blizzards_map.insert(current.1, next_blizzards);
                ret
            }
        };
        for ngbh in ngbhs {
            let ngbh_full = (ngbh, current.1 + 1);
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            if tentative_g_score < *g_score.get(&ngbh_full).unwrap_or(&usize::MAX) {
                came_from.insert(ngbh_full.clone(), current.clone());
                g_score.insert(ngbh_full.clone(), tentative_g_score);
                f_score.insert(
                    ngbh_full.clone(),
                    tentative_g_score + mh_dist(&ngbh, &map.exit_pos()),
                );
                open_set.insert(ngbh_full);
            }
        }
    }
    None
}

fn reconstruct_path(
    came_from: &HashMap<((usize, usize), usize), ((usize, usize), usize)>,
    current: &((usize, usize), usize),
) -> Vec<((usize, usize), usize)> {
    let mut ret = vec![current.clone()];
    let mut cursor = current.clone();
    while came_from.contains_key(&cursor) {
        cursor = came_from[&cursor];
        ret.push(cursor);
    }
    ret.reverse();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_mock_input() -> Vec<String> {
        vec![
            "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
        ]
        .into_iter()
        .map(|l| String::from(l))
        .collect()
    }

    #[test]
    fn test_pt12() {
        let mut map = BasinMap::parse_input(get_mock_input().into_iter());
        let result = calculate_path(&map, false);
        let len = result.unwrap().len() - 1;
        assert_eq!(len, 18);
        for _ in 0..len {
            map.blizzards = map.next_blizzards()
        }
        let result = calculate_path(&map, true);
        let len2 = result.unwrap().len() - 1;
        assert_eq!(len2, 23);
        for _ in 0..len2 {
            map.blizzards = map.next_blizzards()
        }
        let result = calculate_path(&map, false);
        let len3 = result.unwrap().len() - 1;
        assert_eq!(len3, 13);
    }
}
