use nalgebra::DVector;
use nalgebra_sparse::csr::CsrMatrix;
use nalgebra_sparse::ops::serial::spmm_csr_dense;
use nalgebra_sparse::ops::Op;

pub fn applyGainSignal(mut g: DVector<f64>) -> DVector<f64> {
    let n = 64;
    let s = 436; //sinal 30x30
    //let s = 794 //sinal 60x60

    for c in 0..n{
        for l in 0..s{
            let y = 100.0 + 1.0/20.0*(l as f64)*(l as f64).sqrt();
            g[l+c*s] = g[l+c*s]*y;
            println!("{}",g[l+c*s]);
        } 
    }
    return g;

}