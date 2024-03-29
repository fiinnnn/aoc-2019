// GENERATED ON BUILD, DO NOT EDIT
use crate::solver::Solver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

pub fn exec_day(day: i32) {
    match day {
        1 => day01::Problem {}.solve(day),
        2 => day02::Problem {}.solve(day),
        3 => day03::Problem {}.solve(day),
        4 => day04::Problem {}.solve(day),
        5 => day05::Problem {}.solve(day),
        6 => day06::Problem {}.solve(day),
        7 => day07::Problem {}.solve(day),
        8 => day08::Problem {}.solve(day),
        9 => day09::Problem {}.solve(day),
        10 => day10::Problem {}.solve(day),
        11 => day11::Problem {}.solve(day),
        12 => day12::Problem {}.solve(day),
        13 => day13::Problem {}.solve(day),
        14 => day14::Problem {}.solve(day),
        15 => day15::Problem {}.solve(day),
        16 => day16::Problem {}.solve(day),
        17 => day17::Problem {}.solve(day),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}
