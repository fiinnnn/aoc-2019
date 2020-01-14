use std::io::{BufReader, Read, BufRead};

use crate::intcode_computer::instructions::*;
pub use crate::intcode_computer::io::*;

mod instructions;
pub mod io;

pub fn read_program<R: Read>(r: R) -> Vec<i64> {
    BufReader::new(r)
        .split(b',')
        .flatten()
        .flat_map(String::from_utf8)
        .flat_map(|s| s.parse())
        .collect()
}

pub struct IntcodeComputer<T>
    where T: IO {
    pub io: T,
    memory: Vec<i64>,
    pc: usize,
    rel_base: i64,
}

impl<T> IntcodeComputer<T>
    where T: IO {
    pub fn new(memory: &mut Vec<i64>, io: T) -> IntcodeComputer<T> {
        IntcodeComputer {
            io,
            memory: memory.clone(),
            pc: 0,
            rel_base: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.get_instruction();

            match instruction {
                Instruction::ADD(modes) => {
                    self.add(modes);
                    self.inc_pc(4);
                }
                Instruction::MUL(modes) => {
                    self.multiply(modes);
                    self.inc_pc(4);
                }
                Instruction::IN(mode) => {
                    self.input(mode);
                    self.inc_pc(2);
                }
                Instruction::OUT(mode) => {
                    self.output(mode);
                    self.inc_pc(2);
                }
                Instruction::JNZ(modes) => {
                    self.jump_not_zero(modes);
                }
                Instruction::JEZ(modes) => {
                    self.jump_equal_zero(modes);
                }
                Instruction::LT(modes) => {
                    self.less_than(modes);
                    self.inc_pc(4);
                }
                Instruction::EQ(modes) => {
                    self.equals(modes);
                    self.inc_pc(4);
                }
                Instruction::ARB(mode) => {
                    self.adjust_rel_base(mode);
                    self.inc_pc(2);
                }
                Instruction::HLT => break,
            }
        }
    }

    pub fn read(&self, addr: usize) -> i64 {
        if addr >= self.memory.len() {
            return 0;
        }

        self.memory[addr]
    }

    pub fn write(&mut self, addr: usize, val: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }

        self.memory[addr] = val;
    }

    fn get_instruction(&self) -> Instruction {
        self.read(self.pc).into()
    }

    fn inc_pc(&mut self, amount: usize) {
        self.pc = self.pc.wrapping_add(amount);
    }

    fn get_params_3(&self, (m1, m2, m3): (ParameterMode, ParameterMode, ParameterMode)) -> (i64, i64, usize) {
        let param1 = self.read(self.pc + 1);
        let param2 = self.read(self.pc + 2);
        let param3 = self.read(self.pc + 3);

        let p1 = self.get_val(param1, m1);
        let p2 = self.get_val(param2, m2);
        let addr = self.get_dest(param3, m3);

        (p1, p2, addr)
    }

    fn get_params_2(&self, (m1, m2): (ParameterMode, ParameterMode)) -> (i64, i64) {
        let param1 = self.read(self.pc + 1);
        let param2 = self.read(self.pc + 2);

        let p1 = self.get_val(param1, m1);
        let p2 = self.get_val(param2, m2);

        (p1, p2)
    }

    fn get_val(&self, param: i64, mode: ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => self.read(param as usize),
            ParameterMode::Immediate => param,
            ParameterMode::Relative => self.read((self.rel_base + param) as usize),
        }
    }

    fn get_dest(&self, param: i64, mode: ParameterMode) -> usize {
        match mode {
            ParameterMode::Position => param as usize,
            ParameterMode::Immediate => panic!("Not supported"),
            ParameterMode::Relative => (self.rel_base + param) as usize,
        }
    }

    fn add(&mut self, modes: (ParameterMode, ParameterMode, ParameterMode)) {
        let (p1, p2, addr) = self.get_params_3(modes);
        self.write(addr, p1 + p2);
    }

    fn multiply(&mut self, modes: (ParameterMode, ParameterMode, ParameterMode)) {
        let (p1, p2, addr) = self.get_params_3(modes);
        self.write(addr, p1 * p2);
    }

    fn input(&mut self,  mode: ParameterMode) {
        let addr = self.get_dest(self.read(self.pc + 1), mode) as usize;
        let val = self.io.pop_input();
        self.write(addr, val);
    }

    fn output(&mut self, mode: ParameterMode) {
        let val = self.get_val(self.read(self.pc + 1), mode);
        self.io.push_output(val);
    }

    fn jump_not_zero(&mut self, modes: (ParameterMode, ParameterMode)) {
        let (p1, addr) = self.get_params_2(modes);
        if p1 != 0 {
            self.pc = addr as usize;
        }
        else {
            self.inc_pc(3);
        }
    }

    fn jump_equal_zero(&mut self, modes: (ParameterMode, ParameterMode)) {
        let (p1, addr) = self.get_params_2(modes);
        if p1 == 0 {
            self.pc = addr as usize;
        }
        else {
            self.inc_pc(3);
        }
    }

    fn less_than(&mut self, modes: (ParameterMode, ParameterMode, ParameterMode)) {
        let (p1, p2, addr) = self.get_params_3(modes);
        let mut output = 0;
        if p1 < p2 {
            output = 1;
        }
        self.write(addr, output);
    }

    fn equals(&mut self, modes: (ParameterMode, ParameterMode, ParameterMode)) {
        let (p1, p2, addr) = self.get_params_3(modes);
        let mut output = 0;
        if p1 == p2 {
            output = 1;
        }
        self.write(addr, output);
    }

    fn adjust_rel_base(&mut self, mode: ParameterMode) {
        let val = self.get_val(self.read(self.pc + 1 as usize), mode);
        self.rel_base = self.rel_base + val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_instruction() {
        assert_eq!(
            Instruction::from(1002),
            Instruction::MUL(
                (ParameterMode::Position,
                 ParameterMode::Immediate,
                 ParameterMode::Position))
        );

        assert_eq!(
            Instruction::from(1108),
            Instruction::EQ(
                (ParameterMode::Immediate,
                 ParameterMode::Immediate,
                 ParameterMode::Position))
        );

        assert_eq!(
            Instruction::from(2002),
            Instruction::MUL(
                (ParameterMode::Position,
                 ParameterMode::Relative,
                 ParameterMode::Position)
            )
        )
    }

    fn test_program(mut program: Vec<i64>, expected_output: Vec<i64>) {
        let mut computer = IntcodeComputer::new(&mut program, NoIO);
        computer.run();
        assert_eq!(computer.memory, expected_output);
    }

    fn test_program_output(mut program: Vec<i64>, input: i64, expected_output: i64) {
        let mut computer = IntcodeComputer::new(&mut program, SingleIO::new(input));
        computer.run();
        assert_eq!(computer.io.pop_output(), expected_output);
    }

    #[test]
    fn test_day2_compatibility() {
        test_program(vec![1,1,1,4,99,5,6,0,99],
                     vec!(30,1,1,4,2,5,6,0,99));
    }

    #[test]
    fn test_day5_compatibility() {
        test_program(vec![1002,4,3,4,33],
                     vec![1002,4,3,4,99]);
    }

    #[test]
    fn test_day5p2_compatibility() {
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                               1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                               999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        test_program_output(program.clone(), 7, 999);
        test_program_output(program.clone(), 8, 1000);
        test_program_output(program.clone(), 9, 1001);
    }
}