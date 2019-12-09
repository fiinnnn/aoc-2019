use crate::solver::Solver;
use crate::intcode_computer::{IntcodeComputer, IO, SingleIO, read_program};
use std::io::Read;

pub struct Problem;


impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        read_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut cpu = IntcodeComputer::new(&mut input.clone(), SingleIO::new_init(1));
        cpu.run();
        cpu.io.pop_output()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut cpu = IntcodeComputer::new(&mut input.clone(), SingleIO::new_init(2));
        cpu.run();
        cpu.io.pop_output()
    }
}