mod common;
mod task01;
mod task02;
mod task03;

fn main() {
    let cmd = clap::Command::new("aoc")
        .subcommand_required(true)
        .subcommand(task01::cli())
        .subcommand(task02::cli())
        .subcommand(task03::cli());
    
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("day01", sub_matches)) => task01::handle(sub_matches),
        Some(("day02", sub_matches)) => task02::handle(sub_matches),
        Some(("day03", sub_matches)) => task03::handle(sub_matches),
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
