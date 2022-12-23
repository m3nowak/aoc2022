use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
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
    pub fn as_num(&self) -> isize {
        match self {
            Self::E => 0,
            Self::S => 1,
            Self::W => 2,
            Self::N => 3,
        }
    }
    pub fn from_num(num: isize) -> Self {
        let mut acc = num % 4;
        if acc < 0 {
            acc += 4;
        }
        match acc {
            0 => Self::E,
            1 => Self::S,
            2 => Self::W,
            3 => Self::N,
            _ => unreachable!(),
        }
    }

    pub fn neg(&self) -> Self {
        match self {
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
            Self::N => Self::S,
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
        (self.y + 1) * 1000 + (self.x + 1) * 4 + self.heading.as_num()
    }
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

fn coor_rot(coor: &(isize, isize), rot_anchor: &(isize, isize), clockwise: bool) -> (isize, isize) {
    let rel = (coor.0 - rot_anchor.0, coor.1 - rot_anchor.1);
    if clockwise {
        (rot_anchor.0 - rel.1, rot_anchor.1 + rel.0)
    } else {
        (rot_anchor.0 + rel.1, rot_anchor.1 - rel.0)
    }
}

fn tuple_moved(tup: &(isize, isize), headings: Vec<Heading>) -> (isize, isize) {
    let mut ret = tup.clone();
    for heading in headings {
        ret = match heading {
            Heading::N => (ret.0, ret.1 - 1),
            Heading::E => (ret.0 + 1, ret.1),
            Heading::S => (ret.0, ret.1 + 1),
            Heading::W => (ret.0 - 1, ret.1),
        }
    }
    ret
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
        let (sidelen, sides) = Self::calc_lattice(&hmap);
        let mut warp: HashMap<(isize, isize, Heading), (isize, isize, isize)> = HashMap::new();

        MapCubic { hmap, warp }
    }

    fn try_rotate1(
        vsides: &mut HashMap<(isize, isize), (isize, isize, Heading)>,
        anchor: &(isize, isize),
        heading: Heading,
        clockwise: bool,
    ) -> Result<(), ()> {
        //let mut vsides_cp = vsides.clone();
        let rem_pos = tuple_moved(&anchor, vec![heading.clone()]);
        //let rem = vsides[&rem_pos];
        let rem_scf_heading = match clockwise {
            true => Heading::from_num(heading.as_num() + 1),
            false => Heading::from_num(heading.as_num() - 1),
        };
        let scf_pos = tuple_moved(&rem_pos, vec![rem_scf_heading.clone()]);
        if !vsides.contains_key(&rem_pos) || !vsides.contains_key(&scf_pos) || vsides.contains_key(&tuple_moved(&anchor, vec![rem_scf_heading])){
            //Validity initial check
            return Err(());
        }
        let mut scf_bound = vec![scf_pos.clone()];
        {
            let mut queue: VecDeque<(isize, isize)> = VecDeque::from([scf_pos.clone()]);
            while let Some(tpl) = queue.pop_front() {
                for i in 0..4 {
                    let can_pos = tuple_moved(&tpl, vec![Heading::from_num(i)]);
                    if vsides.contains_key(&can_pos)
                        && can_pos != rem_pos
                        && scf_bound.contains(&can_pos)
                    {
                        scf_bound.push(can_pos.clone());
                        queue.push_back(can_pos);
                    }
                }
            }
        }
        let scf_bound_moved: HashMap<(isize, isize), (isize, isize, Heading)> = scf_bound
            .iter()
            .map(|tpl| {
                let mut val = vsides[tpl].clone();
                if clockwise {
                    val.2 = Heading::from_num(val.2.as_num() + 1);
                } else {
                    val.2 = Heading::from_num(val.2.as_num() - 1);
                }
                (tuple_moved(&coor_rot(tpl, &scf_pos, clockwise), vec![heading.neg()]) , val)
            })
            .collect();
        
        for moved_element in scf_bound_moved.keys(){
            if vsides.contains_key(moved_element) && scf_bound.contains(moved_element){
                return Err(()); //after fold a side will be already taken
            }
        }
        //Okay, lets do it!
        for moved_element in scf_bound{
            vsides.remove(&moved_element);
        }
        vsides.extend(scf_bound_moved);
        Ok(())
    }

    fn try_rotate2(
        vsides: &mut HashMap<(isize, isize), (isize, isize, Heading)>,
        anchor: &(isize, isize),
        heading: Heading,
        clockwise: bool,
    ) -> Result<(), ()> {
        let rem_scf_heading = match clockwise {
            true => Heading::from_num(heading.as_num() + 1),
            false => Heading::from_num(heading.as_num() - 1),
        };
        if vsides.contains_key(&tuple_moved(&anchor, vec![rem_scf_heading.clone()]))
            || vsides.contains_key(&tuple_moved(&anchor, vec![heading.clone(), rem_scf_heading.clone()]))
            || vsides.contains_key(&tuple_moved(&anchor, vec![heading.clone(), heading.clone(), rem_scf_heading.clone(), rem_scf_heading.clone()]))
            || vsides.contains_key(&tuple_moved(&anchor, vec![heading.clone(), heading.clone(), heading.clone(), rem_scf_heading.clone()]))
            || !vsides.contains_key(&tuple_moved(&anchor, vec![heading.clone()]))
            || !vsides.contains_key(&tuple_moved(&anchor, vec![heading.clone(), heading.clone()]))
            || !vsides.contains_key(&tuple_moved(&anchor, vec![heading.clone(), heading.clone(), rem_scf_heading.clone()])){
                //Chceck prerequisites
                return Err(());
        }
        todo!();
        
        Ok(())
    }

    fn find_fold(
        vsides: &mut HashMap<(isize, isize), (isize, isize, isize)>,
        analyzed: (isize, isize),
        ignored: &Vec<(isize, isize)>,
    ) {
        todo!()
    }

    fn side_normalize(
        analyzed: (isize, isize),
        sides: HashSet<(isize, isize)>,
    ) -> HashMap<Heading, (isize, isize, isize)> {
        let mut vsides: HashMap<(isize, isize), //current position
            (isize, isize, isize)>  //original position + rotation
            = sides.iter().map(|(x,y)| ((*x,*y), (*x, *y, 0))).collect();
        loop {
            if vsides.contains_key(&(analyzed.0 + 1, analyzed.1))
                && vsides.contains_key(&(analyzed.0, analyzed.1 + 1))
                && vsides.contains_key(&(analyzed.0 - 1, analyzed.1))
                && vsides.contains_key(&(analyzed.0, analyzed.1 - 1))
            {
                break;
            } else {
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
        }
        (sidelen, side_pos)
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
