use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use clap::{value_parser,Command, ArgMatches};

pub fn cli() -> Command {
    Command::new("day02")
        .about("Elvish hand games")
        .arg(clap::arg!(path: <PATH>).required(true).value_parser(value_parser!(std::path::PathBuf)))
}