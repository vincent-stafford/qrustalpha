use nalgebra::*;
use rand::Rng;
use std::f64::consts::FRAC_1_SQRT_2;

mod gates;

///This is a public type declaration used for any single-qubit gate.
pub type SingleGateMatrix = SMatrix<Complex<f64>, 2, 2>;
///The Pauli X gate represents a 180-degree rotation along the X axis on a Bloch Sphere. With |0> and |1> a basis states, starting from |0> and applying a Pauli-X gate would indicate a collapse to |1> with 100% probability.
pub const PAULI_X: SingleGateMatrix = SingleGateMatrix::new(Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
                                                            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0));
///Pauli Y
pub const PAULI_Y: SingleGateMatrix = SingleGateMatrix::new(Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
                                                            Complex::new(0.0, 1.0), Complex::new(0.0, 0.0));
///Pauli Z
pub const PAULI_Z: SingleGateMatrix = SingleGateMatrix::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0));
//Hadamard Gate
pub const HADAMARD_GATE: SingleGateMatrix = SingleGateMatrix::new(Complex::new(1.0 * FRAC_1_SQRT_2, 0.0), Complex::new(1.0 * FRAC_1_SQRT_2, 0.0), 
                                                                  Complex::new(1.0 * FRAC_1_SQRT_2, 0.0), Complex::new(-1.0 * FRAC_1_SQRT_2, 0.0));

#[derive(PartialEq, Copy, Clone)]
pub enum BitType {
    Classical(usize),
    Quantum(usize),
    None,
}

#[derive(Clone, Debug)]
pub struct StateVec {
    pub qubits: Vec<Vector2<Complex<f64>>>,
    pub cbits: Vec<i8>
}

impl StateVec {
    ///Returns a System initialized with all zeros of a specified length. 
    pub fn init(q: usize, c: usize) -> StateVec {
        StateVec {qubits: vec![Vector2::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)); q], cbits: vec![0; c]}
    }

    /* TODO */
    //Make function that sanity checks Q and C

    ///Copies any collapsed qubit to any classical register. Panics if a superposition is copied.
    pub fn copy_to_classical_bit(&self, q: usize, c: usize) -> StateVec {
        if self.qubits[q].x.re != 0.0 && self.qubits[q].x.re != 1.0 || self.qubits[q].y.re != 0.0 && self.qubits[q].y.re != 1.0 {
            panic!("You can not copy a Qubit in superposition to a classical register.");
        }
        let mut return_vector = self.clone();
        if return_vector.qubits[q].x.re == 1.0 {
            return_vector.cbits[c] = 0;
        } else {
            return_vector.cbits[c] = 1;
        }
        return_vector
    }
    

    //Collapses a single Qubit and returns a new Quantum State with the collapsed state.
    pub fn single_collapse(&self, q: usize) -> StateVec {
        let range: f64 = rand::thread_rng().gen_range(0.0..1.0);
        let mut return_vector = self.clone();
        if range > return_vector.qubits[q].x.norm_sqr() {
            return_vector.qubits[q].x = Complex::new(1.0, 0.0);
            return_vector.qubits[q].y = Complex::new(0.0, 0.0);
        } else {
            return_vector.qubits[q].x = Complex::new(0.0, 0.0);
            return_vector.qubits[q].y = Complex::new(1.0, 0.0);
        }
        return_vector
        
    }
}

impl std::fmt::Display for StateVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(Quantum Bits: {:?}, Classical Bits: {:?})", self.qubits, self.cbits)
    }
}

fn execute_conditonal_toffoli(gate: &gates::SingleQuantumGate, system: StateVec, toffoli_index_1: usize, toffoli_index_2: usize, target_index: usize, cond_bit_index: usize) -> StateVec {
    let mut return_vec: StateVec = system.clone(); //I think I need to do the euclidian norm here somehwere. 
    if system.single_collapse(toffoli_index_1).qubits[toffoli_index_1].y.re == 1.0 && system.single_collapse(toffoli_index_2).qubits[toffoli_index_2].y.re == 1.0 && system.cbits[cond_bit_index] == 1 {
        return_vec.qubits[target_index] = gate.matrix_operation * return_vec.qubits[target_index];
    }
    return_vec
}

fn execute_toffoli(gate: &gates::SingleQuantumGate, system: StateVec, toffoli_index_1: usize, toffoli_index_2: usize, target_index: usize) -> StateVec {
    let mut return_vec: StateVec = system.clone(); //I think I need to do the euclidian norm here somehwere. 
    if system.single_collapse(toffoli_index_1).qubits[toffoli_index_1].y.re == 1.0 && system.single_collapse(toffoli_index_2).qubits[toffoli_index_2].y.re == 1.0 {
        return_vec.qubits[target_index] = gate.matrix_operation * return_vec.qubits[target_index];
    }
    return_vec
}

fn execute_classical_cnot(gate: &gates::SingleQuantumGate, system: StateVec, target_index: usize, cond_bit_index: usize) -> StateVec {
    let mut return_vec: StateVec = system.clone(); //I think I need to do the euclidian norm here somehwere. 
    if system.cbits[cond_bit_index] == 1 {
        return_vec.qubits[target_index] = gate.matrix_operation * return_vec.qubits[target_index];
    }
    return_vec
}

fn execute_quantum_cnot(gate: &gates::SingleQuantumGate, system: StateVec, target_index: usize, cond_bit_index: usize) -> StateVec {
    let mut return_vec: StateVec = system.clone(); //I think I need to do the euclidian norm here somehwere. 
    if system.single_collapse(cond_bit_index).qubits[cond_bit_index].y.re == 1.0 {
        return_vec.qubits[target_index] = gate.matrix_operation * return_vec.qubits[target_index];
    }
    return_vec
}

fn execute_single_qubit_gate(gate: &gates::SingleQuantumGate, system: StateVec, target_index: usize) -> StateVec {
    let mut return_vec: StateVec = system.clone(); //I think I need to do the euclidian norm here somehwere. 
    return_vec.qubits[target_index] = gate.matrix_operation * return_vec.qubits[target_index];
    return_vec
}

pub fn execute_gate(gate: gates::SingleQuantumGate, system: StateVec) -> StateVec {
    let return_vec: StateVec;
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

#[cfg(test)]
mod tests {

}
