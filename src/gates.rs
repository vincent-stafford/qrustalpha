use crate::*;
pub struct SingleQuantumGate {
    pub matrix_operation: SingleGateMatrix,
    pub operation_target: BitType, //Mandatory
    pub conditional: BitType, //Optional
    pub toffoli: (BitType, BitType) //Optional
}

impl SingleQuantumGate {
    pub fn paulix(target: BitType) -> Self {
        Self {
            matrix_operation: PAULI_X,
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None),
        }
    }

    pub fn pauliy(target: BitType) -> Self {
        Self {
            matrix_operation: PAULI_Y,
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None),
        }
    }

    pub fn pauliz(target: BitType) -> Self {
        Self {
            matrix_operation: PAULI_Z,
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None),
        }
    }

    pub fn hadamard(target: BitType) -> Self {
        Self {
            matrix_operation: HADAMARD_GATE,
            operation_target: target,
            conditional: BitType::None,
            toffoli: (BitType::None, BitType::None),
        }
    }

    pub fn cnot(cond_bit: BitType, target: BitType) -> Self {
        Self {
            matrix_operation: PAULI_X,
            operation_target: target,
            conditional: cond_bit,
            toffoli: (BitType::None, BitType::None),
        }
    }

    pub fn new(matrix: SingleGateMatrix, cond_bit: BitType, target: BitType, toffoli_index_1: BitType, toffoli_index_2: BitType) -> Self {
        Self {
            matrix_operation: matrix,
            operation_target: target,
            conditional: cond_bit,
            toffoli: (toffoli_index_1, toffoli_index_2),
        }
    }

}