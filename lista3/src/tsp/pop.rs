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
                father1 = (father1 + 1) % n;
            }
            while used2[mother[mother2]] {
                mother2 = (mother2 + 1) % n;
            }
            used1[father[father1]] = true;
            used2[mother[mother2]] = true;
            child1.push(father[father1]);
            child2.push(mother[mother2]);
        } else {
            while used1[mother[mother1]] {
                mother1 = (mother1 + 1) % n;
            }
            while used2[father[father2]] {
                father2 = (father2 + 1) % n;
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
    let half = n / 2;

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
    let half = n / 2;

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
        if ma_to_fa.contains_key(&mother[i]) {
            child1.push(ma_to_fa[&mother[i]]);
        } else {
            child1.push(mother[i]);
        }
        if fa_to_ma.contains_key(&father[i]) {
            child2.push(fa_to_ma[&father[i]]);
        } else {
            child2.push(father[i]);
        }
    }

    (child1, child2)
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
#[pyfunction]
fn population_alg_no_threads_no_isles(
    matrix: &mut Matrix,       // matrix representing the graph
    gen_rand: bool,            // if false, use other metaheuristics
    gen_size: usize,           // size of one generation
    elite_num: usize,          // how many people are in elite (in the whole generation)
    cross_op: usize,           // 0 - HX, 1 - OX, 2 - PMX,
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
        let mut vec: Vec<usize> = (0..n).collect();
        vec.shuffle(&mut rand_thread);
        population.push(vec.to_vec());
    }

    if !gen_rand {
        // to keep the metaheuristics in random isles
        population.shuffle(&mut rand_thread);
    }

    while start.elapsed().as_secs_f64() < max_time {
        // EVALUATION
        let mut ppbs = Vec::with_capacity(gen_size);
        for (i, perm) in population.iter().enumerate() {
            // can be also used in tournament but mainly used in roullete
            ppbs.push((1.0 / (objective_function(perm, matrix) as f64), i));
        }

        // SELECTION
        let num_of_parents = (gen_size - elite_num) + ((gen_size - elite_num) % 2); // can be one extra
        let mut parents = Vec::with_capacity(num_of_parents);
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
                parents.push(population[index1].clone());
                parents.push(population[index2].clone());
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
                parents.push(population[best_parent1].clone());
                parents.push(population[best_parent2].clone());
            }
        }

        // CROSSING
        let mut children = Vec::with_capacity(gen_size);
        if cross_op == 0 {
            for i in 0..num_of_parents / 2 {
                let (child1, child2) = half_crossing(&parents[2 * i], &parents[2 * i + 1]);
                children.push(child1);
                children.push(child2);
            }
        } else if cross_op == 1 {
            for i in 0..num_of_parents / 2 {
                let (child1, child2) = order_crossing(&parents[2 * i], &parents[2 * i + 1]);
                children.push(child1);
                children.push(child2);
            }
        } else {
            for i in 0..num_of_parents / 2 {
                let (child1, child2) =
                    partially_mapped_crossing(&parents[2 * i], &parents[2 * i + 1]);
                children.push(child1);
                children.push(child2);
            }
        }

        // ELITARISM
        ppbs.sort_by(|(a1, _), (b1, _)| b1.partial_cmp(a1).unwrap());
        for i in 0..elite_num {
            children.push(population[ppbs[i].1].clone());
        }

        // MUTATION
        population = children;
        for k in 0..population.len() {
            let do_mutate = rand_thread.gen_range(0.0..1.0);
            if do_mutate < mut_chance {
                let i = rand_thread.gen_range(0..population.len());
                let j = rand_thread.gen_range(1..population.len());
                let j = (i + j) % population.len();
                if swap_change {
                    population[k].swap(i, j);
                } else {
                    reverse(&mut population[k], i, j);
                }
            }
        }
    }

    let mut best_value = std::u64::MAX;
    let mut best_perm_idx = 0;

    for (i, perm) in population.iter().enumerate() {
        let new_value = objective_function(perm, matrix);
        if new_value < best_value {
            best_perm_idx = i;
            best_value = new_value;
        }
    }

    Ok((best_value, population[best_perm_idx].clone()))
}
