use std::{path::PathBuf, collections::HashSet, cmp};

use clap::{value_parser, ArgMatches, Command};
use regex::Regex;

use crate::common;

const ROUND_COUNT:usize = 24;
const ROUND_COUNT2:usize = 32;

#[derive(Clone, Debug)]
struct Blueprint {
    no: usize,
    ore_bot_cost: usize,
    clay_bot_cost: usize,
    obs_bot_cost: (usize, usize), //ore,clay
    geo_bot_cost: (usize, usize), //ore,obsidian
}

impl Blueprint {
    fn from_line(line: &str) -> Self {
        let dreg: Regex = Regex::new(r"\d+").unwrap();
        let numbers_collected: Vec<usize> = dreg
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        Self {
            no: numbers_collected[0],
            ore_bot_cost: numbers_collected[1],
            clay_bot_cost: numbers_collected[2],
            obs_bot_cost: (numbers_collected[3], numbers_collected[4]), //ore,clay
            geo_bot_cost: (numbers_collected[5], numbers_collected[6]), //ore,obsidian
        }
    }
}

#[derive(Clone, Debug)]
struct OperationState {
    blueprint: Blueprint,
    ore_bot_count: usize,
    clay_bot_count: usize,
    obs_bot_count: usize,
    geo_bot_count: usize,
    ore_stock: usize,
    clay_stock: usize,
    obs_stock: usize,
    geo_stock: usize,
    rounds_left: usize,
}

impl OperationState {
    fn new(blueprint: Blueprint, rounds_left: usize) -> Self {
        Self {
            blueprint,
            ore_bot_count: 1,
            clay_bot_count: 0,
            obs_bot_count: 0,
            geo_bot_count: 0,
            ore_stock: 0,
            clay_stock: 0,
            obs_stock: 0,
            geo_stock: 0,
            rounds_left,
        }
    }

    fn decision_possible(&self, decision: &Decision) -> bool {
        match decision {
            Decision::Idle => true,
            Decision::OreBot => self.blueprint.ore_bot_cost <= self.ore_stock,
            Decision::ClayBot => self.blueprint.clay_bot_cost <= self.ore_stock,
            Decision::ObsBot => {
                self.blueprint.obs_bot_cost.0 <= self.ore_stock
                    && self.blueprint.obs_bot_cost.1 <= self.clay_stock
            }
            Decision::GeoBot => {
                self.blueprint.geo_bot_cost.0 <= self.ore_stock
                    && self.blueprint.geo_bot_cost.1 <= self.obs_stock
            }
        }
    }

    fn pass_cycle(&mut self) {
        self.rounds_left -= 1;
        self.ore_stock += self.ore_bot_count;
        self.clay_stock += self.clay_bot_count;
        self.obs_stock += self.obs_bot_count;
        self.geo_stock += self.geo_bot_count;
    }

