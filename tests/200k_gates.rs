use qrustalpha::BitType;
use qrustalpha::gates::SingleQuantumGate;
use qrustalpha::statevec::StateVec;
use qrustalpha::executor::execute_gate;

#[test]
fn two_hundred_thousand_paulix() {
    let mut initial_system = StateVec::init(200000, 1);
    let mut i: u32= 0;
    while i < initial_system.qubits.len() as u32 {
        initial_system = execute_gate(SingleQuantumGate::paulix(BitType::Quantum(i as usize)), initial_system);
        i += 1;
    }
}