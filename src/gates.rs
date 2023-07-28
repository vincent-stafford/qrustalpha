use std::f64::consts::{PI, E, FRAC_1_SQRT_2};
use crate::*;
use lazy_static::lazy_static;

///This is a public type declaration used for any single-qubit gate.
pub type SingleGateMatrix = SMatrix<Complex<f64>, 2, 2>;


lazy_static! {
    ///The Pauli X gate represents a 180-degree rotation along the X axis on a Bloch Sphere. With |0> and |1> a basis states, starting from |0> and applying a Pauli-X gate would indicate a collapse to |1> with 100% probability.
    pub static ref PAULI_X: SingleGateMatrix = SingleGateMatrix::new(Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
                                                                     Complex::new(1.0, 0.0), Complex::new(0.0, 0.0));
    ///Pauli Y
    pub static ref PAULI_Y: SingleGateMatrix = SingleGateMatrix::new(Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
                                                                     Complex::new(0.0, 1.0), Complex::new(0.0, 0.0));                                            
    ///The T phase shift gate rotates the relative phase of a state vector by PI / 4 radians. 
    pub static ref T_GATE: SingleGateMatrix = SingleGateMatrix::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), 
                                                                    Complex::new(0.0, 0.0), Complex::new(0.0, E.powf(PI/4.0)));
    ///Pauli Z
    pub static ref PAULI_Z: SingleGateMatrix = SingleGateMatrix::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0));

    ///Hadamard Gate
    pub static ref HADAMARD_GATE: SingleGateMatrix = SingleGateMatrix::new(Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)) * Complex::new(FRAC_1_SQRT_2, 0.0);

}



pub struct SingleQuantumGate {
    pub matrix_operation: SingleGateMatrix,
    pub operation_target: BitType, //Mandatory
    pub conditional: BitType, //Optional
    pub toffoli: (BitType, BitType) //Optional
}

impl SingleQuantumGate {
    ///This creates a PauliX gate. The argument takes the BitType enum and represents the index of the target qubit.
    pub fn paulix(target: BitType) -> Self {
        Self {
            matrix_operation: *PAULI_X,
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None),
        }
    }

    ///This creates a PauliY gate. The argument takes the BitType enum and represents the index of the target qubit.
    pub fn pauliy(target: BitType) -> Self {
        Self {
            matrix_operation: *PAULI_Y,
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None),
        }
    }

    ///This creates a PauliZ gate. The argument takes the BitType enum and represents the index of the target qubit.
    pub fn pauliz(target: BitType) -> Self {
        Self {
            matrix_operation: *PAULI_Z,
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None),
        }
    }

    
    ///This creates a Hadamard gate. The argument takes the BitType enum and represents the index of the target qubit.
    pub fn hadamard(target: BitType) -> Self { /*!!TODO!! FIX THE HADAMARD GATE THE PROBABILITIES ARE NOT RIGHT!!!!!*/
        Self {
            matrix_operation: *HADAMARD_GATE,
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None),
        }
    }

    ///This creates a standard CNOT/CX gate. The argument takes two arguments. The first one is the index of the control bit. The second is the target Qubit.
    pub fn cnot(cond_bit: BitType, target: BitType) -> Self {
        Self {
            matrix_operation: *PAULI_X,
            operation_target: target,
            conditional: cond_bit,
            toffoli: (BitType::None, BitType::None),
        }
    }

    pub fn t_gate(target: BitType) -> Self {
        Self {
            matrix_operation: *T_GATE, 
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None),
        }
    }

    ///This is a constructor for a Single Qubit Quantum gate. Arg1: SingleGateMatrix, Arg2: The conditional qubit (BitType) [Optional], Arg3: toffoli_index_1: BitType, Arg4: toffoli_index_2: BitType
    pub fn new(matrix: SingleGateMatrix, cond_bit: BitType, target: BitType, toffoli_index_1: BitType, toffoli_index_2: BitType) -> Self {
        Self {
            matrix_operation: matrix,
            operation_target: target,
            conditional: cond_bit,
            toffoli: (toffoli_index_1, toffoli_index_2),
        }
    }

}