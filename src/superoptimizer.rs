use strum::IntoEnumIterator;

use crate::{cpu::Instruction, iters::product};

pub fn generate_and_search_programs(
    max_instructions_length: usize,
    max_memory_cells: usize,
    max_value: usize,
    mut tester: impl FnMut(&Vec<Instruction>) -> bool,
) -> Option<Vec<Instruction>> {
    let mut count = 0;

    // iterating over all possible program sizes
    for instructions_length in 1..=max_instructions_length {
        // unique operations
        let operations = Instruction::iter()
            .map(|instruction| instruction.operation())
            .collect();

        // iterating over all possible combinations of operations
        for operation_combination in product(&operations, instructions_length) {
            let mut possible_instructions = Vec::new();
            // iterating over all possible arguments for each operation
            for operation in operation_combination {
                match operation.as_str() {
                    "LOAD" => {
                        for value in 0..max_value {
                            possible_instructions.push(Instruction::Load(value));
                        }
                    }
                    "SWAP" => {
                        for cell1 in 0..max_memory_cells {
                            for cell2 in 0..max_memory_cells {
                                possible_instructions.push(Instruction::Swap(cell1, cell2));
                            }
                        }
                    }
                    "XOR" => {
                        for cell in 0..max_memory_cells {
                            for cell2 in 0..max_memory_cells {
                                possible_instructions.push(Instruction::Xor(cell, cell2));
                            }
                        }
                    }
                    "INC" => {
                        for cell in 0..max_memory_cells {
                            possible_instructions.push(Instruction::Inc(cell));
                        }
                    }
                    _ => panic!("Unknown operation: {}", operation),
                }
            }

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
    }

    None
}

pub fn superoptimize(
    max_instructions_length: usize,
    max_memory_cells: usize,
    max_value: usize,
    target_state: &Vec<usize>,
) -> Option<Vec<Instruction>> {
    let mut cpu = crate::cpu::CPU::new(max_memory_cells);

    let tester = |program: &Vec<Instruction>| {
        cpu.execute(program);
        let state = cpu.state.clone();

        // check if the state is deep equal to the target state
        let result = target_state
            .iter()
            .zip(state.iter())
            .all(|(target_value, state_value)| target_value == state_value);

        cpu.reset();
        result
    };

    generate_and_search_programs(max_instructions_length, max_memory_cells, max_value, tester)
}
