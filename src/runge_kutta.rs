use crate::{
    butcher::{Butcher, Explicit, Implicit},
    errors::SolverError,
    IvpSolution,
};
use num_traits::Float;
use std::error::Error;
use std::result::Result;
use std::{cell::RefCell, ops::AddAssign};

trait Step<B, T, const S: usize>
where
    T: Float + Default,
    B: Butcher<T, S>,
{
    fn step(&self) -> Result<(), Box<dyn Error>>;
}

pub trait SolveIVP<B, T, const S: usize, const Y: usize>
where
    B: Butcher<T, S>,
    T: Float + Default + ToString,
{
    fn solve_ivp(self, t_0: T, t_max: T, y_0: [T; Y]) -> Result<IvpSolution<T, Y>, Box<dyn Error>>;
}

pub struct RungeKutta<T, B, F, A, const S: usize, const Y: usize>
where
    T: Float + ToString,
    B: Butcher<T, S>,
    F: Fn(T, [T; Y], &A) -> Result<[T; Y], Box<dyn Error>>,
{
    f: F,
    args: A,
    tableau: B,
    t: RefCell<Vec<T>>,
    y: RefCell<Vec<[T; Y]>>,
    h: RefCell<T>,
}

// Base Implementation
impl<T, B, F, A, const S: usize, const Y: usize> RungeKutta<T, B, F, A, S, Y>
where
    T: Float + Default + ToString,
    B: Butcher<T, S>,
    F: Fn(T, [T; Y], &A) -> Result<[T; Y], Box<dyn Error>>,
{
    pub fn new(tableau: B, f: F, args: A, h: T) -> Self {
        Self {
            tableau,
            t: RefCell::new(Vec::new()),
            y: RefCell::new(Vec::new()),
            f,
            args,
            h: RefCell::new(h),
        }
    }

    fn into_solution(self) -> IvpSolution<T, Y> {
        IvpSolution::new(self.t.into_inner(), self.y.into_inner())
    }
}

// Explicit Step
impl<T, E, F, A, const S: usize, const Y: usize> Step<E, T, S> for RungeKutta<T, E, F, A, S, Y>
where
    T: Float + Default + AddAssign + ToString,
    E: Explicit<T, S>,
    F: Fn(T, [T; Y], &A) -> Result<[T; Y], Box<dyn Error>>,
{
    fn step(&self) -> Result<(), Box<dyn Error>> {
        let mut y = self.y.borrow_mut();
        let mut t = self.t.borrow_mut();

        let y_n = y
            .last()
            .ok_or_else(|| SolverError::Uninitialised("y".to_string()))?;

        let t_n = t
            .last()
            .ok_or_else(|| SolverError::Uninitialised("t".to_string()))?;

        let h = self.h.borrow();
        let a = self.tableau.a();
        let b = self.tableau.b();
        let c = self.tableau.c();

        let mut k: [[T; Y]; S] = [[T::default(); Y]; S];

        // Calculate k
        for i in 0..S {
            let _t: T = *t_n + c[i] * *h;
            let mut _y: [T; Y] = [T::default(); Y];

            for d in 0..Y {
                _y[d] = y_n[d];
                for l in 0..i {
                    _y[d] += (a[i][l] * k[l][d]) * *h;
                }
            }

            k[i] = (self.f)(_t, _y, &self.args)?;
        }

        let t_np1 = *t_n + *h;
        let mut y_np1: [T; Y] = [T::default(); Y];

        // Caculate y_n+1
        for d in 0..Y {
            y_np1[d] = y_n[d];
            for i in 0..S {
                y_np1[d] += *h * b[i] * k[i][d]
            }
            if y_np1[d].is_nan() {
                return Err(SolverError::Convergence(d).into());
            }
        }

        // Append to state info
        y.push(y_np1);
        t.push(t_np1);

        Ok(())
    }
}

// Explicit Solve
impl<T, E, F, A, const S: usize, const Y: usize> SolveIVP<E, T, S, Y>
    for RungeKutta<T, E, F, A, S, Y>
where
    T: Float + Default + AddAssign + ToString,
    E: Explicit<T, S>,
    F: Fn(T, [T; Y], &A) -> Result<[T; Y], Box<dyn Error>>,
{
    fn solve_ivp(self, t_0: T, t_max: T, y_0: [T; Y]) -> Result<IvpSolution<T, Y>, Box<dyn Error>> {
        self.t.replace(vec![t_0]);
        self.y.replace(vec![y_0]);

        let mut stop: bool = false;

        loop {
            {
                let t = self.t.borrow_mut();
                let t_n = t.last().unwrap();
                let mut h = self.h.borrow_mut();

                if t_max - *t_n < *h {
                    *h = t_max - *t_n;
                    stop = true;
                }
            }
            self.step()?;
            if stop {
                break;
            }
        }

        Ok(self.into_solution())
    }
}

// // Implicit Step
// impl<T, I, F, A, const S: usize, const Y: usize> Step<I, T, S> for RungeKutta<T, I, F, A, S, Y>
// where
//     T: Float + Default + AddAssign + ToString,
//     I: Implicit<T, S> ,
//     F: Fn(T, [T; Y], &A) -> Result<[T; Y], Box<dyn Error>>,
// {
//     fn step(&self) -> Result<(), Box<dyn Error>> {
//         todo!()
//     }
// }
