use super::alg;
use super::geo::Matrix;
use num_cpus;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

#[pyfunction]
fn tabu_search(matrix_base: &mut Matrix, tabu_size: usize) -> PyResult<(u64, Vec<usize>)> {
    let (mut best_value, mut best_perm) = alg::two_opt(&matrix_base, true);
    let n = matrix_base.n;
    let mut best_perm_arc = Arc::new(best_perm.clone());
    let num_of_threads = num_cpus::get();
    let mut tabu_list: VecDeque<Vec<usize>> = VecDeque::with_capacity(tabu_size);
    let max_time = 1_000_000_000; //1s
    let mut last_change = 0;
    let start = Instant::now();
    let matrix = Arc::new(matrix_base.clone());

    while start.elapsed().as_nanos() < max_time && last_change < 10 * n {
        last_change = last_change + 1;

        let mut global_best_i = 0;
        let mut global_best_j = 0;
        let mut global_best_change = 0;
        let mut found_better = false;

        let mut threads = Vec::with_capacity(num_of_threads);
        let (tx, rx) = mpsc::channel();

        for t in 0..num_of_threads {
            let best_perm_clone = Arc::clone(&best_perm_arc);
            let matrix_clone = Arc::clone(&matrix);
            let step = num_of_threads.clone();
            let tx_t = tx.clone();

            threads.push(thread::spawn(move || {
                let mut best_i = 0;
                let mut best_j = 0;
                let mut best_change = 0;
                for i in (t..n - 1).step_by(step) {
                    for j in i + 1..n {
                        let new_change =
                            alg::change_value_swap(&best_perm_clone, &matrix_clone, i, j);
                        if new_change > best_change {
                            best_i = i;
                            best_j = j;
                            best_change = new_change;
                        }
                    }
                }
                tx_t.send((best_i, best_j, best_change)).unwrap();
            }));
        }

        threads.into_iter().for_each(|thread| {
            thread
                .join()
                .expect("The thread creating or execution failed!")
        });

        for _ in 0..num_of_threads {
            let (new_i, new_j, new_change) = rx.recv().unwrap();
            if global_best_change < new_change {
                found_better = true;
                global_best_i = new_i;
                global_best_j = new_j;
                global_best_change = new_change;
            }
        }

        if found_better {
            alg::reverse(&mut best_perm, global_best_i, global_best_j);
            best_perm_arc = Arc::new(best_perm.clone());
            best_value = best_value - global_best_change;
            if tabu_list.len() == tabu_size {
                tabu_list.pop_front();
            } 
            tabu_list.push_back(best_perm.clone());
        }
    }

    return Ok((best_value, best_perm));
}
