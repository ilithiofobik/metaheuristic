use super::alg;
use super::gen;
use super::geo::Matrix;
use super::io;
use std::cmp::{max, min};
use std::fs;
use std::time::Instant;

fn tour_name(filename: &str) -> String {
    let pre = "data/euc_2d/";
    let post = ".opt.tour";
    let tour = pre.to_owned() + &filename;
    let tour2 = tour + &post;

    return tour2;
}

fn tsp_name(filename: &str) -> String {
    let pre = "data/euc_2d/";
    let post = ".tsp";
    let tour = pre.to_owned() + &filename;
    let tour2 = tour + &post;

    return tour2;
}

pub fn test_tsplib() {
    let files = [
        "a280", "berlin52", "ch130", "ch150", "eil51", "eil76", "lin105", "pcb442", "pr76",
        "rd100", "st70", "tsp225",
    ]; //pr1002

    let mut result_all = String::new();
    let mut result_prd = String::new();
    let mut result_avg_time = String::new();
    let mut result_max_time = String::new();

    for file in files {
        let tour_name = tour_name(&file);
        let best_perm = io::read_tour(&tour_name);

        match best_perm {
            Ok(bst_p) => {
                let tsp_name = tsp_name(&file);
                let matrix = io::read_file(&tsp_name);

                match matrix {
                    Ok(m) => {
                        let best_val = alg::objective_function(&bst_p, &m) as f64;
                        let mut avg_prd_k_rand = 0;
                        let mut avg_time_k_rand = 0;
                        let mut max_time_k_rand = 0;
                        let mut avg_prd_ext_neigh = 0;
                        let mut avg_time_ext_neig = 0;
                        let mut max_time_ext_neig = 0;
                        let mut avg_prd_2_opt = 0;
                        let mut avg_time_2_opt = 0;
                        let mut max_time_2_opt = 0;

                        for _ in 0..10 {
                            let start = Instant::now();
                            let (val, _) = alg::k_random(&m, 1000);
                            let duration = start.elapsed().as_nanos();
                            avg_prd_k_rand = avg_prd_k_rand + val;
                            avg_time_k_rand = avg_time_k_rand + duration;
                            max_time_k_rand = max(max_time_k_rand, duration);

                            let start = Instant::now();
                            let (val, _) = alg::extended_nearest_neighbor(&m);
                            let duration = start.elapsed().as_nanos();
                            avg_prd_ext_neigh = avg_prd_ext_neigh + val;
                            avg_time_ext_neig = avg_time_ext_neig + duration;
                            max_time_ext_neig = max(max_time_ext_neig, duration);

                            let start = Instant::now();
                            let (val, _) = alg::two_opt(&m);
                            let duration = start.elapsed().as_nanos();
                            avg_prd_2_opt = avg_prd_2_opt + val;
                            avg_time_2_opt = avg_time_2_opt + duration;
                            max_time_2_opt = max(max_time_2_opt, duration);
                        }

                        let favg_prd_k_rand = 10.0 * (avg_prd_k_rand as f64 - best_val) / best_val;
                        let favg_prd_ext_neigh =
                            10.0 * (avg_prd_ext_neigh as f64 - best_val) / best_val;
                        let favg_prd_2_opt = 10.0 * (avg_prd_2_opt as f64 - best_val) / best_val;

                        let result = format!(
                            "{};{:.3};{:.3};{:.3};{};{};{};{};{};{}\n",
                            m.n,
                            favg_prd_k_rand,
                            favg_prd_ext_neigh,
                            favg_prd_2_opt,
                            avg_time_k_rand,
                            avg_time_ext_neig,
                            avg_time_2_opt,
                            max_time_k_rand,
                            max_time_ext_neig,
                            max_time_2_opt
                        );
                        result_all.push_str(&result);
                        let result = format!(
                            "{};{:.3};{:.3};{:.3}\n",
                            m.n, favg_prd_k_rand, favg_prd_ext_neigh, favg_prd_2_opt
                        );
                        result_prd.push_str(&result);
                        let result = format!(
                            "{};{};{};{}\n",
                            m.n, avg_time_k_rand, avg_time_ext_neig, avg_time_2_opt,
                        );
                        result_avg_time.push_str(&result);
                        let result = format!(
                            "{};{};{};{}\n",
                            m.n, max_time_k_rand, max_time_ext_neig, max_time_2_opt
                        );
                        result_max_time.push_str(&result);
                    }
                    Err(_) => panic!("Some error with {}", tsp_name),
                }
            }
            Err(_) => panic!("Some error with {}", tour_name),
        }
    }
    fs::write("results/tsp_lib_test.txt", result_all).expect("Unable to write file");
    fs::write("results/tsp_lib_test_prd.txt", result_prd).expect("Unable to write file");
    fs::write("results/tsp_lib_test_avg_time.txt", result_avg_time).expect("Unable to write file");
    fs::write("results/tsp_lib_test_max_time.txt", result_max_time).expect("Unable to write file");
}

