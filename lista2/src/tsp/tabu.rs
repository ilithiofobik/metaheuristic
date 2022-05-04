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
fn tabu_search(
    matrix_base: &mut Matrix,
    tabu_size: usize,
    swap_change: bool,
    best_value: u64,
    best_perm: Vec<usize>,
) -> PyResult<(u64, Vec<usize>, f64)> {
    // swap_change - true if swap, false if invert, no other options
    // if tabu_size == 0, then tabu_size = n
    let start = Instant::now();
    let swap_change = Arc::new(swap_change);
    let n = matrix_base.n;
    let tabu_size = if tabu_size != 0 { tabu_size } else { n / 2 };

    let mut global_minimum_tabu_list: VecDeque<(usize, usize)> = VecDeque::with_capacity(tabu_size);
    let mut global_minimum_perm = best_perm.clone();

    let tabu_list: VecDeque<(usize, usize)> = VecDeque::with_capacity(tabu_size);
    let tabu_list = Arc::new(RwLock::new(tabu_list));
    let best_perm = Arc::new(RwLock::new(best_perm));

    let num_of_threads = 1;
    // let max_time = 10_000_000_000; //10s
    let mut last_change = 0;
    // let start = Instant::now();
    let matrix = Arc::new(matrix_base.clone());

    let mut returned = 0;

    let global_minimum_value = best_value;
    let global_minimum_value = Arc::new(RwLock::new(global_minimum_value));
    let best_value = Arc::new(RwLock::new(best_value));

    let mut counter = 0;

    println!("THREAD: {:?}", global_minimum_perm);


    while counter < n * 10 && last_change < n {
        counter += 1;

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
            let best_value_clone = Arc::clone(&best_value);
            let global_minimum_value_clone = Arc::clone(&global_minimum_value);
            let swap_change_clone = Arc::clone(&swap_change);

            threads.push(thread::spawn(move || {
                let mut best_i = 0;
                let mut best_j = 0;
                let mut best_change = std::i64::MIN;
                let tabu = tabu_clone.read().unwrap();
                let perm = best_perm_clone.read().unwrap();
                let value = best_value_clone.read().unwrap();
                let glo_min = global_minimum_value_clone.read().unwrap();
                let glo_change = *value - *glo_min;

                if *swap_change_clone {
                    let mut old_sums = vec![0; n];
                    let mut new_sums = vec![0; n];
                    // keeping sums of weights from 0 to i in order to calculate cost of path in constant time
                    for i in 0..n - 1 {
                        old_sums[i + 1] = old_sums[i] + matrix_clone.get(perm[i], perm[i + 1]);
                    }

                    // keeping sums of weights from n-1 to i in order to calculate cost of reversed path in constant time
                    for i in (0..n - 1).rev() {
                        new_sums[i] = new_sums[i + 1] + matrix_clone.get(perm[i + 1], perm[i]);
                    }

                    for i in (t..n - 1).step_by(num_of_threads) {
                        for j in i + 1..n {
                            let new_change = alg::change_value_invert(
                                &perm,
                                &matrix_clone,
                                i,
                                j,
                                &old_sums,
                                &new_sums,
                            );
                            if new_change > best_change
                                && (!((*tabu).contains(&(i, j))) || new_change > glo_change as i64)
                            {
                                best_i = i;
                                best_j = j;
                                best_change = new_change;
                            }
                        }
                    }
                } else {
                    for i in (t..n - 1).rev().step_by(num_of_threads) {
                        for j in i + 1..n {
                            let new_change = alg::change_value_swap(&perm, &matrix_clone, i, j);
                            if new_change > best_change
                                && (!((*tabu).contains(&(i, j))) || new_change > glo_change as i64)
                            {
                                best_i = i;
                                best_j = j;
                                best_change = new_change;
                            }
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
            // println!("Suggestion {}: {},{},{}", x, new_i, new_j, new_change);
            if global_best_change < new_change {
                global_best_i = new_i;
                global_best_j = new_j;
                global_best_change = new_change;
            }
        }

        if global_best_change == std::i64::MIN {
            break;
        }


        let mut perm = best_perm.write().unwrap();
        let mut value = best_value.write().unwrap();
        alg::reverse(&mut perm, global_best_i, global_best_j);
        *value = (*value as i64 - global_best_change) as u64;

        let mut global_minimum = global_minimum_value.write().unwrap();

        if *global_minimum > *value {
            *global_minimum = *value;
            global_minimum_tabu_list.clear();
            global_minimum_perm = perm.clone();
            returned = 0;
        }

        if global_minimum_tabu_list.len() < tabu_size {
            global_minimum_tabu_list.push_back((global_best_i, global_best_j));
        }

        let mut tabu = tabu_list.write().unwrap();
        if tabu.len() == tabu_size {
            tabu.pop_front();
        }
        tabu.push_back((global_best_i, global_best_j));

        // changed to better solution
        if global_best_change > 0 {
            last_change = 0;
        } else {
            last_change += 1;
        }

        // nawroty
        if last_change > tabu_size {
            global_minimum_tabu_list.pop_front();
            *tabu = global_minimum_tabu_list.clone();
            *value = *global_minimum;
            *perm = global_minimum_perm.clone();
        }

        if *perm == global_minimum_perm {
            returned += 1;

            if returned > tabu_size {
                break;
            }
        }

        println!("THREAD: {}: {}: {:?}", counter, global_best_change, best_perm);
    }

    let perm = best_perm.read().unwrap();
    let value = best_value.read().unwrap();
    let duration = start.elapsed().as_secs_f64();

    Ok((*value, perm.clone(), duration))
}

#[allow(dead_code)]
#[pyfunction]
fn tabu_search_no_threads(
    matrix: &mut Matrix,
    tabu_size: usize,
    swap_change: bool,
    best_value: u64,
    best_perm: Vec<usize>,
) -> PyResult<(u64, Vec<usize>, f64)> {
    // swap_change - true if swap, false if invert, no other options
    // if tabu_size == 0, then tabu_size = n
    let start = Instant::now();
    let mut best_perm = best_perm;
    let mut best_value = best_value;

    let n = matrix.n;
    let tabu_size = if tabu_size != 0 { tabu_size } else { n / 2 };

    let mut global_minimum_tabu_list: VecDeque<(usize, usize)> = VecDeque::with_capacity(tabu_size);
    let mut global_minimum_perm = best_perm.clone();
    let mut tabu_list: VecDeque<(usize, usize)> = VecDeque::with_capacity(tabu_size);
    // let max_time = 10_000_000_000; //10s
    let mut last_change = 0;
    // let start = Instant::now();
    let mut returned = 0;
    let mut global_minimum_value = best_value;

    let mut counter = 0;

    println!("NO_THREAD: {:?}", global_minimum_perm);

    while counter < 10 * n && last_change < n {
        counter += 1;

        let mut global_best_i = 0;
        let mut global_best_j = 0;
        let mut global_best_change = std::i64::MIN;
        let glo_change = best_value - global_minimum_value;

        if swap_change {
            let mut old_sums = vec![0; n];
            let mut new_sums = vec![0; n];
            // keeping sums of weights from 0 to i in order to calculate cost of path in constant time
            for i in 0..n - 1 {
                old_sums[i + 1] = old_sums[i] + matrix.get(best_perm[i], best_perm[i + 1]);
            }

            // keeping sums of weights from n-1 to i in order to calculate cost of reversed path in constant time
            for i in (0..n - 1).rev() {
                new_sums[i] = new_sums[i + 1] + matrix.get(best_perm[i + 1], best_perm[i]);
            }

            for i in 0..n - 1 {
                for j in i + 1..n {
                    let new_change =
                        alg::change_value_invert(&best_perm, matrix, i, j, &old_sums, &new_sums);
                    if new_change > global_best_change
                        && (!(tabu_list.contains(&(i, j))) || new_change > glo_change as i64)
                    {
                        global_best_i = i;
                        global_best_j = j;
                        global_best_change = new_change;
                    }
                }
            }
        } else {
            for i in 0..n - 1 {
                for j in i + 1..n {
                    let new_change = alg::change_value_swap(&best_perm, matrix, i, j);
                    if new_change > global_best_change
                        && (!(tabu_list.contains(&(i, j))) || new_change > glo_change as i64)
                    {
                        global_best_i = i;
                        global_best_j = j;
                        global_best_change = new_change;
                    }
                }
            }
        }

        if global_best_change == std::i64::MIN {
            break;
        }

        alg::reverse(&mut best_perm, global_best_i, global_best_j);
        best_value = (best_value as i64 - global_best_change) as u64;

        if global_minimum_value > best_value {
            global_minimum_value = best_value;
            global_minimum_tabu_list.clear();
            global_minimum_perm = best_perm.clone();
            returned = 0;
        }

        if global_minimum_tabu_list.len() < tabu_size {
            global_minimum_tabu_list.push_back((global_best_i, global_best_j));
        }

        if tabu_list.len() == tabu_size {
            tabu_list.pop_front();
        }
        tabu_list.push_back((global_best_i, global_best_j));

        // changed to better solution
        if global_best_change > 0 {
            last_change = 0;
        } else {
            last_change += 1;
        }

        // nawroty
        if last_change > tabu_size {
            global_minimum_tabu_list.pop_front();
            tabu_list = global_minimum_tabu_list.clone(); //.clone()?
            best_value = global_minimum_value;
            best_perm = global_minimum_perm.clone();
        }

        if best_perm == global_minimum_perm {
            returned += 1;

            if returned > tabu_size {
                break;
            }
        }

        println!("NO_THREAD: {}: {}: {:?}", counter, global_best_change, best_perm);
    }

    let duration = start.elapsed().as_secs_f64();
    Ok((global_minimum_value, global_minimum_perm, duration))
}
