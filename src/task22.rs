use std::{collections::{HashMap, HashSet}, path::PathBuf};

use clap::{value_parser, ArgMatches, Command};
use regex::{Match, Matches, Regex};

use crate::common;

const MOVE_RE: &str = r"[0-9]+|L|R";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Heading {
    N,
    E,
    S,
    W,
}

impl Heading {
    fn rot_clockwise(&self) -> Self {
        match &self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
    fn rot_counterclockwise(&self) -> Self {
        match &self {
            Self::N => Self::W,
            Self::E => Self::N,
            Self::S => Self::E,
            Self::W => Self::S,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
    heading: Heading,
}

impl Position {
    fn new(map: &Map) -> Position {
        Position {
            y: 0,
            x: map
                .hmap
                .iter()
                .filter_map(|((x, y), _)| match *y {
                    0 => Some(*x),
                    _ => None,
                })
                .min()
                .unwrap(),
            heading: Heading::E,
        }
    }

    fn score(&self) -> isize{
        (self.y+1)*1000 + (self.x+1)*4 + match self.heading {
            Heading::N => 3,
            Heading::E => 0,
            Heading::S => 1,
            Heading::W => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WrapRule {
    Rot0,
    Rot90,
    Rot180,
    Rot270
}

struct Lattice {
    side_pos: HashSet<(isize,isize)>
}

impl Lattice {
    fn find_north(&self, side:(isize,isize)) -> ((isize,isize), WrapRule){
        //find paths to all existing sides and store them as vec of heading
        todo!()
    }
}


struct Map {
    hmap: HashMap<(isize, isize), bool>,
    cubic_wrap: bool,
}

impl Map {
    fn new(lines: &Vec<String>, cubic_wrap: bool) -> Map {
        let mut hmap: HashMap<(isize, isize), bool> = HashMap::new();
        for (pos_y, line) in lines.iter().enumerate() {
            for (pos_x, char) in line.chars().enumerate() {
                match char {
                    ' ' => {
                        //do nothing
                    }
                    '.' => {
                        hmap.insert((pos_x as isize, pos_y as isize), true);
                    }
                    '#' => {
                        hmap.insert((pos_x as isize, pos_y as isize), false);
                    }
                    _ => unreachable!(),
                }
            }
        }
        Map { hmap, cubic_wrap }
    }
    fn calculate_lattice(&self, side_len: isize) -> Lattice{
        let mut side_pos: HashSet<(isize,isize)> = HashSet::new();
        for x_gp in 0..6 {
            for y_gp in 0..6{
                if self.hmap.contains_key(&(x_gp* side_len, y_gp*side_len)){
                    side_pos.insert((x_gp, y_gp));
                }
            }
        }
        Lattice{side_pos}
    }
    fn forward_pos(&self, pos: &Position) -> Position{
        let shift = match pos.heading {
            Heading::N => (0,-1),
            Heading::E => (1, 0),
            Heading::S => (0,1),
            Heading::W => (-1, 0),
        };
        let potential_pos = (pos.x + shift.0, pos.y + shift.1);
        let new_pos = match self.hmap.get(&potential_pos) {
            Some(true) => potential_pos,
            Some(false) =>(pos.x, pos.y),
            None => { //wrap around
                let mut acc = (pos.x, pos.y);
                while let Some(_) = self.hmap.get(&(acc.0 - shift.0, acc.1 - shift.1)) {
                    acc = (acc.0 - shift.0, acc.1 - shift.1);
                };
                match self.hmap[&acc] {
                    true => acc,
                    false => (pos.x, pos.y)
                }
            },
        };
        Position { x: new_pos.0, y: new_pos.1, heading: pos.heading.clone() }
    }
}


#[derive(Debug,PartialEq, Eq)]
enum Move {
    RotCC,
    RotCW,
    Forward(usize),
}

fn gen_moves(source: &str) -> Vec<Move> {
    let regex = Regex::new(MOVE_RE).unwrap();
    regex
        .find_iter(source)
        .map(|val| match val.as_str() {
            "L" => Move::RotCC,
            "R" => Move::RotCW,
            mvmnt => Move::Forward(mvmnt.parse().unwrap()),
        })
        .collect()
}

fn new_position(position: &Position, mv: &Move, map: &Map) -> Position {
    match mv {
        Move::RotCC => {
            let mut newpos = position.clone();
            newpos.heading = newpos.heading.rot_counterclockwise();
            newpos
        },
        Move::RotCW => {
            let mut newpos = position.clone();
            newpos.heading = newpos.heading.rot_clockwise();
            newpos
        },
        Move::Forward(fwd) => {
            let mut newpos = position.clone();
            for _ in 0..*fwd{
                newpos = map.forward_pos(&newpos)
            };
            newpos
        },
    }
}

fn parse_input(lines: impl Iterator<Item = String>, cubic_wrap: bool) -> (Map, Vec<Move>) {
    let mut lines_vec: Vec<String> = lines.collect();
    let moves = gen_moves(&lines_vec.pop().unwrap());

    (Map::new(&lines_vec, cubic_wrap), moves)
}

pub fn cli() -> Command {
    Command::new("day22").about("Jungle traversal").arg(
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
        let (map, moves) = parse_input(lines.map(|l| l.unwrap()), false);
        let mut position = Position::new(&map);
        for mv in moves{
            position = new_position(&position, &mv, &map);
        }
        println!("final score (1): {}", position.score());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_pt1_mock() -> Vec<String>{
        vec![
            "        ...#",
            "        .#..",
            "        #...",
            "        ....",
            "...#.......#",
            "........#...",
            "..#....#....",
            "..........#.",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5",
        ].into_iter().map(|l| String::from(l)).collect()
    }

    #[test]
    fn test_parsing() {
        let lines = get_pt1_mock();
        let (map, moves) = parse_input(lines.into_iter(), false);
        assert_eq!(moves, vec![
            Move::Forward(10),
            Move::RotCW,
            Move::Forward(5),
            Move::RotCC,
            Move::Forward(5),
            Move::RotCW,
            Move::Forward(10),
            Move::RotCC,
            Move::Forward(4),
            Move::RotCW,
            Move::Forward(5),
            Move::RotCC,
            Move::Forward(5),
        ]);
        assert_eq!(map.hmap[&(8,0)], true);
        assert_eq!(map.hmap[&(11,0)], false);
        assert!(!map.hmap.contains_key(&(7,0)));
        assert_eq!(map.hmap[&(0,5)], true);
        assert_eq!(map.hmap[&(2,6)], false);
        assert!(!map.hmap.contains_key(&(2,8)));
    }

    #[test]
    fn test_wraping_pt1() {
        let lines = get_pt1_mock();
        let (map, _) = parse_input(lines.into_iter(), false);
        assert_eq!(map.forward_pos(&Position{
            heading: Heading::N,
            x: 5,
            y: 4
        }), Position{
            heading: Heading::N,
            x: 5,
            y: 7
        });
        assert_eq!(map.forward_pos(&Position{
            heading: Heading::E,
            x: 11,
            y: 6
        }), Position{
            heading: Heading::E,
            x: 0,
            y: 6
        });
        let f = Position{
            heading: Heading::E,
            x: 11,
            y: 2
        };
        assert_eq!(map.forward_pos(&f), f);
    }

    #[test]
    fn test_pt1() {
        let lines = get_pt1_mock();
        let (map, moves) = parse_input(lines.into_iter(), false);
        let mut position = Position::new(&map);
        for mv in moves{
            position = new_position(&position, &mv, &map);
        }
        assert_eq!(position.score(), 6032);
    }

}
