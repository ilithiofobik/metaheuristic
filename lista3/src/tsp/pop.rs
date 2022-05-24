extern crate rand;

use super::alg;
use super::alg::objective_function;
use super::alg::reverse;
use super::alg::two_opt;
use super::geo::Matrix;
use pyo3::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Instant;

fn half_crossing(father: &[usize], mother: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let n = father.len();
    let mut father1 = 0;
    let mut mother1 = 0;
    let mut father2 = 0;
    let mut mother2 = 0;
    let mut used1 = vec![false; n];
    let mut used2 = vec![false; n];
    let mut child1 = Vec::with_capacity(n);
    let mut child2 = Vec::with_capacity(n);
    let mut rand_thread = rand::thread_rng();

    for _ in 0..n {
        if rand_thread.gen_bool(0.5) {
            while used1[father[father1]] {
                father1 += 1;
            }
            while used2[mother[mother2]] {
                mother2 += 1;
            }
            used1[father[father1]] = true;
            used2[mother[mother2]] = true;
            child1.push(father[father1]);
            child2.push(mother[mother2]);
        } else {
            while used1[mother[mother1]] {
                mother1 += 1;
            }
            while used2[father[father2]] {
                father2 += 1;
            }
            used1[mother[mother1]] = true;
            used2[father[father2]] = true;
            child1.push(mother[mother1]);
            child2.push(father[father2]);
        }
    }

    (child1, child2)
}

fn order_crossing(father: &[usize], mother: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let n = father.len();
    let half = (n / 2) + 1;

    let mut father_iter = 0;
    let mut mother_iter = 0;
    let mut used1 = vec![false; n];
    let mut used2 = vec![false; n];
    let mut child1 = Vec::with_capacity(n);
    let mut child2 = Vec::with_capacity(n);

    for i in 0..half {
        child1.push(father[i]);
        used1[father[i]] = true;
        child2.push(mother[i]);
        used2[mother[i]] = true;
    }

    while father[father_iter] != mother[half - 1] {
        father_iter += 1;
    }
    while mother[mother_iter] != father[half - 1] {
        mother_iter += 1;
    }

    for _ in half..n {
        while used1[mother[mother_iter]] {
            mother_iter = (mother_iter + 1) % n;
        }
        while used2[father[father_iter]] {
            father_iter = (father_iter + 1) % n;
        }
        used1[mother[mother_iter]] = true;
        used2[father[father_iter]] = true;
        child1.push(mother[mother_iter]);
        child2.push(father[father_iter]);
    }

    (child1, child2)
}

fn partially_mapped_crossing(father: &[usize], mother: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let n = father.len();
    let half = (n / 2) + 1;

    let mut fa_to_ma = HashMap::new();
    let mut ma_to_fa = HashMap::new();
    let mut child1 = Vec::with_capacity(n);
    let mut child2 = Vec::with_capacity(n);

    for i in 0..half {
        child1.push(father[i]);
        child2.push(mother[i]);
        fa_to_ma.insert(father[i], mother[i]);
        ma_to_fa.insert(mother[i], father[i]);
    }

    for i in half..n {
        let mut new_mother = mother[i];
        while fa_to_ma.contains_key(&new_mother) {
            new_mother = fa_to_ma[&new_mother];
        }
        child1.push(new_mother);
        let mut new_father = father[i];
        while ma_to_fa.contains_key(&new_father) {
            new_father = ma_to_fa[&new_father];
        }
        child2.push(new_father);
    }

    (child1, child2)
}

fn cyclic_crossing(father: &[usize], mother: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let n = father.len();
    let mut child1 = vec![n; n];
    let mut child2 = vec![n; n];
    let mut father_idxs = vec![n; n];
    let mut mother_idxs = vec![n; n];

    for i in 0..n {
        father_idxs[father[i]] = i;
        mother_idxs[mother[i]] = i;
    }

    let mut curr_idx1 = 0;
    while child1[curr_idx1] == n {
        child1[curr_idx1] = father[curr_idx1];
        curr_idx1 = father_idxs[mother[curr_idx1]];
    }
    let mut curr_idx2 = 0;
    while child2[curr_idx2] == n {
        child2[curr_idx2] = mother[curr_idx2];
        curr_idx2 = mother_idxs[father[curr_idx2]];
    }
    for i in 0..n {
        if child1[i] == n {
            child1[i] = mother[i];
        }
        if child2[i] == n {
            child2[i] = father[i];
        }
    }

    (child1, child2)
}

