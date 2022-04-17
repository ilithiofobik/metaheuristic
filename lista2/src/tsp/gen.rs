use super::geo::Matrix;
use super::geo::Point;
use rand::Rng;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn create_euclid(size: usize) -> Matrix {
    let max_coor = size as i64;
    let mut coords = HashMap::new();
    let mut rng = rand::thread_rng();

    for n in 0..size {
        let x = rng.gen_range(0..max_coor);
        let y = rng.gen_range(0..max_coor);
        let p = Point { x, y };
        coords.insert(n, p);
    }

    let mut matrix = Matrix::new(size);

    for i in 0..size {
        let pi = &coords[&i];
        for j in 0..i {
            let pj = &coords[&j];
            let val = pi.distance(pj);
            matrix.put(i, j, val);
            matrix.put(j, i, val);
        }
    }

    matrix
}

#[allow(dead_code)]
pub fn create_atsp(size: usize) -> Matrix {
    let max_distance = size as u64;
    let mut matrix = Matrix::new(size);
    let mut rng = rand::thread_rng();

    for i in 0..size {
        for j in 0..i {
            let val = rng.gen_range(0..max_distance);
            matrix.put(i, j, val);
        }
        for j in i + 1..size {
            let val = rng.gen_range(0..max_distance);
            matrix.put(i, j, val);
        }
    }

    matrix
}

#[allow(dead_code)]
pub fn create_tsp(size: usize) -> Matrix {
    let max_distance = size as u64;
    let mut matrix = Matrix::new(size);
    let mut rng = rand::thread_rng();

    for i in 0..size {
        for j in 0..i {
            let val = rng.gen_range(0..max_distance);
            matrix.put(i, j, val);
            matrix.put(j, i, val);
        }
    }

    matrix
}
