use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};

use crate::common;

struct Tree {
    height: u8,
    is_visible: bool,
}

impl Tree {
    pub fn new(height: u8) -> Self {
        Self {
            height,
            is_visible: false,
        }
    }
    pub fn set_as_visible(&mut self) {
        self.is_visible = true;
    }
}

pub fn cli() -> Command {
    Command::new("day08").about("Elvish tree survey").arg(
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
    let mut rows: Vec<Vec<u8>> = Vec::new();

    if let Ok(lines) = common::read_lines(filepath) {
        for line in lines {
            if let Ok(line_text) = line {
                rows.push(line_to_u8_vec(&line_text));
            }
        }
    }
    let mut tree_map = height_map_to_tree_map(&rows);
    update_tree_map_visibility(&mut tree_map);
    println!(
        "Result - visible trees: {}",
        count_tree_map_visibility(&tree_map)
    );
    println!(
        "Result - scenic score: {}",
        get_best_scenic_score(&tree_map)
    );
}
fn get_best_scenic_score(tree_map: &Vec<Vec<Tree>>) -> usize {
    let x_size = tree_map.len();
    let y_size = tree_map[0].len();
    let mut best_score: usize = 1;
    for x in 0..x_size {
        for y in 0..y_size {
            let score = calculate_scenic_score_on_tree_map(tree_map, x, y);
            if score > best_score {
                best_score = score;
            }
        }
    }
    best_score
}

fn calculate_scenic_score_on_tree_map(tree_map: &Vec<Vec<Tree>>, x: usize, y: usize) -> usize {
    let height = tree_map[x][y].height;
    let x_size = tree_map.len();
    let y_size = tree_map[0].len();
    let mut score: usize = 1;

    let mut direction_acc: usize = 0;
    let mut cursor = x;
    loop {
        if cursor == x_size - 1 {
            score *= direction_acc;
            break;
        }
        cursor += 1;
        direction_acc += 1;
        if tree_map[cursor][y].height >= height {
            score *= direction_acc;
            break;
        }
    }
    cursor = x;
    direction_acc = 0;
    loop {
        if cursor == 0 {
            score *= direction_acc;
            break;
        }
        cursor -= 1;
        direction_acc += 1;
        if tree_map[cursor][y].height >= height {
            score *= direction_acc;
            break;
        }
    }
    cursor = y;
    direction_acc = 0;
    loop {
        if cursor == y_size - 1 {
            score *= direction_acc;
            break;
        }
        cursor += 1;
        direction_acc += 1;
        if tree_map[x][cursor].height >= height {
            score *= direction_acc;
            break;
        }
    }
    cursor = y;
    direction_acc = 0;
    loop {
        if cursor == 0 {
            score *= direction_acc;
            break;
        }
        cursor -= 1;
        direction_acc += 1;
        if tree_map[x][cursor].height >= height {
            score *= direction_acc;
            break;
        }
    }
    score
}

fn count_tree_map_visibility(tree_map: &Vec<Vec<Tree>>) -> u64 {
    let mut count = 0;
    let x_size = tree_map.len();
    let y_size = tree_map[0].len();
    for x in 0..x_size {
        for y in 0..y_size {
            if tree_map[x][y].is_visible {
                count += 1;
            }
        }
    }
    count
}

fn update_tree_map_visibility(tree_map: &mut Vec<Vec<Tree>>) {
    let x_size = tree_map.len();
    let y_size = tree_map[0].len();

    let mut highest_so_far;
    for x in 0..x_size {
        highest_so_far = tree_map[x][0].height;
        tree_map[x][0].set_as_visible();
        for y in 1..y_size {
            if highest_so_far < tree_map[x][y].height {
                tree_map[x][y].set_as_visible();
                highest_so_far = tree_map[x][y].height;
            }
        }
        highest_so_far = tree_map[x][y_size - 1].height;
        tree_map[x][y_size - 1].set_as_visible();
        for y in 2..=y_size {
            if highest_so_far < tree_map[x][y_size - y].height {
                tree_map[x][y_size - y].set_as_visible();
                highest_so_far = tree_map[x][y_size - y].height;
            }
        }
    }
    for y in 0..y_size {
        highest_so_far = tree_map[0][y].height;
        tree_map[0][y].set_as_visible();
        for x in 1..x_size {
            if highest_so_far < tree_map[x][y].height {
                tree_map[x][y].set_as_visible();
                highest_so_far = tree_map[x][y].height;
            }
        }
        highest_so_far = tree_map[x_size - 1][y].height;
        tree_map[x_size - 1][y].set_as_visible();
        for x in 2..=x_size {
            if highest_so_far < tree_map[x_size - x][y].height {
                tree_map[x_size - x][y].set_as_visible();
                highest_so_far = tree_map[x_size - x][y].height;
            }
        }
    }
}

fn height_map_to_tree_map(map: &Vec<Vec<u8>>) -> Vec<Vec<Tree>> {
    Vec::from_iter(
        map.iter()
            .map(|row| Vec::from_iter(row.iter().map(|height| Tree::new(*height)))),
    )
}

fn line_to_u8_vec(line: &str) -> Vec<u8> {
    Vec::from_iter(
        line.chars()
            .into_iter()
            .map(|n| String::from(n).parse().unwrap()),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        calculate_scenic_score_on_tree_map, count_tree_map_visibility, height_map_to_tree_map,
        update_tree_map_visibility,
    };
    #[test]
    fn test_validation() {
        let test_map: Vec<Vec<u8>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let mut tree_map = height_map_to_tree_map(&test_map);
        update_tree_map_visibility(&mut tree_map);
        assert_eq!(count_tree_map_visibility(&tree_map), 21);
    }
    #[test]
    fn test_calculate_position_visibility_on_tree_map() {
        let test_map: Vec<Vec<u8>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let tree_map = height_map_to_tree_map(&test_map);
        assert_eq!(calculate_scenic_score_on_tree_map(&tree_map, 3, 2), 8);
        assert_eq!(calculate_scenic_score_on_tree_map(&tree_map, 1, 2), 4);
    }
}
