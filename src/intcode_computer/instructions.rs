#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
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
pub enum ParameterMode {
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

impl From<i64> for Instruction {
    fn from(n: i64) -> Self {
        let param_mode_1 = ((n / 100) % 10).into();
        let param_mode_2 = ((n / 1000) % 10).into();
        let param_mode_3 = ((n / 10000) % 10).into();

        let opcode = n % 100;
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
}