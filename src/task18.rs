use std::{path::PathBuf, collections::{HashSet, VecDeque}, cmp};

use clap::{value_parser,Command, ArgMatches};
use regex::Regex;

use crate::common;

pub fn cli() -> Command {
    Command::new("day18")
        .about("Magma cupe calculatiuons")
        .arg(clap::arg!(path: <PATH>).required(true).value_parser(value_parser!(std::path::PathBuf)))
}

pub fn handle(matches: &ArgMatches) {
    let path = matches.get_one::<std::path::PathBuf>("path");
    solve(path.unwrap().to_path_buf());
}

pub fn solve(filepath: PathBuf) {
    
    if let Ok(lines) = common::read_lines(filepath) {
        let droplets = parse_lines(lines.map(|l|l.unwrap()));
        let exposed_count = exposed_side_count(&droplets);
        let exposed_count_outside = exposed_side_count_outside(&droplets);
        println!("Result(1): {}", exposed_count);
        println!("Result(2): {}", exposed_count_outside);
    }
}


fn exposed_side_count(droplets: &HashSet<(isize,isize,isize)>) -> usize{
    let mut acc: usize = 0;
    for droplet in droplets{
        if !droplets.contains(&(droplet.0+1, droplet.1, droplet.2)){
            acc += 1;
        }
        if !droplets.contains(&(droplet.0-1, droplet.1, droplet.2)){
            acc += 1;
        }
        if !droplets.contains(&(droplet.0, droplet.1+1, droplet.2)){
            acc += 1;
        }
        if !droplets.contains(&(droplet.0, droplet.1-1, droplet.2)){
            acc += 1;
        }
        if !droplets.contains(&(droplet.0, droplet.1, droplet.2+1)){
            acc += 1;
        }
        if !droplets.contains(&(droplet.0, droplet.1, droplet.2-1)){
            acc += 1;
        }
    }
    acc
}

fn exposed_side_count_outside(droplets: &HashSet<(isize,isize,isize)>) -> usize{
    let bbox = expand_box(calculate_boundary_box(droplets));
    let mut acc: usize = 0;

    let spoints = [
        (bbox.0.0,bbox.1.0, bbox.2.0),
        (bbox.0.1,bbox.1.0, bbox.2.0),
        (bbox.0.0,bbox.1.1, bbox.2.0),
        (bbox.0.1,bbox.1.1, bbox.2.0),
        (bbox.0.0,bbox.1.0, bbox.2.1),
        (bbox.0.1,bbox.1.0, bbox.2.1),
        (bbox.0.0,bbox.1.1, bbox.2.1),
        (bbox.0.1,bbox.1.1, bbox.2.1),
    ];

    let mut visited: HashSet<(isize, isize, isize)> = HashSet::from(spoints);
    //let mut bordering_droplets: HashSet<(isize, isize, isize)> = HashSet::new();
    let mut queue: VecDeque<(isize, isize, isize)> = VecDeque::from(spoints);
    
    while let Some(position) = queue.pop_front() {
        let neighbor_candidates = Vec::from([
            (position.0+1, position.1, position.2),
            (position.0-1, position.1, position.2),
            (position.0, position.1+1, position.2),
            (position.0, position.1-1, position.2),
            (position.0, position.1, position.2+1),
            (position.0, position.1, position.2-1),
        ]);
        for candidate in neighbor_candidates{
            let x_in = bbox.0.0 <= candidate.0 && bbox.0.1 >= candidate.0;
            let y_in = bbox.1.0 <= candidate.1 && bbox.1.1 >= candidate.1;
            let z_in=  bbox.2.0 <= candidate.2 && bbox.2.1 >= candidate.2;
            if droplets.contains(&candidate){
                //bordering_droplets.insert(position.clone());
                acc += 1;
            }
            else if !visited.contains(&candidate) 
                && x_in
                && y_in
                && z_in
                {
                visited.insert(candidate.clone());
                queue.push_back(candidate);
            }
        }
    }
    acc
}

fn expand_box(bbox: ((isize,isize),(isize,isize),(isize,isize))) -> ((isize,isize),(isize,isize),(isize,isize)){
    ((bbox.0.0-1, bbox.0.1+1), (bbox.1.0-1, bbox.1.1+1), (bbox.2.0-1, bbox.2.1+1))
}

fn calculate_boundary_box(droplets: &HashSet<(isize,isize,isize)>) -> ((isize,isize),(isize,isize),(isize,isize)){
    let mut droplets_iter = droplets.iter();
    let first = droplets_iter.next().unwrap();
    let mut pos_x_extremes = (first.0, first.0);
    let mut pos_y_extremes = (first.0, first.0);
    let mut pos_z_extremes = (first.0, first.0);
    for droplet in droplets_iter{
        pos_x_extremes = (cmp::min(pos_x_extremes.0, droplet.0), cmp::max(pos_x_extremes.1, droplet.0));
        pos_y_extremes = (cmp::min(pos_y_extremes.0, droplet.1), cmp::max(pos_y_extremes.1, droplet.1));
        pos_z_extremes = (cmp::min(pos_z_extremes.0, droplet.1), cmp::max(pos_z_extremes.1, droplet.2));
    }
    (pos_x_extremes, pos_y_extremes, pos_z_extremes)
}

fn parse_lines(lines: impl Iterator<Item = String>) -> HashSet<(isize,isize,isize)>{
    let mut ret = HashSet::new();
    let dreg: Regex = Regex::new(r"-?\d+").unwrap();
    for line in lines{
        let mut vals = dreg.find_iter(&line);
        ret.insert((
            vals.next().unwrap().as_str().parse().unwrap(),
            vals.next().unwrap().as_str().parse().unwrap(),
            vals.next().unwrap().as_str().parse().unwrap(),
        ));
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        let srclines = vec![
            "2,2,2",
            "1,2,2",
            "3,2,2",
            "2,1,2",
            "2,3,2",
            "2,2,1",
            "2,2,3",
            "2,2,4",
            "2,2,6",
            "1,2,5",
            "3,2,5",
            "2,1,5",
            "2,3,5",
        ];
        let droplets = parse_lines(srclines.into_iter().map(|l| String::from(l)));
        assert_eq!(exposed_side_count(&droplets), 64);
    }

    #[test]
    fn test_pt2() {
        let srclines = vec![
            "2,2,2",
            "1,2,2",
            "3,2,2",
            "2,1,2",
            "2,3,2",
            "2,2,1",
            "2,2,3",
            "2,2,4",
            "2,2,6",
            "1,2,5",
            "3,2,5",
            "2,1,5",
            "2,3,5",
        ];
        let droplets = parse_lines(srclines.into_iter().map(|l| String::from(l)));
        assert_eq!(exposed_side_count_outside(&droplets), 58);
    }
}
