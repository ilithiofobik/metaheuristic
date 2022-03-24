use super::geo::Matrix;
use super::geo::Point;
use rand::Rng;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn create_euclid(size: usize) -> Matrix {
    let mut coords = HashMap::new();
    let mut rng = rand::thread_rng();

    for n in 0..size {
        let x = rng.gen_range(-100..100);
        let y = rng.gen_range(-100..100);
        let p = Point { x, y };
        coords.insert(n, p);
    }

    let mut matrix = Matrix::new(size);

    for i in 0..size {
        let pi = &coords[&i];
        for j in 0..i {
            let pj = &coords[&j];
            let val = pi.distance(&pj);
            matrix.put(i, j, val);
            matrix.put(j, i, val);
        }
    }

    return matrix;
}

#[allow(dead_code)]
pub fn create_atsp(size: usize) -> Matrix {
    let mut matrix = Matrix::new(size);
    let mut rng = rand::thread_rng();

    for i in 0..size {
        for j in 0..i {
            let val = rng.gen_range(0..1000);
            matrix.put(i, j, val);
        }
        for j in i + 1..size {
            let val = rng.gen_range(0..1000);
            matrix.put(i, j, val);
        }
    }

    return matrix;
}

#[allow(dead_code)]
pub fn create_tsp(size: usize) -> Matrix {
    let mut matrix = Matrix::new(size);
    let mut rng = rand::thread_rng();

    for i in 0..size {
        for j in 0..i {
            let val = rng.gen_range(0..1000);
            matrix.put(i, j, val);
            matrix.put(j, i, val);
        }
    }

    return matrix;
}
