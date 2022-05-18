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

fn crossing(father: &[usize], mother: &[usize], mode: usize) -> (Vec<usize>, Vec<usize>) {
    match mode {
        0 => half_crossing(father, mother),
        1 => order_crossing(father, mother),
        _ => partially_mapped_crossing(father, mother),
    }
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
#[pyfunction]
fn population_alg_no_threads_no_isles(
    matrix: &Matrix,           // matrix representing the graph
    gen_rand: bool,            // if false, use other metaheuristics
    gen_size: usize,           // size of one generation
    elite_num: usize,          // how many people are in elite (in the whole generation)
    cross_op: usize,           // 0 - HX, 1 - OX, >1 - PMX,
    swap_change: bool,         // swap or inverse in a mutation,
    size_of_tournament: usize, // IMPORTANT: if set to 0, then use roulette rule
    mut_chance: f64,           // chance to get mutated,
    max_time: f64,             // max time to stop algorithm,
) -> PyResult<(u64, Vec<usize>)> {
    // swap_change - true if swap, false if invert, no other options
    // if tabu_size == 0, then tabu_size = n
    let start = Instant::now();

    // INITIALIZATION PHASE
    //println!("INITIALIZATION");
    let n = matrix.n;
    let mut population = Vec::with_capacity(gen_size); // that is faster

    // using other metaheuristics, two opt and neighbours
    // at least half of them will be still randomized
    let mut meta_done = 0;
    if !gen_rand {
        let k = std::cmp::min(gen_size / 2, n);
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
        //println!("EVALUATION");
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

        // CHECK IF THERE IS STAGNATION, THEN INSERT A RANDOM INDIVIDUAL AT RANDOM IDX, ELSE CHANGE BEST PERM
        if new_best_idx == std::usize::MAX {
            let mut new_vec: Vec<usize> = (0..n).collect();
            new_vec.shuffle(&mut rand_thread);
            let rand_idx = rand_thread.gen_range(0..gen_size);
            population[rand_idx] = new_vec;
        } else {
            best_perm = population[new_best_idx].clone(); // best value already changed in the loop
        }

        // START CREATING NEW GENERATION
        let mut children = Vec::with_capacity(gen_size);

        // ELITARISM
        //println!("ELITARISM");
        ppbs.sort_by(|(a1, _), (b1, _)| b1.partial_cmp(a1).unwrap());
        for i in 0..elite_num {
            children.push(population[ppbs[i].1].clone());
        }

        // SELECTION AND CROSSING (IN ORDER NOT TO KEEP A VECTOR OF PARENTS)
        //println!("SELECTION");
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
                let (child1, child2) = crossing(&population[index1], &population[index2], cross_op);
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
                );
                children.push(child1);
                children.push(child2);
            }
        }

        // MUTATION
        //println!("MUTATION");
        population = children[0..gen_size].to_vec(); // CUTTING OFF LAST CHILD

        for individual in &mut population {
            let do_mutate = rand_thread.gen_range(0.0..1.0);
            if do_mutate < mut_chance {
                let i = rand_thread.gen_range(0..n);
                let j = rand_thread.gen_range(1..n);
                let j = (i + j) % n;
                if swap_change {
                    individual.swap(i, j);
                } else {
                    reverse(individual, i, j);
                }
            }
        }
    }

    //println!("AFTER LOOP");

    let mut best_perm_idx = 0;

    for (i, perm) in population.iter().enumerate() {
        let new_value = objective_function(perm, matrix);
        if new_value < best_value {
            best_perm_idx = i;
            best_value = new_value;
        }
    }

    if best_perm_idx != std::usize::MAX {
        best_perm = population[best_perm_idx].clone(); // value changed in loop
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
    //println!("INITIALIZATION");
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
        // Idea: one individual from each isle moves to another one and kills one individual
        if iteration % migration_freq == 0 {
            let mut order: Vec<usize> = (0..isles_num).collect();
            order.shuffle(&mut rand_thread);
            for i in 0..isles_num {
                let donor = rand_thread.gen_range(0..isle_size);
                let acceptor = rand_thread.gen_range(0..isle_size);
                isles[order[(i + 1) % isles_num]][acceptor] = isles[order[i]][donor].clone();
            }
        }

        // EVALUATION
        //println!("EVALUATION");
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
                ppbs.push(((1.0 / new_value as f64), i));
            }
            isles_ppbs.push(ppbs);
        }

        // CHECK IF THERE IS STAGNATION, THEN INSERT A RANDOM INDIVIDUAL AT RANDOM IDX, ELSE CHANGE BEST PERM
        if new_best_idx.0 == std::usize::MAX {
            for isle in &mut isles {
                let mut new_vec: Vec<usize> = (0..n).collect();
                new_vec.shuffle(&mut rand_thread);
                let rand_idx = rand_thread.gen_range(0..isle_size);
                isle[rand_idx] = new_vec;
            }
        } else {
            best_perm = isles[new_best_idx.0][new_best_idx.1].clone(); // best value already changed in the loop
        }

        // CREATING NEW GENERATION
        let mut isles_children = Vec::with_capacity(isles_num);

        // ELITARISM
        //println!("ELITARISM");
        for i in 0..isles_num {
            let mut children = Vec::with_capacity(isle_size);
            isles_ppbs[i].sort_by(|(a1, _), (b1, _)| b1.partial_cmp(a1).unwrap());
            for j in 0..elite_num {
                children.push(isles[i][isles_ppbs[i][j].1].clone());
            }
            isles_children.push(children);
        }

        // SELECTION AND CROSSING (IN ORDER NOT TO KEEP A VECTOR OF PARENTS)
        //println!("SELECTION");

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
                    let (child1, child2) = crossing(&isles[i][index1], &isles[i][index1], cross_op);
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
                    let (child1, child2) =
                        crossing(&isles[i][best_parent1], &isles[i][best_parent2], cross_op);
                    isles_children[i].push(child1);
                    isles_children[i].push(child2);
                }
            }
        }

        // MUTATION
        //println!("MUTATION");
        for x in 0..isles_num {
            isles[x] = isles_children[x][0..isle_size].to_vec();
            for k in 0..isle_size {
                let do_mutate = rand_thread.gen_range(0.0..1.0);
                if do_mutate < mut_chance {
                    let i = rand_thread.gen_range(0..n);
                    let j = rand_thread.gen_range(1..n);
                    let j = (i + j) % n;
                    if swap_change {
                        isles[x][k].swap(i, j);
                    } else {
                        reverse(&mut isles[x][k], i, j);
                    }
                }
            }
        }
    }

    // AFTER LOOP

    let mut best_perm_idx = (std::usize::MAX, std::usize::MAX);

    for (i, isle) in isles.iter().enumerate() {
        for (j, perm) in isle.iter().enumerate() {
            let new_value = objective_function(perm, matrix);
            if new_value < best_value {
                best_perm_idx = (i, j);
                best_value = new_value;
            }
        }
    }

    if best_perm_idx.0 != std::usize::MAX {
        best_perm = isles[best_perm_idx.0][best_perm_idx.1].clone(); // value changed in loop
    }

    Ok((best_value, best_perm))
}
