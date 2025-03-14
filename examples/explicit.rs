use anyhow::Result;
use csv::Writer;
use rk::butcher::{Butcher, Explicit};
use rk::rk::{RungaKutta, Solve};

struct RK4 {
    a: [[f64; 4]; 4],
    b: [f64; 4],
    c: [f64; 4],
}

impl RK4 {
    fn new() -> Self {
        let a = [
            [0.0, 0.0, 0.0, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            [0.0, 0.5, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        let b: [f64; 4] = [1.0 / 6.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 6.0];
        let c: [f64; 4] = [0.0, 0.5, 0.5, 1.0];
        Self { a, b, c }
    }
}

impl Butcher<f64, 4> for RK4 {
    fn a(&self) -> &[[f64; 4]; 4] {
        &self.a
    }
    fn b(&self) -> &[f64; 4] {
        &self.b
    }
    fn c(&self) -> &[f64; 4] {
        &self.c
    }
}

impl Explicit<f64, 4> for RK4 {}

fn pendulum(_t: f64, y: [f64; 2], args: &(f64, f64)) -> [f64; 2] {
    let [theta, theta_dot] = y;
    let (g, l) = args;

    let theta_dot_dot: f64 = -(g / l) * f64::sin(theta);

    return [theta_dot, theta_dot_dot];
}

fn main() -> Result<()> {
    let method = RK4::new();

    let args = (9.81, 1.0);

    let solver = RungaKutta::new(method, pendulum, args, 0.05);
    solver.solve(0.0, 5.0, [3.1415926535 / 2.0, 0.0])?;

    let (t, y): (Vec<f64>, Vec<[f64; 2]>) = solver.unpack();

    write_data_to_csv(t, y, "solution.csv")?;

    Ok(())
}

fn write_data_to_csv(t: Vec<f64>, y: Vec<[f64; 2]>, filename: &str) -> Result<()> {
    // Create a CSV writer that writes to the file
    let mut wtr = Writer::from_path(filename)?;

    // Write the header row
    wtr.write_record(&["time", "y1", "y2"])?;

    // Write each row of data
    for (time, y_values) in t.iter().zip(y.iter()) {
        wtr.write_record(&[
            time.to_string(),
            y_values[0].to_string(),
            y_values[1].to_string(),
        ])?;
    }

    // Flush the writer to ensure all data is written
    wtr.flush()?;

    Ok(())
}
