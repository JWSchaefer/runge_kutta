use crate::{
    butcher::{Butcher, Explicit, Implicit},
    errors::SolverError,
};

use anyhow::Result;
use num_traits::Float;
use std::{cell::RefCell, ops::AddAssign, process::Output};

trait Step<T, B, const S: usize, const D: usize>
where
    T: Float,
    B: Butcher<T, S>,
{
    fn step(&self) -> Result<()>;
}

pub struct Solution<T, const D: usize>
where
    T: Float + Default,
{
    t: Vec<T>,
    y: Vec<[T; D]>,
}

impl<T, const D: usize> Solution<T, D>
where
    T: Float + Default,
{
    fn new(t: Vec<T>, y: Vec<[T; D]>) -> Self {
        Self { t, y }
    }
    pub fn t(&self) -> &Vec<T> {
        &self.t
    }
    pub fn y(&self) -> &Vec<[T; D]> {
        &self.y
    }
    pub fn take(self) -> (Vec<T>, Vec<[T; D]>) {
        (self.t, self.y)
    }
}

pub trait Solve<T, B, const S: usize, const D: usize>
where
    T: Float + Default,
    B: Butcher<T, S>,
{
    fn solve(self, t_0: T, t_max: T, y_0: [T; D]) -> Result<Solution<T, D>>;
}

pub struct RungeKutta<T, B, F, A, const S: usize, const D: usize>
where
    T: Float,
    B: Butcher<T, S>,
    F: Fn(T, [T; D], &A) -> [T; D],
{
    f: F,
    args: A,
    tableau: B,
    t: RefCell<Vec<T>>,
    y: RefCell<Vec<[T; D]>>,
    h: RefCell<T>,
}

// Base Implementation
impl<T, B, F, A, const S: usize, const D: usize> RungeKutta<T, B, F, A, S, D>
where
    T: Float + Default,
    B: Butcher<T, S>,
    F: Fn(T, [T; D], &A) -> [T; D],
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

    fn into_solution(self) -> Solution<T, D> {
        Solution::new(self.t.into_inner(), self.y.into_inner())
    }
}

// Explicit Case
impl<T, E, F, A, const S: usize, const D: usize> Step<T, E, S, D> for RungeKutta<T, E, F, A, S, D>
where
    T: Float + Default + AddAssign,
    E: Explicit<T, S>,
    F: Fn(T, [T; D], &A) -> [T; D],
{
    fn step(&self) -> Result<()> {
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

        let mut k: [[T; D]; S] = [[T::default(); D]; S];

        // Calculate k
        for i in 0..S {
            let _t: T = *t_n + c[i] * *h;
            let mut _y: [T; D] = [T::default(); D];

            for d in 0..D {
                _y[d] = y_n[d];
                for l in 0..i {
                    _y[d] += (a[i][l] * k[l][d]) * *h;
                }
            }

            k[i] = (self.f)(_t, _y, &self.args);
        }

        let t_np1 = *t_n + *h;
        let mut y_np1: [T; D] = [T::default(); D];

        // Caculate y_n+1
        for d in 0..D {
            y_np1[d] = y_n[d];
            for i in 0..S {
                y_np1[d] += *h * b[i] * k[i][d]
            }
        }

        // Append to state info
        y.push(y_np1);
        t.push(t_np1);

        Ok(())
    }
}

impl<T, E, F, A, const S: usize, const D: usize> Solve<T, E, S, D> for RungeKutta<T, E, F, A, S, D>
where
    T: Float + Default + AddAssign,
    E: Explicit<T, S>,
    F: Fn(T, [T; D], &A) -> [T; D],
{
    fn solve(self, t_0: T, t_max: T, y_0: [T; D]) -> Result<Solution<T, D>> {
        self.t.replace(vec![t_0]);
        self.y.replace(vec![y_0]);

        let mut stop: bool = false;

        loop {
            {
                if let Some(t) = self.t.borrow_mut().last() {
                    if *t + *self.h.borrow() > t_max {
                        stop = true;
                    }
                } else {
                    return Err(SolverError::Uninitialised("t".to_string()).into());
                }
            }

            if stop {
                break;
            }

            self.step()?;
        }

        Ok(self.into_solution())
    }
}
