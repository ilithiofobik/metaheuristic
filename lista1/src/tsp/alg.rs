extern crate rand;

use super::geo::Matrix;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn objective_function(permutation: &Vec<usize>, matrix: &Matrix) -> u64 {
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

    return nearest_neighbor_count(matrix, start_vertex);
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

    return (best_value, best_perm);
}

fn nearest_neighbor_count(matrix: &Matrix, first_vertex: usize) -> (u64, Vec<usize>) {
    let mut current_vertex = first_vertex;
    let mut unvisited: Vec<usize> = (0..matrix.n).collect();

    let mut final_perm: Vec<usize> = Vec::new();
    let mut final_value: u64 = 0;

    let index = unvisited.iter().position(|x| *x == current_vertex).unwrap();
    unvisited.remove(index);
    final_perm.push(current_vertex as usize);

    while !unvisited.is_empty() {
        let mut closest_town = matrix.n;
        let mut best_value = std::u64::MAX;

        for town in unvisited.iter() {
            if matrix.get(current_vertex, *town) < best_value {
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

    final_value = final_value + matrix.get(current_vertex, first_vertex);

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

fn change_value(
    perm: &Vec<usize>,
    matrix: &Matrix,
    first: usize,
    last: usize,
    old_sums: &Vec<u64>,
    new_sums: &Vec<u64>,
) -> u64 {
    let n = matrix.n;

    let prev = (first + n - 1) % n;
    let succ = (last + 1) % n;

    let mut old_val = {
        if succ != first {
            matrix.get(perm[prev], perm[first]) + matrix.get(perm[last], perm[succ])
        } else {
            matrix.get(perm[last], perm[first])
        }
    };

    let mut new_val = {
        if succ != first {
            matrix.get(perm[prev], perm[last]) + matrix.get(perm[first], perm[succ])
        } else {
            matrix.get(perm[first], perm[last])
        }
    };

    old_val = old_val + old_sums[last] - old_sums[first];
    new_val = new_val + new_sums[first] - new_sums[last];

    if new_val < old_val {
        return old_val - new_val;
    }
    return 0;
}

// approx_type = true -> extended_nearest_neighbor
// approx_type = false -> 1000-random
pub fn two_opt(matrix: &Matrix, approx_type: bool) -> (u64, Vec<usize>) {
    let (mut best_value, mut best_perm) = if approx_type {
            extended_nearest_neighbor(&matrix)
        } else {
            k_random(&matrix, 1000)
        };
    let n = matrix.n;
    let mut found_better = true;
    let mut best_change;
    let mut best_i = 0;
    let mut best_j = 0;

    let mut old_sums: Vec<u64> = vec![0; n];
    let mut new_sums: Vec<u64> = vec![0; n];

    while found_better {
        found_better = false;
        best_change = 0;

        // keeping sums of weights from 0 to i in order to calculate cost of path in constant time
        for i in 0..n - 1 {
            old_sums[i + 1] = old_sums[i] + matrix.get(best_perm[i], best_perm[i + 1]);
        }

        // keeping sums of weights from n-1 to i in order to calculate cost of reversed path in constant time
        for i in (0..n - 1).rev() {
            new_sums[i] = new_sums[i + 1] + matrix.get(best_perm[i + 1], best_perm[i]);
        }

        for i in 0..matrix.n - 1 {
            for j in i + 1..matrix.n {
                let new_change = change_value(&best_perm, &matrix, i, j, &old_sums, &new_sums);
                if new_change > best_change {
                    found_better = true;
                    best_i = i;
                    best_j = j;
                    best_change = new_change;
                }
            }
        }
        if found_better {
            reverse(&mut best_perm, best_i, best_j);
            best_value = best_value - best_change;
        }
    }

    return (best_value, best_perm);
}