    fn decision_cycle_copy(&self, decision: &Decision) -> Self {
        let mut ret = self.clone();
        ret.pass_cycle();
        match decision {
            Decision::Idle => {
                //nothing happens
            }
            Decision::OreBot => {
                ret.ore_bot_count += 1;
                ret.ore_stock -= ret.blueprint.ore_bot_cost;
            }
            Decision::ClayBot => {
                ret.clay_bot_count += 1;
                ret.ore_stock -= ret.blueprint.clay_bot_cost;
            }
            Decision::ObsBot => {
                ret.obs_bot_count += 1;
                ret.ore_stock -= ret.blueprint.obs_bot_cost.0;
                ret.clay_stock -= ret.blueprint.obs_bot_cost.1;
            }
            Decision::GeoBot => {
                ret.geo_bot_count += 1;
                ret.ore_stock -= ret.blueprint.geo_bot_cost.0;
                ret.obs_stock -= ret.blueprint.geo_bot_cost.1;
            }
        }
        ret
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Decision {
    Idle,
    OreBot,
    ClayBot,
    ObsBot,
    GeoBot,
}

impl Decision {
    pub fn iterator() -> impl Iterator<Item = Decision> {
        [
            Self::Idle,
            Self::OreBot,
            Self::ClayBot,
            Self::ObsBot,
            Self::GeoBot,
        ]
        .iter()
        .copied()
    }
}

fn best_outcome(op_state: OperationState, opp_cost: &HashSet<Decision>, multiplied: bool, mk_geocount: Option<usize>) -> (usize,bool,usize) {
    let mut geocount = mk_geocount.unwrap_or(0);
    if op_state.rounds_left == 0 {
        if multiplied{
            (op_state.blueprint.no * op_state.geo_stock, true, op_state.geo_bot_count)
        }
        else {
            (op_state.geo_stock, true, op_state.geo_bot_count)
        }
        
    }
    else if op_state.rounds_left == 1 { //Building anything in last round is useless
        let (score, _, geocount) = best_outcome(op_state.decision_cycle_copy(&Decision::Idle), &HashSet::new(), multiplied, None);
        (score, true, geocount)
    }
    else {
        let mut best_score = 0;
        let mut geobots_only = false;
        
        //Pioritize making geobots
        if op_state.decision_possible(&Decision::GeoBot){
            let geocount_candidate;
            (best_score, geobots_only, geocount_candidate) = best_outcome(op_state.decision_cycle_copy(&Decision::GeoBot), &HashSet::new(), multiplied, None);
            geocount = cmp::max(geocount, geocount_candidate)
        }
        //Can we acieve higher geobot count when we are not making a bot?
        let achieveable = geocount < op_state.rounds_left-1;
        //If best score wasn't acieved by making only geobots, try other options
        if !geobots_only && achieveable{
            let possible_decisions: HashSet<Decision> = Decision::iterator().filter(|decision| 
                decision != &Decision::GeoBot &&
                op_state.decision_possible(&decision) && 
                !opp_cost.contains(decision)).collect(); //excluding making geobot, we already tried that
            let mut next_opp_cost = opp_cost.clone();
            for possible_decision in &possible_decisions{
                if *possible_decision != Decision::Idle{
                    next_opp_cost.insert(*possible_decision);
                }
            };
    
            for possible_decision in &possible_decisions{
                let (can_score, can_geobots, can_geocount) = match possible_decision {
                    Decision::Idle => {
                        best_outcome(op_state.decision_cycle_copy(&possible_decision ), &next_opp_cost, multiplied, Some(geocount))
                    }
                    _ => {
                        best_outcome(op_state.decision_cycle_copy(&possible_decision ), &HashSet::new(), multiplied, Some(geocount))
                    }
                };
                if can_score > best_score{ //not >=
                    best_score = can_score;
                    geobots_only = can_geobots;
                    geocount = can_geocount;
                }
            }
        }
        (best_score, geobots_only, geocount)
    }
}

pub fn cli() -> Command {
    Command::new("day19").about("Elvish geode collecting").arg(
        clap::arg!(path: <PATH>)
            .required(true)
            .value_parser(value_parser!(std::path::PathBuf)),
    )
}

pub async fn handle(matches: &ArgMatches) {
    let path = matches.get_one::<std::path::PathBuf>("path");
    solve(path.unwrap().to_path_buf()).await
}

pub async fn solve(filepath: PathBuf) {
    if let Ok(lines) = common::read_lines(filepath) {
        let blueprints: Vec<Blueprint> = parse_lines(lines.map(|l| l.unwrap())).collect();
        let mut msg_vec = Vec::new();
        for blueprint in &blueprints{
            let addr = SimulatorSUA.start();
            msg_vec.push(addr.send(Task(blueprint.clone(), ROUND_COUNT, true)));
        }
        let mut acc = 0;
        for msg in msg_vec{
            match msg.await{
                Ok(val) => {
                    println!("Got response: {}", val);
                    acc += val;
                }
                Err(_) => {
                    println!("Couldn't get response!");
                }
            }
        }
        
        let mut msg_vec = Vec::new();
        let mut acc2: usize = 1;
        for i in 0..3{
            let addr = SimulatorSUA.start();
            msg_vec.push(addr.send(Task(blueprints[i].clone(), ROUND_COUNT2, false)));
        }
        for msg in msg_vec{
            match msg.await{
                Ok(val) => {
                    println!("Got response: {}", val);
                    acc2 *= val;
                }
                Err(_) => {
                    println!("Couldn't get response!");
                }
            }
        }
        println!("Total sum: {} (pt1)", acc);
        println!("Total procuct: {} (pt2)", acc2);

    } else {
        println!("Could not open file!")
    }
}

fn parse_lines<'a>(
    lines: impl Iterator<Item = String> + 'a,
) -> impl Iterator<Item = Blueprint> + 'a {
    lines.map(|l| Blueprint::from_line(&l))
}

use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "usize")]
struct Task(Blueprint, usize, bool);

struct SimulatorSUA;

impl Actor for SimulatorSUA {
    type Context = Context<Self>;
}

impl Handler<Task> for SimulatorSUA {
    type Result = usize;

    fn handle(&mut self, msg: Task, _ctx: &mut Context<Self>) -> Self::Result {
        let (score,_,_) = best_outcome(OperationState::new(msg.0, msg.1), &HashSet::new(), msg.2, None);
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        let bp1 = Blueprint::from_line("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
        let bp2 = Blueprint::from_line("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
        let (s1,_,_) = best_outcome(OperationState::new(bp1, ROUND_COUNT), &HashSet::new(), true, None);
        assert_eq!(s1, 9);
        println!("PT1 done!");
        let (s2,_,_) = best_outcome(OperationState::new(bp2, ROUND_COUNT), &HashSet::new(), true, None);
        assert_eq!(s2, 24);
    }
}
