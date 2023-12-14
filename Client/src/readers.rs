use chrono::NaiveDateTime;
use csv::ReaderBuilder;
use nalgebra:: DVector;
use serde::Deserialize;

use std::{error::Error, fs::File, io::Read, time::Duration};

pub fn create_vector_from_csv(filename: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut record = csv::ByteRecord::new();
    let mut deserialized_records: Vec<f64> = Vec::new();

    while rdr.read_byte_record(&mut record)? {
        deserialized_records.push(record.deserialize::<f64>(None)?);
    }

    Ok(deserialized_records)
}
#[derive(Deserialize)]

pub struct UsageReport{
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub cpu_usage: Vec<f32>,
    pub ram_usage:Vec<f32>
}
pub fn read_relatorio_server()->UsageReport{
      let file_path = "../Server/report_output/usageReport.json";
      let mut file = File::open(file_path).expect("Não foi possível abrir o arquivo");
      let mut contents = String::new();
      file.read_to_string(&mut contents)
          .expect("Erro ao ler o conteúdo do arquivo");
      let parsed: UsageReport = serde_json::from_str(&contents).expect("Erro ao fazer o parsing do JSON");
  
    return parsed
}
