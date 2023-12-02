use csv::ReaderBuilder;
use nalgebra::{DMatrix, DVector};
use nalgebra_sparse::{csr::CsrMatrix, CooMatrix};
use std::{error::Error, fs::File, io::BufReader, time::Instant};

pub fn create_vector_from_csv(filename: &str) -> Result<DVector<f64>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut record = csv::ByteRecord::new();
    let mut deserialized_records: Vec<f64> = Vec::new();

    while rdr.read_byte_record(&mut record)? {
        deserialized_records.push(record.deserialize::<f64>(None)?);
    }

    Ok(DVector::from_vec(deserialized_records))
}

pub fn create_matrix_from_csv(filename: &str, num_rows:usize,num_columns:usize) -> Result<CsrMatrix<f64>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let buffered = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buffered.get_ref());
    
    let mut record = csv::StringRecord::with_capacity(num_rows, num_columns);
    let mut coo: CooMatrix<f64> = CooMatrix::new(num_rows, num_columns);

    let mut i = 0;
    while csv_reader.read_record(&mut record)? {
        for (j, field) in record.iter().enumerate() {
            if field != "0" {
                let parsed = field.parse::<f64>()?;
                coo.push(i, j, parsed);
            }
        }
        i = i + 1;
    }

    Ok(CsrMatrix::from(&coo))
}
