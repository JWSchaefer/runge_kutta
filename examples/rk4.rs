use std::error::Error;

use rk::methods::explicit::RK4;
use rk::{RungeKutta, Solve};

fn pendulum(_t: f64, y: [f64; 2], args: &(f64, f64)) -> [f64; 2] {
    let [theta, theta_dot] = y;
    let (g, l) = args;

    let theta_dot_dot: f64 = -(g / l) * f64::sin(theta);

    return [theta_dot, theta_dot_dot];
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = (9.81, 1.0);
    let y_0 = [3.1415926535 / 2.0, 0.0];
    let solver = RungeKutta::new(RK4, pendulum, args, 0.05);
    let solution = solver.solve(0.0, 5.0, y_0)?;
    Ok(())
}
