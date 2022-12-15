use std::{collections::HashSet, path::PathBuf, cmp};

use clap::{value_parser, ArgMatches, Command};

use regex::Regex;

use crate::common;

const LINE_Y: isize = 2000000;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Sensor {
    pos_x: isize,
    pos_y: isize,
    radius: usize,
}

impl Sensor {
    fn from_sensor_beacon(sensor_pos: &(isize, isize), beacon_pos: &(isize, isize)) -> Self {
        Self {
            pos_x: sensor_pos.0,
            pos_y: sensor_pos.1,
            radius: mh_length(sensor_pos, beacon_pos),
        }
    }
}

pub fn cli() -> Command {
    Command::new("day15").about("Elvish cave scanning").arg(
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
        let mut segment_vec: Vec<(isize, isize)> = Vec::new();
        let (sensor_set, beacon_set) = parse_lines(lines.map(|l| l.unwrap()));
        for sensor in sensor_set {
            if let Some(segment) = scan_y_line_to_segment(&sensor, LINE_Y) {
                add_no_overlap(&mut segment_vec, segment);
            }
        }
        let bad_beacon_spots: Vec<isize> = beacon_set.iter().filter(|(_,y)| *y == LINE_Y).map(|(x,_)| *x).collect();
        let count = count_segments(&segment_vec) - count_bad_spots(&segment_vec, &bad_beacon_spots);
        println!("Found {} locations.", count);
    }
}

fn mh_length(a: &(isize, isize), b: &(isize, isize)) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

fn add_no_overlap(segments: &mut Vec<(isize, isize)>, new_segment: (isize, isize)) {
    let to_add = cut_segment_without_overlap(new_segment, &segments, 0);
    segments.extend(to_add.into_iter());
}

fn count_segments(segments: &Vec<(isize, isize)>) -> usize{
    let mut ret = 0;
    for segment in segments{
        ret += segment.1 - segment.0 + 1;
    }
    ret as usize
}

fn count_segments_limited(segments: &Vec<(isize, isize)>, limit_lo: isize, limit_hi: isize) -> usize{
    let mut ret = 0;
    for segment in segments{
        if segment.1 >= limit_lo && segment.0 <= limit_hi{

        }
        ret += cmp::min(segment.1, limit_hi) - cmp::max(segment.0, limit_lo) + 1;
    }
    ret as usize
}

fn count_bad_spots(segments: &Vec<(isize, isize)>, bad_spots: &Vec<isize>) -> usize{
    let mut ret = 0;
    for segment in segments{
        for bad_spot in bad_spots{
            if segment.0 <= *bad_spot && *bad_spot <= segment.1{
                ret += 1;
            }
        }
    }
    ret
    
}

fn cut_segment_without_overlap(
    new_segment: (isize, isize),
    existing: &Vec<(isize, isize)>,
    starting_index: usize,
) -> Vec<(isize, isize)> {
    if new_segment.0 > new_segment.1 {
        panic!("sanity_check")
    }
    if starting_index >= existing.len() {
        return vec![new_segment];
    }

    let tested_segment = existing[starting_index];

    if tested_segment.0 <= new_segment.0 && new_segment.1 <= tested_segment.1 {
        //tested segment contains new
        return vec![];
    } else if new_segment.0 < tested_segment.0 && tested_segment.1 < new_segment.1 {
        //new segment contains tested
        let mut retvec = Vec::new();
        retvec.extend(
            cut_segment_without_overlap(
                (new_segment.0, tested_segment.0 - 1),
                existing,
                starting_index + 1,
            )
            .into_iter(),
        );
        retvec.extend(
            cut_segment_without_overlap(
                (tested_segment.1 + 1, new_segment.1),
                existing,
                starting_index + 1,
            )
            .into_iter(),
        );
        return retvec;
    } else if new_segment.1 < tested_segment.0 || tested_segment.1 < new_segment.0 {
        //new segment is completly outside of tested
        return cut_segment_without_overlap(new_segment, existing, starting_index + 1);
    } else if tested_segment.0 <= new_segment.1 && new_segment.1 < tested_segment.1 {
        //left side overlap
        return cut_segment_without_overlap(
            (new_segment.0, tested_segment.0 - 1),
            existing,
            starting_index + 1,
        );
    } else if new_segment.0 <= tested_segment.1 && tested_segment.1 < new_segment.1 {
        //right side overlap
        return cut_segment_without_overlap(
            (tested_segment.1 + 1, new_segment.1),
            existing,
            starting_index + 1,
        );
    }
    else{ panic!()};
}

fn scan_y_line_to_segment(sensor: &Sensor, y: isize) -> Option<(isize, isize)> {
    let sensor_line_distance = (sensor.pos_y - y).abs();
    if sensor_line_distance > sensor.radius as isize {
        return None;
    } else {
        let movement_points = sensor.radius - sensor_line_distance as usize;
        return Some((
            sensor.pos_x - movement_points as isize,
            sensor.pos_x + movement_points as isize,
        ));
    }
}

fn scan_x_line_to_segment(sensor: &Sensor, x: isize) -> Option<(isize, isize)> {
    let sensor_line_distance = (sensor.pos_x - x).abs();
    if sensor_line_distance > sensor.radius as isize {
        return None;
    } else {
        let movement_points = sensor.radius - sensor_line_distance as usize;
        return Some((
            sensor.pos_y - movement_points as isize,
            sensor.pos_y + movement_points as isize,
        ));
    }
}

