
mod libs {
    pub mod transform;
    pub mod decompose;
    pub mod window;
    pub mod mp;
    pub mod types;
    pub mod math;
}

use std::time::Instant;
use libs::decompose::DecomposedEvent;
use crate::libs::decompose::dynamic_decompose;
use crate::libs::mp::{generate_atoms, generate_dictionary, matching, rebuild};
use std::fs::File;
use wav_io::header::WavHeader;



fn open_file(path: &String) -> (WavHeader, Vec<f64>) {

    let file = File::open(path).unwrap_or_else(|err| {
        eprintln!("ERROR: file {path} not found!: {err}\n");
        std::process::exit(1)
    });
    
    let (head, samples) = wav_io::read_from_file(file).unwrap();
    (head, samples.iter().map(|&x| x as f64).collect())
}

fn main() {

    let start_time = Instant::now();

    let target_path: String = String::from("./audio_file/vox.wav");
    let source_path: String = String::from("./audio_file/classical.wav");

    let (target_head, target_samples) = open_file(&target_path);
    println!("{:?}\n", target_head);
    println!("{}\n", target_samples.len());
    
    let (source_head, source_samples) = open_file(&source_path);
    println!("{:?}\n", source_head);
    println!("{}\n", source_samples.len());

    const K: i32 = 10;
    
    println!("DECOMPOSE TARGET...\n");
    
    let dec: DecomposedEvent = dynamic_decompose(&target_samples, 0.25, 0.75);
    let seg = dec.segments;
    let pick = dec.pickup_points;
    let sizes = dec.segment_sizes;
    // println!("SEGMENTS: {:?}\nPICKUP POINTS: {:?}\nSIZES: {:?}", seg, pick, sizes);
    println!("LAST PICKUP POINTS: {}\n", pick[pick.len() - 1]);
    println!("GENERATE ATOMS...\n");
    
    let atoms = generate_atoms(&seg);
    // println!("ATOMS: {atoms:?}\n");
    
    println!("GENERATE DICTIONARY...\n");
    
    let dictionary = generate_dictionary(&source_samples, &sizes);
    // println!("DICTIONARY: {dictionary:?}\nDICTIONARY KEY: {:?}", dictionary.keys().cloned().collect::<Vec<usize>>());
    
    println!("GENERATE MATCHING ATOMS...\n");
    
    let matching_atoms = matching(&atoms, &dictionary, K);
    // println!("MATCHING ATOMS: {:?}\n", matching_atoms);
    
    println!("PERFORM REBUILD...\n");
    
    let rebuild = rebuild(&matching_atoms, &pick);
    
    // println!("REBUILDED: {:?}...\n", rebuild);
    println!("SIZE OF TARGET: {}\nSIZE OF REBUILDED SIGNAL: {}\n", target_samples.len(), rebuild.len());

    let end = Instant::now();
    let end_time = (end - start_time).as_secs() as f64;
    println!("ELAPSED TIME: {end_time}\n");


}
