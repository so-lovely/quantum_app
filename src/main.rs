use rain_quantum_simulator::{QuantumCircuit, QuantumGate, QuantumRegister};

fn parse(program: &str) -> QuantumCircuit<f64> {
    let mut circuit = QuantumCircuit::new();
    for line in program.lines() {
        let parts: Vec<&str> = 
        line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        let gate_name = parts[0];
        let targets: Vec<usize> = parts[1..].iter().map(|s|s.parse().unwrap()).collect();

        match gate_name.to_uppercase().as_str() {
            "H" => circuit.add_gate(&targets, QuantumGate::h()),
            "X" => circuit.add_gate(&targets, QuantumGate::x()),
            "CNOT" => circuit.add_gate(&targets, QuantumGate::cnot()),
            _ => {
                println!("Warning: unknown gate:{}",gate_name);
            }
        }
    }

    circuit
}

fn main() {
    let quil_program = "H 0\nCNOT 0 1";
    println!("input:\n{}\n", quil_program);
    let circuit = parse(quil_program);
    println!("parsed! len {}", circuit.len());
}