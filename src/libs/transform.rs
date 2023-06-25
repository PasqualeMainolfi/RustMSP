use num::Complex;
use rustfft::FftPlanner;

type FloatVec = Vec<f64>;
type ComplexVec = Vec<Complex<f64>>;

pub struct Fft {
    planner: FftPlanner<f64>,
}

impl Fft {
    pub fn new() -> Self {
        let planner = FftPlanner::new();
        Self {
            planner,
        }
    }

    pub fn fft(&mut self, x: &FloatVec) -> ComplexVec {
    
        let mut buffer: ComplexVec = x
            .iter()
            .map(|&v| Complex {re: v, im: 0.0})
            .collect::<Vec<Complex<f64>>>();

        let f = self.planner.plan_fft_forward(buffer.len());
        f.process(&mut buffer);

        buffer
    }

    pub fn ifft(&mut self, x: &ComplexVec) -> FloatVec {

        let mut buffer = x.to_vec();
        let f = self.planner.plan_fft_inverse(x.len());
        f.process(&mut buffer);

        let y = buffer
            .iter()
            .map(|&x| (x.re + x.im)/buffer.len() as f64)
            .collect();

        y
    }
}