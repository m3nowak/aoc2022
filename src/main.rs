mod common;
mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod task06;
mod task07;
mod task08;

fn main() {
    let cmd = clap::Command::new("aoc")
        .subcommand_required(true)
        .subcommand(task01::cli())
        .subcommand(task02::cli())
        .subcommand(task03::cli())
        .subcommand(task04::cli())
        .subcommand(task05::cli())
        .subcommand(task06::cli())
        .subcommand(task07::cli())
        .subcommand(task08::cli());
    
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("day01", sub_matches)) => task01::handle(sub_matches),
        Some(("day02", sub_matches)) => task02::handle(sub_matches),
        Some(("day03", sub_matches)) => task03::handle(sub_matches),
        Some(("day04", sub_matches)) => task04::handle(sub_matches),
        Some(("day05", sub_matches)) => task05::handle(sub_matches),
        Some(("day06", sub_matches)) => task06::handle(sub_matches),
        Some(("day07", sub_matches)) => task07::handle(sub_matches),
        Some(("day08", sub_matches)) => task08::handle(sub_matches),
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
