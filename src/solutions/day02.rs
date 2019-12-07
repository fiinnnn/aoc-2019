use crate::solver::Solver;
use crate::intcode_computer::IntcodeComputer;
use std::io::{
            self,
            BufReader,
            Read
};

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
        let mut memory = input.clone();
        memory[1] = 12;
        memory[2] = 2;

        let mut computer = IntcodeComputer::new(&mut memory, 0);
        computer.run();
        computer.read(0)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        for noun in  0..99 {
            for verb in 0..99 {
                let mut memory = input.clone();
                memory[1] = noun;
                memory[2] = verb;

                let mut computer = IntcodeComputer::new(&mut memory, 0);

                computer.run();

                if computer.read(0) == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }

        unreachable!();
    }
}