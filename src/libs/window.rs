pub enum WindowFunction {
    Hann,
    // implement other window functions...
}


impl WindowFunction {
    pub fn apply(&self, x: &[f64]) -> Vec<f64>{
        let mut windowed: Vec<f64> = vec![0.0; x.len()];
        match self {
            Self::Hann => {
                let n = windowed.len();
                for i in 0..n {
                    let factor = 0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64/(n - 1) as f64).cos());
                    windowed[i] = x[i] * factor;
                }
            }
        }

        windowed
    }
}

pub struct Windowing {
    pub window: WindowFunction,
}


impl Windowing {
    pub fn new(window: WindowFunction) -> Self {
        Self { 
            window, 
        }
    }

    pub fn apply_to(&self, x: &[f64]) -> Vec<f64>{
        self.window.apply(x)
    }
}

