use crate::operations::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Instruction {
    Load(i32),
    Swap(usize, usize),
    Xor(usize, usize),
    Inc(usize),
}
#[derive(Debug, Clone)]
pub struct CPU {
    pub state: Vec<i32>,
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
