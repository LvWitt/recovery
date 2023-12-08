use chrono::DateTime;
use chrono::Local;
use nalgebra::DVector;
use serde::Serialize;
use std::time::Duration;
#[derive(Serialize)]
pub enum Alghorithm {
    CGNE,
    CGNR,
}
pub struct CGNEReturnType {
    pub image_vector: DVector<f64>,
    pub iterations: i32,
    pub reconstruction_time: Duration,
    pub reconstruction_start_time: DateTime<Local>,
    pub reconstruction_end_time: DateTime<Local>,
    pub alghorithm: Alghorithm,
}


pub struct CGNRReturnType {
    pub image_vector: DVector<f64>,
    pub iterations: i32,
    pub reconstruction_time: Duration,
    pub reconstruction_start_time: DateTime<Local>,
    pub reconstruction_end_time: DateTime<Local>,
    pub alghorithm: Alghorithm,
}
