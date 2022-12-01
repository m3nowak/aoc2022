use std::env;

mod task01;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    task01::solve(String::from(query));
}
