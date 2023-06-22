use num::{Complex, pow};
use std::{f64, f64::consts::PI};


type FloatVec = Vec<f64>;
type ComplexVec = Vec<Complex<f64>>;

pub struct FFT;

impl FFT {
    pub fn new() -> Self {
        Self {}
    }

    fn fft_helper(&self, x: &ComplexVec, inverse: bool) -> ComplexVec {
        
        let n = x.len();

        if n == 1 {
            return x.to_owned();
        }

        let even = x.iter().step_by(2).cloned().collect();
        let odd = x.iter().skip(1).step_by(2).cloned().collect();

        let even_part = self.fft_helper(&even, inverse);
        let odd_part = self.fft_helper(&odd, inverse);

        let mut dft = vec![Complex {re: 0.0, im: 0.0}; n];
        let coeff = if inverse {2.0} else {-2.0};
        let c = Complex{re: 0.0, im: coeff};
        for k in 0..(n/2) {
            let phase = (c * PI * (k as f64)/(n as f64)).exp();
            let t = odd_part[k] * phase;
            dft[k] = even_part[k] + t;
            dft[k + n/2] = even_part[k] - t;
        }

        return dft;

    }

    pub fn fft(&self, x: &FloatVec) -> ComplexVec {
    
        let x_resize: Vec<f64>;

        if !is_power_of_two(x.len()) {
            x_resize = zeropad(&x);
        } else {
            x_resize = x.clone();
        }

        let input: ComplexVec = x_resize
            .iter()
            .map(|v| Complex {re: *v, im: 0.0})
            .collect::<Vec<Complex<f64>>>();

        let fft = self.fft_helper(&input, false);
        return fft;
    }

    pub fn ifft(&self, x: &ComplexVec) -> FloatVec {

        let ifft = self.fft_helper(x, true);
        return ifft
            .to_owned()
            .into_iter()
            .map(|v| ((v.re + v.im)/x.len() as f64) as f64)
            .collect::<Vec<f64>>();

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

pub fn is_power_of_two(value: usize) -> bool {
    let is_power = value > 0 && value & (value - 1) == 0;
    return is_power;
}
