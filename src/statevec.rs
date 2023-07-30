use nalgebra::*;
use rand::Rng;

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
    fn q_is_in_bounds(&self, q: usize) {
        if self.qubits.len() < q {
            panic!("Your qubit index is greater than the amount of qubits in your system.");
        }
    }

    fn c_is_in_bounds(&self, c: usize) {
        if self.cbits.len() < c {
            panic!("Your classical bit index is greater than the amount of classical bits in your system.");
        }
    }

    ///Copies any collapsed qubit to any classical register. Panics if a superposition is copied.
    pub fn copy_to_classical_bit(&self, q: usize, c: usize) -> StateVec {
        self.q_is_in_bounds(q);
        self.c_is_in_bounds(c);
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

    ///Collapses a single Qubit and returns a new Quantum State with the collapsed state.
    pub fn single_collapse(&self, q: usize) -> Self {
        self.q_is_in_bounds(q);
        let range: f64 = rand::thread_rng().gen_range(0.0..1.0);
        let mut return_vector = self.clone();
        if range < return_vector.qubits[q].x.re { //NOTE TO SELF: VALIDATE THIS FURTHER
            return_vector.qubits[q].x = Complex::new(1.0, 0.0);
            return_vector.qubits[q].y = Complex::new(0.0, 0.0);
        } else {
            return_vector.qubits[q].x = Complex::new(0.0, 0.0);
            return_vector.qubits[q].y = Complex::new(1.0, 0.0);
        }
        return_vector
        
    }

    pub fn norm(mut self, target_index: usize) -> Self {
        self.qubits[target_index] = self.qubits[target_index].map(|x| x.powf(2.0));
        self
    }
}

impl std::fmt::Display for StateVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(Quantum Bits: {:?}, Classical Bits: {:?})", self.qubits, self.cbits)
    }
}