use nalgebra::*;
pub mod gates;
pub mod statevec;
pub mod executor;

#[derive(PartialEq, Copy, Clone)]
pub enum BitType {
    Classical(usize),
    Quantum(usize),
    None,
}

#[cfg(test)]
mod tests {
    use crate::{BitType, statevec::StateVec, executor::execute_gate, gates::*};
    #[test]
    fn create_quantum_system_1q_1c() {
        StateVec::init(1, 1);
    }

    #[test]
    fn create_quantum_system_100q_100c() {
        StateVec::init(100, 100);
    }

    #[test]
    fn create_quantum_system_100000q_100000c() {
        StateVec::init(100000, 100000);
    }

    #[test]
    #[should_panic]
    fn check_classical_bit_bounds() {
        let test_system1 = StateVec::init(1, 1);
        let test_system2 = execute_gate(SingleQuantumGate::paulix(BitType::Quantum(0)), test_system1);
        test_system2.copy_to_classical_bit(0, 2);
    }

    #[test]
    #[should_panic]
    fn check_qubit_bounds() {
        let test_system1 = StateVec::init(1, 1);
        let test_system2 = execute_gate(SingleQuantumGate::paulix(BitType::Quantum(0)), test_system1);
        test_system2.single_collapse(2);
    }

    #[test]
    fn run_pauli_x() {
        let test_system1 = StateVec::init(1, 1);
        let test_system2 = execute_gate(SingleQuantumGate::paulix(BitType::Quantum(0)), test_system1);
        assert_eq!(1.0, test_system2.qubits[0].y.re);
    }

    #[test]
    fn run_cnot() {
        let test_system1 = StateVec::init(2, 1);
        let test_system2 = execute_gate(SingleQuantumGate::paulix(BitType::Quantum(0)), test_system1);  
        let test_system3 = execute_gate(SingleQuantumGate::cnot(BitType::Quantum(0), BitType::Quantum(1)), test_system2);
        assert_eq!(1.0, test_system3.qubits[0].y.re);
    }
    #[test]
    fn run_toffoli() {
        let test_system1 = StateVec::init(3, 1);
        let test_system2 = execute_gate(SingleQuantumGate::paulix(BitType::Quantum(0)), test_system1); 
        let test_system3 = execute_gate(SingleQuantumGate::paulix(BitType::Quantum(1)), test_system2); 
        let test_system4 = execute_gate(SingleQuantumGate::toffoli(BitType::Quantum(0), BitType::Quantum(1), BitType::Quantum(2)), test_system3);
        assert_eq!(1.0, test_system4.qubits[2].y.re);
    }

    #[test]
    fn half_adder() {
        let test_system1 = StateVec::init(4, 1);
        let test_system2 = execute_gate(SingleQuantumGate::paulix(BitType::Quantum(0)), test_system1); 
        let test_system3 = execute_gate(SingleQuantumGate::paulix(BitType::Quantum(1)), test_system2); 
        let test_system4 = execute_gate(SingleQuantumGate::toffoli(BitType::Quantum(0), BitType::Quantum(1), BitType::Quantum(3)), test_system3);
        assert_eq!(1.0, test_system4.qubits[3].y.re);
        let test_system5 = execute_gate(SingleQuantumGate::cnot(BitType::Quantum(0), BitType::Quantum(1)), test_system4);
        assert_eq!(0.0, test_system5.qubits[1].y.re);
    }

}
