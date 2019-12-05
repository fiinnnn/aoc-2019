use crate::solver::Solver;
use std::io::{
            self,
            BufReader,
            Read
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i32>;
    type Output1 = i32;
    type Output2 = i32;

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
        let mut program = Program {
            memory,
            pc: 0,
        };

        program.exec();

        program.memory[0]
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        for noun in  0..99 {
            for verb in 0..99 {
                let mut memory = input.clone();
                memory[1] = noun;
                memory[2] = verb;

                let mut program = Program {
                    memory,
                    pc: 0,
                };

                program.exec();

                if program.memory[0] == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }

        unreachable!();
    }
}

pub struct Program {
    memory: Vec<i32>,
    pc: usize,
}

enum Instruction {
    ADD((usize, usize, usize)),
    MUL((usize, usize, usize)),
    HALT
}

impl Program {

    pub fn exec(&mut self) {
        loop {
            let instruction = self.get_instruction();

            match instruction {
                Instruction::ADD(args) => {
                    self.add(args);
                    self.pc += 4;
                },
                Instruction::MUL(args) => {
                    self.mul(args);
                    self.pc += 4;
                },
                Instruction::HALT => break,
            }
        }
    }

    fn get_instruction(&self) -> Instruction {
        let opcode =  self.memory[self.pc];
        match opcode {
            1 => Instruction::ADD(self.get_args()),
            2 => Instruction::MUL(self.get_args()),
            99 => Instruction::HALT,
            _ => panic!("Unknown opcode {}, pc: {}", opcode, self.pc),
        }
    }

    fn get_args(&self) -> (usize, usize, usize) {
        (self.memory[self.pc + 1] as usize,
         self.memory[self.pc + 2] as usize,
         self.memory[self.pc + 3] as usize)
    }

    fn add(&mut self, (p1, p2, res): (usize, usize, usize)) {
        self.memory[res] = self.memory[p1] + self.memory[p2];
    }

    fn mul(&mut self, (p1, p2, res): (usize, usize, usize)) {
        self.memory[res] = self.memory[p1] * self.memory[p2];
    }
}