pub fn test_k_random(gen: fn(usize) -> Matrix, filename: &str) {
    let mut result_prd = String::new();
    for nn in 1..11 {
        let n = 100 * nn;
        let mut avg10 = 0;
        let mut avg100 = 0;
        let mut avg1000 = 0;
        let mut ffavg10 = 0.0;
        let mut ffavg100 = 0.0;
        let mut ffavg1000 = 0.0;
        for _ in 0..10 {
            let m = gen(n);
            let mut min_val = std::u64::MAX;
            for _ in 0..10 {
                let (val10, _) = alg::k_random(&m, 10);
                let (val100, _) = alg::k_random(&m, 100);
                let (val1000, _) = alg::k_random(&m, 1000);
                avg10 = avg10 + val10;
                avg100 = avg100 + val100;
                avg1000 = avg1000 + val1000;
                min_val = min(val10, min_val);
                min_val = min(val100, min_val);
                min_val = min(val1000, min_val);
            }
            let fmin = min_val as f64;
            let favg10 = 100.0 * (avg10 as f64 - fmin) / fmin;
            let favg100 = 100.0 * (avg100 as f64 - fmin) / fmin;
            let favg1000 = 100.0 * (avg1000 as f64 - fmin) / fmin;
            ffavg10 = ffavg10 + favg10;
            ffavg100 = ffavg100 + favg100;
            ffavg1000 = ffavg1000 + favg1000;
        }

        let result = format!(
            "{};{};{};{}\n",
            n,
            ffavg10 / 10.0,
            ffavg100 / 10.0,
            ffavg1000 / 10.0
        );
        result_prd.push_str(&result);
    }
    fs::write(filename, result_prd).expect("Unable to write file");
}

pub fn test_time_optimality() {
    let mut result_time = String::from("n;k_rand;neigh;ext-neigh;opt\n");
    let mut result_optimality = String::from("n;k_rand;neigh;ext-neigh;opt\n");

    for n in 1..30 {
        let m = gen::create_euclid(n * 10);

        let start_k_rand = Instant::now();
        let (k_rand_best_val, _k_rand_best_perm) = alg::k_random(&m, m.n);
        let duration_k_rand = start_k_rand.elapsed().as_nanos();

        let start_neigh = Instant::now();
        let (neigh_best_val, _neigh_best_perm) = alg::nearest_neighbor(&m);
        let duration_neigh = start_neigh.elapsed().as_nanos();

        let start_ext_neigh = Instant::now();
        let (ext_neigh_best_val, _ext_neigh_best_perm) = alg::extended_nearest_neighbor(&m);
        let duration_ext_neigh = start_ext_neigh.elapsed().as_nanos();

        let start_opt = Instant::now();
        let (opt_best_val, _opt_best_perm) = alg::two_opt(&m);
        let duration_opt = start_opt.elapsed().as_nanos();

        // Optymalność
        let min_val = min(
            k_rand_best_val,
            min(neigh_best_val, min(ext_neigh_best_val, opt_best_val)),
        );
        // println!("{}", min_val);
        // println!("{}, {}, {}, {}", k_rand_best_val, neigh_best_val, ext_neigh_best_val, opt_best_val);

        let optimality_k_rand =
            ((k_rand_best_val as f64 - min_val as f64) / min_val as f64) * 100.0;
        let optimality_neigh = ((neigh_best_val as f64 - min_val as f64) / min_val as f64) * 100.0;
        let optimality_ext_neigh =
            ((ext_neigh_best_val as f64 - min_val as f64) / min_val as f64) * 100.0;
        let optimality_opt = ((opt_best_val as f64 - min_val as f64) / min_val as f64) * 100.0;

        // Uzupełnianie wynikowych stringów
        let new_results_time = format!(
            "{};{};{};{};{}\n",
            n * 10,
            duration_k_rand,
            duration_neigh,
            duration_ext_neigh,
            duration_opt
        );
        result_time.push_str(&new_results_time);

        let new_results_optimality = format!(
            "{};{};{};{};{}\n",
            n * 10,
            optimality_k_rand,
            optimality_neigh,
            optimality_ext_neigh,
            optimality_opt
        );
        result_optimality.push_str(&new_results_optimality);
    }

    println!("{}", &result_time);
    println!("{}", &result_optimality);
}
