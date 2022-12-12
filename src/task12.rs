use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
};

use clap::{value_parser, ArgMatches, Command};

use crate::common;

struct Graph {
    start: (usize, usize),
    end: (usize, usize),
    nodes: HashMap<(usize, usize), i32>,
}

impl Graph {
    fn new() -> Self {
        Self {
            start: (0, 0),
            end: (0, 0),
            nodes: HashMap::new(),
        }
    }
    fn add_edge(&mut self, x: usize, y: usize, val: i32) {
        self.nodes.insert((x, y), val);
    }
    fn set_start(&mut self, x: usize, y: usize) {
        self.start = (x, y)
    }
    fn set_end(&mut self, x: usize, y: usize) {
        self.end = (x, y)
    }
    fn edge_exists(&self, from: (usize,usize), to:(usize,usize)) -> bool{
        if from == to{
            false
        }
        else if !self.nodes.contains_key(&to) || !self.nodes.contains_key(&from){
            false
        }
        else if (from.0 as i32 - to.0 as i32).abs() == 1 && from.1 == to.1{
            self.height_diff(from,to) >= -1
        }
        else if (from.1 as i32 - to.1 as i32).abs() == 1 && from.0 == to.0{
            self.height_diff(from,to) >= -1
        }
        else{
            false
        }
    }
    fn height_diff(&self, from: (usize,usize), to:(usize,usize)) -> i32{
        self.nodes[&to] - self.nodes[&from]
    }

    fn calc_path(&self, to_elev: Option<i32>) -> Option<usize>{
        let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        distance.insert(self.end, 0);
        queue.push_back(self.end);
        while let Some((x,y)) = queue.pop_front(){
            let current_len = distance[&(x,y)];
            //println!("Looking at ({},{}), with distance {}", x,y, current_len);
            match to_elev{
                None => {
                    if (x,y) == self.start{
                        return Some(current_len);
                    }
                }
                Some(val) =>{
                    if self.nodes[&(x,y)] == val{
                        return Some(current_len);
                    }
                }
            }
            
            if self.edge_exists((x,y), (x,y+1)) && !distance.contains_key(&(x,y+1)) {
                distance.insert((x,y+1), current_len+1);
                queue.push_back((x,y+1));
            }
            if self.edge_exists((x,y), (x+1,y)) && !distance.contains_key(&(x+1,y)) {
                distance.insert((x+1,y), current_len+1);
                queue.push_back((x+1,y));
            }
            if y!=0 && self.edge_exists((x,y), (x,y-1)) && !distance.contains_key(&(x,y-1)) {
                distance.insert((x,y-1), current_len+1);
                queue.push_back((x,y-1));
            }
            if x!=0 &&self.edge_exists((x,y), (x-1,y)) && !distance.contains_key(&(x-1,y)) {
                distance.insert((x-1,y), current_len+1);
                queue.push_back((x-1,y));
            }
        }
        None
    }
}

pub fn cli() -> Command {
    Command::new("day12").about("Elvish hill climbing").arg(
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
        let graph = parse_lines(lines.map(|l| l.unwrap()));
        match graph.calc_path(None){
            Some(x) => {
                println!("Shortest path to end has {} length", x);
            }
            None => {
                println!("No path");
            }
        }
        match graph.calc_path(Some('a' as i32)){
            Some(x) => {
                println!("Shortest path to depression has {} length", x);
            }
            None => {
                println!("No path");
            }
        }
    }
}

fn parse_lines(lines: impl Iterator<Item = String>) -> Graph {
    let mut graph = Graph::new();
    for (line_index, line) in lines.enumerate() {
        for (ch_index, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    graph.add_edge(ch_index, line_index, 'a' as i32);
                    graph.set_start(ch_index, line_index);
                }
                'E' => {
                    graph.add_edge(ch_index, line_index, 'z' as i32);
                    graph.set_end(ch_index, line_index);
                }
                _ => {
                    graph.add_edge(ch_index, line_index, ch as i32);
                }
            }
        }
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::parse_lines;


    #[test]
    fn test_example() {
        let lines = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];
        let graph = parse_lines(lines.into_iter());
        assert_eq!(graph.calc_path(None), Some(31));
        assert_eq!(graph.calc_path(Some('a' as i32)), Some(29));
    }
}
