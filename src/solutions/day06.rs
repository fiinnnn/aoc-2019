use crate::solver::Solver;
use std::{
    io::{self, BufRead, BufReader},
};

pub struct Problem;

struct Planet {
    name: String,
    children: Vec<Planet>,
}

impl Solver for Problem {
    type Input = Planet;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);

        Planet {
            name: "aa".into(),
            children: vec![],
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        1
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        1
    }
}