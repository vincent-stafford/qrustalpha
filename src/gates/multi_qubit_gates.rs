use crate::BitType;
use crate::gates::*;
use nalgebra::Complex;
use lazy_static::lazy_static;

type MultiQubitMatrix<const N: usize> = SMatrix<Complex<f64>, N, N>;

lazy_static! {
    pub static ref SWAP: MultiQubitMatrix<4> = MultiQubitMatrix::<4>::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
                                                                        Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                                                                        Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
                                                                        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0));
}

pub struct MultiQuantumGate<const N: usize> {
    pub matrix_operation: MultiQubitMatrix<N>,
    pub operation_target: Vec<BitType>,
    pub qubit_indicies: Vec<BitType>
}

impl MultiQuantumGate<2> {

}

impl MultiQuantumGate<4> {
    pub fn swap(op_vector: Vec<BitType>) -> Self {
        Self {
            matrix_operation: *SWAP,
            operation_target: op_vector.clone(),
            qubit_indicies: op_vector,
        }
    }
}