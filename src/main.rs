mod cpu;
mod parser;
mod superoptimizer;

fn main() {
    // Test 1
    let assembly = "
LOAD 3
SWAP 0, 1
LOAD 3
SWAP 0, 2
LOAD 3
SWAP 0, 3
LOAD 3
    ";

    superoptimizer::optimal_from_code(assembly, 4, 4, 5);

    // Test 2
    // let state = vec![0, 2, 1];
    // superoptimizer::optimal_from_state(&state, 3, 5);
}
