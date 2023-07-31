use crate::BitType;
use crate::gates::*;

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
            toffoli: (BitType::None, BitType::None)
        }
    }

    ///This creates a standard CNOT/CX gate. The argument takes two arguments. The first one is the index of the control bit. The second is the target Qubit.
    pub fn cnot(cond_bit: BitType, target: BitType) -> Self {
        Self {
            matrix_operation: *PAULI_X,
            operation_target: target,
            conditional: cond_bit,
            toffoli: (BitType::None, BitType::None)
        }
    }

    pub fn toffoli(toffoli_index_1: BitType, toffoli_index_2: BitType, target: BitType) -> Self {
        Self {
            matrix_operation: *PAULI_X,
            operation_target: target,
            conditional: BitType::None,
            toffoli: (toffoli_index_1, toffoli_index_2)
        }
    }

    pub fn t_gate(target: BitType) -> Self {
        Self {
            matrix_operation: *T_GATE, 
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None)
        }
    }

    pub fn s_gate(target: BitType) -> Self {
        Self {
            matrix_operation: *S_GATE, 
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None)
        }
    }

    pub fn inv_s_gate(target: BitType) -> Self {
        Self {
            matrix_operation: *S_DAGGER_GATE, 
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None)
        }
    }

    ///This is a constructor for a Single Qubit Quantum gate. Arg1: SingleGateMatrix, Arg2: The conditional qubit (BitType) [Optional], Arg3: toffoli_index_1: BitType, Arg4: toffoli_index_2: BitType
    pub fn new(matrix: SingleGateMatrix, cond_bit: BitType, target: BitType, toffoli_index_1: BitType, toffoli_index_2: BitType) -> Self {
        Self {
            matrix_operation: matrix,
            operation_target: target,
            conditional: cond_bit,
            toffoli: (toffoli_index_1, toffoli_index_2)
        }
    }

}