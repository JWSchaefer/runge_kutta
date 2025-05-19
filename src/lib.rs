mod butcher;
mod errors;
mod runge_kutta;
pub mod methods {
    pub mod explicit;
}
mod solution;
pub use butcher::{Adaptive, Butcher, Explicit, Implicit, Nystrom};

pub use runge_kutta::{RungeKutta, SolveIVP};
pub use solution::IvpSolution;
