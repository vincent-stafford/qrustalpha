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
    qubit_count: usize,
}