use crate::{
    cpu::{Instruction, CPU},
    iters::product,
};

use strum::IntoEnumIterator;

pub fn generate_and_search_programs(
    max_instructions_length: usize,
    max_memory_cells: usize,
    max_value: usize,
    tester: impl Fn(&Vec<Instruction>) -> bool,
) -> Option<Vec<Instruction>> {
    let mut count = 0;

    // iterating over all possible program sizes
    for instructions_length in 1..=max_instructions_length {
        // unique operations
        let operations: Vec<String> = Instruction::iter()
            .map(|instruction| instruction.operation())
            .collect();

        let possible_instructions = operations
            .into_iter()
            .flat_map(|operation| match operation.as_str() {
                "LOAD" => (0..max_value)
                    .map(|value| Instruction::Load(value))
                    .collect::<Vec<_>>(),
                "SWAP" => product(&(0..max_memory_cells).collect::<Vec<_>>(), 2)
                    .iter()
                    .map(|cells| Instruction::Swap(cells[0], cells[1]))
                    .collect::<Vec<_>>(),
                "XOR" => product(&(0..max_memory_cells).collect::<Vec<_>>(), 2)
                    .iter()
                    .map(|cells| Instruction::Xor(cells[0], cells[1]))
                    .collect::<Vec<_>>(),
                "INC" => (0..max_memory_cells)
                    .map(|cell| Instruction::Inc(cell))
                    .collect::<Vec<_>>(),
                _ => panic!("Unknown operation: {}", operation),
            })
            .collect::<Vec<_>>();

        // iterating over all possible instruction combinations
        for instruction_combination in product(&possible_instructions, instructions_length) {
            if tester(&instruction_combination) {
                return Some(instruction_combination);
            }
            count += 1;

            if count % 100000 == 0 {
                println!("[SUPEROPTIMIZER] Programs generated: {}", count);
            }
        }
    }

    None
}

pub fn superoptimize(
    max_instructions_length: usize,
    max_memory_cells: usize,
    max_value: usize,
    target_state: &Vec<usize>,
) -> Option<Vec<Instruction>> {
    let tester = |program: &Vec<Instruction>| {
        let mut cpu = CPU::new(max_memory_cells);
        cpu.execute(program);
        let state = cpu.state.clone();

        // check if the state is deep equal to the target state
        let result = target_state
            .iter()
            .zip(state.iter())
            .all(|(target_value, state_value)| target_value == state_value);

        result
    };

    generate_and_search_programs(max_instructions_length, max_memory_cells, max_value, tester)
}
