use super::alg;
use super::geo::Matrix;
use num_cpus;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Instant;

#[pyfunction]
fn tabu_search(matrix_base: &mut Matrix, tabu_size: usize) -> PyResult<(u64, Vec<usize>)> {
    let (mut best_value, best_perm) = alg::two_opt(matrix_base, true);
    let mut tabu_list: VecDeque<Vec<usize>> = VecDeque::with_capacity(tabu_size);
    tabu_list.push_back(best_perm.clone());
    let tabu_list = Arc::new(RwLock::new(tabu_list));
    let best_perm = Arc::new(RwLock::new(best_perm));

    let n = matrix_base.n;
    let num_of_threads = num_cpus::get();
    let max_time = 1_000_000_000; //1s
    let mut last_change = 0;
    let start = Instant::now();
    let matrix = Arc::new(matrix_base.clone());


    let mut returned = 0;

    let mut global_minimum_value = std::i64::MAX;
    let global_minimum_value = Arc::new(RwLock::new(global_minimum_value));

    let mut global_minimum_tabu_list: VecDeque<Vec<usize>> = VecDeque::with_capacity(tabu_size);
    let mut global_minimum_perm = best_perm.clone();

    while start.elapsed().as_nanos() < max_time && last_change < n {
        let mut global_best_i = 0;
        let mut global_best_j = 0;
        let mut global_best_change = std::i64::MIN;

        let mut threads = Vec::with_capacity(num_of_threads);
        let (tx, rx) = mpsc::channel();

        for t in 0..num_of_threads {
            let matrix_clone = Arc::clone(&matrix);
            let tx_t = tx.clone();
            let tabu_clone = Arc::clone(&tabu_list);
            let best_perm_clone = Arc::clone(&best_perm);

            threads.push(thread::spawn(move || {
                let mut best_i = 0;
                let mut best_j = 0;
                let mut best_change = std::i64::MIN;
                let tabu = tabu_clone.read().unwrap();
                let perm = best_perm_clone.read().unwrap();

                for i in (t..n - 1).step_by(num_of_threads) {
                    for j in i + 1..n {
                        let new_change = alg::change_value_swap(&perm, &matrix_clone, i, j);
                        if new_change > best_change && ((best_value as i64 - new_change) < global_minimum_value.read().unwrap() || !alg::perm_on_tabu(&tabu, &perm, i, j, n)) {
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
                global_best_i = new_i;
                global_best_j = new_j;
                global_best_change = new_change;
            }
        }

        let mut perm = best_perm.write().unwrap();
        alg::reverse(&mut perm, global_best_i, global_best_j);
        best_value = (best_value as i64 - global_best_change) as u64;

        let mut global_minimum = global_minimum_value.write().unwrap();

        if (global_minimum < best_value) {
            global_minimum = best_value;
            global_minimum_tabu_list.clear();
            global_minimum_perm = perm.clone();
            returned = 0;
        }

        if (global_tabu.len() < tabu_size) {
            global_tabu.push_back(perm.clone());
        }

        let mut tabu = tabu_list.write().unwrap();
        if tabu.len() == tabu_size {
            tabu.pop_front();
        }
        tabu.push_back(perm.clone());

        // changed to better solution
        if global_best_change > 0 {
            last_change = 0;
        } else {
            last_change += 1;
        }

        if (last_change > tabu_size) {
            tabu = global_minimum_tabu_list; //.clone()?
            best_value = global_minimum;
            best_perm = global_minimum_perm.clone();
        }

        if (best_value == global_minimum) { //czy może porównywać same permutacje, ale jak, skoro to różne obiekty?
            returned = returned + 1;

            if (returned > 2) {
                break;
            }
        }
    }

    let perm = best_perm.read().unwrap();
    Ok((best_value, perm.clone()))
}
