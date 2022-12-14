mod common;
mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod task06;
mod task07;
mod task08;
mod task09;
mod task10;
mod task11;
mod task12;
mod task13;
mod task14;

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
        .subcommand(task08::cli())
        .subcommand(task09::cli())
        .subcommand(task10::cli())
        .subcommand(task11::cli())
        .subcommand(task12::cli())
        .subcommand(task13::cli())
        .subcommand(task14::cli());
    
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
        Some(("day09", sub_matches)) => task09::handle(sub_matches),
        Some(("day10", sub_matches)) => task10::handle(sub_matches),
        Some(("day11", sub_matches)) => task11::handle(sub_matches),
        Some(("day12", sub_matches)) => task12::handle(sub_matches),
        Some(("day13", sub_matches)) => task13::handle(sub_matches),
        Some(("day14", sub_matches)) => task14::handle(sub_matches),
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
