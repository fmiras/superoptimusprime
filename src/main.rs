mod cpu;
mod iters;
mod operations;
mod parser;
mod superoptimizer_rayon;

fn main() {
    let assembly = "LOAD 3
SWAP 0, 1
LOAD 3
SWAP 0, 2
LOAD 3
SWAP 0, 3
LOAD 3
";

    println!("🤖 Assembly program:");
    println!("{}", assembly);

    let max_memory_cells = 4;

    let program = parser::parse(assembly).unwrap();
    let mut cpu = cpu::CPU::new(max_memory_cells);
    cpu.execute(&program);
    let target_state = cpu.state.clone();

    println!("🎯 Target state: {:?}", target_state);

    // measure execution duration
    let start = std::time::Instant::now();
    let superoptimized_program =
        superoptimizer_rayon::superoptimize(4, max_memory_cells, 5, &target_state);
    let end = std::time::Instant::now();

    println!("⏱️ Execution duration: {:?}", end - start);

    if let Some(superoptimized_program) = superoptimized_program {
        println!("🤖 Superoptimized program:");
        println!("{}", parser::output(&superoptimized_program));
        std::process::exit(0);
    }

    println!("🤖 No superoptimized program found");
}
