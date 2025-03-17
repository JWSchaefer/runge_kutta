use num_traits::Float;

pub trait Butcher<F, const S: usize>
where
    F: Float,
{
    fn a(&self) -> &[[F; S]; S];
    fn b(&self) -> &[F; S];
    fn c(&self) -> &[F; S];
}

pub trait Adaptive<F, const S: usize>: Butcher<F, S>
where
    F: Float,
{
    fn b_star(&self) -> [&F; S];
}

pub trait Nystrom<F, const S: usize>: Butcher<F, S>
where
    F: Float,
{
    fn b_bar(&self) -> [&F; S];
}

pub trait Explicit<F, const S: usize>: Butcher<F, S>
where
    F: Float,
{
}

pub trait Implicit<F, const S: usize>: Butcher<F, S>
where
    F: Float,
{
}
