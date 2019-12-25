use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        vec![]
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        1
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        1
    }
}