use crate::*;

//NOTE TO SELF: VALIDATE return_vec.qubits[target_index] = return_vec.qubits[target_index].map(|x| x.powf(2.0)) ASAP
fn execute_conditonal_toffoli(gate: &gates::SingleQuantumGate, mut system: statevec::StateVec, toffoli_index_1: usize, toffoli_index_2: usize, target_index: usize, cond_bit_index: usize) -> statevec::StateVec {
    // let mut return_vec: statevec::StateVec = system.clone(); //I think I need to do the euclidian norm here somehwere. 
    system.single_collapse(toffoli_index_1).single_collapse(toffoli_index_2);
    if system.qubits[toffoli_index_1].y.re == 1.0 && system.qubits[toffoli_index_2].y.re == 1.0 && system.cbits[cond_bit_index] == 1 {
        system.qubits[target_index] = gate.matrix_operation * system.qubits[target_index];
    }
    system
}

fn execute_toffoli(gate: &gates::SingleQuantumGate, mut system: statevec::StateVec, toffoli_index_1: usize, toffoli_index_2: usize, target_index: usize) -> statevec::StateVec {
    system.single_collapse(toffoli_index_1).single_collapse(toffoli_index_2);
    if system.qubits[toffoli_index_1].y.re == 1.0 && system.qubits[toffoli_index_2].y.re == 1.0 {
        system.qubits[target_index] = gate.matrix_operation * system.qubits[target_index];
    }
    system
}

fn execute_classical_cnot(gate: &gates::SingleQuantumGate, mut system: statevec::StateVec, target_index: usize, cond_bit_index: usize) -> statevec::StateVec {
    if system.cbits[cond_bit_index] == 1 {
        system.qubits[target_index] = gate.matrix_operation * system.qubits[target_index];
    }
    system
}

fn execute_quantum_cnot(gate: &gates::SingleQuantumGate, mut system: statevec::StateVec, target_index: usize, cond_bit_index: usize) -> statevec::StateVec {
    if system.single_collapse(cond_bit_index).qubits[cond_bit_index].y.re == 1.0 {
        system.qubits[target_index] = gate.matrix_operation * system.qubits[target_index];
    }
    system
}

fn execute_single_qubit_gate(gate: &gates::SingleQuantumGate, mut system: statevec::StateVec, target_index: usize) -> statevec::StateVec {
    system.qubits[target_index] = gate.matrix_operation * system.qubits[target_index];
    system
}

pub fn execute_gate(gate: gates::SingleQuantumGate, system: statevec::StateVec) -> statevec::StateVec {
    let return_vec: statevec::StateVec;
    //Step 1: Sanity Checking and forking.
    if gate.operation_target == BitType::None {
        panic!("All gates require a target Qubit.");
    }
    let target_index: usize;
    match gate.operation_target {
        BitType::Classical(_) => panic!("Only Qubits are supported."),
        BitType::None => panic!("You have to specify a target."),
        BitType::Quantum(x) if x > system.qubits.len() => panic!("The target has to be within the bounds of your state vector."),
        BitType::Quantum(x) => target_index = x
    }

    //World's most convoluted pattern matching block!!!! You've seen it here first folks.
    match (gate.toffoli, gate.conditional) {
        ((BitType::Quantum(x), BitType::Quantum(y)), BitType::None) if x <= system.qubits.len() && y <= system.qubits.len() && y != x => return_vec = execute_toffoli(&gate, system, x, y, target_index),
        ((BitType::Classical(_), BitType::Classical(_)), _) => panic!("Only Quantum Bits are supported in toffoli gates this time."),
        ((BitType::Quantum(x), BitType::Quantum(y)), BitType::Classical(z)) if x <= system.qubits.len() && y <= system.qubits.len() && y != x && z <= system.cbits.len() => return_vec = execute_conditonal_toffoli(&gate, system, x, y, target_index, z),
        ((BitType::None, BitType::None), BitType::Classical(x)) if x <= system.cbits.len() => return_vec = execute_classical_cnot(&gate, system, target_index, x),
        ((BitType::None, BitType::None), BitType::Quantum(y)) if y <= system.cbits.len() => return_vec = execute_quantum_cnot(&gate, system, target_index, y),
        ((BitType::None, BitType::None), BitType::None) => return_vec = execute_single_qubit_gate(&gate, system, target_index),
        _ => panic!("There has been an unspecified error while sanity-checking your gate.")
    }
    return_vec
}
