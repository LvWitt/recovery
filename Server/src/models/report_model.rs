use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct UsageReport{
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub cpu_usage: Vec<f32>,
    pub ram_usage:Vec<f32>
}