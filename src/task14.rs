use std::{collections::{HashMap, HashSet}, path::PathBuf, cmp};

use clap::{value_parser, ArgMatches, Command};

use crate::common;


// enum CaveStructure{
//     Sand,
//     Rock
// }

pub fn cli() -> Command {
    Command::new("day14").about("Elvish cave collapse").arg(
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
        let mut counter = 0;
        let (mut cave_map, depth) = parse_lines(lines.map(|l| l.unwrap()));
        while let Some(new_sand) = calculate_sand_physics((500,0), &cave_map, depth, false) {
            cave_map.insert(new_sand);
            counter += 1;
        }
        println!("{} units of sand before it noclips out of reality", counter);
        loop {
            let new_sand = calculate_sand_physics((500,0), &cave_map, depth, true).unwrap();
            cave_map.insert(new_sand);
            counter += 1;
            if new_sand == (500,0){
                break;
            }
        }
        println!("{} units of sand before it fills sand source hole", counter);
    }
}

fn calculate_sand_physics(sand_pos: (usize, usize), cave_map: &HashSet<(usize, usize)>, max_depth: usize, magic_floor: bool) -> Option<(usize, usize)>{
    if sand_pos.1 == max_depth && !magic_floor{
        return None;
    }
    else if sand_pos.1 == max_depth+1 && magic_floor{
        return Some(sand_pos);
    }
    else if !cave_map.contains(&(sand_pos.0, sand_pos.1+1)){
        return calculate_sand_physics((sand_pos.0, sand_pos.1+1), cave_map, max_depth, magic_floor);
    }
    else if !cave_map.contains(&(sand_pos.0-1, sand_pos.1+1)){
        return calculate_sand_physics((sand_pos.0-1, sand_pos.1+1), cave_map, max_depth, magic_floor);
    }
    else if !cave_map.contains(&(sand_pos.0+1, sand_pos.1+1)){
        return calculate_sand_physics((sand_pos.0+1, sand_pos.1+1), cave_map, max_depth, magic_floor);
    }
    else{
        return Some(sand_pos);
    }
}

fn parse_lines(lines: impl Iterator<Item = String>) -> (HashSet<(usize, usize)>, usize) {
    let mut hset: HashSet<(usize, usize)> = HashSet::new();
    let mut depth: usize = 0;
    for line in lines {
        if !line.is_empty() {
            for rock in parse_line(&line){
                if depth < rock.1{
                    depth = rock.1;
                }
                hset.insert(rock);
            }
        }
    }
    (hset, depth)
}

fn tpl_parse(utpl: &str) -> (usize,usize){
    let mut spl = utpl.split(',');
    (spl.next().unwrap().parse().unwrap(), spl.next().unwrap().parse().unwrap())
}

fn parse_line(line: &str) -> HashSet<(usize, usize)> {
    let mut rocks: HashSet<(usize,usize)> = HashSet::new();
    let mut coors = line.split(" -> ").map(tpl_parse);
    let mut cursor = coors.next().unwrap();
    //rocks.insert(cursor);
    for target in coors{
        if cursor.0 == target.0 {
            for i in cmp::min(cursor.1, target.1)..=cmp::max(cursor.1, target.1){
                rocks.insert((cursor.0, i));
            }
        }
        else if cursor.1 == target.1 {
            for i in cmp::min(cursor.0, target.0)..=cmp::max(cursor.0, target.0){
                rocks.insert((i, cursor.1));
            }
        }
        else {panic!()}
        cursor = target;
        //rocks.insert(cursor);
    }
    rocks
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::parse_line;

    #[test]
    fn test_parse_line() {
        let s1 = parse_line("498,4 -> 498,6 -> 496,6");
        assert_eq!(s1, HashSet::from([(498,4), (498,5), (498,6), (497,6), (496,6)]));
        let mut s2 = Vec::from_iter(parse_line("503,4 -> 502,4 -> 502,9 -> 494,9").into_iter());
        s2.sort();
        let mut c2:Vec<(usize, usize)> = vec![(503,4),(502,4),(502,5),(502,6),(502,7),(502,8),(502,9),(501,9),(500,9),(499,9),(498,9),(497,9),(496,9),(495,9),(494,9)];
        c2.sort();
        assert_eq!(s2, c2);
    }
}
