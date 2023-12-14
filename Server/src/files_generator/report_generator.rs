use std::{fs::{File, self}, io::{BufWriter, Write}};

use crate::models::UsageReport;

const FILEPATH: &str = "report_output/usageReport.json";

pub fn creat_usage_report(data: UsageReport) {
    let file = File::create(FILEPATH)
        .expect("Error creating JSON file.");
    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &data);
    let _ = writer.flush();
}


pub fn clear_usage_report(){
   fs::remove_file(FILEPATH);
}