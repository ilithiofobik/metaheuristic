use super::alg;
use super::gen;
use super::geo::Matrix;
use std::cmp::min;
use std::fs;
use std::time::Instant;

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
