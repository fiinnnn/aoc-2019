pub struct IntcodeComputer {
    memory: Vec<i64>,
    pc: usize,
    input: i64,
    output: Vec<i64>,
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    ADD((ParameterMode, ParameterMode)),
    MUL((ParameterMode, ParameterMode)),

    IN,
    OUT(ParameterMode),

    JNZ((ParameterMode, ParameterMode)),
    JEZ((ParameterMode, ParameterMode)),

    LT((ParameterMode, ParameterMode)),
    EQ((ParameterMode, ParameterMode)),

    HALT
}

#[derive(Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<i64> for ParameterMode {
    fn from(n: i64) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown parameter mode: {}", n),
        }
    }
}

fn decode_instruction(instruction: i64) -> Instruction {
    let param_mode_1 = ((instruction / 100) % 10).into();
    let param_mode_2 = ((instruction / 1000) % 10).into();

    let opcode = instruction % 100;
    match opcode {
        1 => Instruction::ADD((param_mode_1, param_mode_2)),
        2 => Instruction::MUL((param_mode_1, param_mode_2)),
        3 => Instruction::IN,
        4 => Instruction::OUT(param_mode_1),
        5 => Instruction::JNZ((param_mode_1, param_mode_2)),
        6 => Instruction::JEZ((param_mode_1, param_mode_2)),
        7 => Instruction::LT((param_mode_1, param_mode_2)),
        8 => Instruction::EQ((param_mode_1, param_mode_2)),
        99 => Instruction::HALT,
        _ => panic!("Unknown opcode: {}", opcode),
    }
}

impl IntcodeComputer {
    pub fn new(memory: &mut Vec<i64>, input: i64) -> IntcodeComputer {
        IntcodeComputer {
            memory: memory.clone(),
            pc: 0,
            input,
            output: Vec::new(),
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
                Instruction::IN => {
                    self.input();
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
                Instruction::HALT => break,
            }
        }
    }

    pub fn read(&self, addr: usize) -> i64 {
        self.memory[addr]
    }

    pub fn write(&mut self, addr: usize, val: i64) {
        self.memory[addr] = val;
    }

    pub fn get_output(&self) -> &Vec<i64> {
        &self.output
    }

    fn get_instruction(&self) -> Instruction {
        decode_instruction(self.read(self.pc))
    }

    fn inc_pc(&mut self, amount: usize) {
        self.pc = self.pc.wrapping_add(amount);
    }

    fn get_params_3(&self, (m1, m2): (ParameterMode, ParameterMode)) -> (i64, i64, usize) {
        let param1 = self.read(self.pc + 1);
        let param2 = self.read(self.pc + 2);
        let addr = self.read(self.pc + 3) as usize;

        let p1 = self.get_val(param1, m1);
        let p2 = self.get_val(param2, m2);

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
        }
    }

    fn add(&mut self, modes: (ParameterMode, ParameterMode)) {
        let (p1, p2, addr) = self.get_params_3(modes);
        self.write(addr, p1 + p2);
    }

    fn multiply(&mut self, modes: (ParameterMode, ParameterMode)) {
        let (p1, p2, addr) = self.get_params_3(modes);
        self.write(addr, p1 * p2);
    }

    fn input(&mut self) {
        let addr = self.read(self.pc + 1) as usize;
        self.write(addr, self.input);
    }

    fn output(&mut self, mode: ParameterMode) {
        let val = self.get_val(self.read(self.pc + 1), mode);
        self.output.push(val);
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

    fn less_than(&mut self, modes: (ParameterMode, ParameterMode)) {
        let (p1, p2, addr) = self.get_params_3(modes);
        let mut output = 0;
        if p1 < p2 {
            output = 1;
        }
        self.write(addr, output);
    }

    fn equals(&mut self, modes: (ParameterMode, ParameterMode)) {
        let (p1, p2, addr) = self.get_params_3(modes);
        let mut output = 0;
        if p1 == p2 {
            output = 1;
        }
        self.write(addr, output);
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
                 ParameterMode::Immediate))
        );

        assert_eq!(
            decode_instruction(1108),
            Instruction::EQ(
                (ParameterMode::Immediate,
                 ParameterMode::Immediate))
        );
    }

    fn test_program(mut program: Vec<i64>, expected_output: Vec<i64>) {
        let mut computer = IntcodeComputer::new(&mut program, 0);
        computer.run();
        assert_eq!(computer.memory, expected_output);
    }

    fn test_program_output(mut program: Vec<i64>, input: i64, expected_output: i64) {
        let mut computer = IntcodeComputer::new(&mut program, input);
        computer.run();
        let output = computer.get_output().last().unwrap();
        assert_eq!(*computer.get_output().last().unwrap(), expected_output);
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