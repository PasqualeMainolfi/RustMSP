
mod libs {
    pub mod transform;
    pub mod decompose;
    pub mod window;
    pub mod mp;
    pub mod types;
}


use std::vec;
use libs::decompose::DecomposedEvent;
use crate::libs::decompose::dynamic_decompose;
use crate::libs::mp::{generate_atoms, generate_dictionary, matching, rebuild};
use rand::{thread_rng, Rng};


fn main() {

    const TARGET_SIZE: usize = 32;
    const SOURCE_SIZE: usize = 64;
    const K: i32 = 10;

    let mut target: Vec<f64> = vec![0.0; TARGET_SIZE];
    for index in 0..TARGET_SIZE {
        target[index] = (2.0 * std::f64::consts::PI * index as f64/TARGET_SIZE as f64).sin();
    }

    // println!("TARGET: {:?}\n", target);
    
    
    let mut source: Vec<f64> = vec![0.0; SOURCE_SIZE];
    
    let mut rng = thread_rng();
    for index in 0..SOURCE_SIZE {
        let r: f64 = rng.gen();
        source[index] = r;
        
    }
    
    // println!("SOURCE: {:?}\n", source);
    
    println!("DECOMPOSE TARGET...\n");
    
    let dec: DecomposedEvent = dynamic_decompose(&target, 0.25, 0.75);
    let seg = dec.segments;
    let pick = dec.pickup_points;
    let sizes = dec.segment_sizes;
    // println!("SEGMENTS: {:?}\nPICKUP POINTS: {:?}\nSIZES: {:?}", seg, pick, sizes);
    
    println!("GENERATE ATOMS...\n");
    
    let atoms = generate_atoms(&seg);
    // println!("ATOMS: {atoms:?}\n");
    
    println!("GENERATE DICTIONARY...\n");
    
    let dictionary = generate_dictionary(&source, &sizes);
    // println!("DICTIONARY: {dictionary:?}\nDICTIONARY KEY: {:?}", dictionary.keys().cloned().collect::<Vec<usize>>());
    
    println!("GENERATE MATCHING ATOMS...\n");
    
    let matching_atoms = matching(&atoms, &dictionary, K);
    // println!("MATCHING ATOMS: {:?}\n", matching_atoms);
    
    println!("PERFORM REBUILD...\n");
    
    let rebuild = rebuild(&matching_atoms, &pick);
    
    println!("REBUILDED: {:?}...\n", rebuild);
    println!("SIZE OF TARGET: {}\nSIZE OF REBUILDED SIGNAL: {}\n", target.len(), rebuild.len());


}
