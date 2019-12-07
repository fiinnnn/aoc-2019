use crate::solver::Solver;
use std::io::{
    self,
    BufReader,
    Read
};
use crate::intcode_computer::IntcodeComputer;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut r = BufReader::new(r);
        let mut s = String::new();
        r.read_to_string(&mut s).unwrap();
        s.split(',').flat_map(|n| n.parse()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut program = input.clone();
        let mut computer = IntcodeComputer::new(&mut program, 1);
        computer.run();
        computer.get_output().last().expect("Can't get last output value").clone()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut program = input.clone();
        let mut computer = IntcodeComputer::new(&mut program, 5);
        computer.run();
        computer.get_output().last().expect("Can't get last output value").clone()
    }
}