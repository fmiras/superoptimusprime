use crate::cpu::CPU;
use crate::{cpu::Instruction, iters::product};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use strum::IntoEnumIterator;

pub fn generate_and_search_programs(
    max_instructions_length: usize,
    max_memory_cells: usize,
    max_value: usize,
    target_state: &Vec<usize>,
) -> Option<Vec<Instruction>> {
    let result = Arc::new(Mutex::new(None));
    let cvar = Arc::new(Condvar::new());

    let mut handles = vec![];

    for instructions_length in 1..=max_instructions_length {
        let operations = Instruction::iter()
            .map(|instruction| instruction.operation())
            .collect();

        let result = Arc::clone(&result);
        let cvar = Arc::clone(&cvar);

        let target_state = target_state.clone();

        let handle = thread::spawn(move || {
            for operation_combination in product(&operations, instructions_length) {
                let mut possible_instructions = Vec::new();

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

                for instruction_combination in product(&possible_instructions, instructions_length)
                {
                    let mut cpu = CPU::new(max_memory_cells);
                    cpu.execute(&instruction_combination);
                    let state = cpu.state.clone();

                    // check if the state is deep equal to the target state
                    let program_found = target_state
                        .iter()
                        .zip(state.iter())
                        .all(|(target_value, state_value)| target_value == state_value);

                    if program_found {
                        let mut result = result.lock().unwrap();
                        *result = Some(instruction_combination);

                        cvar.notify_all();
                        return;
                    }
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let result = result.lock().unwrap();

    result.clone()
}

pub fn superoptimize(
    max_instructions_length: usize,
    max_memory_cells: usize,
    max_value: usize,
    target_state: &Vec<usize>,
) -> Option<Vec<Instruction>> {
    generate_and_search_programs(
        max_instructions_length,
        max_memory_cells,
        max_value,
        target_state,
    )
}
