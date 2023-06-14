use std::sync::Arc;
use strum::IntoEnumIterator;
use tokio::sync::mpsc;
use tokio::task;

use crate::cpu::CPU;
use crate::{cpu::Instruction, iters::product};

pub async fn generate_and_search_programs(
    max_instructions_length: usize,
    max_memory_cells: usize,
    max_value: usize,
    target_state: &Vec<usize>,
) -> Option<Vec<Instruction>> {
    let (sender, mut receiver) = mpsc::channel(1);
    let target_state = Arc::new(target_state.clone());

    for instructions_length in 1..=max_instructions_length {
        let operations: Vec<String> = Instruction::iter()
            .map(|instruction: Instruction| instruction.operation())
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

        let sender = sender.clone();
        let target_state = Arc::clone(&target_state);

        task::spawn(async move {
            for instruction_combination in product(&possible_instructions, instructions_length) {
                let mut cpu = CPU::new(max_memory_cells);
                cpu.execute(&instruction_combination);
                let state = cpu.state.clone();

                // check if the state is deep equal to the target state
                let program_found = target_state
                    .iter()
                    .zip(state.iter())
                    .all(|(target_value, state_value)| target_value == state_value);

                if program_found {
                    sender.send(instruction_combination).await.unwrap();
                    return;
                }
            }
        });
    }

    receiver.recv().await
}

pub async fn superoptimize(
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
    .await
}
