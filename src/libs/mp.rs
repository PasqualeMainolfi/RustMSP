use std::{collections::{HashMap, HashSet}, process::exit};
use super::{transform::Fft, window::Windowing, window, types::{VecFloatVec, VecComplexVec, FloatVec}, decompose::{DecomposedEvent, static_decompose}};
use ndarray::prelude::*;
use ndarray_linalg;
use num::Complex;


pub fn generate_atoms(segments: &VecFloatVec) -> VecComplexVec {

    let mut atoms: VecComplexVec = Vec::new();
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
    let new_atom = atom.clone();
    let mut r = Array::from_vec(new_atom.to_vec());
    let mut d: Array2<f64> = Array::zeros((dictionary.len(), dictionary[0].len()));

    for i in 0..d.nrows() {
        for j in 0..d.ncols() {
            let value = dictionary[i][j];
            d[[i, j]] = value;
        }
    }

    let mut coeffs: FloatVec = Vec::new();
    let mut atoms: VecFloatVec = Vec::new();

    let mut i = 0;

    loop {

        let dot = d.dot(&r);
        let max_ndx = argmax(&dot.to_vec());
        let coeff = dot[max_ndx];
        let atom_from_dict = d.row(max_ndx);

        coeffs.push(coeff);
        atoms.push(atom_from_dict.to_vec());

        let mult = coeff * Array::from_vec(atom_from_dict.to_vec());
        r = r - mult;

        i += 1;
        if i == k {
            break;
        } 
    }

    println!("\n");

    (coeffs, atoms)

}

pub fn matching(atoms: &VecComplexVec, dictionary: &HashMap<usize, VecComplexVec>, k: i32) -> VecFloatVec {


    let win = Windowing::new(window::WindowFunction::Hann);
    let mut ifft_planner = Fft::new();
    let mut matching_atoms: VecFloatVec = Vec::new();

    for frame in atoms {

        let mut float_frame: FloatVec = frame
            .iter()
            .map(|&c| c.re + c.im)
            .collect();

        let frames_from_dict: VecFloatVec = match dictionary.get(&float_frame.len()) {
            Some(f) => f
                .iter()
                .map(|inner_vec| inner_vec.iter().map(|&c| (c.re + c.im)/float_frame.len() as f64).collect())
                .collect(),
            None => {
                eprintln!("ERROR: -> in fn matching dictionary is empty!\n");
                exit(1)
            }

        };

        let (coeffs, atoms) = find_coeffs_and_atoms(&mut float_frame, &frames_from_dict, k);
        let coeffs: Array1<f64> = Array::from_vec(coeffs.to_vec());
        let mut natoms: Array2<f64> = Array::zeros((atoms.len(), atoms[0].len()));

        for i in 0..natoms.nrows() {
            for j in 0..natoms.ncols() {
                natoms[[i, j]] = atoms[i][j];
            }
        }

        let atoms_transpose = natoms.t();
        let mut y: Array1<f64> = Array1::zeros(atoms_transpose.nrows());

        for i in 0..atoms_transpose.nrows() {
            for j in 0..atoms_transpose.ncols() {
                y[i] += atoms_transpose[[i, j]] * coeffs[j];
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

    let n: usize = pickup_points[pickup_points.len() - 1] + matching_atoms[matching_atoms.len() - 1].len();
    let mut y: Vec<f64> = vec![0.0; n + 2];

    for (i, atoms) in matching_atoms.iter().enumerate() {
        for (j, value) in atoms.iter().enumerate() {
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

fn argmax(x: &[f64]) -> usize {

    let mut max_value: f64 = x[0];
    let mut max_index: usize = 0;

    for (i, &value) in x.iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_index = i;
        }
    }

    max_index
}