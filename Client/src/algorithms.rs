use std::{fs::File, io::Write};

use csv::Writer;
use nalgebra::{ DVector, Dyn};

pub fn apply_gain_signal(g: DVector<f64>, s: usize) -> Vec<f64> {
    let n = 64;
   // let aux: Vec<f64> = g.data.as_vec().to_vec();
    let mut matrix = g.reshape_generic(Dyn(n), Dyn(s));
    for c in 0..n {
        for l in 0..s {
            let y = 100.0 +( 1.0 / 20.0) * (l as f64) * (l as f64).sqrt();
            let val = matrix[(c, l)] * y;
            //println!("{}", val);
            //let adsa = matrix[(c,l)];
            matrix[(c, l)] = val;
        }
    }
  //  println!("{:?}", matrix.data);
    let flattened_matrix = matrix.as_slice().to_vec();
    write_to_file(&flattened_matrix);
    return flattened_matrix;
}
fn write_to_file(data: &Vec<f64>)  {
    let mut file = File::create("teste.txt").unwrap();

    for &value in data {
        let _ = file.write_all(format!("{}\n", value).as_bytes());
    }
   

}