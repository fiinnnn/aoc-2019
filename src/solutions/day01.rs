use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().flat_map(|l| l.parse()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        Iterator::sum(input.iter().map(|n| n / 3 - 2))
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut total = 0;

        for n in input {
            let mut fuel = n / 3 - 2;
            let mut fuel_weight = fuel;
            loop {
                fuel_weight = fuel_weight / 3 - 2;
                if fuel_weight > 0 {
                    fuel += fuel_weight;
                } else {
                    break;
                }
            }

            total += fuel;
        }

        total
    }
}