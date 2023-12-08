use std::{fs::File, io::{BufWriter, Write}};

use crate::models::JSONFileData;

pub fn create_json_file(data: JSONFileData, filename: u32) {
    let file = File::create(format!("client_output/{:?}.json", filename))
        .expect("Error creating JSON file.");
    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &data);
    let _ = writer.flush();
}
