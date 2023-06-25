
mod libs {
    pub mod transform;
    pub mod decompose;
}

use std::vec;

use libs::transform;
use libs::decompose::{DecomposedEvent, static_decompose};

use crate::libs::decompose::dynamic_decompose;

fn main() {

    let x: Vec<f64> = vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    
    println!("start vector: {x:?}\n");

    let mut planner = transform::Fft::new();
    
    let fft = planner.fft(&x);
    println!("FFT: {fft:?}\n");

    let ifft = planner.ifft(&fft);
    println!("IFFT: {ifft:?}\n");

    let dec: DecomposedEvent = dynamic_decompose(&x, 0.25, 0.75);
    let seg = dec.segments;
    println!("{:?}\n", seg);

}
