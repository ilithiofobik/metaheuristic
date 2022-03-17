extern crate rand;

use super::geo::Matrix;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn objective_function(permutation: &Vec<usize>, matrix: &Matrix) -> std::io::Result<u64> {
    let mut cost = 0;

    for i in 1..matrix.n - 1 {
        cost = cost + matrix.get(permutation[i - 1], permutation[i]);
    }

    return Ok(cost);
}

pub fn k_random(matrix: &Matrix, k: usize, mut best_perm: &mut [usize]) -> u64 {
    let mut vec: Vec<usize> = (0..matrix.n).collect();
    let mut best_value = u64::MAX;

    for _ in 0..k {
        let slice: &mut [usize] = &mut vec;
        let mut rng = rand::thread_rng();

        slice.shuffle(&mut rng);

        println!("{:?}", slice);

        let new_value = objective_function(&slice.to_vec(), &matrix);

        match new_value {
            Ok(m) => {
                if best_value > m {
                    best_value = m;
                    *best_perm = *slice;
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    return best_value;
}
