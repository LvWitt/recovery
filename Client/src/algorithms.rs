use nalgebra::DVector;


pub fn apply_gain_signal(mut g: DVector<f64>, s: usize) -> DVector<f64> {
    let n = 64;
    for c in 0..n{
        for l in 0..s{
            let y = 100.0 + 1.0/20.0*(l as f64)*(l as f64).sqrt();
            g[l+c*s] = g[l+c*s]*y;
            //println!("{}",g[l+c*s]);
        } 
    }
    return g;

}