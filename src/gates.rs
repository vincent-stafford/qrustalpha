use std::f64::consts::{PI, FRAC_1_SQRT_2};
use crate::*;
use lazy_static::lazy_static;

pub mod single_qubit_gates;
pub mod multi_qubit_gates;
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
                                                                    Complex::new(0.0, 0.0), Complex::new((PI/4.0).cos(), (PI/4.0).sin()));

    ///S Gate rotates the S gate along the Z axis by 90 degrees
    pub static ref S_GATE: SingleGateMatrix = SingleGateMatrix::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), 
                                                                    Complex::new(0.0, 0.0), Complex::new(0.0, 1.0));
    
    ///Conjugate transpose of the S gate
    pub static ref S_DAGGER_GATE: SingleGateMatrix = SingleGateMatrix::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), 
                                                                    Complex::new(0.0, 0.0), Complex::new(0.0, -1.0));

    ///Pauli Z
    pub static ref PAULI_Z: SingleGateMatrix = SingleGateMatrix::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0));

    ///Hadamard Gate
    pub static ref HADAMARD_GATE: SingleGateMatrix = SingleGateMatrix::new(Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)) * Complex::new(FRAC_1_SQRT_2, 0.0);

}