extern crate rand;

use super::geo::Matrix;
use rand::seq::SliceRandom;
use rand::Rng;

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

pub fn nearest_neighbor(matrix: &Matrix) -> (u64, Vec<usize>) {
    let mut rng = rand::thread_rng();
    let start_vertex = rng.gen_range(0..matrix.n);

    return nearest_neighbor_count(matrix, start_vertex)
}

pub fn extended_nearest_neighbor(matrix: &Matrix) -> (u64, Vec<usize>) {
    let mut best_perm: Vec<usize> = Vec::new();
    let mut best_value: u64 = std::u64::MAX;
    
    for start_vertex in 0..matrix.n {
        let (value, perm) = nearest_neighbor_count(matrix, start_vertex);
        if value < best_value {
            best_value = value;
            best_perm = perm;
        }
    }

    return (best_value, best_perm)
}

fn nearest_neighbor_count(matrix: &Matrix, mut current_vertex: usize) -> (u64, Vec<usize>) { 
    let mut unvisited: Vec<usize> = (0..matrix.n).collect();

    let mut final_perm: Vec<usize> = Vec::new();
    let mut final_value: u64 = 0;

    final_perm.push(current_vertex as usize);

    while !unvisited.is_empty() {
        let mut closest_town = matrix.n;
        let mut best_value = std::u64::MAX;

        for town in unvisited.iter() {
            if current_vertex != *town && matrix.get(current_vertex, *town) < best_value {
                best_value = matrix.get(current_vertex, *town);
                closest_town = *town;
            }
        }
        
        current_vertex = closest_town;
        final_value = final_value + best_value;
        final_perm.push(current_vertex as usize);

        let index = unvisited.iter().position(|x| *x == current_vertex).unwrap();
        unvisited.remove(index);
    }

    return (final_value, final_perm);
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
