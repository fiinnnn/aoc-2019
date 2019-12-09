use crate::solver::Solver;
use crate::intcode_computer::{IntcodeComputer, NoIO, read_program};
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
        let mut memory = input.clone();
        memory[1] = 12;
        memory[2] = 2;

        let mut computer = IntcodeComputer::new(&mut memory, NoIO);
        computer.run();
        computer.read(0)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        for noun in  0..99 {
            for verb in 0..99 {
                let mut memory = input.clone();
                memory[1] = noun;
                memory[2] = verb;

                let mut computer = IntcodeComputer::new(&mut memory, NoIO);

                computer.run();

                if computer.read(0) == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }

        unreachable!();
    }
}