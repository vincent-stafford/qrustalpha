# Simple Quantum Simulator

Hello, and welcome to my new and on-going major Rust project. My goal is to make a simple quantum circuit simulator in Rust.
The reason for doing so is to help me grasp the fundamental concepts behind the mathematics of Quantum Gates and Algorithms.
This project is in extremly early alpha and is not ready for production whatsoever. 

Features implemented so far:
- Basic Gates (Pauli X, Pauli Y, Pauli Z, Hadamard, CNOT, Toffoli, and phase shift T + S gates)
- Quantum Circuits containing any number and combination of classical and quantum registers
- Measuring and gate execution circuits
- Basic Circuit Sanity testing
- Benchmarking large qubit systems
- Constructor for making custom gates (untested and without validation)
- Relative phase (untested)

This project heavily relies on the types provided by nalgebra. This also means that you can act on individual qubits with linear algebra methods for extended functionality.
Here are some planned features:
- Implementation of Quantum Oracles (The secret sauce in Deutsch's algorithm)
- Gates that act on multiple qubits
- More single qubit quantum gates
- Example files providing extensive examples of the library and basic quantum algorithms,


Example Code for a circuit that first uses a Pauli X gate on Qubit 0 and then runs a CNOT gate on Qubit 1 using Qubit 0 as a control:
```rust
use qrustalpha::*;

fn main() {
    //Make A System
    let my_state = StateVec::init(10, 2);
    //Operations
    let state_iter_1 = execute_gate(gates::SingleQuantumGate::paulix(
        BitType::Quantum(0)), 
        my_state);
    println!("{}\n", state_iter_1);
    
    let state_iter_2 = execute_gate(gates::SingleQuantumGate::cnot(BitType::Quantum(0), BitType::Quantum(1)), state_iter_1);
    println!("{}", state_iter_2);
}
```

Issues:
- Phase shifts now work nearly as intended. It is still not fully tested and there may be bugs. 

This project is also entirely for fun and research purposes. It is licensed under the GPL.
I started this project on Monday 07/24/2023.