use std::time::Duration;

use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use super::Alghorithm;

#[derive(Serialize)]

pub struct JSONFileData{
    pub iterations: i32,
    pub reconstruction_time: Duration,
    pub reconstruction_start_time: NaiveDateTime,
    pub reconstruction_end_time: NaiveDateTime,
    pub image_size:u32,
    pub algorithm: Alghorithm,
    pub client_id:Uuid
}