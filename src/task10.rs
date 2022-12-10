use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command};

use crate::common;

enum Operation{
    Noop,
    Addx(i32),
    Unknown
}

struct ProcessorSim{
    acc_during: i32,
    acc_after: i32,
    operations: Vec<Operation>,
    current_cycle: u32,
    instruction_pos: usize,
    //processed_op: &Operation,
    wait_timer: u32,
}

impl ProcessorSim {
    pub fn new(operations: Vec<Operation>) -> Self {
        Self { 
            acc_during: 1,
            acc_after: 1,
            operations,
            current_cycle: 0,
            instruction_pos:0,
            //processed_op:&Operation::Noop,
            wait_timer: 0,
        }
    }
    pub fn pass_cycle(&mut self){
        self.current_cycle += 1; //increase cycle count
        self.acc_during = self.acc_after;

        // --- Reading phase
        //check if should draw next operation
        if self.wait_timer == 0 { //if yes, draw it
            self.wait_timer = Self::op_to_wait(&self.operations[self.instruction_pos])
        }
        else { //if no, wait
            self.wait_timer -= 1;
        }
        // --- Execution phase
        if self.wait_timer == 0 {
            match self.operations[self.instruction_pos] {
                Operation::Noop  => {
                    //do nothing
                },
                Operation::Addx(val)  => {
                    self.acc_after += val;
                },
                Operation::Unknown => panic!()
            }
            self.instruction_pos += 1;
        }
    }

    pub fn signal_strength(&self) -> i128{
        self.acc_during as i128*self.current_cycle as i128
    }

    pub fn is_exhausted(&self) -> bool{
        self.instruction_pos == self.operations.len() && self.wait_timer == 0
    }

    fn op_to_wait(op: &Operation) -> u32{
        match op {
            Operation::Noop  => 0,
            Operation::Addx(_)  => 1,
            Operation::Unknown => panic!()
        }
    }
    pub fn get_pixel(&self) -> char{
        let position_cursor = ((self.current_cycle-1) % 40) as i32;
        let position_sprite_start = self.acc_during - 1;
        let position_sprite_end = self.acc_during + 1;
        if position_cursor >= position_sprite_start && position_cursor <= position_sprite_end{
            '#'
        }
        else{
            ' '
        }

    }

}

pub fn cli() -> Command {
    Command::new("day10")
        .about("Elvish processor cycles")
        .arg(
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
        let operations: Vec<Operation> = lines.map(|line| line_to_operation(&line.unwrap())).collect();
        let mut simulator = ProcessorSim::new(operations);
        let cycles_of_intrest:Vec<u32> = vec![20,60,100,140,180,220];
        let mut acc: i128 = 0;
        let mut str_acc = String::new();
        while !simulator.is_exhausted() {
            simulator.pass_cycle();
            str_acc.push(simulator.get_pixel());
            if str_acc.len() >=40{
                println!("B{}E", str_acc);
                str_acc = String::new();
            }

            if cycles_of_intrest.contains(&(simulator.current_cycle)){
                //println!("State in cycle {}: {} strength({})", simulator.current_cycle, simulator.acc_during, simulator.signal_strength());
                acc += simulator.signal_strength();
            }
        }
        println!("Total signal strength: {}", acc);
    }
}

fn line_to_operation(line: &str) -> Operation{
    match line{
        "noop" => Operation::Noop,
        lmatched if lmatched.starts_with("addx ") => Operation::Addx(line.split(' ').nth(1).unwrap().parse().unwrap()),
        _ => Operation::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::{Operation, ProcessorSim, line_to_operation};

    #[test]
    fn test_cycles(){
        let ops = vec![Operation::Noop, Operation::Addx(3), Operation::Addx(-5)];
        let mut simulator = ProcessorSim::new(ops);

        simulator.pass_cycle();
        assert_eq!(simulator.acc_during, 1);
        assert_eq!(simulator.acc_after, 1);
        assert_eq!(simulator.current_cycle, 1);
        simulator.pass_cycle();
        assert_eq!(simulator.acc_during, 1);
        assert_eq!(simulator.acc_after, 1);
        assert_eq!(simulator.current_cycle, 2);
        simulator.pass_cycle();
        assert_eq!(simulator.acc_during, 1);
        assert_eq!(simulator.acc_after, 4);
        assert_eq!(simulator.current_cycle, 3);
        simulator.pass_cycle();
        assert_eq!(simulator.acc_during, 4);
        assert_eq!(simulator.acc_after, 4);
        assert_eq!(simulator.current_cycle, 4);
        simulator.pass_cycle();
        assert_eq!(simulator.acc_during, 4);
        assert_eq!(simulator.acc_after, -1);
        assert_eq!(simulator.current_cycle, 5);

    }
    #[test]
    fn test_cycles2(){
        let lines = vec!(
            "addx 15",
            "addx -11",
            "addx 6",
            "addx -3",
            "addx 5",
            "addx -1",
            "addx -8",
            "addx 13",
            "addx 4",
            "noop",
            "addx -1",
            "addx 5"
        );
        let ops = lines.iter().map(|l| line_to_operation(l)).collect();
        let mut simulator = ProcessorSim::new(ops);
        while !simulator.is_exhausted() {
            simulator.pass_cycle();
            if simulator.current_cycle == 20{
                assert_eq!(simulator.acc_during, 21);
                assert_eq!(simulator.signal_strength(), 420);
            }
        }

    }
}

