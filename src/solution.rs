// use csv::Writer;
use num_traits::Float;
use std::path::Path;

pub struct IvpSolution<T, const Y: usize>
where
    T: Float + ToString,
{
    t: Vec<T>,
    y: Vec<[T; Y]>,
}

impl<T, const Y: usize> IvpSolution<T, Y>
where
    T: Float + ToString,
{
    pub fn new(t: Vec<T>, y: Vec<[T; Y]>) -> Self {
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
    // pub fn to_csv(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     let path = Path::new(path);
    //     let mut wtr = Writer::from_path(path)?;
    //
    //     // Write header
    //     let header: Vec<String> = std::iter::once("t".to_string())
    //         .chain((0..Y).map(|i| format!("y{}", i)))
    //         .collect();
    //     wtr.write_record(header)?;
    //
    //     // Write rows
    //     for (t_value, y_values) in self.t.iter().zip(self.y.iter()) {
    //         let mut record: Vec<String> = vec![t_value.to_string()];
    //         for &y_value in y_values.iter() {
    //             record.push(y_value.to_string());
    //         }
    //         wtr.write_record(record)?;
    //     }
    //
    //     wtr.flush()?;
    //     Ok(())
    // }
}
