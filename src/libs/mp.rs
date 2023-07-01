use std::{collections::{HashMap, HashSet}, process::exit};
use super::{transform::Fft, window::Windowing, window, types::{VecFloatVec, VecComplexVec, FloatVec}, decompose::{DecomposedEvent, static_decompose}, math::{mat_mul, max_arg}};
use num::Complex;

pub fn generate_atoms(segments: &VecFloatVec) -> VecComplexVec {

    let mut atoms: VecComplexVec = Vec::with_capacity(segments.len());
    let mut fft_planner = Fft::new();
    let win = Windowing::new(window::WindowFunction::Hann);
    
    for segment in segments {
        let frame = win.apply_to(segment);
        let frame_fft = fft_planner.fft(&frame);
        atoms.push(frame_fft);
    }

    atoms
}

pub fn generate_dictionary(source: &FloatVec, target_frame_lengths: &HashSet<usize>) -> HashMap<usize, VecComplexVec>{
    
    let mut dict: HashMap<usize, VecComplexVec> = HashMap::new();
    let mut fft_planner = Fft::new();

    for length in target_frame_lengths {
        let source_decompose: DecomposedEvent = static_decompose(source, *length, 0.5);
        let segments = source_decompose.segments;
        let win = Windowing::new(window::WindowFunction::Hann);
        let mut temp_frames: VecComplexVec = Vec::new();
        for frame in segments {
            let windowed = win.apply_to(&frame);
            let frame_fft = fft_planner.fft(&windowed);
            temp_frames.push(frame_fft);
        }
        dict.insert(*length, temp_frames);
    }

    dict
}

pub fn find_coeffs_and_atoms(atom: &mut FloatVec, dictionary: &VecFloatVec, k: i32) -> (FloatVec, VecFloatVec) {

    avoid_zero(atom);
    let mut coeffs: FloatVec = Vec::with_capacity(k as usize);
    let mut atoms: VecFloatVec = Vec::with_capacity(k as usize);

    let mut i = 0;

    loop {

        let dot = mat_mul(dictionary, atom);
        let max_ndx: usize = max_arg(&dot);

        coeffs.push(dot[max_ndx]);
        atoms.push(dictionary[max_ndx].to_vec());
        
        for (n, value) in atom.iter_mut().enumerate() {
            *value -= coeffs[coeffs.len() - 1] * atoms[atoms.len() - 1][n];
        }

        i += 1;
        if i == k as usize{
            break;
        } 
    }

    (coeffs, atoms)

}

pub fn matching(atoms: &VecComplexVec, dictionary: &HashMap<usize, VecComplexVec>, k: i32) -> VecFloatVec {


    let win = Windowing::new(window::WindowFunction::Hann);
    let mut ifft_planner = Fft::new();
    let mut matching_atoms: VecFloatVec = Vec::with_capacity(atoms.len());

    for frame in atoms {

        let mut float_frame: FloatVec = frame
            .iter()
            .map(|&c| c.re)
            .collect();

        let frames_from_dict: VecFloatVec = match dictionary.get(&float_frame.len()) {
            Some(f) => f
                .iter()
                .map(|inner_vec| inner_vec.iter().map(|&c| c.re/float_frame.len() as f64).collect())
                .collect(),
            None => {
                eprintln!("ERROR: -> in fn matching dictionary is empty!\n");
                exit(1)
            }

        };

        let (coeffs, to_atoms) = find_coeffs_and_atoms(&mut float_frame, &frames_from_dict, k);

        let mut y = vec![0.0; to_atoms[0].len()];
        for (i, value) in y.iter_mut().enumerate() {
            for j in 0..to_atoms.len() {
                *value += to_atoms[j][i] * coeffs[j];
            }
        }
        
        let y_complex: Vec<Complex<f64>> = y
            .iter()
            .map(|&value| Complex {re: value, im: 0.0})
            .collect::<Vec<Complex<f64>>>();

        let ifft = ifft_planner.ifft(&y_complex);
        let windowed_ifft = win.apply_to(&ifft);
        matching_atoms.push(windowed_ifft);
    }

    matching_atoms

}

pub fn rebuild(matching_atoms: &VecFloatVec, pickup_points: &Vec<usize>) -> Vec<f64> {

    let p: usize = pickup_points[pickup_points.len() - 1];
    let m: usize = matching_atoms[matching_atoms.len() - 1].len();

    let mut max_len: usize = p + m;
    for (i, pick) in pickup_points.iter().enumerate() {
        let current_len = pick + matching_atoms[i].len();
        if current_len > max_len {
            max_len = current_len;
        }
    }

    let mut y: Vec<f64> = vec![0.0; max_len];

    for (i, atom) in matching_atoms.iter().enumerate() {
        for (j, value) in atom.iter().enumerate() {
            let hop = pickup_points[i];
            y[hop + j] += value;
        }
    }

    y
}

fn avoid_zero(x: &mut FloatVec) {

    let is_zeros = x.iter().all(|&value| value == 0.0);
    if is_zeros {
        *x = x.iter_mut().map(|value| *value + 1e-12).collect();
    }

}