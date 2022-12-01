use std::env;

mod task01;
fn main() {
    let cmd = clap::Command::new("aoc")
        .subcommand_required(true)
        .subcommand(task01::cli());
    
    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("day01", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };
    let path = matches.get_one::<std::path::PathBuf>("path");

    task01::solve(path.unwrap().to_path_buf());
}
