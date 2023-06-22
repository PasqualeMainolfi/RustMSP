
mod libs {
    
    pub mod fft {
        pub mod transform;
    }
    pub mod mp {}
    pub mod operations {}
    pub mod segmentation {}
}

use libs::fft::transform;

fn main() {

    let x: Vec<f64> = (0..2000).map(|x| x as f64).collect();
    let z = transform::zeropad(&x);
    println!("{}\n", z.len());
    println!("{z:?}\n");
    
    
}
