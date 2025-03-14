use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolverError {
    #[error("{0} was uninitialised.")]
    Uninitialised(String),
}
