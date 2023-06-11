use strum::EnumIter;

use crate::operations::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, EnumIter)]
pub enum Instruction {
    Load(usize),
    Swap(usize, usize),
    Xor(usize, usize),
    Inc(usize),
}

impl Instruction {
    pub fn operation(&self) -> String {
        match self {
            Instruction::Load(_) => String::from("LOAD"),
            Instruction::Swap(_, _) => String::from("SWAP"),
            Instruction::Xor(_, _) => String::from("XOR"),
            Instruction::Inc(_) => String::from("INC"),
        }
    }

    pub fn arguments(&self) -> Vec<usize> {
        match self {
            Instruction::Load(value) => vec![*value],
            Instruction::Swap(memory1, memory2) => vec![*memory1, *memory2],
            Instruction::Xor(memory1, memory2) => vec![*memory1, *memory2],
            Instruction::Inc(memory) => vec![*memory],
        }
    }
}

#[derive(Debug, Clone)]
pub struct CPU {
    pub state: Vec<usize>,
}

impl CPU {
    pub fn new(max_allowed_memory_cells: usize) -> CPU {
        CPU {
            state: vec![0; max_allowed_memory_cells],
        }
    }

    pub fn reset(&mut self) {
        self.state = vec![0; self.state.len()];
    }

    pub fn execute(&mut self, program: &Vec<Instruction>) {
        for instruction in program {
            match *instruction {
                Instruction::Load(value) => load(&mut self.state, value),
                Instruction::Swap(memory1, memory2) => swap(&mut self.state, memory1, memory2),
                Instruction::Xor(memory1, memory2) => xor(&mut self.state, memory1, memory2),
                Instruction::Inc(memory) => inc(&mut self.state, memory),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_execute_program() {
        let program = vec![
            Instruction::Load(3),
            Instruction::Swap(0, 2),
            Instruction::Load(2),
            Instruction::Swap(0, 4),
            Instruction::Inc(4),
            Instruction::Xor(2, 4),
        ];
        let mut cpu = CPU::new(6);
        cpu.execute(&program);
        assert_eq!(cpu.state, vec![0, 0, 0, 0, 3, 0]);
    }
}
