use std::time::Instant;
use chrono::Local;
use nalgebra::DVector;
use nalgebra_sparse::csr::CsrMatrix;
use nalgebra_sparse::ops::serial::spmm_csr_dense;
use nalgebra_sparse::ops::Op;


use crate::models::{CGNEReturnType, Alghorithm, CGNRReturnType, AlgorithmsReturnType};



fn cgne(matrix_h: &CsrMatrix<f64>, vector_g: &DVector<f64>, tolerance: f64) -> CGNEReturnType {
    let start_timer = Instant::now();
    let start_local_time = Local::now();
    let matrix_h_transposed = matrix_h.transpose();
    let mut f = DVector::zeros(matrix_h.ncols());
    let mut r = vector_g - (matrix_h * &f);
    let mut p = &matrix_h_transposed * &r;
    let mut error_tolerance = 1.0;
    let mut iteration_count = 0;

    while error_tolerance > tolerance  && iteration_count<30{
        let residual_dot_residual = r.dot(&r);
        let hp = matrix_h * &p;
        let residual_norm = r.norm();
        let alpha = residual_dot_residual / (p.dot(&p));
        let beta: f64;

        f = f + alpha * &p;
        // f.axpy(alpha, &p, 1.0);
        r = r - (alpha * &hp);
        beta = (&r.dot(&r)) / residual_dot_residual;
        // p = (&matrix_h_transposed * &r) + (beta * p);
        spmm_csr_dense(
            beta,
            &mut p,
            1.0,
            Op::NoOp(&matrix_h_transposed),
            Op::NoOp(&r),
        );
        error_tolerance = (r.norm() - residual_norm).abs();
        //println!("Error Tolerance: {:?}", error_tolerance);
        iteration_count += 1;
    }
    let end_timer = Instant::now();
    let end_local_time = Local::now();
    
    return CGNEReturnType {
        image_vector: f,
        iterations: iteration_count,
        reconstruction_time: end_timer - start_timer,
        reconstruction_start_time: start_local_time,
        reconstruction_end_time: end_local_time,
        alghorithm: Alghorithm::CGNE,
    };
}

fn cgnr(matrix_h: &CsrMatrix<f64>, vector_g: &DVector<f64>, tolerance: f64) -> CGNRReturnType {
    let start_timer = Instant::now();
    let start_local_time = Local::now();
    let matrix_h_transposed = matrix_h.transpose();
    let mut f = DVector::zeros(matrix_h.ncols());
    let mut r = vector_g - (matrix_h * &f);
    let mut z = &matrix_h_transposed * &r;
    let mut p = z.clone();

    let mut error_tolerance = 1.0;
    let mut iteration_count = 0;
    let mut z_old_norm = z.norm_squared();
    while error_tolerance > tolerance && iteration_count<30 {
        let w = matrix_h * &p;
        let alpha = z.norm_squared() / w.norm_squared();
        let r_old_norm = r.norm();
        let beta: f64;

        f = f + alpha * &p;
        r = &r - alpha * w;
        z = &matrix_h_transposed * &r;
        let z_norm = z.norm_squared();
        beta = z_norm / z_old_norm;
        p = &z + beta * p;

        z_old_norm = z_norm;
        error_tolerance = (r.norm() - r_old_norm).abs();
        //println!("Error Tolerance: {:?}", error_tolerance);
        iteration_count += 1;
    }
    let end_timer = Instant::now();
    let end_local_time = Local::now();
    //println!("Iterations: {:?}", iteration_count);
    return CGNRReturnType {
        image_vector: f,
        iterations: iteration_count,
        reconstruction_time: end_timer - start_timer,
        reconstruction_start_time: start_local_time,
        reconstruction_end_time: end_local_time,
        alghorithm: Alghorithm::CGNR,
    };
}


pub fn process_algorithm(
    mh: &CsrMatrix<f64>,
    vt: &DVector<f64>,
    tolerance: f64,
    algorithm_name: &str,
) -> Result<AlgorithmsReturnType, String> {
    let algorithm_data:AlgorithmsReturnType = match algorithm_name {
        "cgne" => AlgorithmsReturnType::CGNEReturnType(cgne(mh, vt, tolerance)),
        "cgnr" => AlgorithmsReturnType::CGNRReturnType(cgnr(mh, vt, tolerance)),
        _ => return Err(format!("Unsupported algorithm type: {}", algorithm_name)), 
    };
    println!("Processed");
   return Ok(algorithm_data)
}
