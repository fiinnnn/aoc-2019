use std::io::{BufReader, Read, BufRead};
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Sender, Receiver};

pub fn read_program<R: Read>(r: R) -> Vec<i64> {
    BufReader::new(r)
        .split(b',')
        .flatten()
        .flat_map(String::from_utf8)
        .flat_map(|s| s.parse())
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    ADD((ParameterMode, ParameterMode, ParameterMode)),
    MUL((ParameterMode, ParameterMode, ParameterMode)),

    IN(ParameterMode),
    OUT(ParameterMode),

    JNZ((ParameterMode, ParameterMode)),
    JEZ((ParameterMode, ParameterMode)),

    LT((ParameterMode, ParameterMode, ParameterMode)),
    EQ((ParameterMode, ParameterMode, ParameterMode)),

    ARB(ParameterMode),

    HLT
}

#[derive(Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(n: i64) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown parameter mode: {}", n),
        }
    }
}

fn decode_instruction(instruction: i64) -> Instruction {
    let param_mode_1 = ((instruction / 100) % 10).into();
    let param_mode_2 = ((instruction / 1000) % 10).into();
    let param_mode_3 = ((instruction / 10000) % 10).into();

    let opcode = instruction % 100;
    match opcode {
        1 => Instruction::ADD((param_mode_1, param_mode_2, param_mode_3)),
        2 => Instruction::MUL((param_mode_1, param_mode_2, param_mode_3)),
        3 => Instruction::IN(param_mode_1),
        4 => Instruction::OUT(param_mode_1),
        5 => Instruction::JNZ((param_mode_1, param_mode_2)),
        6 => Instruction::JEZ((param_mode_1, param_mode_2)),
        7 => Instruction::LT((param_mode_1, param_mode_2, param_mode_3)),
        8 => Instruction::EQ((param_mode_1, param_mode_2, param_mode_3)),
        9 => Instruction::ARB(param_mode_1),
        99 => Instruction::HLT,
        _ => panic!("Unknown opcode: {}", opcode),
    }
}

pub trait IO {
    fn push_input(&mut self, val: i64);
    fn pop_input(&mut self) -> i64;

    fn push_output(&mut self, val: i64);
    fn pop_output(&mut self) -> i64;
}

pub struct NoIO;

impl IO for NoIO {
    fn push_input(&mut self, _: i64) {}
    fn pop_input(&mut self) -> i64 { 0 }

    fn push_output(&mut self, _: i64) {}
    fn pop_output(&mut self) -> i64 { 0 }
}

pub struct SingleIO {
    val: i64,
}

impl SingleIO {
    pub fn new() -> Self { Self { val:0 } }
    pub fn new_init(val: i64) -> Self { Self { val } }
}

impl IO for SingleIO {
    fn push_input(&mut self, val: i64) { self.val = val; }
    fn pop_input(&mut self) -> i64 { self.val }

    fn push_output(&mut self, val: i64) { self.val = val; }
    fn pop_output(&mut self) -> i64 { self.val }
}

pub struct QueueIO {
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl QueueIO {
    pub fn new() -> Self {
        Self {
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn new_init(init: Vec<i64>) -> Self {
        Self {
            input: VecDeque::from(init),
            output: VecDeque::new(),
        }
    }
}

impl IO for QueueIO {
    fn push_input(&mut self, val: i64) {
        self.input.push_back(val);
    }

    fn pop_input(&mut self) -> i64 {
        self.input.pop_front().expect("No input available")
    }

    fn push_output(&mut self, val: i64) {
        self.output.push_back(val);
    }

    fn pop_output(&mut self) -> i64 {
        self.output.pop_front().expect("No output available")
    }
}

pub struct AsyncIO {
    tx: Vec<Sender<i64>>,
    rx: Option<Receiver<i64>>,
    input: Vec<i64>,
}

impl AsyncIO {
    pub fn new() -> Self {
        Self {
            tx: Vec::new(),
            rx: None,
            input: Vec::new(),
        }
    }

    pub fn new_init(input: Vec<i64>) -> Self {
        Self {
            tx: Vec::new(),
            rx: None,
            input,
        }
    }

    pub fn get_receiver(&mut self) -> Receiver<i64> {
        let (tx, rx) = channel();
        self.tx.push(tx);
        rx
    }

    pub fn set_receiver(&mut self, rx: Receiver<i64>) {
        self.rx = Some(rx);
    }
}

impl IO for AsyncIO {
    fn push_input(&mut self, val: i64) {
        self.input.push(val);
    }

    fn pop_input(&mut self) -> i64 {
        if let Some(val) = self.input.pop() {
            val
        }
        else if let Some(rx) = &self.rx {
            rx.recv().unwrap()
        }
        else {
            0
        }
    }

    fn push_output(&mut self, val: i64) {
        for tx in &self.tx {
            let _ = tx.send(val);
        }
    }

    fn pop_output(&mut self) -> i64 {
        unimplemented!()
    }
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
        decode_instruction(self.read(self.pc))
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
            decode_instruction(1002),
            Instruction::MUL(
                (ParameterMode::Position,
                 ParameterMode::Immediate,
                 ParameterMode::Position))
        );

        assert_eq!(
            decode_instruction(1108),
            Instruction::EQ(
                (ParameterMode::Immediate,
                 ParameterMode::Immediate,
                 ParameterMode::Position))
        );

        assert_eq!(
            decode_instruction(2002),
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
        let mut computer = IntcodeComputer::new(&mut program, SingleIO::new_init(input));
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