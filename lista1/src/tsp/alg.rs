extern crate rand;

use super::geo::Matrix;
use rand::seq::SliceRandom;

fn objective_function(permutation: &Vec<usize>, matrix: &Matrix) -> u64 {
    let n = matrix.n;
    let mut cost = matrix.get(permutation[n - 1], permutation[0]);

    for i in 1..n {
        cost = cost + matrix.get(permutation[i - 1], permutation[i]);
    }

    return cost;
}

pub fn k_random(matrix: &Matrix, k: usize) -> (u64, Vec<usize>) {
    let mut vec: Vec<usize> = (0..matrix.n).collect();
    let mut best_perm: Vec<usize> = Vec::new();
    let mut best_value = u64::MAX;

    for _ in 0..k {
        let slice: &mut [usize] = &mut vec;
        let mut rng = rand::thread_rng();

        slice.shuffle(&mut rng);

        let new_value = objective_function(&slice.to_vec(), &matrix);

        if best_value > new_value {
            best_value = new_value;
            best_perm = Vec::new();
            for x in slice {
                best_perm.push(*x);
            }
        }
    }

    return (best_value, best_perm);
}

fn reverse(perm: &mut Vec<usize>, x: usize, y: usize) {
    let mut i = x;
    let mut j = y;
    let mut temp: usize;

    while i < j {
        temp = perm[i];
        perm[i] = perm[j];
        perm[j] = temp;
        i = i + 1;
        j = j - 1;
    }
}

pub fn two_opt(matrix: &Matrix) -> (u64, Vec<usize>) {
    let (mut best_value, mut best_perm) = k_random(&matrix, matrix.n);
    let mut found_better = true;
    let mut best_i = 0;
    let mut best_j = 0;

    while found_better {
        found_better = false;
        for i in 0..matrix.n - 1 {
            for j in i + 1..matrix.n {
                reverse(&mut best_perm, i, j);
                let new_value = objective_function(&best_perm, &matrix);
                if new_value < best_value {
                    found_better = true;
                    best_i = i;
                    best_j = j;
                    best_value = new_value;
                }
                reverse(&mut best_perm, i, j);
            }
        }
        if found_better {
            reverse(&mut best_perm, best_i, best_j);
        }
    }

    return (best_value, best_perm);
}
