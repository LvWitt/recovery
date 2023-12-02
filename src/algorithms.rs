use nalgebra::DVector;
use nalgebra_sparse::csr::CsrMatrix;
use nalgebra_sparse::ops::serial::spmm_csr_dense;
use nalgebra_sparse::ops::Op;

pub fn cgne(matrix_h: CsrMatrix<f64>, vector_g: DVector<f64>, tolerance: f64) -> DVector<f64> {
    let matrix_h_transposed = matrix_h.transpose();
    let mut f = DVector::zeros(matrix_h.ncols());
    let mut r = &vector_g - (&matrix_h * &f);
    let mut p = &matrix_h_transposed * &r;
    let mut error_tolerance = 1.0;
    let mut iteration_count = 0;
    
    while error_tolerance > tolerance {
        let residual_dot_residual = r.dot(&r);
        let hp = &matrix_h * &p;
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

    println!("Iterations: {:?}", iteration_count);
    return f;
}

pub fn cgnr(matrix_h: CsrMatrix<f64>, vector_g: DVector<f64>, tolerance: f64) -> DVector<f64> {
    let matrix_h_transposed = matrix_h.transpose();
    let mut f = DVector::zeros(matrix_h.ncols());
    let mut r = &vector_g - (&matrix_h * &f);
    let mut z = &matrix_h_transposed * &r;
    let mut p = z.clone();

    let mut error_tolerance = 1.0;
    let mut iteration_count = 0;
    let mut z_old_norm = z.norm_squared();
    while error_tolerance > tolerance {
        let w = &matrix_h * &p;
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

    println!("Iterations: {:?}", iteration_count);
    return f;
}
