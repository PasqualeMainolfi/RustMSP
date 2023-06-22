
mod libs {
    pub mod transform;
}

use libs::transform;

fn main() {

    let x: Vec<f64> = vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

    let planner = transform::FFT::new();
    let fft = planner.fft(&x);

    println!("{fft:?}\n");

    let ifft = planner.ifft(&fft);
    
    println!("{ifft:?}\n");

}
