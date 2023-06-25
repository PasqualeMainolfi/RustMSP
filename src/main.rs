
mod libs {
    pub mod transform;
    pub mod decompose;
    pub mod window;
}

use std::vec;

use libs::transform;
use libs::decompose::{DecomposedEvent, static_decompose};

use crate::libs::decompose::dynamic_decompose;
use crate::libs::window::Windowing;

fn main() {

    let x: Vec<f64> = vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    
    println!("SOURCE: {x:?}\n");

    let mut planner = transform::Fft::new();

    let win = Windowing::new(libs::window::WindowFunction::Hann);
    let windowed = win.apply_to(&x);

    println!("WINDOWED: {windowed:?}\n");

    let fft = planner.fft(&x);
    println!("FFT: {fft:?}\n");

    let ifft = planner.ifft(&fft);
    println!("IFFT: {ifft:?}\n");

    let dec: DecomposedEvent = dynamic_decompose(&x, 0.25, 0.75);
    let seg = dec.segments;
    let pick = dec.pickup_points;
    let sizes = dec.segment_sizes;
    println!("SEGMENTS: {:?}\nPICKUP POINTS: {:?}\nSIZES: {:?}", seg, pick, sizes);

}
