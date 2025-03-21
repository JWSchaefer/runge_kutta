# Runge-Kutta

A lightweight, generalised, n-dimentional, Runge-Kutta integrator. Supports Explicit, Implicit, Adaptive, and Nystrom methods.

## Example

```rust
use std::error::Error;

use runge_kutta::methods::explicit::RK4;
use runge_kutta::{RungeKutta, SolveIVP};

fn pendulum(_t: f64, y: [f64; 2], args: &(f64, f64)) -> Result<[f64; 2], Box<dyn Error>> {
    let [theta, theta_dot] = y;
    let (g, l) = args;

    let theta_dot_dot: f64 = -(g / l) * f64::sin(theta);

    return Ok([theta_dot, theta_dot_dot]);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = (9.81, 1.0);
    let y_0 = [3.1415926535 / 2.0, 0.0];
    let (t_0, t_max, delta_t) = (0.0, 5.0, 0.01);

    let solver = RungeKutta::new(RK4, pendulum, args, delta_t);
    let solution = solver.solve_ivp(t_0, t_max, y_0)?;
    Ok(())
}
```
