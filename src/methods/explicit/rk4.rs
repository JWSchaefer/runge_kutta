use crate::{Butcher, Explicit};

pub struct Rk4 {}

const A: [[f64; 4]; 4] = [
    [0.0, 0.0, 0.0, 0.0],
    [0.5, 0.0, 0.0, 0.0],
    [0.0, 0.5, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
];

const B: [f64; 4] = [1.0 / 6.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 6.0];

const C: [f64; 4] = [0.0, 0.5, 0.5, 1.0];

impl Rk4 {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Butcher<f64, 4> for Rk4 {
    fn a(&self) -> &[[f64; 4]; 4] {
        &A
    }
    fn b(&self) -> &[f64; 4] {
        &B
    }
    fn c(&self) -> &[f64; 4] {
        &C
    }
}

impl Explicit<f64, 4> for Rk4 {}
