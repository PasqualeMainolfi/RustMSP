use std::collections::HashSet;
use rand::Rng;

use super::types::VecFloatVec;


const LENSIZES: usize = 3;
// const SIZES: [usize; LENSIZES] = [64, 128, 256, 512, 1024, 2048, 4096, 16384];
const SIZES: [usize; LENSIZES] = [2, 4, 8];


#[derive(Debug)]
pub struct DecomposedEvent {
    pub segments: VecFloatVec,
    pub pickup_points: Vec<usize>,
    pub segment_sizes: HashSet<usize>,
}


pub fn static_decompose(x: &[f64], winsize: usize, hopsize: f32) -> DecomposedEvent {

    let mut segments: VecFloatVec = Vec::new();
    let mut pickup_points: Vec<usize> = Vec::new();
    let mut segment_sizes: HashSet<usize> = HashSet::new();
    let size = x.len();
    
    segment_sizes.insert(winsize);

    let hop: usize = (winsize as f32 * hopsize) as usize; 
    
    for i in (0..size).step_by(hop) {
        let end_index = i + winsize;
        if end_index <= x.len() {
            let frame: Vec<f64> = x[i..end_index].to_vec();
            segments.push(frame);
            pickup_points.push(i);
        }
    }


    DecomposedEvent {
        segments,
        pickup_points,
        segment_sizes
    }

}


pub fn dynamic_decompose(x: &[f64], hopminsize: f32, hopmaxsize: f32) -> DecomposedEvent {

    let mut segments: VecFloatVec = Vec::new();
    let mut pickup_points: Vec<usize> = Vec::new();
    let mut segment_sizes: HashSet<usize> = HashSet::new();

    let mut rng = rand::thread_rng();
    let mut wsize = SIZES[rng.gen_range(0..LENSIZES)];

    let mut hop: usize = 0;
    let mut endhop = hop + wsize;
    
    while endhop < x.len() {
        
        let frame: Vec<f64> = x[hop..endhop].to_vec();
        
        segments.push(frame);
        segment_sizes.insert(wsize);
        pickup_points.push(hop);

        let hmin = (wsize as f32 * hopminsize) as usize;
        let hmax = (wsize as f32 * hopmaxsize) as usize;
        let mut hopsize: usize = rng.gen_range(hmin..hmax);
        hopsize = if hopsize == 0 {1} else {hopsize};
        wsize = SIZES[rng.gen_range(0..LENSIZES)];
        
        hop += hopsize;
        endhop = hop + wsize;

    }


    DecomposedEvent {
        segments,
        pickup_points,
        segment_sizes
    }

}