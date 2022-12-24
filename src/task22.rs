use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};

use crate::{common, task22utils::*};


fn parse_input(lines: impl Iterator<Item = String>) -> (Map, Vec<Move>) {
    let mut lines_vec: Vec<String> = lines.collect();
    let moves = gen_moves(&lines_vec.pop().unwrap());

    (Map::new(&lines_vec), moves)
}

fn parse_input_cubic(lines: impl Iterator<Item = String>) -> (MapCubic, Vec<Move>) {
    let mut lines_vec: Vec<String> = lines.collect();
    let moves = gen_moves(&lines_vec.pop().unwrap());

    (MapCubic::new(&lines_vec), moves)
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
    if let Ok(lines) = common::read_lines(&filepath) {
        let (map, moves) = parse_input(lines.map(|l| l.unwrap()));
        let mut position = map.start_pos();
        for mv in moves{
            position = new_position(&position, &mv, &map);
        }
        println!("final score (1): {}", position.score());
    }
    if let Ok(lines) = common::read_lines(&filepath) {
        let (map, moves) = parse_input_cubic(lines.map(|l| l.unwrap()));
        let mut position = map.start_pos();
        for mv in moves{
            position = new_position(&position, &mv, &map);
        }
        println!("final score (2): {}", position.score());
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
        let (map, moves) = parse_input(lines.into_iter());
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
        let (map, _) = parse_input(lines.into_iter());
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
        let (map, moves) = parse_input(lines.into_iter());
        let mut position = map.start_pos();
        for mv in moves{
            position = new_position(&position, &mv, &map);
        }
        assert_eq!(position.score(), 6032);
    }

    #[test]
    fn test_warp_w(){
        let lines = get_pt1_mock();
        let (map, _) = parse_input_cubic(lines.into_iter());
        let pos_src = Position{
            heading: Heading::E,
            x: 11,
            y: 5
        };
        let pos_tgt = Position{
            heading: Heading::S,
            x: 14,
            y: 8
        };
        assert_eq!(map.forward_pos(&pos_src), pos_tgt)
    }

    #[test]
    fn test_warp_n(){
        let lines = get_pt1_mock();
        let (map, _) = parse_input_cubic(lines.into_iter());
        let pos_src = Position{
            heading: Heading::S,
            x: 10,
            y: 3
        };
        let pos_tgt = Position{
            heading: Heading::S,
            x: 10,
            y: 4
        };
        assert_eq!(map.forward_pos(&pos_src), pos_tgt)
    }

    #[test]
    fn test_pt2() {
        let lines = get_pt1_mock();
        let (map, moves) = parse_input_cubic(lines.into_iter());
        let mut position = map.start_pos();
        for mv in moves{
            position = new_position(&position, &mv, &map);
        }
        assert_eq!(position.score(), 5031);
    }

}
