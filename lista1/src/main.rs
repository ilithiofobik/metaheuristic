mod tsp;
use std::time::Instant;
use tsp::alg;
use tsp::io;

fn main() {
    let matrix = io::read_file("data/euc_2d/pr1002.tsp");

    match matrix {
        // using ref if another match matrix is needed
        Ok(ref m) => {
            let (k_rand_best_val, k_rand_best_perm) = alg::k_random(&m, m.n);

            println!("[RAND] Best value is {}", k_rand_best_val);
            println!("[RAND] Best perm is {:?}", k_rand_best_perm);

            let (neigh_best_val, neigh_best_perm) = alg::nearest_neighbor(&m);

            println!("[NEIGH] Best value is {}", neigh_best_val);
            println!("[NEIGH] Best perm is {:?}", neigh_best_perm);

            let (ext_neigh_best_val, ext_neigh_best_perm) = alg::extended_nearest_neighbor(&m);

            println!("[EXT-NEIGH] Best value is {}", ext_neigh_best_val);
            println!("[EXT-NEIGH] Best perm is {:?}", ext_neigh_best_perm);

            let start = Instant::now();
            let (opt_best_val, opt_best_perm) = alg::two_opt(&m);
            let duration = start.elapsed();

            println!("[2-OPT] Best value is {}", opt_best_val);
            println!("[2-OPT] Best perm is {:?}", opt_best_perm);

            println!("Time elapsed in 2-opt is: {:?}", duration);
        }
        Err(ref e) => println!("{}", e),
    }
}
