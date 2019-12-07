mod solutions;
mod solver;
mod intcode_computer;

use std::env;
use crate::solutions::exec_day;

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1);
    exec_day(day);
}