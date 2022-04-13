use super::alg;
use super::geo::Matrix;
use std::collections::VecDeque;
use std::time::Instant;

pub fn tabu_search(matrix: &Matrix) -> (u64, Vec<usize>) {
    let (mut best_value, mut best_perm) = alg::two_opt(&matrix);

    let tabu_list: VecDeque<u32> = VecDeque::with_capacity(13);

    let max_time = 100000;
    let mut last_change = 0;
    let start = Instant::now();

    while (start.elapsed().as_nanos() < max_time && last_change < 10 * &matrix.n) {
        last_change = last_change + 1;

    // Wygeneruj otoczenie
    // W otoczeniu wybierz najlepsze rozwiązanie, które nie jest zabronione

        let mut best_i = 0;
        let mut best_j = 0;

        let mut old_sums: Vec<u64> = vec![0; matrix.n];
        let mut new_sums: Vec<u64> = vec![0; matrix.n];

        let mut best_change = 0;

        // keeping sums of weights from 0 to i in order to calculate cost of path in constant time
        for i in 0..matrix.n - 1 {
            old_sums[i + 1] = old_sums[i] + matrix.get(best_perm[i], best_perm[i + 1]);
        }

        // keeping sums of weights from n-1 to i in order to calculate cost of reversed path in constant time
        for i in (0..&matrix.n - 1).rev() {
            new_sums[i] = new_sums[i + 1] + matrix.get(best_perm[i + 1], best_perm[i]);
        }

        for i in 0..matrix.n - 1 {
            for j in i + 1..matrix.n {
                let new_change = alg::change_value(&best_perm, &matrix, i, j, &old_sums, &new_sums);
                if new_change > best_change {
                    best_i = i;
                    best_j = j;
                    best_change = new_change;
                }
            }
        }

        alg::reverse(&mut best_perm, best_i, best_j);
        best_value = best_value - best_change;

    // Dodaj do listy tabu
        
    }

    return (best_value, best_perm);
}