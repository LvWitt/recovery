use std::{fs::File, io::{BufWriter, Write}};

use uuid::Uuid;

use crate::models::JSONFileData;

pub fn create_json_file(data: JSONFileData, filename: Uuid) {
    let escaped_filename = filename.to_string().replace("\"", "\\\"");
    let path = format!("client_output/{}.json", &escaped_filename);
    let file = File::create(path)
        .expect("Error creating JSON file.");
    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &data);
    let _ = writer.flush();
}
