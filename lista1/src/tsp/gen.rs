use super::geo::Matrix;
use super::geo::Point;
use rand::Rng;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn create_coords(size: i32) -> std::collections::HashMap<i32, Point> {
    let mut coords = HashMap::new();
    let mut rng = rand::thread_rng();

    for n in 0..size {
        let x = rng.gen_range(-100..100);
        let y = rng.gen_range(-100..100);
        let p = Point { x, y };
        coords.insert(n, p);
    }

    return coords;
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
