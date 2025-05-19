use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolverError {
    #[error("{0} was uninitialised.")]
    Uninitialised(String),
    #[error("Failed to converge in output dimention {0}.")]
    Convergence(usize),
}
