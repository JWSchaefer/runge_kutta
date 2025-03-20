use crate::{
    butcher::{Butcher, Explicit, Implicit},
    errors::SolverError,
};

use anyhow::Result;
use num_traits::Float;
use std::{cell::RefCell, ops::AddAssign};

trait Step<T, B, const S: usize, const Y: usize>
where
    T: Float,
    B: Butcher<T, S>,
{
    fn step(&self) -> Result<()>;
}

pub struct Solution<T, const Y: usize>
where
    T: Float + Default,
{
    t: Vec<T>,
    y: Vec<[T; Y]>,
}

impl<T, const Y: usize> Solution<T, Y>
where
    T: Float + Default,
{
    fn new(t: Vec<T>, y: Vec<[T; Y]>) -> Self {
        Self { t, y }
    }
    pub fn t(&self) -> &Vec<T> {
        &self.t
    }
    pub fn y(&self) -> &Vec<[T; Y]> {
        &self.y
    }
    pub fn take(self) -> (Vec<T>, Vec<[T; Y]>) {
        (self.t, self.y)
    }
}

pub trait SolveIVP<T, B, const S: usize, const Y: usize>
where
    T: Float + Default,
    B: Butcher<T, S>,
{
    fn solve(self, t_0: T, t_max: T, y_0: [T; Y]) -> Result<Solution<T, Y>>;
}

pub struct RungeKutta<T, B, F, A, const S: usize, const Y: usize>
where
    T: Float,
    B: Butcher<T, S>,
    F: Fn(T, [T; Y], &A) -> [T; Y],
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
    T: Float + Default,
    B: Butcher<T, S>,
    F: Fn(T, [T; Y], &A) -> [T; Y],
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

    fn into_solution(self) -> Solution<T, Y> {
        Solution::new(self.t.into_inner(), self.y.into_inner())
    }
}

// Explicit Case
impl<T, E, F, A, const S: usize, const Y: usize> Step<T, E, S, Y> for RungeKutta<T, E, F, A, S, Y>
where
    T: Float + Default + AddAssign,
    E: Explicit<T, S>,
    F: Fn(T, [T; Y], &A) -> [T; Y],
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

            k[i] = (self.f)(_t, _y, &self.args);
        }

        let t_np1 = *t_n + *h;
        let mut y_np1: [T; Y] = [T::default(); Y];

        // Caculate y_n+1
        for d in 0..Y {
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

impl<T, E, F, A, const S: usize, const Y: usize> SolveIVP<T, E, S, Y>
    for RungeKutta<T, E, F, A, S, Y>
where
    T: Float + Default + AddAssign,
    E: Explicit<T, S>,
    F: Fn(T, [T; Y], &A) -> [T; Y],
{
    fn solve(self, t_0: T, t_max: T, y_0: [T; Y]) -> Result<Solution<T, Y>> {
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
