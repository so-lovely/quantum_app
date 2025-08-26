use rain_quantum_simulator::{QuantumGate, QuantumRegister};
use complex_rs::Complex;
use num_traits::Float;


fn print_state_vector<T: Float + std::fmt::Display>(reg: &QuantumRegister<T>) {
    print!("[");
    for (i, amp) in 
    reg.state_vector().elements().iter().enumerate() {
        if i > 0 { print!(", "); }
        if amp.magnitude_squared() > T::from(1e-10).unwrap() {
            print!("{}", amp)
        } else {
            print!("0")
        }
    }
    println!("]");
}

fn main() {
    println!("---Quantum Transportation---");
    let mut register = QuantumRegister::<f64>::new(3);
    let theta = std::f64::consts::PI / 3.0;
    let ry_gate = QuantumGate::new(
        "Ry(pi/3)".to_string(),
        rain_linalg::Matrix::new(2,2, vec![
            Complex::new((theta/2.0).cos(), 0.0),
            Complex::new(-(theta/2.0).sin(), 0.0),
            Complex::new((theta/2.0).sin(), 0.0),
            Complex::new((theta/2.0).cos(), 0.0),
        ])
    );
    register.apply_gate(&[0], &ry_gate);
    println!("\n1. 초기상태 (||ψ>|00>):");
    print_state_vector(&register);

    // Generate entanlgment (q1, q2)
    register.apply_gate(&[1], &QuantumGate::h());
    register.apply_gate(&[1,2], &QuantumGate::cnot());
    println!("\n2. After entanglement:");
    print_state_vector(&register);
    // Preparing measure bell state
    // Applying CNOT(control=q0, target=q1)
    register.apply_gate(&[0,1], &QuantumGate::cnot());
    // Applying H Gate
    register.apply_gate(&[0], &QuantumGate::h());

    // Validation
    println!("---Validation---");
    println!("Check Bob's q2 after Alice measure");
    let final_state = register.state_vector().elements();
    let c0 = (theta/2.0).cos();
    let c1 = (theta/2.0).sin();
    println!("앨리스가 '00' 측정 -> 밥의 상태: [{}, {}] (원래 |ψ> 상태)", final_state[0], final_state[1]);
    println!("앨리스가 '01' 측정 -> 밥의 상태: [{}, {}] (X 게이트 적용된 상태)", final_state[2], final_state[3]);
    println!("앨리스가 '10' 측정 -> 밥의 상태: [{}, {}] (Z 게이트 적용된 상태)", final_state[4], final_state[5]);
    println!("앨리스가 '11' 측정 -> 밥의 상태: [{}, {}] (Z, X 게이트 적용된 상태)", final_state[6], final_state[7]);
}

