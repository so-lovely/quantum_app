use rain_quantum_simulator::{QuantumCircuit, QuantumGate, QuantumRegister};

fn parse(program: &str) -> Result<QuantumCircuit<f64>, String>{
    let mut circuit = QuantumCircuit::new();
    for (line_num, line) in program.lines().enumerate() {
        let parts: Vec<&str> = 
        line.split_whitespace().collect();
        if parts.is_empty() { continue; }
        
        let gate_name = parts[0];
        let targets: Result<Vec<usize>, _> = parts[1..]
        .iter()
        .map(|s|s.parse::<usize>())
        .collect();

        let targets = match targets {
            Ok(t) => t,
            Err(_) => {
                return Err(format!("Error: line {} unexpected qubit num:", line_num+1));
            }
        };
        match gate_name.to_uppercase().as_str() {
            "H" | "X" | "Y" | "Z" | "S" | "T" => {
                if targets.len() != 1 {
                    return Err(format!("Err line {} {} gate requires exactly 1 target qubit. Received {}", line_num+1, gate_name, targets.len()));
                }
                let gate = match gate_name.to_uppercase().as_str() {
                "H" => QuantumGate::h(),
                "X" => QuantumGate::x(),
                "Y" => QuantumGate::y(),
                "Z" => QuantumGate::z(),
                "S" => QuantumGate::s(),
                "T" => QuantumGate::t(),
                _ => unreachable!(),
            };
            circuit.add_gate(&targets, gate);
            },
            "CNOT" => {
                if targets.len() != 2 {
                    return Err(format!{"Err Line {}, CNOT requires 2 target qubits. Recived {}", line_num + 1, targets.len()});
                }
                circuit.add_gate(&targets, QuantumGate::cnot());
            },
            _ => {
                println!("Warning! Line {}, Unknown gate {}.", line_num + 1, gate_name)
            },
    }
}

    Ok(circuit)
}

fn main() {
    let valid_program = "H 0\nCNOT 0 1";
    let invalid_qubit_program = "H 0\nX 1a";
    let wrong_arg_count_program = "H 0\nCNOT 1";
    println!("input:\n{}\n", valid_program);
    match parse(valid_program) {
        Ok(circuit) => {
        println!("parse Success! target num {}, ", circuit.len());
        let mut register = QuantumRegister::new(2);
        circuit.run(&mut register);
        }
        Err(e) => 
        {println!("parse Failed: {}", e); }
}
    println!("입력:\n{}\n", invalid_qubit_program);
    match parse(invalid_qubit_program) {
        Ok(_) => println!("Err: Parsing should not be succeed"),
        Err(e) => println!("parse Failed: {}", e),
    }

    println!("입력:\n{}\n", wrong_arg_count_program);
    match parse(wrong_arg_count_program) {
        Ok(_) => println!("Err: Parsing should not be succeed"),
        Err(e) => println!("parse Failed: {}", e),
    }
}