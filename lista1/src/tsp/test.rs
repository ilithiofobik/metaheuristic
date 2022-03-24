use super::alg;
use super::gen;
use std::cmp::min;
use std::time::Instant;

pub fn test_time_optimality() {
    let mut result_time = String::from("n;k_rand;neigh;ext-neigh;opt\n");
    let mut result_optimality = String::from("n;k_rand;neigh;ext-neigh;opt\n");

    for n in 1..=30 {
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
