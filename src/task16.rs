use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::PathBuf,
};

use clap::{value_parser, ArgMatches, Command};
use regex::Regex;

use crate::common;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    flow_rate: usize,
    targets: Vec<String>,
}

pub fn cli() -> Command {
    Command::new("day16").about("Elephant rescue").arg(
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
        let og_graph = lines_as_hmap(lines.map(|l| l.unwrap()));

        let (simplified_nodes, simplified_verticies) = simplify_graph(&og_graph);
        let (_, score) = calculate_path(
            &simplified_nodes,
            &simplified_verticies,
            vec![String::from("AA")],
            0,
            0,
            30,
        );
        println!("BEST SCORE {}", score);

        let ((_, score1), (_, score2)) =
            calculate_path_with_helper(&simplified_nodes, &simplified_verticies, 26);
        println!("BEST SCORE WITH HELPER {}", score1 + score2);
    }
}

fn calculate_path_with_helper(
    snodes: &HashMap<String, Node>,
    svertices: &HashMap<(String, String), usize>,
    time_left: usize,
) -> ((Vec<String>, usize), (Vec<String>, usize)) {
    let key_set: HashSet<String> = svertices.keys().map(|(_, tgt)| tgt.clone()).collect();
    let mut best_score: usize = 0;
    let mut result = None;
    let mut i = 0;
    let pow_set = powerset(&key_set);
    let pow_set_len = pow_set.len();


    for prot_pset in pow_set {
        i += 1;
        if i % 100 == 0 {
            println!("{}/{}", i, pow_set_len);
        }

        let pset: HashSet<String> = HashSet::from_iter(prot_pset.into_iter());
        let (sv1, sv2) = divide_svertices(&svertices, &pset);
        let (vec1, score1) =
            calculate_path(&snodes, &sv1, vec![String::from("AA")], 0, 0, time_left);
        let (vec2, score2) =
            calculate_path(&snodes, &sv2, vec![String::from("AA")], 0, 0, time_left);
        if score1 + score2 > best_score {
            result = Some(((vec1, score1), (vec2, score2)));
            best_score = score1 + score2;
        }
    }
    result.unwrap()
}

fn divide_svertices(
    svertices: &HashMap<(String, String), usize>,
    subset: &HashSet<String>,
) -> (
    HashMap<(String, String), usize>,
    HashMap<(String, String), usize>,
) {
    let mut r1 = HashMap::new();
    let mut r2 = HashMap::new();

    for (s_key, s_value) in svertices {
        if subset.contains(&s_key.1) {
            r1.insert(s_key.clone(), *s_value);
        } else {
            r2.insert(s_key.clone(), *s_value);
        }
    }
    (r1, r2)
}

fn powerset<T>(s: &HashSet<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    //stolen from https://stackoverflow.com/posts/40719103/revisions
    let upper_limit = 2usize.pow(s.len() as u32) / 2;
    (0..upper_limit)
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        })
        .collect()
}

fn calculate_path(
    snodes: &HashMap<String, Node>,
    svertices: &HashMap<(String, String), usize>,
    visited: Vec<String>,
    current_flow: usize,
    current_score: usize,
    time_left: usize,
) -> (Vec<String>, usize) {
    //assert!(time_left >= 0);
    let location = visited.last().unwrap();
    let do_nothing_score = current_score + current_flow * time_left;
    let mut sub_call_vec: Vec<(Vec<String>, usize)> = vec![];

    let possible_targets = snodes.keys().filter(|name| {
        !visited.contains(name)
            && match svertices.get(&(location.clone(), (*name).clone())) {
                Some(val) if *val < time_left => true,
                _ => false,
            }
    });
    for target in possible_targets {
        let mut new_visited = visited.clone();
        new_visited.push(target.clone());
        let distance = svertices[&(location.clone(), target.clone())];
        let new_current_flow = current_flow + snodes[target].flow_rate;
        let new_current_score = current_score + current_flow * (distance + 1);
        let new_time_left = time_left - distance - 1;
        sub_call_vec.push(calculate_path(
            &snodes,
            &svertices,
            new_visited,
            new_current_flow,
            new_current_score,
            new_time_left,
        ));
    }
    let mut best_route = visited;
    let mut best_score = do_nothing_score;

    for (candidate_route, candidate_score) in sub_call_vec {
        if candidate_score > best_score {
            best_route = candidate_route;
            best_score = candidate_score;
        }
    }

    (best_route, best_score)
}

fn simplify_graph(
    nodes: &HashMap<String, Node>,
) -> (HashMap<String, Node>, HashMap<(String, String), usize>) {
    let source = String::from("AA");
    let names_of_intrest: HashSet<String> = nodes
        .into_iter()
        .filter_map(|(name, node)| match (name, node.flow_rate) {
            (sname, _) if sname.eq(&source) => Some(name.clone()),
            (_, 0) => None,
            (_, _) => Some(name.clone()),
        })
        .collect();

    let mut distance_map: HashMap<(String, String), usize> = HashMap::new();

    for source in &names_of_intrest {
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut local_distances: HashMap<String, usize> = HashMap::new();

        distance_map.insert((source.clone(), source.clone()), 0);
        local_distances.insert(source.clone(), 0);
        queue.push_back(source.clone());
        while let Some(position) = queue.pop_front() {
            let current_distance = local_distances[&position];
            for neighbour in &nodes[&position].targets {
                if !local_distances.contains_key(neighbour) {
                    if names_of_intrest.contains(neighbour) && neighbour != source {
                        distance_map
                            .insert((source.clone(), neighbour.clone()), current_distance + 1);
                    }
                    local_distances.insert(neighbour.clone(), current_distance + 1);
                    queue.push_back(neighbour.clone());
                }
            }
        }
    }
    let mut nodes_filtered = HashMap::new();
    for name in names_of_intrest {
        let node = &nodes[&name];
        nodes_filtered.insert(name, node.clone());
    }
    (nodes_filtered, distance_map)
}

fn lines_as_hmap(lines: impl Iterator<Item = String>) -> HashMap<String, Node> {
    let mut ret = HashMap::new();
    for line in lines {
        let node = line_to_node(&line);
        ret.insert(node.name.clone(), node);
    }
    ret
}

fn line_to_node(line: &str) -> Node {
    let nreg: Regex = Regex::new(r"[A-Z][A-Z]").unwrap();
    let freg: Regex = Regex::new(r"\d+").unwrap();
    let mut names = nreg.find_iter(line);
    let flow_rate: usize = freg.find(line).unwrap().as_str().parse().unwrap();
    let name: String = String::from(names.next().unwrap().as_str());
    let targets: Vec<String> = names.map(|m| String::from(m.as_str())).collect();

    Node {
        name,
        flow_rate,
        targets,
    }
}

#[cfg(test)]
mod tests {
    use crate::task16::{calculate_path_with_helper, lines_as_hmap, simplify_graph};

    #[test]
    fn test_pt2() {
        let input = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ];
        let og_graph = lines_as_hmap(input.iter().map(|l| String::from(*l)));

        let (simplified_nodes, simplified_verticies) = simplify_graph(&og_graph);

        let ((_, score1), (_, score2)) =
            calculate_path_with_helper(&simplified_nodes, &simplified_verticies, 26);
        assert_eq!(score1 + score2, 1707);
    }
}
