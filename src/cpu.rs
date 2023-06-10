pub const MAX_DISTINCT_OPERATIONS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Operation {
    Load(usize),
    Swap(usize, usize),
    Xor(usize, usize),
    Inc(usize),
}

pub fn get_operation(index: usize) -> Operation {
    match index {
        0 => Operation::Load(0),
        1 => Operation::Swap(0, 0),
        2 => Operation::Xor(0, 0),
        3 => Operation::Inc(0),
        _ => panic!("Invalid operation index"),
    }
}

#[derive(Debug, Clone)]
pub struct CPU {
    max_allowed_memory_cells: usize,
    pub state: Vec<i32>,
}

impl CPU {
    pub fn new(max_allowed_memory_cells: usize) -> CPU {
        CPU {
            max_allowed_memory_cells,
            state: vec![0; max_allowed_memory_cells],
        }
    }

    pub fn execute(&mut self, program: &Vec<Operation>) {
        for operation in program {
            match *operation {
                Operation::Load(val) => self.load(val),
                Operation::Swap(mem1, mem2) => self.swap(mem1, mem2),
                Operation::Xor(mem1, mem2) => self.xor(mem1, mem2),
                Operation::Inc(mem) => self.inc(mem),
            }
        }
    }

    fn load(&mut self, val: usize) {
        self.state[0] = val as i32;
    }

    fn swap(&mut self, mem1: usize, mem2: usize) {
        let temp = self.state[mem1];
        self.state[mem1] = self.state[mem2];
        self.state[mem2] = temp;
    }

    fn xor(&mut self, mem1: usize, mem2: usize) {
        self.state[mem1] ^= self.state[mem2];
    }

    fn inc(&mut self, mem: usize) {
        self.state[mem] += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load() {
        let mut cpu = CPU::new(4);
        cpu.load(1);
        assert_eq!(cpu.state, vec![1]);
    }

    #[test]
    fn swap() {
        let mut cpu = CPU::new(2);
        cpu.state = vec![1, 2];
        cpu.swap(0, 1);
        assert_eq!(cpu.state, vec![2, 1]);
    }

    #[test]
    fn xor() {
        let mut cpu = CPU::new(4);
        cpu.state = vec![0, 1];
        cpu.xor(0, 1);
        assert_eq!(cpu.state, vec![1, 0]);
    }

    #[test]
    fn inc() {
        let mut cpu = CPU::new(4);
        cpu.state = vec![0, 1, 0, 0];
        cpu.inc(1);
        assert_eq!(cpu.state, vec![2]);
    }
}
