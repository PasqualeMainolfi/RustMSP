
pub fn mat_mul(mat: &[Vec<f64>], vec: &[f64]) -> Vec<f64> {

    let mut y = vec![0.0; mat.len()];

    for i in 0..mat.len() {
        let mut s = 0.0;
        for (j, value) in vec.iter().enumerate() {
            s += mat[i][j] * value;
        }
        y[i] = s;
    }

    y
}

pub fn max_arg(vec: &[f64]) -> usize {
    let mut max_index: usize = 0;
    for (i, value) in vec.iter().enumerate() {
        if value.abs() > vec[max_index].abs() {
            max_index = i;
        }
    }
    max_index
}