// CHANGE: MUTATION INSIDE CROSSING
fn crossing(
    father: &[usize],
    mother: &[usize],
    mode: usize,
    swap_change: bool,
    mut_chance: f64,
) -> (Vec<usize>, Vec<usize>) {
    let (mut child1, mut child2) = match mode {
        0 => half_crossing(father, mother),
        1 => order_crossing(father, mother),
        2 => partially_mapped_crossing(father, mother),
        _ => cyclic_crossing(father, mother),
    };
    let mut rand_thread = rand::thread_rng();
    let n = child1.len();
    if rand_thread.gen_bool(mut_chance) {
        let i = rand_thread.gen_range(0..n);
        let j = (i + rand_thread.gen_range(1..n)) % n;
        if swap_change {
            child1.swap(i, j);
        } else {
            reverse(&mut child1, i, j);
        }
    }
    if rand_thread.gen_bool(mut_chance) {
        let i = rand_thread.gen_range(0..n);
        let j = (i + rand_thread.gen_range(1..n)) % n;
        if swap_change {
            child2.swap(i, j);
        } else {
            reverse(&mut child2, i, j);
        }
    }
    (child1, child2)
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
#[pyfunction]
fn population_alg_no_threads_no_isles(
    matrix: &Matrix,           // matrix representing the graph
    gen_rand: bool,            // if false, use other metaheuristics
    gen_size: usize,           // size of one generation
    elite_num: usize,          // how many people are in elite (in the whole generation)
    cross_op: usize,           // 0 - HX, 1 - OX, 2 - CX, >2 - PMX,
    swap_change: bool,         // swap or inverse in a mutation,
    size_of_tournament: usize, // IMPORTANT: if set to 0, then use roulette rule
    mut_chance: f64,           // chance to get mutated,
    max_time: f64,             // max time to stop algorithm,
) -> PyResult<(u64, Vec<usize>)> {
    // swap_change - true if swap, false if invert, no other options
    // if tabu_size == 0, then tabu_size = n
    let start = Instant::now();

    // INITIALIZATION PHASE
    let n = matrix.n;
    let mut population = Vec::with_capacity(gen_size); // that is faster

    // using other metaheuristics, two opt and neighbours
    // at least 3/4 of them will be still randomized
    let mut meta_done = 0;
    if !gen_rand {
        let k = std::cmp::min(gen_size / 4, n);
        for i in 0..k {
            let (_, neigh_vec) = alg::nearest_neighbor_count(matrix, i);
            population.push(neigh_vec);
        }
        let (_, two_opt_perm) = two_opt(matrix, true);
        population.push(two_opt_perm);
        meta_done = k + 1;
    }

    let mut rand_thread = rand::thread_rng();

    for _ in meta_done..gen_size {
        let mut new_vec: Vec<usize> = (0..n).collect();
        new_vec.shuffle(&mut rand_thread);
        population.push(new_vec);
    }

    let mut best_value = std::u64::MAX;
    let mut best_perm = Vec::new();

    while start.elapsed().as_secs_f64() < max_time {
        // EVALUATION
        let mut ppbs = Vec::with_capacity(gen_size);
        let mut new_best_idx = std::usize::MAX;
        for (i, perm) in population.iter().enumerate() {
            // can be also used in tournament but mainly used in roullete
            let new_value = objective_function(perm, matrix);
            if best_value > new_value {
                best_value = new_value;
                new_best_idx = i;
            }
            ppbs.push(((1.0 / new_value as f64), i));
        }

        // CHECK IF THERE IS STAGNATION, THEN RANDOMIZE 1/10 OF GENERATION
        if new_best_idx == std::usize::MAX {
            for individual in &mut population {
                if rand_thread.gen_bool(0.1) {
                    let mut new_vec: Vec<usize> = (0..n).collect();
                    new_vec.shuffle(&mut rand_thread);
                    *individual = new_vec;
                }
            }
        } else {
            best_perm = population[new_best_idx].clone(); // best value already changed in the loop
        }

        // START CREATING NEW GENERATION
        let mut children = Vec::with_capacity(gen_size);

        // ELITARISM
        ppbs.sort_by(|(a1, _), (b1, _)| b1.partial_cmp(a1).unwrap());
        for i in 0..elite_num {
            children.push(population[ppbs[i].1].clone());
        }

        // SELECTION AND CROSSING (IN ORDER NOT TO KEEP A VECTOR OF PARENTS)
        let num_of_parents = (gen_size - elite_num) + ((gen_size - elite_num) % 2); // can be one extra

        // use roulette
        if size_of_tournament == 0 {
            let ppbs_sum: f64 = ppbs.iter().map(|(a, _)| a).sum();
            for _ in 0..num_of_parents / 2 {
                let rand_parent1 = rand_thread.gen_range(0.0..ppbs_sum);
                let mut index1 = 0;
                let mut ppbs_subsum = ppbs[0].0;
                while ppbs_subsum < rand_parent1 {
                    index1 += 1;
                    ppbs_subsum += ppbs[index1].0;
                }
                let mut index2 = index1;
                while index1 == index2 {
                    let rand_parent2 = rand_thread.gen_range(0.0..ppbs_sum);
                    index2 = 0;
                    let mut ppbs_subsum = ppbs[0].0;
                    while ppbs_subsum < rand_parent2 {
                        index2 += 1;
                        ppbs_subsum += ppbs[index2].0;
                    }
                }
                let (child1, child2) = crossing(
                    &population[index1],
                    &population[index2],
                    cross_op,
                    swap_change,
                    mut_chance,
                );
                children.push(child1);
                children.push(child2);
            }
        } else {
            for _ in 0..num_of_parents / 2 {
                let mut best_parent1 = 0;
                let mut best_parent2 = 0;
                let mut best_value1 = 0.0; // because we are considering 1/x
                let mut best_value2 = 0.0; // two best in tournament are parents
                for _ in 0..size_of_tournament {
                    let rand_parent = rand_thread.gen_range(0..gen_size);
                    if ppbs[rand_parent].0 > best_value1 {
                        best_value2 = best_value1;
                        best_parent2 = best_parent1;
                        best_value1 = ppbs[rand_parent].0;
                        best_parent1 = rand_parent;
                    } else if ppbs[rand_parent].0 > best_value2 {
                        best_value2 = ppbs[rand_parent].0;
                        best_parent2 = rand_parent;
                    }
                }

                let (child1, child2) = crossing(
                    &population[best_parent1],
                    &population[best_parent2],
                    cross_op,
                    swap_change,
                    mut_chance,
                );
                children.push(child1);
                children.push(child2);
            }
        }

        population = children[0..gen_size].to_vec(); // CUTTING OFF LAST CHILD
    }

    Ok((best_value, best_perm))
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
#[pyfunction]
fn population_alg_no_threads_isles(
    matrix: &Matrix,           // matrix representing the graph
    gen_rand: bool,            // if false, use other metaheuristics
    isle_size: usize,          // size of one isle
    elite_num: usize,          // how many people are in elite (in the whole generation)
    cross_op: usize,           // 0 - HX, 1 - OX, 2 - PMX,
    swap_change: bool,         // swap or inverse in a mutation,
    size_of_tournament: usize, // IMPORTANT: if set to 0, then use roulette rule
    mut_chance: f64,           // chance to get mutated,
    max_time: f64,             // max time to stop algorithm,
    isles_num: usize,          // number of isles
    migration_freq: usize,     // how often should isles communicate
) -> PyResult<(u64, Vec<usize>)> {
    // swap_change - true if swap, false if invert, no other options
    // if tabu_size == 0, then tabu_size = n
    let start = Instant::now();

    // INITIALIZATION PHASE
    let n = matrix.n;
    let mut isles = Vec::with_capacity(isles_num); // that is faster

    let mut rand_thread = rand::thread_rng();
    let mut first_isle = Vec::with_capacity(isle_size);
    if gen_rand {
        // all is completely random
        for _ in 0..isles_num {
            let mut i_isle = Vec::with_capacity(isle_size);
            for _ in 0..isle_size {
                let mut new_vec: Vec<usize> = (0..n).collect();
                new_vec.shuffle(&mut rand_thread);
                i_isle.push(new_vec);
            }
            isles.push(i_isle);
        }
    } else {
        // first isle has two_opt, the others have random nearest neighbour
        let (_, two_opt_perm) = two_opt(matrix, true);
        first_isle.push(two_opt_perm);
        for _ in 1..isle_size {
            let mut new_vec: Vec<usize> = (0..n).collect();
            new_vec.shuffle(&mut rand_thread);
            first_isle.push(new_vec);
        }
        isles.push(first_isle);
        for _ in 1..isles_num {
            let mut i_isle = Vec::with_capacity(isle_size);
            let i = rand_thread.gen_range(0..n);
            let (_, nnc) = alg::nearest_neighbor_count(matrix, i);
            i_isle.push(nnc);
            for _ in 1..isle_size {
                let mut new_vec: Vec<usize> = (0..n).collect();
                new_vec.shuffle(&mut rand_thread);
                i_isle.push(new_vec);
            }
            isles.push(i_isle);
        }
    }

    let mut best_value = std::u64::MAX;
    let mut best_perm = Vec::new();

    let mut iteration = 0;

    while start.elapsed().as_secs_f64() < max_time {
        iteration += 1;
        // MIGRATION BETWEEN ISLES
        // Idea: every isle exchanges one individual per one indiviual with every isle
        if iteration % migration_freq == 0 {
            for a in 0..isles_num - 1 {
                for b in a..isles_num {
                    let a_idx = rand_thread.gen_range(0..isle_size);
                    let b_idx = rand_thread.gen_range(0..isle_size);
                    let temp = isles[a][a_idx].clone();
                    isles[a][a_idx] = isles[b][b_idx].clone();
                    isles[b][b_idx] = temp;
                }
            }
        }

        // EVALUATION
        let mut isles_ppbs = Vec::with_capacity(isles_num);
        let mut new_best_idx = (std::usize::MAX, std::usize::MAX);
        for (i, isle) in isles.iter().enumerate() {
            let mut ppbs = Vec::with_capacity(isle_size);
            for (j, perm) in isle.iter().enumerate() {
                // can be also used in tournament but mainly used in roullete
                let new_value = objective_function(perm, matrix);
                if best_value > new_value {
                    best_value = new_value;
                    new_best_idx = (i, j);
                }
                ppbs.push(((1.0 / new_value as f64), j));
            }
            isles_ppbs.push(ppbs);
        }

        // CHECK IF THERE IS STAGNATION, THEN RANDOMIZE 1/10 OF GENERATION
        if new_best_idx.0 == std::usize::MAX {
            for isle in &mut isles {
                for individual in isle.iter_mut() {
                    if rand_thread.gen_bool(0.1) {
                        let mut new_vec: Vec<usize> = (0..n).collect();
                        new_vec.shuffle(&mut rand_thread);
                        *individual = new_vec;
                    }
                }
            }
        } else {
            best_perm = isles[new_best_idx.0][new_best_idx.1].clone(); // best value already changed in the loop
        }

        // CREATING NEW GENERATION
        let mut isles_children = Vec::with_capacity(isles_num);

        // ELITARISM
        for i in 0..isles_num {
            let mut children = Vec::with_capacity(isle_size);
            isles_ppbs[i].sort_by(|(a1, _), (b1, _)| b1.partial_cmp(a1).unwrap());
            for j in 0..elite_num {
                children.push(isles[i][isles_ppbs[i][j].1].clone());
            }
            isles_children.push(children);
        }

        // SELECTION AND CROSSING (IN ORDER NOT TO KEEP A VECTOR OF PARENTS)

        let num_of_parents = (isle_size - elite_num) + ((isle_size - elite_num) % 2); // can be one extra

        for i in 0..isles_num {
            // use roulette
            if size_of_tournament == 0 {
                let ppbs_sum: f64 = isles_ppbs[i].iter().map(|(a, _)| a).sum();
                for _ in 0..num_of_parents / 2 {
                    let rand_parent1 = rand_thread.gen_range(0.0..ppbs_sum);
                    let mut index1 = 0;
                    let mut ppbs_subsum = isles_ppbs[i][0].0;
                    while ppbs_subsum < rand_parent1 {
                        index1 += 1;
                        ppbs_subsum += isles_ppbs[i][index1].0;
                    }
                    let mut index2 = index1;
                    while index1 == index2 {
                        let rand_parent2 = rand_thread.gen_range(0.0..ppbs_sum);
                        index2 = 0;
                        let mut ppbs_subsum = isles_ppbs[i][0].0;
                        while ppbs_subsum < rand_parent2 {
                            index2 += 1;
                            ppbs_subsum += isles_ppbs[i][index2].0;
                        }
                    }
                    let (child1, child2) = crossing(
                        &isles[i][index1],
                        &isles[i][index1],
                        cross_op,
                        swap_change,
                        mut_chance,
                    );
                    isles_children[i].push(child1);
                    isles_children[i].push(child2);
                }
            } else {
                for _ in 0..num_of_parents / 2 {
                    let mut best_parent1 = 0;
                    let mut best_parent2 = 0;
                    let mut best_value1 = 0.0; // because we are considering 1/x
                    let mut best_value2 = 0.0; // two best in tournament are parents
                    for _ in 0..size_of_tournament {
                        let rand_parent = rand_thread.gen_range(0..isle_size);
                        if isles_ppbs[i][rand_parent].0 > best_value1 {
                            best_value2 = best_value1;
                            best_parent2 = best_parent1;
                            best_value1 = isles_ppbs[i][rand_parent].0;
                            best_parent1 = rand_parent;
                        } else if isles_ppbs[i][rand_parent].0 > best_value2 {
                            best_value2 = isles_ppbs[i][rand_parent].0;
                            best_parent2 = rand_parent;
                        }
                    }
                    let (child1, child2) = crossing(
                        &isles[i][best_parent1],
                        &isles[i][best_parent2],
                        cross_op,
                        swap_change,
                        mut_chance,
                    );
                    isles_children[i].push(child1);
                    isles_children[i].push(child2);
                }
            }
        }

        for x in 0..isles_num {
            isles[x] = isles_children[x][0..isle_size].to_vec();
        }
    }

    Ok((best_value, best_perm))
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
#[pyfunction]
fn population_alg_threads_no_isles(
    base_matrix: &Matrix,      // matrix representing the graph
    gen_rand: bool,            // if false, use other metaheuristics
    gen_size: usize,           // size of one generation
    elite_num: usize,          // how many people are in elite (in the whole generation)
    cross_op: usize,           // 0 - HX, 1 - OX, >1 - PMX,
    swap_change: bool,         // swap or inverse in a mutation,
    size_of_tournament: usize, // IMPORTANT: if set to 0, then use roulette rule
    mut_chance: f64,           // chance to get mutated,
    max_time: f64,             // max time to stop algorithm,
    num_of_threads: usize,     // number of threads
) -> PyResult<(u64, Vec<usize>)> {
    // swap_change - true if swap, false if invert, no other options
    // if tabu_size == 0, then tabu_size = n
    let start = Instant::now();

    // INITIALIZATION PHASE
    let n = base_matrix.n; // base matrix left for the sake of main thread
    let mut population = Vec::with_capacity(gen_size); // that is faster
    let matrix = Arc::new(base_matrix.clone());

    // first individual
    if !gen_rand {
        let (_, two_opt_perm) = two_opt(base_matrix, true);
        population.push(two_opt_perm);
    } else {
        let mut rand_thread = rand::thread_rng();
        let mut new_vec: Vec<usize> = (0..n).collect();
        new_vec.shuffle(&mut rand_thread);
        population.push(new_vec);
    }

    let gen_rand = Arc::new(gen_rand);

    // using other metaheuristics, two opt and neighbours
    // at least 3/4 of them will be still randomized
    let mut threads = Vec::with_capacity(num_of_threads);
    let (tx, rx) = mpsc::channel();

    for t in 0..num_of_threads {
        let matrix_clone = Arc::clone(&matrix);
        let gen_rand_clone = Arc::clone(&gen_rand);
        let tx_t = tx.clone();

        threads.push(thread::spawn(move || {
            let mut meta_done = 0;

            if !*gen_rand_clone {
                let k = std::cmp::min(gen_size / 4, n);
                for i in (0..k).skip(t).step_by(num_of_threads) {
                    let (_, neigh_vec) = alg::nearest_neighbor_count(&matrix_clone, i);
                    tx_t.send(neigh_vec).unwrap();
                }
                meta_done = k + 1;
            }

            let mut rand_thread = rand::thread_rng();

            for _ in (meta_done..gen_size).skip(t).step_by(num_of_threads) {
                let mut new_vec: Vec<usize> = (0..n).collect();
                new_vec.shuffle(&mut rand_thread);
                tx_t.send(new_vec).unwrap();
            }
        }));
    }

    for _ in 1..gen_size {
        let new_indiviual = rx.recv().unwrap();
        population.push(new_indiviual);
    }

    threads.into_iter().for_each(|thread| {
        thread
            .join()
            .expect("The thread creating or execution failed!")
    });

    let mut best_value = std::u64::MAX;
    let mut best_perm = Vec::new();

    while start.elapsed().as_secs_f64() < max_time {
        // EVALUATION

        let mut ppbs = Vec::with_capacity(gen_size);
        let mut new_best_idx = std::usize::MAX;

        let mut threads = Vec::with_capacity(num_of_threads);
        let (tx, rx) = mpsc::channel();
        let (tx_best, rx_best) = mpsc::channel();
        let population_rw = Arc::new(RwLock::new(population));

        for t in 0..num_of_threads {
            let matrix_clone = Arc::clone(&matrix);
            let tx_t = tx.clone();
            let tx_best_t = tx_best.clone();
            let population_clone = Arc::clone(&population_rw);

            threads.push(thread::spawn(move || {
                let population_r = population_clone.read().unwrap();
                let mut thread_best_val = std::u64::MAX;
                let mut thread_best_i = std::usize::MAX;
                for (i, perm) in population_r
                    .iter()
                    .enumerate()
                    .skip(t)
                    .step_by(num_of_threads)
                {
                    // can be also used in tournament but mainly used in roullete
                    let new_value = objective_function(perm, &matrix_clone);
                    if new_value < thread_best_val {
                        thread_best_val = new_value;
                        thread_best_i = i;
                    }
                    tx_t.send(((1.0 / new_value as f64), i)).unwrap();
                }
                tx_best_t.send((thread_best_val, thread_best_i)).unwrap();
            }));
        }

        for _ in 0..gen_size {
            let pair = rx.recv().unwrap();
            ppbs.push(pair);
        }

        for _ in 0..num_of_threads {
            let (new_value, idx) = rx_best.recv().unwrap();
            if best_value > new_value {
                best_value = new_value;
                new_best_idx = idx;
            }
        }

        threads.into_iter().for_each(|thread| {
            thread
                .join()
                .expect("The thread creating or execution failed!")
        });

        // CHECK IF THERE IS STAGNATION, THEN RANDOMIZE 1/10 OF GENERATION
        if new_best_idx == std::usize::MAX {
            let mut rand_thread = rand::thread_rng();
            let mut population_w = population_rw.write().unwrap();
            for i in 0..gen_size {
                if rand_thread.gen_bool(0.1) {
                    let mut new_vec: Vec<usize> = (0..n).collect();
                    new_vec.shuffle(&mut rand_thread);
                    population_w[i] = new_vec;
                }
            }
        } else {
            let population_r = population_rw.read().unwrap();
            best_perm = population_r[new_best_idx].clone(); // best value already changed in the loop
        }

        // START CREATING NEW GENERATION
        let mut children = Vec::with_capacity(gen_size);

        // ELITARISM
        ppbs.sort_by(|(a1, _), (b1, _)| b1.partial_cmp(a1).unwrap());
        let population_r = population_rw.read().unwrap();
        for i in 0..elite_num {
            children.push(population_r[ppbs[i].1].clone());
        }

        // SELECTION AND CROSSING (IN ORDER NOT TO KEEP A VECTOR OF PARENTS)
        let num_of_parents = (gen_size - elite_num) + ((gen_size - elite_num) % 2); // can be one extra

        // use roulette
        let mut threads = Vec::with_capacity(num_of_threads);
        let (tx, rx) = mpsc::channel();
        let ppbs_arc = Arc::new(ppbs);

        for t in 0..num_of_threads {
            let tx_t = tx.clone();
            let population_clone = Arc::clone(&population_rw);
            let ppbs_clone = Arc::clone(&ppbs_arc);

            threads.push(thread::spawn(move || {
                let population_r = population_clone.read().unwrap();

                if size_of_tournament == 0 {
                    let mut rand_thread = rand::thread_rng();
                    let ppbs_sum: f64 = ppbs_clone.iter().map(|(a, _)| a).sum();
                    for _ in (t..num_of_parents / 2).step_by(num_of_threads) {
                        let rand_parent1 = rand_thread.gen_range(0.0..ppbs_sum);
                        let mut index1 = 0;
                        let mut ppbs_subsum = ppbs_clone[0].0;
                        while ppbs_subsum < rand_parent1 {
                            index1 += 1;
                            ppbs_subsum += ppbs_clone[index1].0;
                        }
                        let mut index2 = index1;
                        while index1 == index2 {
                            let rand_parent2 = rand_thread.gen_range(0.0..ppbs_sum);
                            index2 = 0;
                            let mut ppbs_subsum = ppbs_clone[0].0;
                            while ppbs_subsum < rand_parent2 {
                                index2 += 1;
                                ppbs_subsum += ppbs_clone[index2].0;
                            }
                        }
                        let (child1, child2) = crossing(
                            &population_r[index1],
                            &population_r[index2],
                            cross_op,
                            swap_change,
                            mut_chance,
                        );
                        tx_t.send((child1, child2)).unwrap();
                    }
                } else {
                    for _ in (t..num_of_parents / 2).step_by(num_of_threads) {
                        let mut rand_thread = rand::thread_rng();
                        let mut best_parent1 = 0;
                        let mut best_parent2 = 0;
                        let mut best_value1 = 0.0; // because we are considering 1/x
                        let mut best_value2 = 0.0; // two best in tournament are parents
                        for _ in 0..size_of_tournament {
                            let rand_parent = rand_thread.gen_range(0..gen_size);
                            if ppbs_clone[rand_parent].0 > best_value1 {
                                best_value2 = best_value1;
                                best_parent2 = best_parent1;
                                best_value1 = ppbs_clone[rand_parent].0;
                                best_parent1 = rand_parent;
                            } else if ppbs_clone[rand_parent].0 > best_value2 {
                                best_value2 = ppbs_clone[rand_parent].0;
                                best_parent2 = rand_parent;
                            }
                        }

                        let (child1, child2) = crossing(
                            &population_r[best_parent1],
                            &population_r[best_parent2],
                            cross_op,
                            swap_change,
                            mut_chance,
                        );
                        tx_t.send((child1, child2)).unwrap();
                    }
                }
            }));
        }

        for _ in 0..num_of_parents / 2 {
            let (child1, child2) = rx.recv().unwrap();
            children.push(child1);
            children.push(child2);
        }

        threads.into_iter().for_each(|thread| {
            thread
                .join()
                .expect("The thread creating or execution failed!")
        });

        population = children[0..gen_size].to_vec(); // CUTTING OFF LAST CHILD
    }

    Ok((best_value, best_perm))
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
#[pyfunction]
fn population_alg_threads_isles(
    base_matrix: &Matrix,      // matrix representing the graph
    gen_rand: bool,            // if false, use other metaheuristics
    isle_size: usize,          // size of one isle
    elite_num: usize,          // how many people are in elite (in the whole generation)
    cross_op: usize,           // 0 - HX, 1 - OX, 2 - PMX,
    swap_change: bool,         // swap or inverse in a mutation,
    size_of_tournament: usize, // IMPORTANT: if set to 0, then use roulette rule
    mut_chance: f64,           // chance to get mutated,
    max_time: f64,             // max time to stop algorithm,
    isles_num: usize,          // number of isles
    migration_freq: usize,     // how often should isles communicate
    num_of_threads: usize,     // number of threads
) -> PyResult<(u64, Vec<usize>)> {
    // swap_change - true if swap, false if invert, no other options
    // if tabu_size == 0, then tabu_size = n
    let start = Instant::now();

    // INITIALIZATION PHASE
    let n = base_matrix.n;
    let mut isles_rw = Vec::with_capacity(isles_num);
    for _ in 0..isles_num {
        isles_rw.push(RwLock::new(Vec::with_capacity(isle_size)));
    }
    let isles_rw = Arc::new(isles_rw);

    let matrix = Arc::new(base_matrix.clone());
    let gen_rand = Arc::new(gen_rand);
    let mut threads = Vec::with_capacity(num_of_threads);

    for t in 0..num_of_threads {
        let matrix_clone = Arc::clone(&matrix);
        let gen_rand_clone = Arc::clone(&gen_rand);
        let isles_clone = Arc::clone(&isles_rw);

        threads.push(thread::spawn(move || {
            let mut rand_thread = rand::thread_rng();
            if *gen_rand_clone {
                // all is completely random
                for i in (t..isles_num).step_by(num_of_threads) {
                    let mut i_isle = isles_clone[i].write().unwrap();
                    for _ in 0..isle_size {
                        let mut new_vec: Vec<usize> = (0..n).collect();
                        new_vec.shuffle(&mut rand_thread);
                        i_isle.push(new_vec);
                    }
                }
            } else {
                // first isle has two_opt, the others have random nearest neighbour
                for i in (t..isles_num).step_by(num_of_threads) {
                    let mut i_isle = isles_clone[i].write().unwrap();
                    let j = rand_thread.gen_range(0..n);
                    if i == 0 {
                        let (_, two_opt_perm) = two_opt(&matrix_clone, true);
                        i_isle.push(two_opt_perm);
                    } else {
                        let (_, nnc) = alg::nearest_neighbor_count(&matrix_clone, j);
                        i_isle.push(nnc);
                    }
                    for _ in 1..isle_size {
                        let mut new_vec: Vec<usize> = (0..n).collect();
                        new_vec.shuffle(&mut rand_thread);
                        i_isle.push(new_vec);
                    }
                }
            }
        }));
    }

    threads.into_iter().for_each(|thread| {
        thread
            .join()
            .expect("The thread creating or execution failed!")
    });

    let mut best_value = std::u64::MAX;
    let mut best_perm = Vec::new();
    let mut iteration = 0;

    while start.elapsed().as_secs_f64() < max_time {
        iteration += 1;
        // MIGRATION BETWEEN ISLES
        // Idea: every isle exchanges one individual per one indiviual with every isle
        if iteration % migration_freq == 0 {
            let mut rand_thread = rand::thread_rng();
            for a in 0..isles_num - 1 {
                let mut isle_a = isles_rw[a].write().unwrap();
                for b in a + 1..isles_num {
                    let mut isle_b = isles_rw[b].write().unwrap();
                    let a_idx = rand_thread.gen_range(0..isle_size);
                    let b_idx = rand_thread.gen_range(0..isle_size);
                    let temp = isle_a[a_idx].clone();
                    isle_a[a_idx] = isle_b[b_idx].clone();
                    isle_b[b_idx] = temp;
                }
            }
        }

        let mut isles_ppbs_rw = Vec::with_capacity(isles_num);
        for _ in 0..isles_num {
            isles_ppbs_rw.push(RwLock::new(Vec::with_capacity(isle_size)));
        }
        let isles_ppbs_rw = Arc::new(isles_ppbs_rw);

        let mut new_best_idx = (std::usize::MAX, std::usize::MAX);

        let mut threads = Vec::with_capacity(num_of_threads);
        let (tx_best, rx_best) = mpsc::channel();

        for t in 0..num_of_threads {
            let matrix_clone = Arc::clone(&matrix);
            let tx_best_t = tx_best.clone();
            let isles_clone = Arc::clone(&isles_rw);
            let isles_ppbs_clone = Arc::clone(&isles_ppbs_rw);

            threads.push(thread::spawn(move || {
                let mut thread_best_val = std::u64::MAX;
                let mut thread_best_ij = (std::usize::MAX, std::usize::MAX);

                for i in (t..isles_num).step_by(num_of_threads) {
                    let isle = isles_clone[i].read().unwrap();
                    let mut ppbs = isles_ppbs_clone[i].write().unwrap();
                    for (j, perm) in isle.iter().enumerate() {
                        // can be also used in tournament but mainly used in roullete
                        let new_value = objective_function(perm, &matrix_clone);
                        if thread_best_val > new_value {
                            thread_best_val = new_value;
                            thread_best_ij = (i, j);
                        }
                        ppbs.push(((1.0 / new_value as f64), j));
                    }
                }
                tx_best_t.send((thread_best_val, thread_best_ij)).unwrap();
            }));
        }

        for _ in 0..num_of_threads {
            let (new_value, idxs) = rx_best.recv().unwrap();
            if best_value > new_value {
                best_value = new_value;
                new_best_idx = idxs;
            }
        }

        threads.into_iter().for_each(|thread| {
            thread
                .join()
                .expect("The thread creating or execution failed!")
        });

        // CHECK IF THERE IS STAGNATION, THEN RANDOMIZE 1/10 OF GENERATION
        if new_best_idx.0 == std::usize::MAX {
            let mut rand_thread = rand::thread_rng();
            for i in 0..isles_num {
                let mut isle = isles_rw[i].write().unwrap();
                for individual in isle.iter_mut() {
                    if rand_thread.gen_bool(0.1) {
                        let mut new_vec: Vec<usize> = (0..n).collect();
                        new_vec.shuffle(&mut rand_thread);
                        *individual = new_vec;
                    }
                }
            }
        } else {
            let isle_i = isles_rw[new_best_idx.0].read().unwrap();
            best_perm = isle_i[new_best_idx.1].clone(); // best value already changed in the loop
        }

        // CREATING NEW GENERATION
        let mut isles_children = Vec::with_capacity(isles_num);

        // ELITARISM
        for i in 0..isles_num {
            let mut children = Vec::with_capacity(isle_size);
            let mut isles_ppbs_i = isles_ppbs_rw[i].write().unwrap();
            isles_ppbs_i.sort_by(|(a1, _), (b1, _)| b1.partial_cmp(a1).unwrap());
            let isle_i = isles_rw[i].read().unwrap();
            for j in 0..elite_num {
                children.push(isle_i[isles_ppbs_i[j].1].clone());
            }
            isles_children.push(children);
        }

        // SELECTION AND CROSSING (IN ORDER NOT TO KEEP A VECTOR OF PARENTS)

        let num_of_parents = (isle_size - elite_num) + ((isle_size - elite_num) % 2); // can be one extra
        let mut threads = Vec::with_capacity(num_of_threads);
        let (tx, rx) = mpsc::channel();

        for t in 0..num_of_threads {
            let tx_t = tx.clone();
            let isles_clone = Arc::clone(&isles_rw);
            let isles_ppbs_clone = Arc::clone(&isles_ppbs_rw);

            threads.push(thread::spawn(move || {
                for i in (t..isles_num).step_by(num_of_threads) {
                    let isle_i = isles_clone[i].read().unwrap();
                    let isle_ppbs_i = isles_ppbs_clone[i].read().unwrap();
                    let mut children = Vec::with_capacity(2 * num_of_parents);
                    let mut rand_thread = rand::thread_rng();
                    // use roulette
                    if size_of_tournament == 0 {
                        let ppbs_sum: f64 = isle_ppbs_i.iter().map(|(a, _)| a).sum();
                        for _ in 0..num_of_parents / 2 {
                            let rand_parent1 = rand_thread.gen_range(0.0..ppbs_sum);
                            let mut index1 = 0;
                            let mut ppbs_subsum = isle_ppbs_i[0].0;
                            while ppbs_subsum < rand_parent1 {
                                index1 += 1;
                                ppbs_subsum += isle_ppbs_i[index1].0;
                            }
                            let mut index2 = index1;
                            while index1 == index2 {
                                let rand_parent2 = rand_thread.gen_range(0.0..ppbs_sum);
                                index2 = 0;
                                let mut ppbs_subsum = isle_ppbs_i[0].0;
                                while ppbs_subsum < rand_parent2 {
                                    index2 += 1;
                                    ppbs_subsum += isle_ppbs_i[index2].0;
                                }
                            }
                            let (child1, child2) = crossing(
                                &isle_i[index1],
                                &isle_i[index1],
                                cross_op,
                                swap_change,
                                mut_chance,
                            );
                            // CHANGE - IMMEDIATE MUTATION OF CHILDREN
                            children.push(child1);
                            children.push(child2);
                        }
                    } else {
                        for _ in 0..num_of_parents / 2 {
                            let mut best_parent1 = 0;
                            let mut best_parent2 = 0;
                            let mut best_value1 = 0.0; // because we are considering 1/x
                            let mut best_value2 = 0.0; // two best in tournament are parents
                            for _ in 0..size_of_tournament {
                                let rand_parent = rand_thread.gen_range(0..isle_size);
                                if isle_ppbs_i[rand_parent].0 > best_value1 {
                                    best_value2 = best_value1;
                                    best_parent2 = best_parent1;
                                    best_value1 = isle_ppbs_i[rand_parent].0;
                                    best_parent1 = rand_parent;
                                } else if isle_ppbs_i[rand_parent].0 > best_value2 {
                                    best_value2 = isle_ppbs_i[rand_parent].0;
                                    best_parent2 = rand_parent;
                                }
                            }
                            let (child1, child2) = crossing(
                                &isle_i[best_parent1],
                                &isle_i[best_parent2],
                                cross_op,
                                swap_change,
                                mut_chance,
                            );
                            children.push(child1);
                            children.push(child2);
                        }
                    }
                    tx_t.send(children).unwrap();
                }
            }));
        }

        for elite_children in isles_children.iter_mut() {
            let mut children = rx.recv().unwrap();
            elite_children.append(&mut children);
        }

        threads.into_iter().for_each(|thread| {
            thread
                .join()
                .expect("The thread creating or execution failed!")
        });

        for x in 0..isles_num {
            let mut isle_x = isles_rw[x].write().unwrap();
            *isle_x = isles_children[x][0..isle_size].to_vec();
        }
    }

    Ok((best_value, best_perm))
}