fn parse_lines(lines: impl Iterator<Item = String>) -> (HashSet<Sensor>, HashSet<(isize, isize)>) {
    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();
    for line in lines {
        let (sensor_pos, beacon_pos) = parse_line(&line);
        sensors.insert(Sensor::from_sensor_beacon(&sensor_pos, &beacon_pos));
        beacons.insert(beacon_pos);
    }
    (sensors, beacons)
}

fn parse_line(line: &str) -> ((isize, isize), (isize, isize)) {
    let sreg: Regex = Regex::new(r"-?\d+").unwrap();
    let mut caps = sreg.find_iter(line);
    (
        (
            caps.next().unwrap().as_str().parse().unwrap(),
            caps.next().unwrap().as_str().parse().unwrap(),
        ),
        (
            caps.next().unwrap().as_str().parse().unwrap(),
            caps.next().unwrap().as_str().parse().unwrap(),
        ),
    )
}

fn grid_scan(sensor_set: &HashSet<Sensor>, x_y: isize) -> (Vec<(isize, isize)>, Vec<(isize, isize)>){
    let mut segment_vec_y = Vec::new();
    let mut segment_vec_x = Vec::new();
    for sensor in sensor_set {
        if let Some(segment) = scan_y_line_to_segment(&sensor, x_y) {
            add_no_overlap(&mut segment_vec_y, segment);
        }
        if let Some(segment) = scan_x_line_to_segment(&sensor, x_y) {
            add_no_overlap(&mut segment_vec_x, segment);
        }
    }
    (segment_vec_x, segment_vec_y)
}

#[cfg(test)]
mod tests {

    use crate::task15::{scan_y_line_to_segment, scan_x_line_to_segment, add_no_overlap, count_segments, count_bad_spots, grid_scan, count_segments_limited};

    use super::{cut_segment_without_overlap, parse_line, parse_lines, Sensor};

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            ((2, 18), (-2, 15))
        );
        assert_eq!(
            parse_line("Sensor at x=14, y=17: closest beacon is at x=10, y=16"),
            ((14, 17), (10, 16))
        );
    }

    #[test]
    fn test_coors_to_sensor() {
        assert_eq!(
            Sensor {
                pos_x: 0,
                pos_y: 0,
                radius: 10
            },
            Sensor::from_sensor_beacon(&(0, 0), &(5, 5))
        );
        assert_eq!(
            Sensor {
                pos_x: 10,
                pos_y: 10,
                radius: 20
            },
            Sensor::from_sensor_beacon(&(10, 10), &(10, 30))
        );
    }

    #[test]
    fn test_scan_line_to_segment() {
        let s = Sensor {
            pos_x: 0,
            pos_y: 0,
            radius: 10,
        };
        assert_eq!(scan_y_line_to_segment(&s, 0).unwrap(), (-10,10));
        assert_eq!(scan_y_line_to_segment(&s, 10).unwrap(), (0,0));
        assert_eq!(scan_y_line_to_segment(&s, -10).unwrap(), (0,0));
        assert!(scan_y_line_to_segment(&s, 11).is_none());
        assert!(scan_y_line_to_segment(&s, -11).is_none());
    }

    #[test]
    fn test_cut_segment_without_overlap() {
        let segments: Vec<(isize, isize)> = vec![(0, 5), (10, 20), (30, 50)];
        assert_eq!(
            cut_segment_without_overlap((-10, 2), &segments, 0),
            vec![(-10, -1)]
        );
        assert_eq!(
            cut_segment_without_overlap((-10, 8), &segments, 0),
            vec![(-10, -1), (6, 8)]
        );
        assert_eq!(cut_segment_without_overlap((12, 15), &segments, 0), vec![]);
        assert_eq!(cut_segment_without_overlap((-100, 100), &segments, 0), vec![(-100,-1), (6,9), (21,29), (51,100)]);
    }

    #[test]
    fn test_example() {
        let input = vec![
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        ];
        let (sensor_set, beacon_set) = parse_lines(input.into_iter().map(|l| String::from(l)));
        let mut segment_vec: Vec<(isize, isize)> = Vec::new();
        for sensor in &sensor_set {
            if let Some(segment) = scan_y_line_to_segment(&sensor, 10) {
                add_no_overlap(&mut segment_vec, segment);
            }
        }
        let bad_beacon_spots: Vec<isize> = beacon_set.iter().filter(|(_,y)| *y == 10).map(|(x,_)| *x).collect();
        assert_eq!(count_segments(&segment_vec), 27);
        assert_eq!(count_bad_spots(&segment_vec, &bad_beacon_spots), 1);

        let mut x_pos:Option<isize> = None;
        let mut y_pos:Option<isize> = None;

        for i in 0..=20 as isize{
            let (v_x, v_y) = grid_scan(&sensor_set, i);
            let x_can = count_segments_limited(&v_x, 0, 20);
            let y_can = count_segments_limited(&v_y, 0, 20);
            if x_can == 20{
                x_pos = Some(i);
                println!("found x: {}", i)
            }
            if y_can == 20{
                y_pos = Some(i);
                println!("found y: {}", i)
            }
        }
        assert_eq!(x_pos.unwrap(), 14);
        assert_eq!(y_pos.unwrap(), 11);

    }
}
