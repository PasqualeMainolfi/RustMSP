use num::{Complex, pow};
use std::f64;


type FloatVec = Vec<f64>;
type ComplexVec = Vec<Complex<f64>>;

struct FFT {
    pub winsize: usize,
    pub hopsize: usize,
}

impl FFT {
    pub fn new(winsize: usize, hopsize: usize) -> Self {

        Self { 
            winsize, 
            hopsize,
        }
        
    }

    pub fn fft_forward(&self, x: &FloatVec) -> ComplexVec {
        unimplemented!()
    }

    pub fn fft_inverse(&self, x: &ComplexVec) -> FloatVec {
        unimplemented!()
    }

}

pub fn zeropad(x: &FloatVec) -> FloatVec {
    
    let size = x.len();
    let power: usize = f64::log2(size as f64) as usize;
    let new_size: usize = if (pow(2, power) as usize) == size {size} else {pow(2, power + 1)};
    let mut y = vec![0.0; new_size];
    y[0..size].copy_from_slice(x);
    return y;
}