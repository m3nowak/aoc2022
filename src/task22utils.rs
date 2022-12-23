use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use regex::Regex;

const MOVE_RE: &str = r"[0-9]+|L|R";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Heading {
    N,
    E,
    S,
    W,
}

impl Heading {
    pub fn rot_clockwise(&self) -> Self {
        match &self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
    pub fn rot_counterclockwise(&self) -> Self {
        match &self {
            Self::N => Self::W,
            Self::E => Self::N,
            Self::S => Self::E,
            Self::W => Self::S,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    RotCC,
    RotCW,
    Forward(usize),
}

pub fn gen_moves(source: &str) -> Vec<Move> {
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

pub fn new_position(position: &Position, mv: &Move, map: &Map) -> Position {
    match mv {
        Move::RotCC => {
            let mut newpos = position.clone();
            newpos.heading = newpos.heading.rot_counterclockwise();
            newpos
        }
        Move::RotCW => {
            let mut newpos = position.clone();
            newpos.heading = newpos.heading.rot_clockwise();
            newpos
        }
        Move::Forward(fwd) => {
            let mut newpos = position.clone();
            for _ in 0..*fwd {
                newpos = map.forward_pos(&newpos)
            }
            newpos
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
    pub heading: Heading,
}

impl Position {
    pub fn new(map: &Map) -> Position {
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

    pub fn score(&self) -> isize {
        (self.y + 1) * 1000
            + (self.x + 1) * 4
            + match self.heading {
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
    Rot270,
}

fn hmap_parse(lines: &Vec<String>) -> HashMap<(isize, isize), bool> {
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
    hmap
}

pub struct Map {
    pub hmap: HashMap<(isize, isize), bool>,
}

impl Map {
    pub fn new(lines: &Vec<String>) -> Map {
        Map {
            hmap: hmap_parse(lines),
        }
    }
    pub fn forward_pos(&self, pos: &Position) -> Position {
        let shift = match pos.heading {
            Heading::N => (0, -1),
            Heading::E => (1, 0),
            Heading::S => (0, 1),
            Heading::W => (-1, 0),
        };
        let potential_pos = (pos.x + shift.0, pos.y + shift.1);
        let new_pos = match self.hmap.get(&potential_pos) {
            Some(true) => potential_pos,
            Some(false) => (pos.x, pos.y),
            None => {
                //wrap around
                let mut acc = (pos.x, pos.y);
                while let Some(_) = self.hmap.get(&(acc.0 - shift.0, acc.1 - shift.1)) {
                    acc = (acc.0 - shift.0, acc.1 - shift.1);
                }
                match self.hmap[&acc] {
                    true => acc,
                    false => (pos.x, pos.y),
                }
            }
        };
        Position {
            x: new_pos.0,
            y: new_pos.1,
            heading: pos.heading.clone(),
        }
    }
}

fn gcd(first: isize, second: isize) -> isize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn alter_tuple(tup: &(isize, isize), heading: Heading)-> (isize,isize) {
    match heading{
        Heading::N => (tup.0, tup.1-1),
        Heading::E => (tup.0+1, tup.1),
        Heading::S => (tup.0, tup.1+1),
        Heading::W => (tup.0-1, tup.1),
    }
}

pub struct MapCubic {
    hmap: HashMap<(isize, isize), bool>,
    warp: HashMap<
        (isize, isize, Heading), //source side x, source side y, exit heading
        (isize, isize, isize),
    >, // tgt side x, tgt side y, rotation (/90)
}

impl MapCubic {
    pub fn new(lines: &Vec<String>) -> MapCubic {
        let hmap = hmap_parse(lines);
        let (sidelen,sides) = Self::calc_lattice(&hmap);
        let mut warp: HashMap<(isize, isize, Heading), (isize, isize, isize)> = HashMap::new();

        MapCubic { hmap, warp }
    }

    fn try_fold(vsides: &mut HashMap<(isize, isize),(isize, isize, isize)>, analyzed: (isize, isize), ignored: &Vec<(isize, isize)>){
        let n_niegh = alter_tuple(&analyzed, Heading::N);
        let e_niegh = alter_tuple(&analyzed, Heading::E);
        let s_niegh = alter_tuple(&analyzed, Heading::S);
        let w_niegh = alter_tuple(&analyzed, Heading::W);
        if !ignored.contains(&n_niegh) && vsides.contains_key(&n_niegh){ //looking north
            if !vsides.contains_key(&e_niegh) && vsides.contains_key(&alter_tuple(&n_niegh, Heading::E)){
                // check if NE side can be rotated to E
            }
            if !vsides.contains_key(&w_niegh) && vsides.contains_key(&alter_tuple(&n_niegh, Heading::W)){
                // check if NW side can be rotated to W
            }
        }
    }

    fn side_normalize(analyzed: (isize, isize),sides: HashSet<(isize, isize)>) -> HashMap<Heading, (isize, isize, isize)> {
        let mut vsides: HashMap<(isize, isize), //current position
            (isize, isize, isize)>  //original position + rotation
            = sides.iter().map(|(x,y)| ((*x,*y), (*x, *y, 0))).collect();
        loop {
            if vsides.contains_key(&(analyzed.0+1, analyzed.1))
                && vsides.contains_key(&(analyzed.0, analyzed.1+1))
                && vsides.contains_key(&(analyzed.0-1, analyzed.1))
                && vsides.contains_key(&(analyzed.0, analyzed.1-1)){
                    break;
                }
            else {

            }
        }
 
        todo!();
    }

    fn calc_lattice(hmap: &HashMap<(isize, isize), bool>) -> (isize, HashSet<(isize, isize)>) {
        let (maxx, maxy) = hmap
            .into_iter()
            .fold((0, 0), |(maxx, maxy), ((canx, cany), _)| {
                (cmp::max(maxx, *canx + 1), cmp::max(maxy, *cany + 1))
            });
        let sidelen = gcd(maxx, maxy);
        let mut side_pos: HashSet<(isize, isize)> = HashSet::new();
        for x_gp in 0..6 {
            for y_gp in 0..6 {
                if hmap.contains_key(&(x_gp * sidelen, y_gp * sidelen)) {
                    side_pos.insert((x_gp, y_gp));
                }
            }
        };
        (sidelen,side_pos)
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(200, 150), 50);
        assert_eq!(gcd(250, 100), 50);
    }
}
