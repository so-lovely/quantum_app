use rain_quantum_simulator::{QuantumCircuit, QuantumGate, QuantumRegister};

fn parse(program: &str) -> QuantumCircuit <f64> {
    let mut circuit = QuantumCircuit::new();
    for line in program.lines() {
        let parts: Vec<&str> = 
        line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        println!("Status: {:?}", parts);
    }

    circuit
}

fn main() {
    let quil_program = "H 0\nCNOT 0 1";
    println!("input:\n{}\n", quil_program);
    let circuit = parse(quil_program);
    println!("parsed!");
}