use std::time::Duration;

use nalgebra::DVector;
pub enum Alghorithm {
    CGNE(),
    CGNR(),
}
pub struct CGNEReturnType {
    pub image_vector: DVector<f64>,
    pub iterations: i32,
    pub reconstruction_time: Duration,
    pub reconstruction_start_time: Duration,
    pub reconstruction_end_time: Duration,
    pub alghorithm: Alghorithm,
}
