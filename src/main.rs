use std::env;
use std::fs;
use rain_quantum_simulator::{QuantumCircuit, QuantumGate, QuantumRegister};

fn parse(program: &str) -> Result<(QuantumCircuit<f64>, usize, Option<Vec<usize>>), String> {
    let mut circuit = QuantumCircuit::new();
    let mut num_qubits: Option<usize> = None;
    let mut measure_targets: Option<Vec<usize>> = None;
    for (line_num, line) in program.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }

        let command = parts[0].to_uppercase();

        if command == "QUBITS" {
            if num_qubits.is_some() {
                return Err(format!("Error: line {}, QUBITS can only be declared once.", line_num + 1));
            }
            if parts.len() != 2 {
                return Err(format!("Error: line {}, QUBITS requires exactly one argument.", line_num + 1));
            }
            match parts[1].parse::<usize>() {
                Ok(n) => num_qubits = Some(n),
                Err(_) => return Err(format!("Error: line {}, invalid number for QUBITS.", line_num + 1)),
            }
            continue;
        }

        if num_qubits.is_none() {
            return Err(format!("Error: line {}, QUBITS must be declared before any gates.", line_num + 1));
        }

        let targets: Result<Vec<usize>, _> = parts[1..]
            .iter()
            .map(|s| s.parse::<usize>())
            .collect();

        let targets = match targets {
            Ok(t) => t,
            Err(_) => {
                return Err(format!("Error: line {} unexpected qubit num:", line_num + 1));
            }
        };

        match command.as_str() {
            "H" | "X" | "Y" | "Z" | "S" | "T" => {
                if targets.len() != 1 {
                    return Err(format!("Err line {} {} gate requires exactly 1 target qubit. Received {}", line_num + 1, command, targets.len()));
                }
                let gate = match command.as_str() {
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
                    return Err(format!("Err Line {}, CNOT requires 2 target qubits. Recived {}", line_num + 1, targets.len()));
                }
                circuit.add_gate(&targets, QuantumGate::cnot());
            },
            "MEASURE" => {
                if targets.is_empty() {
                    return Err(format!("Error: Line {} MEASURE requires at least one target qubit.", line_num+1));
                }
                if measure_targets.is_some() {
                    return Err(format!("Error: line {} MEASURE can only be performed only at the end.", line_num+1));
                }
                measure_targets = Some(targets.to_vec());
                break;
            },
            _ => {
                println!("Warning! Line {}, Unknown gate {}.", line_num + 1,command);
            }
        }
    }

    if let Some(n) = num_qubits {
        Ok((circuit, n, measure_targets))
    } else {
        Err("Error: QUBITS declaration not found.".to_string())
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    let program_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            return;
        }
    };

    println!("Running program from: {}\n", file_path);
    match parse(&program_content) {
        Ok((circuit, num_qubits, measure_targets)) => {
            println!("Parse Success! Circuit has {} steps for {} qubits.", circuit.len(), num_qubits);
            let mut register = QuantumRegister::new(num_qubits);
            circuit.run(&mut register);
            println!("Simulation finished. Final state vector:\n{:?}", register.state_vector());
            if let Some(targets) = measure_targets {
                println!("Performing measurement on qubits {:?}", targets);
                let outcome = register.measure();
                let mut results = Vec::new();
                for &target_qubit in &targets {
                    let bit = (outcome >> target_qubit) & 1;
                    results.push(bit);
                }
                println!("Measurement results (qubits {:?}): {:?}", targets, results);
            } else {
                println!("No measurement was performed. Final state vector:");
                println!("{:?}", register.state_vector());
            }
        }
        Err(e) => {
            eprintln!("Parse Failed: {}", e); 
        }
    }
}