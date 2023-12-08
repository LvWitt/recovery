use std::{fs::File, io::{BufWriter, Write}};

use crate::models::UsageReport;


pub fn creat_usage_report(data: UsageReport) {
    let file = File::create("report_output/usageReport.json")
        .expect("Error creating JSON file.");
    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &data);
    let _ = writer.flush();
}
